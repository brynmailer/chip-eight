use std::{
    sync::{
        atomic,
        mpmc,
        Arc,
    },
    thread,
    time::Duration,
};

use ctrlc;
use rand::{self, Rng};

use crate::{
    config::Config,
    memory::Memory,
    timer::Timer,
    devices::{
        DeviceEvent,
        Display,
        Audio,
        Input,
    },
    instructions::Instruction,
};

pub struct ChipEight {
    // General configuration
    config: Config,

    // Stack containing addressess used to call/return from functions and subroutines.
    stack: Vec<usize>,

    // Program counter which points to the current instruction in memory.
    pc: usize,
    
    // 16 8-bit general purpose variable registers.
    v: [u8; 16],

    // Index register to point at locations in memory.
    i: usize,

    // Delay timer which is decremented at a rate of 60 Hz until it reaches 0. Can
    // be set and read.
    delay: Timer,

    // Sound timer. Functions like the delay timer, but additionally makes a beeping
    // sound when the value is not 0.
    sound: Timer,

    // Memory model
    memory: Memory,

    // Frame data used to determine what to draw to each pixel, as
    // well as whether drawing a pixel resulted in a collision.
    frame_buffer: Vec<bool>,

    // MPSC receiver for device events
    device_channel: (mpmc::Sender<DeviceEvent>, mpmc::Receiver<DeviceEvent>),

    // Devices
    display: Option<Box<dyn Display>>,
    audio: Option<Box<dyn Audio>>,
    input: Option<Box<dyn Input>>,
}

impl From<Config> for ChipEight {
    fn from(config: Config) -> Self {
        let (device_tx, device_rx) = mpmc::channel();

        Self {
            stack: Vec::new(),
            pc: config.memory.program_start, 
            v: [0; 16],
            i: 0,
            delay: Timer::new(None),
            sound: Timer::new(Some(device_tx.clone())),
            memory: config.memory.into(),
            frame_buffer: vec![false; config.display.width * config.display.height],
            device_channel: (device_tx, device_rx),
            display: config.display.into(),
            audio: config.audio.into(),
            input: config.input.into(),
            config,
        }
    }
}

impl ChipEight {
    pub fn play(&mut self, rom: &[u8]) {
        let running = Arc::new(atomic::AtomicBool::new(true));

        let running_clone = running.clone();
        ctrlc::set_handler(move || {
            println!("\nShutting down...");
            running_clone.store(false, atomic::Ordering::SeqCst);
        }).expect("Failed to set Ctrl-C handler");

        // Store default font
        self.memory.write_buf(self.config.memory.font_start, &self.config.memory.default_font).unwrap_or_else(|error| {
            panic!("Failed to load default font: {}", error);
        });

        // Store ROM
        self.memory.write_buf(self.config.memory.program_start, rom).unwrap_or_else(|error| {
            panic!("Failed to load rom: {}", error);
        });

        let (device_tx, device_rx) = &self.device_channel;
        let default_keys_pressed = [false; 16];
        let should_draw = Arc::new(atomic::AtomicBool::new(false));

        let running_clone = running.clone();
        let device_tx_clone = device_tx.clone();
        let should_draw_clone = should_draw.clone();
        thread::spawn(move || {
            let tick_duration = Duration::from_millis(1000 / 60); // 60hz
            
            while running_clone.load(atomic::Ordering::SeqCst) {
                device_tx_clone.send(DeviceEvent::Draw)
                    .expect("Failed to send draw event");

                let _ = should_draw_clone.compare_exchange(
                    false,
                    true,
                    atomic::Ordering::Acquire,
                    atomic::Ordering::SeqCst,
                );

                thread::sleep(tick_duration);
            }
        });

        while running.load(atomic::Ordering::SeqCst) {
            // Handle device events
            if let Ok(event) = device_rx.try_recv() {
                match event {
                    DeviceEvent::Draw => if let Some(display) = &mut self.display {
                        display.draw(&self.frame_buffer);
                    },
                    DeviceEvent::PlayTone => if let Some(audio) = &self.audio {
                        audio.play_tone();
                    },
                    DeviceEvent::StopTone => if let Some(audio) = &self.audio {
                        audio.stop_tone();
                    },
                }
            }

            let keys_pressed = if let Some(input) = &mut self.input {
                input.get_keys_down()
            } else {
                &default_keys_pressed
            };

            // Fetch and decode current instruction
            let parts = self.memory.read_buf(self.pc, 2).unwrap_or_else(|error| {
                panic!("Failed to fetch instruction: {}", error);
            });
            let opcode = ((parts[0] as u16) << 8) | parts[1] as u16;
            let instruction: Instruction = opcode
                .try_into()
                .unwrap_or_else(|error| {
                    panic!("Failed to parse instruction: {}", error);
                });

            // Increment PC to point to next instruction
            self.pc += 2;

            // Execute instruction
            match instruction {
                Instruction::Clear => {
                    self.frame_buffer.fill(false);

                    device_tx.send(DeviceEvent::Draw)
                        .expect("Failed to send draw event");
                },
                Instruction::Return => {
                    self.pc = self.stack.pop()
                        .expect("Failed to return from subroutine: stack is empty");
                },
                Instruction::Jump(addr) => self.pc = addr,
                Instruction::Call(addr) => {
                    self.stack.push(self.pc);
                    self.pc = addr;
                }
                Instruction::IfVxEq(reg, val) => {
                    if self.v[reg] == val {
                        self.pc += 2;
                    }
                },
                Instruction::IfVxNotEq(reg, val) => {
                    if self.v[reg] != val {
                        self.pc += 2;
                    }
                },
                Instruction::IfVxEqVy(reg_x, reg_y) => {
                    if self.v[reg_x] == self.v[reg_y] {
                        self.pc += 2;
                    }
                },
                Instruction::SetVx(reg, val) => self.v[reg] = val,
                Instruction::AddToVx(reg, val) => self.v[reg] = self.v[reg].wrapping_add(val),
                Instruction::SetVxToVy(reg_x, reg_y) => self.v[reg_x] = self.v[reg_y],
                Instruction::SetVxOrVy(reg_x, reg_y) => {
                    self.v[reg_x] |= self.v[reg_y];

                    if !self.config.quirks.skip_reset_vf {
                        self.v[0xF] = 0;
                    }
                },
                Instruction::SetVxAndVy(reg_x, reg_y) => {
                    self.v[reg_x] &= self.v[reg_y];

                    if !self.config.quirks.skip_reset_vf {
                        self.v[0xF] = 0;
                    }
                },
                Instruction::SetVxXorVy(reg_x, reg_y) => {
                    self.v[reg_x] ^= self.v[reg_y];

                    if !self.config.quirks.skip_reset_vf {
                        self.v[0xF] = 0;
                    }
                },
                Instruction::AddVyToVx(reg_x, reg_y) => {
                    let (result, overflowed) = self.v[reg_x].overflowing_add(self.v[reg_y]);
                    self.v[reg_x] = result;
                    self.v[0xF] = overflowed.into();
                },
                Instruction::SubVyFromVx(reg_x, reg_y) => {
                    let (result, overflowed) = self.v[reg_x].overflowing_sub(self.v[reg_y]);
                    self.v[reg_x] = result;
                    self.v[0xF] = (!overflowed).into();
                },
                Instruction::RightShiftVx(reg_x, reg_y) => {
                    let reg = if self.config.quirks.skip_shift_set {
                        reg_x
                    } else {
                        reg_y
                    };

                    let bit = self.v[reg] & 1;
                    self.v[reg_x] = self.v[reg] >> 1;
                    self.v[0xF] = bit;
                },
                Instruction::SubVxFromVy(reg_x, reg_y) => {
                    let (result, overflowed) = self.v[reg_y].overflowing_sub(self.v[reg_x]);
                    self.v[reg_x] = result;
                    self.v[0xF] = (!overflowed).into();
                },
                Instruction::LeftShiftVx(reg_x, reg_y) => {
                    let reg = if self.config.quirks.skip_shift_set {
                        reg_x
                    } else {
                        reg_y
                    };

                    let bit = (self.v[reg] >> 7) & 1;
                    self.v[reg_x] = self.v[reg] << 1;
                    self.v[0xF] = bit;
                },
                Instruction::IfVxNotEqVy(reg_x, reg_y) => {
                    if self.v[reg_x] != self.v[reg_y] {
                        self.pc += 2;
                    }
                },
                Instruction::SetI(addr) => self.i = addr,
                Instruction::JumpWithOffset(addr) => {
                    let offset = if self.config.quirks.jump_with_vx {
                        self.v[(addr >> 8) & 0xF]
                    } else {
                        self.v[0]
                    };

                    self.pc = addr + offset as usize;
                },
                Instruction::SetVxRand(reg, val) => self.v[reg] = rand::rng().random::<u8>() & val,
                Instruction::Draw(reg_x, reg_y, sprite_height) => {
                    let config = &self.config.display;

                    self.v[0xF] = 0;

                    let x = self.v[reg_x] as usize % config.width;
                    let y = self.v[reg_y] as usize % config.height;

                    let sprite = self.memory
                        .read_buf(self.i, sprite_height.into())
                        .unwrap_or_else(|error| {
                            panic!("Failed to fetch sprite: {}", error);
                        });

                    for (layer, byte) in sprite.iter().enumerate() {
                        let mut current_y = y + layer;
                        
                        if !self.config.quirks.wrap_sprites {
                            if current_y >= config.height {
                                break;
                            }
                        } else {
                            current_y = current_y % config.height;
                        }


                        for position in 0..8 {
                            let mut current_x = x + position;

                            if !self.config.quirks.wrap_sprites {
                                if current_x >= config.width {
                                    break;
                                }
                            } else {
                                current_x = current_x % config.width;
                            }

                            let bit = (byte.reverse_bits() >> position) & 1;

                            if bit != 0 {
                                if let Some(pixel) = self.frame_buffer.get_mut(current_y * config.width + current_x) {
                                    if *pixel {
                                        self.v[0xF] = 1;
                                    }

                                    *pixel = !*pixel;
                                }
                            }
                        }
                    }

                    if !self.config.quirks.skip_draw_wait {
                        loop {
                            if running.load(atomic::Ordering::SeqCst) {
                                if let Ok(true) = should_draw.compare_exchange(
                                    true,
                                    false,
                                    atomic::Ordering::Acquire,
                                    atomic::Ordering::Relaxed,
                                ) {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                },
                Instruction::IfKeyPressed(reg) => {
                    let key = self.v[reg] & 0xF;

                    if keys_pressed[key as usize] {
                        self.pc += 2;
                    }
                },
                Instruction::IfKeyNotPressed(reg) => {
                    let key = self.v[reg] & 0xF;

                    if !keys_pressed[key as usize] {
                        self.pc += 2;
                    }
                },
                Instruction::SetVxToDelay(reg) => self.v[reg] = self.delay.get(),
                Instruction::SetVxToKey(reg) => {
                    if let Some(input) = &mut self.input {
                        self.v[reg] = input.wait_for_key(running.clone());
                    } else {
                        panic!("Attempt to wait for key press failed: no available input peripheral");
                    }
                },
                Instruction::SetDelayToVx(reg) => self.delay.set(self.v[reg]),
                Instruction::SetSoundToVx(reg) => self.sound.set(self.v[reg]),
                Instruction::AddVxToI(reg) => self.i = self.i.wrapping_add(self.v[reg] as usize),
                Instruction::SetIToCharInVx(reg) => self.i = self.config.memory.font_start + ((self.v[reg] & 0xF) * 5) as usize,
                Instruction::StoreVxBCDAtI(reg) => {
                    let mut value = self.v[reg];
                    for index in (0..3).rev() {
                        self.memory.write_byte(self.i + index, value % 10)
                            .unwrap_or_else(|error| {
                                panic!("Failed to store BCD digit to memory: {}", error);
                            });
                        
                        value /= 10;
                    }
                },
                Instruction::VDump(reg) => {
                    for index in 0..=reg {
                        self.memory.write_byte(self.i + index, self.v[index])
                            .unwrap_or_else(|error| {
                                panic!("Failed to store value in register to memory: {}", error);
                            });
                    }

                    if !self.config.quirks.preserve_index {
                        self.i += reg + 1;
                    }
                },
                Instruction::VLoad(reg) => {
                    for index in 0..=reg {
                        let byte = self.memory.read_byte(self.i + index)
                            .unwrap_or_else(|error| {
                                panic!("Failed to load value from memory to register: {}", error);
                            });
                        self.v[index] = byte;
                    }

                    if !self.config.quirks.preserve_index {
                        self.i += reg + 1;
                    }
                },
            }

            // Sleep to ensure roughly correct clock speed
            thread::sleep(Duration::from_millis(1000 / self.config.clock_speed));
        }
    }
}

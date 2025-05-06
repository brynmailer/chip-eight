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
    config::Config, devices::{
        create_audio_device, create_display_device, create_input_device, Audio, DeviceEvent, Display, Input, Key
    }, memory::Memory, timer::Timer
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
            memory: Memory::new(config.memory.clone()),
            frame_buffer: vec![false; config.display.width * config.display.height],
            device_channel: (device_tx, device_rx),
            display: create_display_device(config.display.clone()),
            audio: create_audio_device(config.audio.clone()),
            input: create_input_device(config.input.clone()),
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

            let keys_down = if let Some(input) = &mut self.input {
                input.get_keys_down()
            } else {
                vec![]
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
            execute!(opcode, {
                00E0 => {
                    self.frame_buffer.fill(false);
                    device_tx.send(DeviceEvent::Draw)
                        .expect("Failed to send draw event");
                },
                00EE => {
                    self.pc = self.stack.pop()
                        .expect("Failed to return from subroutine: stack is empty");
                },
                1NNN => {
                    self.pc = $NNN;
                },
                2NNN => {
                    self.stack.push(self.pc);
                    self.pc = $NNN;
                },
                3XNN => {
                    if self.v[$X] == $NN {
                        self.pc += 2;
                    }
                },
                4XNN => {
                    if self.v[$X] != $NN {
                        self.pc += 2;
                    }
                },
                5XY0 => {
                    if self.v[$X] == self.v[$Y] {
                        self.pc += 2;
                    }
                },
                6XNN => {
                    self.v[$X] = $NN;
                },
                7XNN => {
                    self.v[$X] = self.v[$Y].wrapping_add($NN)
                },
                8XY0 => {
                    self.v[$X] = self.v[$Y];
                },
                8XY1 => {
                    self.v[$X] |= self.v[$Y];

                    if !self.config.quirks.skip_reset_vf {
                        self.v[0xF] = 0;
                    }
                },
                8XY2 => {
                    self.v[$X] &= self.v[$Y];

                    if !self.config.quirks.skip_reset_vf {
                        self.v[0xF] = 0;
                    }
                },
                8XY3 => {
                    self.v[$X] ^= self.v[$Y];

                    if !self.config.quirks.skip_reset_vf {
                        self.v[0xF] = 0;
                    }
                },
                8XY4 => {
                    let (result, overflowed) = self.v[$X].overflowing_add(self.v[$Y]);
                    self.v[$X] = result;
                    self.v[0xF] = overflowed.into();
                },
                8XY5 => {
                    let (result, overflowed) = self.v[$X].overflowing_sub(self.v[$Y]);
                    self.v[$X] = result;
                    self.v[0xF] = (!overflowed).into();
                },
                8XY6 => {
                    let reg = if self.config.quirks.skip_shift_set {
                        $X
                    } else {
                        $Y
                    };

                    let bit = self.v[reg] & 1;
                    self.v[$X] = self.v[reg] >> 1;
                    self.v[0xF] = bit;
                },
                8XY7 => {
                    let (result, overflowed) = self.v[$Y].overflowing_sub(self.v[$X]);
                    self.v[$X] = result;
                    self.v[0xF] = (!overflowed).into();
                },
                8XYE => {
                    let reg = if self.config.quirks.skip_shift_set {
                        $X
                    } else {
                        $Y
                    };

                    let bit = (self.v[reg] >> 7) & 1;
                    self.v[$X] = self.v[reg] << 1;
                    self.v[0xF] = bit;
                },
                9XY0 => {
                    if self.v[$X] != self.v[$Y] {
                        self.pc += 2;
                    }
                },
                ANNN => {
                    self.i = $NNN;
                },
                BNNN => {
                    let offset = if self.config.quirks.jump_with_vx {
                        self.v[($NNN >> 8) & 0xF]
                    } else {
                        self.v[0]
                    };

                    self.pc = $NNN + offset as usize;
                },
                CXNN => {
                    self.v[$X] = rand::rng().random::<u8>() & $NN;
                },
                DXYN => {
                    let config = &self.config.display;

                    self.v[0xF] = 0;

                    let x = self.v[$X] as usize % config.width;
                    let y = self.v[$Y] as usize % config.height;

                    let sprite = self.memory
                        .read_buf(self.i, $N.into())
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
                EX9E => {
                    let key = self.v[$X] & 0xF;

                    if keys_down.contains(
                        &Key::try_from(key)
                            .expect("Attempted to check an invalid keycode")
                    ) {
                        self.pc += 2;
                    }
                },
                EXA1 => {
                    let key = self.v[$X] & 0xF;

                    if !keys_down.contains(
                        &Key::try_from(key)
                            .expect("Attempted to check an invalid keycode")
                    ) {
                        self.pc += 2;
                    }
                },
                FX07 => {
                    self.v[$X] = self.delay.get();
                },
                FX0A => {
                    if let Some(input) = &mut self.input {
                        if let [key, ..] = keys_down.as_slice() {
                            while running.load(atomic::Ordering::SeqCst) {
                                if !input.get_keys_down().contains(key) {
                                    break;
                                }
                            }

                            self.v[$X] = *key as u8;
                        } else {
                            self.pc -= 2;
                        }
                    } else {
                        panic!("Attempt to wait for key press failed: no available input peripheral");
                    }
                },
                FX15 => {
                    self.delay.set(self.v[$X]);
                },
                FX18 => {
                    self.sound.set(self.v[$X]);
                },
                FX1E => {
                    self.i = self.i.wrapping_add(self.v[$X] as usize);
                },
                FX29 => {
                    self.i = self.config.memory.font_start + ((self.v[$X] & 0xF) * 5) as usize;
                },
                FX33 => {
                    let mut value = self.v[$X];
                    for index in (0..3).rev() {
                        self.memory.write_byte(self.i + index, value % 10)
                            .unwrap_or_else(|error| {
                                panic!("Failed to store BCD digit to memory: {}", error);
                            });
                        
                        value /= 10;
                    }
                },
                FX55 => {
                    for index in 0..=$X {
                        self.memory.write_byte(self.i + index, self.v[index])
                            .unwrap_or_else(|error| {
                                panic!("Failed to store value in register to memory: {}", error);
                            });
                    }

                    if !self.config.quirks.preserve_index {
                        self.i += $X + 1;
                    }
                },
                FX65 => {
                    for index in 0..=$X {
                        let byte = self.memory.read_byte(self.i + index)
                            .unwrap_or_else(|error| {
                                panic!("Failed to load value from memory to register: {}", error);
                            });
                        self.v[index] = byte;
                    }

                    if !self.config.quirks.preserve_index {
                        self.i += $X + 1;
                    }
                },
            });

            // Sleep to ensure roughly correct clock speed
            thread::sleep(Duration::from_millis(1000 / self.config.clock_speed));
        }
    }
}

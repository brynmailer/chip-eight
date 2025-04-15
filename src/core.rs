mod timer;
mod memory;
mod interface;
mod instructions;

use std::sync::mpsc::{channel, Receiver};

use crate::config::DEFAULT_FONT;

use timer::Timer;
use memory::Memory;
use interface::InterfaceEvent;
use instructions::Instruction;

pub use interface::{
    Display,
    Input,
    Audio,
};

pub struct ChipEight {
    // Stack containing 16-bit addressess used to call/return from functions and subroutines.
    stack: Vec<u16>,

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

    memory: Memory,

    // MPSC receiver for interface events
    event_rx: Receiver<InterfaceEvent>,

    // Interfaces
    display: Option<Box<dyn Display>>,
    input: Option<Box<dyn Input>>,
    audio: Option<Box<dyn Audio>>,
}

impl ChipEight {
    pub fn new() -> Self {
        let (event_tx, event_rx) = channel();

        Self {
            stack: Vec::new(),
            // Program counter starts at 0x200 for compatibility with old CHIP-8 programs. Where
            // the first 512 bytes of memory were kept free for the interpreter and font data.
            pc: 0x200, 
            v: [0; 16],
            i: 0,
            delay: Timer::new(None),
            sound: Timer::new(Some(event_tx.clone())),
            memory: Memory::new(),
            event_rx,
            display: None,
            input: None,
            audio: None,
        }
    }

    pub fn set_display(&mut self, display: Box<dyn Display>) -> &mut Self {
        self.display = Some(display);
        self
    }

    pub fn set_input(&mut self, input: Box<dyn Input>) -> &mut Self {
        self.input = Some(input);
        self
    }

    pub fn set_audio(&mut self, audio: Box<dyn Audio>) -> &mut Self {
        self.audio = Some(audio);
        self
    }

    pub fn start(&mut self, rom: &[u8]) {
        // Load font
        self.memory.write_buf(0x050, &DEFAULT_FONT)
            .expect("Failed to load default font");

        // Load ROM from 0x200
        self.memory.write_buf(0x200, rom)
            .expect("Failed to load provided rom");

        loop {
            // Handle interface events
            if let Ok(event) = self.event_rx.try_recv() {
                match event {
                    InterfaceEvent::PlayTone => if let Some(audio) = &self.audio {
                        audio.play_tone();
                    },
                    InterfaceEvent::StopTone => if let Some(audio) = &self.audio {
                        audio.stop_tone();
                    },
                }
            }
            
            // Fetch and decode current instruction
            let parts = self.memory.read_buf(self.pc, 2).unwrap_or_else(|error| {
                eprintln!("Failed to fetch instruction: {}", error);
                std::process::exit(1);
            });
            let opcode = ((parts[0] as u16) << 8) | parts[1] as u16;
            let instruction: Instruction = opcode
                .try_into()
                .unwrap_or_else(|error| {
                    eprintln!("Failed to parse instruction: {}", error);
                    std::process::exit(1);
                });

            // Increment PC to point to next instruction
            self.pc += 2;
        }
    }
}

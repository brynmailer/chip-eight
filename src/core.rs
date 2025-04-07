mod timer;
mod memory;
mod interface;

use std::sync::Arc;

use crate::config::DEFAULT_FONT;

use timer::Timer;
use memory::{Memory, MemoryError};
use interface::Interface;

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

    interface: Arc<Interface>,
}

impl ChipEight {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            // Program counter starts at 0x200 for compatibility with old CHIP-8 programs. Where
            // the first 512 bytes of memory were kept free for the interpreter and font data.
            pc: 0x200, 
            v: [0; 16],
            i: 0,
            delay: Timer::new(),
            sound: Timer::new(),
            memory: Memory::new(),
            interface: Arc::new(Interface::new()),
        }
    }

    pub fn load_rom(mut self, rom: &[u8]) -> Result<Self, MemoryError> {
        // Load font
        self.memory.write_buf(0x050, &DEFAULT_FONT)?;

        // Load ROM from 0x200
        self.memory.write_buf(0x200, rom)?;

        Ok(self)
    }

    pub fn play(&mut self) {
        self.delay.start(None);
        let interface = Arc::clone(&self.interface);
        self.sound.start(Some(Box::new(move || interface.play_sound())));
    }
}

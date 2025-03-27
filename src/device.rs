mod cpu;
mod memory;

use self::cpu::CPU;
use self::memory::Memory;

pub struct ChipEight {
    cpu: CPU,
    memory: Memory,
}

impl ChipEight {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            memory: Memory::new(),
        }
    }
}

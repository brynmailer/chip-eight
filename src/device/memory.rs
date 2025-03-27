use std::{error::Error, fmt};

const MEMORY_SIZE: usize = 0x1000; // 4kB

#[derive(Debug)]
enum MemoryError {
    OutOfBounds(u16),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::OutOfBounds(addr) => write!(f, "attempt to write to memory failed: address {} out of bounds", addr),
        }
    }
}

impl Error for MemoryError {}

pub struct Memory {
    ram: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0; MEMORY_SIZE],
        }
    }

    pub fn write(
        &mut self,
        data: u8,
        addr: u16,
    ) -> Result<(), MemoryError> {
        if addr as usize > MEMORY_SIZE - 1 {
            Err(MemoryError::OutOfBounds(addr))
        } else {
            self.ram[addr as usize] = data;
            Ok(())
        }
    }
}

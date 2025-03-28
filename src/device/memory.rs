use std::{error::Error, fmt};

const MEMORY_SIZE: usize = 0x1000; // 4kB

#[derive(Debug)]
pub enum MemoryError {
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
    buf: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            buf: [0; MEMORY_SIZE],
        }
    }

    pub fn write(
        &mut self,
        data: u8,
        addr: u16,
    ) -> Result<(), MemoryError> {
        // This could be an issue. Not sure if this range should be
        // inclusive or exclusive...
        if (0x200..MEMORY_SIZE).contains(&(addr as usize)) {
            self.buf[addr as usize] = data;
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds(addr))
        }
    }

    pub fn write_buf(
        &mut self,
        data: &[u8],
        addr: u16,
    ) -> Result<(), MemoryError> {
        for (index, data) in data.iter().enumerate() {
            self.write(*data, addr + index as u16)?
        }

        Ok(())
    }
}

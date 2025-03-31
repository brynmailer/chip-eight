use std::{error::Error, fmt, usize};

const MEMORY_SIZE: usize = 0x1000; // 4kB

#[derive(Debug, PartialEq)]
pub enum MemoryError {
    OutOfBounds(usize),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::OutOfBounds(addr) => write!(f, "attempt to access memory at position {} failed: address out of bounds", addr),
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

    fn is_in_bounds(&self, addr: usize) -> bool {
        addr < self.buf.len()
    }

    pub fn read_byte(&self, addr: usize) -> Result<u8, MemoryError> {
        if self.is_in_bounds(addr) {
            Ok(self.buf[addr])
        } else {
            Err(MemoryError::OutOfBounds(addr))
        }
    }

    // TODO: Rewrite to ensure error is returned with the correct addr
    pub fn read_buf(
        &self,
        addr: usize,
        len: usize,
    ) -> Result<&[u8], MemoryError> {
        if self.is_in_bounds(addr + len) {
            Ok(&self.buf[addr..=len])
        } else {
            Err(MemoryError::OutOfBounds(addr))
        }
    }

    pub fn write_byte(
        &mut self,
        addr: usize,
        data: u8,
    ) -> Result<(), MemoryError> {
        if self.is_in_bounds(addr) {
            self.buf[addr] = data;
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds(addr))
        }
    }

    // TODO: Rewrite to ensure that nothing is written before ensuring
    // entire input data buffer falls within memory range
    pub fn write_buf(
        &mut self,
        addr: usize,
        data: &[u8],
    ) -> Result<(), MemoryError> {
        for (index, data) in data.iter().enumerate() {
            self.write_byte(addr + index, *data)?
        }

        Ok(())
    }
}

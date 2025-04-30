use std::{error::Error, fmt, usize};

use crate::config::MemoryConfig;

#[derive(Debug, PartialEq)]
pub enum MemoryError {
    AddrOutOfBounds(usize),
    RangeOutOfBounds(usize, usize),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::AddrOutOfBounds(addr) => write!(f, "attempt to access byte at {} failed: address out of bounds", addr),
            MemoryError::RangeOutOfBounds(addr, len) => write!(f, "attempt to access range from {} to {} failed: range out of bounds", addr, addr + (len - 1)),
        }
    }
}

impl Error for MemoryError {}

pub struct Memory {
    config: MemoryConfig,
    buffer: Vec<u8>,
}

impl From<MemoryConfig> for Memory {
    fn from(config: MemoryConfig) -> Self {
        Self {
            buffer: vec![0; config.length],
            config,
        }
    }
}

impl Memory {
    fn is_in_bounds(&self, addr: usize) -> bool {
        addr < self.buffer.len()
    }

    pub fn read_byte(&self, addr: usize) -> Result<u8, MemoryError> {
        if !self.is_in_bounds(addr) {
            return Err(MemoryError::AddrOutOfBounds(addr));
        }

        Ok(self.buffer[addr])
    }

    pub fn read_buf(
        &self,
        addr: usize,
        len: usize,
    ) -> Result<&[u8], MemoryError> {
        if len < 1 {
            return Ok(&[]);
        }

        if !self.is_in_bounds(addr + (len - 1)) {
            return Err(MemoryError::RangeOutOfBounds(addr, len));
        }

        Ok(&self.buffer[addr..addr + len])
    }

    pub fn write_byte(
        &mut self,
        addr: usize,
        data: u8,
    ) -> Result<(), MemoryError> {
        if !self.is_in_bounds(addr) {
            return Err(MemoryError::AddrOutOfBounds(addr));
        }

        self.buffer[addr] = data;
        Ok(())
    }

    pub fn write_buf(
        &mut self,
        addr: usize,
        data: &[u8],
    ) -> Result<(), MemoryError> {
        if data.len() < 1 {
            return Ok(());
        }

        if !self.is_in_bounds(addr + (data.len() - 1)) {
            return Err(MemoryError::RangeOutOfBounds(addr, data.len()));
        }

        self.buffer[addr..(addr + data.len())].copy_from_slice(data);
        Ok(())
    }
}

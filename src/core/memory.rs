use std::{error::Error, fmt, usize};

use crate::config::MEMORY_SIZE;

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
        if !self.is_in_bounds(addr) {
            return Err(MemoryError::AddrOutOfBounds(addr));
        }

        Ok(self.buf[addr])
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

        Ok(&self.buf[addr..addr + len])
    }

    pub fn write_byte(
        &mut self,
        addr: usize,
        data: u8,
    ) -> Result<(), MemoryError> {
        if !self.is_in_bounds(addr) {
            return Err(MemoryError::AddrOutOfBounds(addr));
        }

        self.buf[addr] = data;
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

        self.buf[addr..(addr + data.len())].copy_from_slice(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_initialization() {
        let memory = Memory::new();
        for i in 0..MEMORY_SIZE {
            assert_eq!(memory.read_byte(i), Ok(0));
        }
    }
    
    #[test]
    fn test_read_write_byte() {
        let mut memory = Memory::new();
        
        // Test different locations
        let test_locations = [0, 1, MEMORY_SIZE / 2, MEMORY_SIZE - 2, MEMORY_SIZE - 1];
        
        for addr in test_locations {
            memory.write_byte(addr, 0xAA).unwrap();
            assert_eq!(memory.read_byte(addr), Ok(0xAA));
            
            memory.write_byte(addr, 0x55).unwrap();
            assert_eq!(memory.read_byte(addr), Ok(0x55));
        }
    }
    
    #[test]
    fn test_read_write_buffer() {
        let mut memory = Memory::new();
        let test_data = [0x12, 0x34, 0x56, 0x78, 0x9A];
        
        memory.write_buf(100, &test_data).unwrap();
        let read_data = memory.read_buf(100, test_data.len()).unwrap();
        
        assert_eq!(read_data, test_data);
    }
    
    #[test]
    fn test_out_of_bounds_access() {
        let mut memory = Memory::new();
        
        // Test out of bounds byte access
        assert_eq!(
            memory.read_byte(MEMORY_SIZE), 
            Err(MemoryError::AddrOutOfBounds(MEMORY_SIZE))
        );
        
        assert_eq!(
            memory.write_byte(MEMORY_SIZE, 0xFF),
            Err(MemoryError::AddrOutOfBounds(MEMORY_SIZE))
        );
    }
    
    #[test]
    fn test_buffer_boundary_conditions() {
        let mut memory = Memory::new();
        let test_data = [0xFF; 10];
        
        // Test buffer that fits exactly at the end
        let start_addr = MEMORY_SIZE - test_data.len();
        assert!(memory.write_buf(start_addr, &test_data).is_ok());
        
        // Test buffer that extends past the end
        let invalid_addr = MEMORY_SIZE - test_data.len() + 1;
        assert_eq!(
            memory.write_buf(invalid_addr, &test_data),
            Err(MemoryError::RangeOutOfBounds(invalid_addr, test_data.len()))
        );
    }
    
    #[test]
    fn test_empty_buffer() {
        let mut memory = Memory::new();
        let empty: [u8; 0] = [];
        
        // Empty buffer writes should succeed
        assert!(memory.write_buf(0, &empty).is_ok());
        assert!(memory.write_buf(MEMORY_SIZE - 1, &empty).is_ok());
        
        // Empty buffer reads should return empty slice
        assert_eq!(memory.read_buf(100, 0).unwrap(), &[]);
    }
    
    #[test]
    fn test_overlapping_writes() {
        let mut memory = Memory::new();
        
        // Write first buffer
        let first_data = [0x11, 0x22, 0x33, 0x44, 0x55];
        memory.write_buf(10, &first_data).unwrap();
        
        // Write overlapping buffer
        let second_data = [0xAA, 0xBB, 0xCC];
        memory.write_buf(12, &second_data).unwrap();
        
        // Verify results
        assert_eq!(memory.read_byte(10).unwrap(), 0x11);
        assert_eq!(memory.read_byte(11).unwrap(), 0x22);
        assert_eq!(memory.read_byte(12).unwrap(), 0xAA); // Overwritten
        assert_eq!(memory.read_byte(13).unwrap(), 0xBB); // Overwritten
        assert_eq!(memory.read_byte(14).unwrap(), 0xCC); // Overwritten
    }
    
    #[test]
    fn test_read_buf_range() {
        let mut memory = Memory::new();
        
        // Initialize some memory
        for i in 0..20 {
            memory.write_byte(i, i as u8).unwrap();
        }
        
        // Test read_buf range
        let buf = memory.read_buf(5, 10).unwrap();
        
        let expected: Vec<u8> = (5..15).map(|i| i as u8).collect();
        assert_eq!(buf, expected.as_slice());
    }
}

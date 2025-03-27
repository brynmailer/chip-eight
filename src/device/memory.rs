const MEMORY_SIZE: usize = 0x1000; // 4kB

pub struct Memory {
    ram: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0; MEMORY_SIZE],
        }
    }

    pub fn write_to_addr(&self, data: &[u8], addr: u16) -> Result<(), MemoryError> {
        todo!();
    }
}

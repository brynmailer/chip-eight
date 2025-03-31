mod cpu;
mod memory;

use self::cpu::CPU;
use self::memory::{Memory, MemoryError};

const DEFAULT_FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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

    pub fn load(&mut self, rom: &[u8]) -> Result<(), MemoryError> {
        // Load font
        self.memory.write_buf(0x050, &DEFAULT_FONT)?;

        // Load ROM from 0x200
        self.memory.write_buf(0x200, rom)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_chip8_initializes_correctly() {
        let chip8 = ChipEight::new();
        // This is just testing the constructor works without panicking
        // Further tests could verify initial memory/CPU state if those were public
    }

    #[test]
    fn test_load_rom_successfully() {
        let mut chip8 = ChipEight::new();
        let test_rom = vec![0xA2, 0xB4, 0xC6, 0xD8]; // Some arbitrary test bytes
        
        let result = chip8.load(&test_rom);
        assert!(result.is_ok());
    }

    #[test]
    fn test_load_font_is_at_correct_location() {
        let mut chip8 = ChipEight::new();
        let empty_rom = vec![];
        
        chip8.load(&empty_rom).unwrap();
        
        for index in 0..DEFAULT_FONT.len() {
            assert_eq!(chip8.memory.read_byte(0x050 + index), Ok(DEFAULT_FONT[index]));
        }
    }

    #[test]
    fn test_load_rom_is_at_correct_location() {
        let mut chip8 = ChipEight::new();
        let test_rom = vec![0x12, 0x34, 0x56, 0x78];
        
        chip8.load(&test_rom).unwrap();
        
        for index in 0..test_rom.len() {
            assert_eq!(chip8.memory.read_byte(0x200 + index), Ok(test_rom[index]));
        }
    }

    #[test]
    fn test_load_oversized_rom() {
        let mut chip8 = ChipEight::new();
        // Create a ROM that's too large for memory
        let oversized_rom = vec![0xFF; 0x2000]; // Assuming 8KB is too large
        
        let result = chip8.load(&oversized_rom);
        // TODO: Check for specific error
        assert!(result.is_err());
    }

    #[test]
    fn test_load_multiple_roms_sequentially() {
        let mut chip8 = ChipEight::new();
        let first_rom = vec![0x11, 0x22];
        let second_rom = vec![0x33, 0x44];
        
        // Load first ROM
        chip8.load(&first_rom).unwrap();
        
        // Load second ROM (should overwrite the first)
        let result = chip8.load(&second_rom);
        assert!(result.is_ok());
        
        for index in 0..second_rom.len() {
            assert_eq!(chip8.memory.read_byte(0x200 + index), Ok(second_rom[index]));
        }
    }
}

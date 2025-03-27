const MEMORY_SIZE: usize = 0x1000; // 4kB
const SPEED_IN_HZ: usize = 700;
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

fn main() {
    let machine = ChipEight::new();

    // Initialize memory
    // 1. Load font data
    // 2. Load program starting from 0x200
    
    // Initialize peripherals (display, keyboard, sound)

    loop {
        // Fetch the current instruction from memory
        
        // Decode the instruction

        // Execute the instruction
    }
}

struct ChipEight {
    // 4kB of contiguous RAM.
    memory: [u8; MEMORY_SIZE],

    // Stack containing 16-bit addressess used to call/return from functions and subroutines.
    stack: Vec<u16>,

    // Program counter which points to the current instruction in memory.
    pc: u16,

    // 16 8-bit general purpose variable registers.
    v: [u8; 16],

    // Index register to point at locations in memory.
    i: u16,

    // Delay timer which is decremented at a rate of 60 Hz until it reaches 0. Can
    // be set and read.
    delay: u8,

    // Sound timer. Functions like the delay timer, but additionally makes a beeping
    // sound when the value is not 0.
    sound: u8,
}

impl ChipEight {
    fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            stack: Vec::new(),
            // Program counter starts at 0x200 for compatibility with old CHIP-8 programs. Where
            // the first 512 bytes of memory were kept free for the interpreter and font data.
            pc: 0x200, 
            v: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
        }
    }

    fn write_to_memory(&self, data: &[u8], addr: u16) -> Result<(), MemoryError> {
        todo!();
    }
}

pub struct CPU {
    // Stack containing 16-bit addressess used to call/return from functions and subroutines.
    stack: Vec<u16>,

    // Stack pointer
    sp: u8,

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

impl CPU {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            sp: 0,
            // Program counter starts at 0x200 for compatibility with old CHIP-8 programs. Where
            // the first 512 bytes of memory were kept free for the interpreter and font data.
            pc: 0x200, 
            v: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
        }
    }
}

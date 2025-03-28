mod device;

use crate::device::ChipEight;

const SPEED_IN_HZ: usize = 700;

fn main() {
    let device = ChipEight::new();

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

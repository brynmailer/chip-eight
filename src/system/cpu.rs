use std::thread;
use std::time::Duration;
use std::sync::{
    Arc,
    atomic::{
        AtomicU8,
        AtomicBool,
        Ordering,
    },
};

pub struct CPU {
    // Stack containing 16-bit addressess used to call/return from functions and subroutines.
    stack: Vec<u16>,

    // Stack pointer
    sp: usize,

    // Program counter which points to the current instruction in memory.
    pc: usize,

    // 16 8-bit general purpose variable registers.
    v: [u8; 16],

    // Index register to point at locations in memory.
    i: usize,

    // Delay timer which is decremented at a rate of 60 Hz until it reaches 0. Can
    // be set and read.
    delay: Timer,

    // Sound timer. Functions like the delay timer, but additionally makes a beeping
    // sound when the value is not 0.
    sound: Timer,
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
            delay: Timer::new(),
            sound: Timer::new(),
        }
    }
}

struct Timer {
    value: Arc<AtomicU8>,
    running: Arc<AtomicBool>,
    handle: thread::JoinHandle<()>,
}

impl Timer {
    pub fn new() -> Self {
        let value = Arc::new(AtomicU8::new(0));
        let running = Arc::new(AtomicBool::new(false));

        let value_clone = Arc::clone(&value);
        let running_clone = Arc::clone(&running);

        // TODO: implement running functionality
        let handle = thread::spawn(move || {
            let tick_duration = Duration::from_millis(1000 / 60); // 60Hz
            
            loop {
                thread::sleep(tick_duration);

                let current = value_clone.load(Ordering::Acquire);

                if current > 0 {
                    value_clone.store(current - 1, Ordering::Release);
                }
            }
        });

        Self {
            value,
            handle,
        }
    }

    pub fn get(&self) -> u8 {
        self.value.load(Ordering::Acquire)
    }

    pub fn set(&self, new_value: u8) {
        self.value.store(new_value, Ordering::Release)
    }
}

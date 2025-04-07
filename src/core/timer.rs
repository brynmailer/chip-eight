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

pub struct Timer {
    value: Arc<AtomicU8>,
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Timer {
    pub fn new() -> Self {
        let value = Arc::new(AtomicU8::new(0));
        let running = Arc::new(AtomicBool::new(false));

        Self {
            value,
            running,
            handle: None,
        }
    }

    pub fn start(&mut self, on_tick: Option<Box<dyn Fn() + Send>>) {
        let value = Arc::clone(&self.value);
        let running = Arc::clone(&self.running);

        let handle = thread::spawn(move || {
            let tick_duration = Duration::from_millis(1000 / 60); // 60hz
            
            while running.load(Ordering::Relaxed) {
                thread::sleep(tick_duration);

                let current = value.load(Ordering::Acquire);

                if current > 0 {
                    value.store(current - 1, Ordering::Release);
                    if let Some(callback) = &on_tick {
                        callback();
                    };
                }
            }
        });

        self.handle = Some(handle);
    }

    pub fn get(&self) -> u8 {
        self.value.load(Ordering::Acquire)
    }

    pub fn set(&self, new_value: u8) {
        self.value.store(new_value, Ordering::Release)
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);

        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

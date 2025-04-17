use std::thread;
use std::time::Duration;
use std::sync::{
    Arc,
    mpsc::Sender,
    atomic::{
        AtomicU8,
        AtomicBool,
        Ordering,
    },
};

use super::peripherals::PeripheralEvent;

pub struct Timer {
    value: Arc<AtomicU8>,
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Timer {
    pub fn new(event_channel: Option<Sender<PeripheralEvent>>) -> Self {
        let value = Arc::new(AtomicU8::new(0));
        let running = Arc::new(AtomicBool::new(true));

        let value_clone = Arc::clone(&value);
        let running_clone = Arc::clone(&running);

        let handle = thread::spawn(move || {
            let tick_duration = Duration::from_millis(1000 / 60); // 60hz
            
            while running_clone.load(Ordering::Relaxed) {
                thread::sleep(tick_duration);

                let current = value_clone.load(Ordering::Acquire);

                if current > 0 {
                    value_clone.store(current - 1, Ordering::Release);
                    if let Some(sender) = &event_channel {
                        let _ = sender.send(PeripheralEvent::PlayTone);
                    };
                } else {
                    if let Some(sender) = &event_channel {
                        let _ = sender.send(PeripheralEvent::StopTone);
                    };
                }
            }
        });

        Self {
            value,
            running,
            handle: Some(handle),
        }
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

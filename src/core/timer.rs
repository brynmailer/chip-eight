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
        let running = Arc::new(AtomicBool::new(true));

        let value_clone = Arc::clone(&value);
        let running_clone = Arc::clone(&running);

        let handle = thread::spawn(move || {
            let tick_duration = Duration::from_millis(1000 / 60); // 60Hz
            
            while running_clone.load(Ordering::Relaxed) {
                thread::sleep(tick_duration);

                let current = value_clone.load(Ordering::Acquire);

                if current > 0 {
                    value_clone.store(current - 1, Ordering::Release);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_value() {
        let timer = Timer::new();
        assert_eq!(timer.get(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let timer = Timer::new();
        timer.set(42);
        assert_eq!(timer.get(), 42);
    }

    #[test]
    fn test_decrement_works() {
        let timer = Timer::new();
        timer.set(10);
        
        // Sleep long enough for at least one decrement (but not too long)
        thread::sleep(Duration::from_millis(20));
        
        // Value should be less than what we set
        assert!(timer.get() < 10);
    }

    #[test]
    fn test_decrement_stops_at_zero() {
        let timer = Timer::new();
        timer.set(1);
        
        // Sleep long enough for the timer to reach zero
        thread::sleep(Duration::from_millis(50));
        
        // Value should be exactly zero
        assert_eq!(timer.get(), 0);
        
        // Sleep a bit longer to ensure it doesn't go below zero
        thread::sleep(Duration::from_millis(50));
        assert_eq!(timer.get(), 0);
    }

    #[test]
    fn test_accurate_timing() {
        let timer = Timer::new();
        timer.set(60);
        
        // Sleep for one second - at 60Hz, should decrement to 0
        thread::sleep(Duration::from_secs(1));
        
        // Allow some timing flexibility (should be between 0 and 5)
        let value = timer.get();
        assert!(value <= 5, "Expected timer close to 0, got {}", value);
    }

    #[test]
    fn test_multiple_timers() {
        let timer1 = Timer::new();
        let timer2 = Timer::new();
        
        timer1.set(100);
        timer2.set(50);
        
        thread::sleep(Duration::from_millis(100));
        
        // Both timers should decrease independently
        assert!(timer1.get() < 100);
        assert!(timer2.get() < 50);
        
        // Second timer should be lower
        assert!(timer2.get() < timer1.get() - 40);
    }

    #[test]
    fn test_thread_safety() {
        let timer = Timer::new();
        timer.set(100);
        
        let timer_clone = Arc::new(timer);
        let readers: Vec<_> = (0..5)
            .map(|_| {
                let timer_ref = Arc::clone(&timer_clone);
                thread::spawn(move || {
                    for _ in 0..100 {
                        let _ = timer_ref.get();
                        thread::sleep(Duration::from_millis(1));
                    }
                })
            })
            .collect();
        
        let writers: Vec<_> = (0..3)
            .map(|i| {
                let timer_ref = Arc::clone(&timer_clone);
                thread::spawn(move || {
                    for j in 0..10 {
                        timer_ref.set(((i * 10) + j) as u8);
                        thread::sleep(Duration::from_millis(5));
                    }
                })
            })
            .collect();
        
        // Wait for all threads to complete
        for handle in readers {
            handle.join().unwrap();
        }
        
        for handle in writers {
            handle.join().unwrap();
        }
        
        // Timer should still be operational
        let before = timer_clone.get();
        thread::sleep(Duration::from_millis(50));
        let after = timer_clone.get();
        
        // If before wasn't already 0, after should be less
        if before > 0 {
            assert!(after < before);
        }
    }

    #[test]
    fn test_drop_stops_thread() {
        // This test checks if Drop implementation properly stops the thread
        // We can verify this indirectly by checking resource usage
        
        // Create and drop many timers in sequence
        for _ in 0..100 {
            let timer = Timer::new();
            timer.set(10);
            drop(timer);
            // If threads aren't properly stopped, we'd exhaust resources
            // or see performance degradation
        }
        
        // If we reached here without system resource errors, 
        // the test is considered successful
    }
}

pub mod sdl3;

use std::sync::{atomic::AtomicBool, Arc};

use sdl3::{SDL3Audio, SDL3Display, SDL3Input};

use crate::config;

pub enum DeviceEvent {
    PlayTone,
    StopTone,
}


pub trait Display {
    fn clear(&mut self);
    fn draw_pixel(&mut self, x: usize, y: usize, color: usize);
    fn render(&mut self);
}

impl From<config::DisplayConfig> for Option<Box<dyn Display>> {
    fn from(config: config::DisplayConfig) -> Self {
        match config.engine {
            config::DisplayEngine::SDL3 => {
                Some(Box::new(SDL3Display::new(config)))
            },
            _ => None,
        }
    }
}


pub trait Audio {
    fn play_tone(&self);
    fn stop_tone(&self);
}

impl From<config::AudioConfig> for Option<Box<dyn Audio>> {
    fn from(config: config::AudioConfig) -> Self {
        match config.engine {
            config::AudioEngine::SDL3 => {
                Some(Box::new(SDL3Audio::new(config)))
            },
            _ => None,
        }
    }
}


pub trait Input {
    fn get_keys_down(&mut self) -> &[bool; 16];
    fn wait_for_key(&mut self, running: Arc<AtomicBool>) -> u8;
}

impl From<config::InputConfig> for Option<Box<dyn Input>> {
    fn from(config: config::InputConfig) -> Self {
        match config.engine {
            config::InputEngine::SDL3 => {
                Some(Box::new(SDL3Input::new(config)))
            },
            _ => None,
        }
    }
}

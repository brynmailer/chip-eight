mod sdl3;

use std::{
    rc::Rc,
    fmt,
    error::Error
};

use sdl3::{SDL3Audio, SDL3Display, SDL3Input};

use crate::config;

pub enum DeviceEvent {
    PlayTone,
    StopTone,
    Draw,
}


pub trait Display {
    fn draw(&mut self, frame: &[bool]);
}

pub fn create_display_device(config: Rc<config::DisplayConfig>) -> Option<Box<dyn Display>> {
    match config.engine {
        config::DisplayEngine::SDL3 => {
            Some(Box::new(SDL3Display::new(config)))
        },
        _ => None,
    }
}


pub trait Audio {
    fn play_tone(&self);
    fn stop_tone(&self);
}

pub fn create_audio_device(config: Rc<config::AudioConfig>) -> Option<Box<dyn Audio>> {
    match config.engine {
        config::AudioEngine::SDL3 => {
            Some(Box::new(SDL3Audio::new(config)))
        },
        _ => None,
    }
}


#[derive(Debug, PartialEq)]
pub struct InvalidKeyError(u8);

impl fmt::Display for InvalidKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid keycode {}", self.0)
    }
}

impl Error for InvalidKeyError {}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Key {
    _0, _1, _2, _3,
    _4, _5, _6, _7,
    _8, _9, A, B,
    C, D, E, F,
}

impl TryFrom<u8> for Key {
    type Error = InvalidKeyError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Key::_0),
            0x1 => Ok(Key::_1),
            0x2 => Ok(Key::_2),
            0x3 => Ok(Key::_3),
            0x4 => Ok(Key::_4),
            0x5 => Ok(Key::_5),
            0x6 => Ok(Key::_6),
            0x7 => Ok(Key::_7),
            0x8 => Ok(Key::_8),
            0x9 => Ok(Key::_9),
            0xA => Ok(Key::A),
            0xB => Ok(Key::B),
            0xC => Ok(Key::C),
            0xD => Ok(Key::D),
            0xE => Ok(Key::E),
            0xF => Ok(Key::F),
            _ => Err(InvalidKeyError(value)),
        }
    }
}

pub trait Input {
    fn get_keys_down(&mut self) -> Vec<Key>;
}

pub fn create_input_device(config: Rc<config::InputConfig>) -> Option<Box<dyn Input>> {
    match config.engine {
        config::InputEngine::SDL3 => {
            Some(Box::new(SDL3Input::new(config)))
        },
        _ => None,
    }
}

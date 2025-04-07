use crate::config::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

pub struct Interface {
}

impl Interface {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn update(&mut self, buf: &[[u8; DISPLAY_HEIGHT]; DISPLAY_WIDTH]) {
        todo!();
    }

    pub fn get_pressed_keys(&self) -> Vec<u8> {
        todo!();
    }

    pub fn play_sound(&self) {
        todo!();
    }
}

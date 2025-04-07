use crate::config::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

pub trait Interface {
    fn update(
        &mut self,
        buf: &[[u8; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    );

    fn get_pressed_keys(&self) -> Vec<u8>;

    fn play_sound(&self) {}
}

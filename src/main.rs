mod config;
mod core;

use config::{DISPLAY_WIDTH, DISPLAY_HEIGHT};
use core::ChipEight;

fn main() {
    ChipEight::new()
        .with_interface(/* Interface trait */)
        .load_rom(/* Rom */)
        .unwrap()
        .play();
}

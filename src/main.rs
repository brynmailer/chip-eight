mod config;
mod core;

use std::{fs::File, io::Read};

use core::ChipEight;

fn main() {
    let mut file = File::open("roms/IBM-logo.ch8").unwrap();
    let mut rom_buf = Vec::new();
    file.read_to_end(&mut rom_buf).unwrap();

    ChipEight::new()
        .load_rom(&rom_buf).unwrap()
        .play();
}

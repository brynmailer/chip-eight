#![feature(mpmc_channel)]

mod config;
mod system;
mod timer;
mod memory;
mod instructions;
mod devices;

use std::{fs::File, io::Read};

use clap::Parser;

use system::ChipEight;
use config::{Args, Config};

fn main() {
    let args = Args::parse();

    let mut file = File::open(&args.rom_path).unwrap();
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap();

    ChipEight::from(Config::from(args))
        .play(&rom);
}

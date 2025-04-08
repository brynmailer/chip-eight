mod config;
mod core;

use std::{fs::File, io::Read};

use clap::Parser;

use core::ChipEight;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required(true))]
    path_to_rom: String,
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.path_to_rom).unwrap();
    let mut rom_buf = Vec::new();
    file.read_to_end(&mut rom_buf).unwrap();

    ChipEight::new()
        .start(&rom_buf);
}

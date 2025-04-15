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

macro_rules! index {
    ($x:expr, $y:expr) => {
        $y * self.width + $x
    };
}

struct Display {
    width: usize,
    height: usize,
    buf: Vec<bool>,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buf: vec![false; width * height],
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.path_to_rom).unwrap();
    let mut rom_buf = Vec::new();
    file.read_to_end(&mut rom_buf).unwrap();

    ChipEight::new()
        .set_display(display)
        .start(&rom_buf);
}

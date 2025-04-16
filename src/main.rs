mod config;
mod core;

use std::{fs::File, io::Read};

use clap::Parser;
use sdl3::{self, pixels::Color, render::{FRect, WindowCanvas}};

use core::{ChipEight, Display};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required(true))]
    path_to_rom: String,
}

macro_rules! index {
    ($x:expr, $y:expr, $width:expr) => {
        $y * $width + $x
    };
}

struct SDL3Screen {
    width: usize,
    height: usize,
    buf: Vec<bool>,
    canvas: WindowCanvas,
}

impl SDL3Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let scaled_width: u32 = (width * 20).try_into().unwrap();
        let scaled_height: u32 = (height * 20).try_into().unwrap();

        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip Eight", scaled_width, scaled_height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Self {
            width,
            height,
            buf: vec![false; width * height],
            canvas,
        }
    }
}

impl Display for SDL3Screen {
    fn clear(&mut self) {
        self.buf.fill(false);

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let bounded_x = x % self.width;
        let bounded_y = y % self.height;
        let mut collision = false;

        for (layer, byte) in sprite.iter().enumerate() {
            if bounded_y + layer >= self.height {
                break;
            }

            for position in 0..8 {
                if bounded_x + position >= self.width {
                    break;
                }

                let bit = (byte.reverse_bits() >> position) & 1;

                if let Some(pixel) = self.buf.get_mut(index!(
                    (bounded_x + position),
                    (bounded_y + layer),
                    self.width
                )) {
                    if bit == 1 {
                        if *pixel {
                            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                            collision = true;
                        } else {
                            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                        }

                        self.canvas.fill_rect(Some(FRect::new(((bounded_x + position) * 20) as f32, ((bounded_y + layer) * 20) as f32, 20.0, 20.0)))
                            .expect("Failed to draw pixel");
                        *pixel = !*pixel;
                    }
                }
            }
        }

        self.canvas.present();

        collision
    }
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.path_to_rom).unwrap();
    let mut rom_buf = Vec::new();
    file.read_to_end(&mut rom_buf).unwrap();

    let display = SDL3Screen::new(64, 32);

    ChipEight::new()
        .set_display(Box::new(display))
        .start(&rom_buf);
}

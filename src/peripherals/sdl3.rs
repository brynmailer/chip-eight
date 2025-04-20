use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use sdl3::{
    event::Event, keyboard::Keycode, pixels::Color, render::{FRect, WindowCanvas}, EventPump, VideoSubsystem
};

use super::{Display, DisplaySettings, Input};

macro_rules! color {
    ($config:expr, $index:tt) => {
        Color::RGB(
            $config.colors[$index].0,
            $config.colors[$index].1,
            $config.colors[$index].2,
        )
    }
}

pub struct SDL3Display {
    settings: DisplaySettings,
    canvas: WindowCanvas,
}

impl SDL3Display {
    pub fn new(video_subsystem: VideoSubsystem, settings: DisplaySettings) -> Self {

        let scaled_width: u32 = settings.scaled_width().try_into().unwrap();
        let scaled_height: u32 = settings.scaled_height().try_into().unwrap();

        let window = video_subsystem.window("Chip Eight", scaled_width, scaled_height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas();
        canvas.set_draw_color(color!(settings, 0));
        canvas.clear();
        canvas.present();

        Self {
            settings,
            canvas,
        }
    }
}

impl Display for SDL3Display {
    fn clear(&mut self) {
        self.canvas.set_draw_color(color!(self.settings, 0));
        self.canvas.clear();
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: usize) {
        self.canvas.set_draw_color(color!(self.settings, color));
        self.canvas.fill_rect(Some(FRect::new(
            x as f32,
            y as f32,
            self.settings.scale_factor as f32,
            self.settings.scale_factor as f32,
        ))).expect("Failed to draw pixel");
    }

    fn render(&mut self) {
        self.canvas.present();
    }
}

pub struct SDL3Input {
    event_pump: EventPump,
    keys_pressed: [bool; 16],
}

impl SDL3Input {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
            keys_pressed: [false; 16],
        }
    }
}

impl Input for SDL3Input {
    fn get_keys_down(&mut self) -> &[bool; 16] {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::_1), .. } => {
                    self.keys_pressed[0x1] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::_2), .. } => {
                    self.keys_pressed[0x2] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::_3), .. } => {
                    self.keys_pressed[0x3] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::_4), .. } => {
                    self.keys_pressed[0xC] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    self.keys_pressed[0x4] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    self.keys_pressed[0x5] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    self.keys_pressed[0x6] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    self.keys_pressed[0xD] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    self.keys_pressed[0x7] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    self.keys_pressed[0x8] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    self.keys_pressed[0x9] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    self.keys_pressed[0xE] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    self.keys_pressed[0xA] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    self.keys_pressed[0x0] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    self.keys_pressed[0xB] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    self.keys_pressed[0xF] = true;
                },

                Event::KeyUp { keycode: Some(Keycode::_1), .. } => {
                    self.keys_pressed[0x1] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::_2), .. } => {
                    self.keys_pressed[0x2] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::_3), .. } => {
                    self.keys_pressed[0x3] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::_4), .. } => {
                    self.keys_pressed[0xC] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                    self.keys_pressed[0x4] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    self.keys_pressed[0x5] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::E), .. } => {
                    self.keys_pressed[0x6] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::R), .. } => {
                    self.keys_pressed[0xD] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    self.keys_pressed[0x7] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    self.keys_pressed[0x8] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    self.keys_pressed[0x9] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::F), .. } => {
                    self.keys_pressed[0xE] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
                    self.keys_pressed[0xA] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::X), .. } => {
                    self.keys_pressed[0x0] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::C), .. } => {
                    self.keys_pressed[0xB] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::V), .. } => {
                    self.keys_pressed[0xF] = false;
                },
                _ => {},
            }
        }

        &self.keys_pressed
    }

    fn wait_for_key(&mut self, running: Arc<AtomicBool>) -> u8 {
        while running.load(Ordering::SeqCst) {
            if let Some(event) = self.event_pump.wait_event_timeout(1000 / 700) {
                match event {
                    Event::KeyDown { keycode: Some(Keycode::_1), .. } => {
                        return 1;
                    },
                    Event::KeyDown { keycode: Some(Keycode::_2), .. } => {
                        return 2;
                    },
                    Event::KeyDown { keycode: Some(Keycode::_3), .. } => {
                        return 3;
                    },
                    Event::KeyDown { keycode: Some(Keycode::_4), .. } => {
                        return 0xC;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                        return 4;
                    },
                    Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                        return 5;
                    },
                    Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                        return 6;
                    },
                    Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                        return 0xD;
                    },
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                        return 7;
                    },
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                        return 8;
                    },
                    Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                        return 9;
                    },
                    Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                        return 0xE;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                        return 0xA;
                    },
                    Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                        return 0;
                    },
                    Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                        return 0xB;
                    },
                    Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                        return 0xF;
                    },
                    _ => {},
                }
            }
        }

        0
    }
}

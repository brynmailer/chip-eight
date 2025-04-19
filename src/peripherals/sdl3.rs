use std::{
    sync::{Arc, RwLock},
    thread,
};

use sdl3::{
    event::{Event, EventWatch, EventWatchCallback}, keyboard::Keycode, pixels::Color, render::{FRect, WindowCanvas}, EventPump, EventSubsystem, VideoSubsystem
};

use super::{Display, DisplaySettings, Input, Key};

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

struct InputEventCallback;

impl EventWatchCallback for InputEventCallback {
    fn callback(&mut self, event: Event) {
        match event {
            Event::KeyDown { scancode, ..} => {
                println!("{scancode:?} down");
            },
            Event::KeyUp { scancode, ..} => {
                println!("{scancode:?} up");
            },
            _ => {},
        }
    }
}

pub struct SDL3Input<'a> {
    keys_pressed: Arc<RwLock<Vec<Key>>>,
    event_watch: EventWatch<'a, InputEventCallback>,
}

impl<'a> SDL3Input<'a> {
    pub fn new(event_subsystem: EventSubsystem) -> Self {
        let keys_pressed = Arc::new(RwLock::new(vec![]));

        let event_watch = event_subsystem.add_event_watch(InputEventCallback);

        Self {
            keys_pressed,
            event_watch,
        }
    }
}

impl<'a> Input for SDL3Input<'a> {
    fn get_keys_down(&mut self) -> Vec<Key> {
        let mut keys: Vec<Key> = vec![];

        /*
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::_1), .. } => {
                    keys.push(Key::One);
                },
                Event::KeyDown { keycode: Some(Keycode::_2), .. } => {
                    keys.push(Key::Two);
                },
                Event::KeyDown { keycode: Some(Keycode::_3), .. } => {
                    keys.push(Key::Three);
                },
                Event::KeyDown { keycode: Some(Keycode::_4), .. } => {
                    keys.push(Key::C);
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    keys.push(Key::Four);
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    keys.push(Key::Five);
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    keys.push(Key::Six);
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    keys.push(Key::D);
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    keys.push(Key::Seven);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    keys.push(Key::Eight);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    keys.push(Key::Nine);
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    keys.push(Key::E);
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    keys.push(Key::A);
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    keys.push(Key::Zero);
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    keys.push(Key::B);
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    keys.push(Key::F);
                },
                _ => {},
            }
        }
        */

        keys
    }

    fn wait_for_key(&mut self) -> Key {
        /*
        loop {
            let event = self.event_pump.wait_event();

            match event {
                Event::KeyDown { keycode: Some(Keycode::_1), .. } => {
                    return Key::One;
                },
                Event::KeyDown { keycode: Some(Keycode::_2), .. } => {
                    return Key::Two;
                },
                Event::KeyDown { keycode: Some(Keycode::_3), .. } => {
                    return Key::Three;
                },
                Event::KeyDown { keycode: Some(Keycode::_4), .. } => {
                    return Key::C;
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    return Key::Four;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    return Key::Five;
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    return Key::Six;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    return Key::D;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    return Key::Seven;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    return Key::Eight;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    return Key::Nine;
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    return Key::E;
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    return Key::A;
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    return Key::Zero;
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    return Key::B;
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    return Key::F;
                },
                _ => {},
            }
        }
        */
        return Key::Zero;
    }
}

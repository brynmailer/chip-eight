use std::rc::Rc;

use sdl3::{
    pixels::Color,
    render,
    audio,
    EventPump,
};

use crate::config::{AudioConfig, DisplayConfig, InputConfig};

use super::{Audio, Display, Input, Key};


/* Display */

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
    config: Rc<DisplayConfig>,
    canvas: render::WindowCanvas,
}

impl SDL3Display {
    pub fn new(config: Rc<DisplayConfig>) -> Self {
        let context = sdl3::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let scaled_width: u32 = config.scaled_width().try_into().unwrap();
        let scaled_height: u32 = config.scaled_height().try_into().unwrap();

        let window = video_subsystem.window("Chip Eight", scaled_width, scaled_height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas();
        canvas.set_draw_color(color!(config, 0));
        canvas.clear();
        canvas.present();

        Self {
            config,
            canvas,
        }
    }
}

impl Display for SDL3Display {
    fn draw(&mut self, frame: &[bool]) {
        let mut on: Vec<render::FRect> = Vec::new();
        let mut off: Vec<render::FRect> = Vec::new();

        for (index, &value) in frame.iter().enumerate() {
            let rect = render::FRect::new(
                ((index % self.config.width) * self.config.scale_factor) as f32,
                ((index / self.config.width) * self.config.scale_factor) as f32,
                self.config.scale_factor as f32,
                self.config.scale_factor as f32,
            );

            if value {
                on.push(rect);
            } else {
                off.push(rect);
            }
        }

        self.canvas.set_draw_color(color!(self.config, 1));
        self.canvas.fill_rects(&on)
            .expect("Failed to draw");

        self.canvas.set_draw_color(color!(self.config, 0));
        self.canvas.fill_rects(&off)
            .expect("Failed to draw");


        self.canvas.present();
    }
}


/* Audio */

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl audio::AudioCallback<f32> for SquareWave {
    fn callback(&mut self, stream: &mut audio::AudioStream, len: i32) {
        let mut out = vec![0.0; len as usize];

        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }

        stream.put_data_f32(&out)
            .expect("Failed to push samples to audio stream");
    }
}

pub struct SDL3Audio {
    stream: audio::AudioStreamWithCallback<SquareWave>,
}

impl SDL3Audio {
    pub fn new(_config: Rc<AudioConfig>) -> Self {
        let context = sdl3::init().unwrap();
        let audio_subsystem = context.audio().unwrap();

        let source_freq = 44100;
        let source_spec = audio::AudioSpec {
            freq: Some(source_freq),
            channels: Some(1),                      // mono
            format: Some(audio::AudioFormat::f32_sys())    // floating 32 bit samples
        };

        let stream = audio_subsystem.open_playback_stream(&source_spec, SquareWave {
            phase_inc: 440.0 / source_freq as f32,
            phase: 0.0,
            volume: 0.03,
        }).unwrap();

        Self {
            stream,
        }
    }
}

impl Audio for SDL3Audio {
    fn play_tone(&self) {
        self.stream.resume()
            .expect("Failed to play audio");
    }

    fn stop_tone(&self) {
        self.stream.pause()
            .expect("Failed to stop audio");
    }
}


/* Input */

pub struct SDL3Input {
    config: Rc<InputConfig>,
    event_pump: EventPump,
}

impl SDL3Input {
    pub fn new(config: Rc<InputConfig>) -> Self {
        let context = sdl3::init().unwrap();
        let event_pump = context.event_pump().unwrap();

        Self {
            config,
            event_pump,
        }
    }
}

impl Input for SDL3Input {
    fn get_keys_down(&mut self) -> Vec<Key> {
        self.event_pump.pump_events();

        self.event_pump.keyboard_state()
            .pressed_scancodes()
            .filter_map(|scancode| {
                if let Some(index) = self.config.key_map.iter().position(|mapping| mapping.1 == scancode.name()) {
                    return Some(self.config.key_map[index].0.clone());
                }

                None
            })
            .collect()
    }
}

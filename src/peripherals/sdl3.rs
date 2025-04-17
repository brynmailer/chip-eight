use sdl3::{
    pixels::Color,
    render::{FRect, WindowCanvas},
    VideoSubsystem,
};

use super::{Display, DisplaySettings};

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

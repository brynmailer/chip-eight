pub mod sdl3;

pub enum PeripheralEvent {
    PlayTone,
    StopTone,
}

pub enum Key {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Clone)]
pub enum DisplayEngine {
    SDL3,
}

#[derive(Clone)]
pub struct DisplaySettings {
    pub engine: Option<DisplayEngine>,
    // Width/height in virtual pixels
    pub width: usize,
    pub height: usize,
    // Number of device pixels to render per virtual pixel
    pub scale_factor: usize,
    // Pixel colors
    pub colors: [(u8, u8, u8); 2],
}

impl DisplaySettings {
    // Width in device pixels
    pub fn scaled_width(&self) -> usize {
        self.width * self.scale_factor
    }

    // Height in device pixels
    pub fn scaled_height(&self) -> usize {
        self.height * self.scale_factor
    }
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            engine: Some(DisplayEngine::SDL3),
            width: 64,
            height: 32,
            scale_factor: 20,
            colors: [
                // Off
                (0, 0, 0),
                // On
                (255, 255, 255),
            ],
        }
    }
}

pub trait Display {
    fn clear(&mut self);
    fn draw_pixel(&mut self, x: usize, y: usize, color: usize);
    fn render(&mut self);
}

pub trait Audio {
    fn play_tone(&self);
    fn stop_tone(&self);
}

pub trait Input {
    fn get_keys_down(&self) -> Vec<Key>;
}


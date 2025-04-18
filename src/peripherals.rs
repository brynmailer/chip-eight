pub mod sdl3;

pub enum PeripheralEvent {
    PlayTone,
    StopTone,
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
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

macro_rules! u8_to_key {
    ($val:ident; $($key:ident),+) => (
        match $val {
            $(${index()} => Ok(Key::$key)),+,
            _ => Err(KeyError),
        }
    )
}

#[derive(Debug)]
pub struct KeyError;

impl TryFrom<u8> for Key {
    type Error = KeyError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        u8_to_key!(
            value;
            Zero, One, Two, Three, Four, Five, Six,
            Seven, Eight, Nine, A, B, C, D, E, F
        )
    }
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

#[derive(Clone)]
pub enum InputEngine {
    SDL3,
}

#[derive(Clone)]
pub struct InputSettings {
    pub engine: Option<InputEngine>,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            engine: Some(InputEngine::SDL3),
        }
    }
}

pub trait Input {
    fn get_keys_down(&mut self) -> Vec<Key>;
    fn wait_for_key(&mut self) -> Key;
}


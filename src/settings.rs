pub struct DisplaySettings {
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

pub struct AudioSettings {}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {}
    }
}

pub struct InputSettings { 
    pub keymap: [u8; 16],
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
        }
    }
}

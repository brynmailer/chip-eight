use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to a ROM file.
    pub rom_path: String,


    /// Number of instruction to process per second.
    #[arg(short, long, default_value_t = 600)]
    pub clock_speed: u64,

    /// Skip setting vF to zero after executing opcodes 8XY1, 8XY2 and 8XY3.
    #[arg(short = 'v', long)]
    pub skip_reset_vf: bool,

    /// Avoid incrementing the index register while executing opcodes FX55 and FX65.
    #[arg(short = 'n', long)]
    pub preserve_index: bool,

    /// Opcode DXYN draws sprite immediately, instead of waiting for the display interrupt.
    #[arg(short = 'w', long)]
    pub skip_draw_wait: bool,

    /// Wrap sprites drawn at the edge of the display.
    #[arg(short = 'l', long)]
    pub wrap_sprites: bool,

    /// Skip setting vX to vY before shifting vX for opcodes 8XY6 and 8XYE.
    #[arg(short, long)]
    pub skip_shift_set: bool,

    /// Use vX (instead of v0) as the jump offset for opcode BNNN. Where X is the most significant nibble of NNN.
    #[arg(short, long)]
    pub jump_with_vx: bool,


    /// Size of memory in bytes.
    #[arg(short, long, default_value_t = 0x1000)]
    pub memory_length: usize,

    /// Memory address of the first intruction of the loaded program.
    #[arg(short, long, default_value_t = 0x200)]
    pub program_start: usize,

    /// Memory address of the first byte of the default font.
    #[arg(short = 'o', long, default_value_t = 0x50)]
    pub font_start: usize,


    /// Display engine.
    #[arg(short, long, value_enum, default_value_t = DisplayEngine::SDL3)]
    pub display_engine: DisplayEngine,

    /// Display width in virtual pixels.
    #[arg(short = 'y', long, default_value_t = 64)]
    pub width: usize,

    /// Display height in virtual pixels.
    #[arg(short = 'x', long, default_value_t = 32)]
    pub height: usize,

    /// Number of device pixels to render per virtual pixel.
    #[arg(short = 'f', long, default_value_t = 20)]
    pub scale_factor: usize,


    /// Audio engine.
    #[arg(short, long, value_enum, default_value_t = AudioEngine::SDL3)]
    pub audio_engine: AudioEngine,


    /// Input engine.
    #[arg(short, long, value_enum, default_value_t = InputEngine::SDL3)]
    pub input_engine: InputEngine,
}

pub struct Config {
    pub clock_speed: u64,
    pub quirks: QuirksConfig,
    pub memory: MemoryConfig,
    pub display: DisplayConfig,
    pub audio: AudioConfig,
    pub input: InputConfig,
}

pub struct QuirksConfig {
    pub skip_reset_vf: bool,
    pub preserve_index: bool,
    pub skip_draw_wait: bool,
    pub wrap_sprites: bool,
    pub skip_shift_set: bool,
    pub jump_with_vx: bool,
}

#[derive(Clone, Copy)]
pub struct MemoryConfig {
    pub length: usize,
    pub program_start: usize,
    pub font_start: usize,
    pub default_font: [u8; 80],
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DisplayEngine {
    SDL3,
    None,
}

#[derive(Clone, Copy)]
pub struct DisplayConfig {
    pub engine: DisplayEngine,
    pub width: usize,
    pub height: usize,
    pub scale_factor: usize,
    pub colors: [(u8, u8, u8); 2],
}

impl DisplayConfig {
    // Width in device pixels
    pub fn scaled_width(&self) -> usize {
        self.width * self.scale_factor
    }

    // Height in device pixels
    pub fn scaled_height(&self) -> usize {
        self.height * self.scale_factor
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AudioEngine {
    SDL3,
    None,
}

#[derive(Clone, Copy)]
pub struct AudioConfig {
    pub engine: AudioEngine,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum InputEngine {
    SDL3,
    None,
}

#[derive(Clone, Copy)]
pub struct InputConfig {
    pub engine: InputEngine,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Self {
            clock_speed: args.clock_speed,
            quirks: QuirksConfig {
                skip_reset_vf: args.skip_reset_vf,
                preserve_index: args.preserve_index,
                skip_draw_wait: args.skip_draw_wait,
                wrap_sprites: args.wrap_sprites,
                skip_shift_set: args.skip_shift_set,
                jump_with_vx: args.jump_with_vx,
            },
            memory: MemoryConfig {
                length: args.memory_length,
                program_start: args.program_start,
                font_start: args.font_start,
                default_font: [
                    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                    0x20, 0x60, 0x20, 0x20, 0x70, // 1
                    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
                ],
            },
            display: DisplayConfig {
                engine: args.display_engine,
                width: args.width,
                height: args.height,
                scale_factor: args.scale_factor,
                colors: [
                    // Off
                    (0, 0, 0),
                    // On
                    (255, 255, 255),
                ],
            },
            audio: AudioConfig {
                engine: args.audio_engine,
            },
            input: InputConfig {
                engine: args.input_engine,
            },
        }
    }
}

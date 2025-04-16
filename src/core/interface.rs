#[derive(Clone)]
pub enum InterfaceEvent {
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

pub trait Display {
    fn clear(&mut self);
    fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool;
}

pub trait Input {
    fn get_keys_down(&self) -> Vec<Key>;
}

pub trait Audio {
    fn play_tone(&self);

    fn stop_tone(&self);
}

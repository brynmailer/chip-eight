#[derive(Clone)]
pub enum InterfaceEvent {
    Render(Vec<u8>),
    GetKeysDown,
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
    fn render(&mut self, buf: &[[u8; 32]; 64]);
}

pub trait Input {
    fn get_keys_down(&self) -> Vec<Key>;
}

pub trait Audio {
    fn play_tone(&mut self);

    fn stop_tone(&mut self);
}

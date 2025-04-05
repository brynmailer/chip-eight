pub enum Key {
    Key0 = 0x0,
    Key1 = 0x1,
    Key2 = 0x2,
    Key3 = 0x3,
    Key4 = 0x4,
    Key5 = 0x5,
    Key6 = 0x6,
    Key7 = 0x7,
    Key8 = 0x8,
    Key9 = 0x9,
    A = 0xA,
    B = 0xB,
    C = 0xC,
    D = 0xD,
    E = 0xE,
    F = 0xF,
}

pub trait Screen {
    fn draw(
        &mut self,
        x: usize,
        y: usize,
        sprite: &[u8]
    ) -> bool;

    fn clear(&mut self);

    fn render(&mut self) {}
}

pub trait Input {
    fn wait_for_key_press(&self) -> u8;
}

pub struct Interface {

}

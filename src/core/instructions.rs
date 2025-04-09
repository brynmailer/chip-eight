pub enum Instruction {
    Clear,
    Jump(usize),
    SetV(usize, u8),
    AddToV(usize, u8),
    SetI(usize),
    Draw(usize, usize, u8),
}

pub enum Instruction {
    Clear,
    Jump(usize),
    SetV(usize, u8),
    AddToV(usize, u8),
    SetI(usize),
    Draw(usize, usize, u8),
}

macro_rules! opcode {
    () => {
        
    };
}

pub fn parse_opcode(opcode: u16) -> Instruction {
    match opcode {
        opcode!(00E0) => {}.
        opcode!(1NNN) => {}.
        opcode!(6XNN) => {}.
        opcode!(7XNN) => {}.
        opcode!(ANNN) => {}.
        opcode!(DXYN) => {}.
    }
}

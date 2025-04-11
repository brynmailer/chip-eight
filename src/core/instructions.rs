pub enum Instruction {
    Clear,
    Return,
    Jump(usize),
    Call(usize),
    IfVxEq(usize, u8),
    IfVxNotEq(usize, u8),
    IfVxEqVy(usize, usize),
    SetVx(usize, u8),
    AddToVx(usize, u8),
    SetVxToVy(usize, usize),
    SetVxOrVy(usize, usize),
    SetVxAndVy(usize, usize),
    SetVxXorVy(usize, usize),
    AddVyToVx(usize, usize),
    SubVyFromVx(usize, usize),
    RightShiftVx(usize),
    SubVxFromVy(usize, usize),
    LeftShiftVx(usize),
    IfVxNotEqVy(usize, usize),
    SetI(usize),
    JumpWithOffset(usize),
    SetVxRand(u8),
    Draw(usize, usize, u8),
    IfKeyPressed(usize),
    IfKeyNotPressed(usize),
    SetVxToDelay(usize),
    SetVxToKey(usize),
    SetDelayToVx(usize),
    SetSoundToVx(usize),
    AddVxToI(usize),
    SetIToCharInVx(usize),
    StoreVBCD(usize),
    VDump(usize),
    VLoad(usize),
}

macro_rules! opcode {
    (00E0) => {
        0x00E0 => Instruction::Clear,
    };
    (1NNN) => {
        0x1NNN => {
            Instruction::Jump(nnn)
        },
    };
}

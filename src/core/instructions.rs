use std::{error::Error, fmt};

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
    SetVxRand(usize, u8),
    Draw(usize, usize, u8),
    IfKeyPressed(usize),
    IfKeyNotPressed(usize),
    SetVxToDelay(usize),
    SetVxToKey(usize),
    SetDelayToVx(usize),
    SetSoundToVx(usize),
    AddVxToI(usize),
    SetIToCharInVx(usize),
    StoreVxBCDAtI(usize),
    VDump(usize),
    VLoad(usize),
}

#[derive(Debug, PartialEq)]
pub struct InvalidOpcodeError(u16);

impl fmt::Display for InvalidOpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid opcode 0x{:04X}", self.0)
    }
}

impl Error for InvalidOpcodeError {}

impl TryFrom<u16> for Instruction {
    type Error = InvalidOpcodeError;

    fn try_from(opcode: u16) -> Result<Self, Self::Error> {
        let op_type = (opcode >> 12) & 0xF;
        let x = (opcode >> 8) & 0xF;
        let y = (opcode >> 4) & 0xF;
        let nnn = opcode & 0xFFF;
        let nn = (opcode & 0xFF) as u8;
        let n = (opcode & 0xF) as u8;

        match op_type {
            0x0 => {
                match nn {
                    0xE0 => Ok(Self::Clear),
                    0xEE => Ok(Self::Return),
                    _ => Err(InvalidOpcodeError(opcode)),
                }
            },
            0x1 => Ok(Self::Jump(nnn.into())),
            0x2 => Ok(Self::Call(nnn.into())),
            0x3 => Ok(Self::IfVxEq(x.into(), nn)),
            0x4 => Ok(Self::IfVxNotEq(x.into(), nn)),
            0x5 => Ok(Self::IfVxEqVy(x.into(), y.into())),
            0x6 => Ok(Self::SetVx(x.into(), nn)),
            0x7 => Ok(Self::AddToVx(x.into(), nn)),
            0x8 => {
                match n {
                    0x0 => Ok(Self::SetVxToVy(x.into(), y.into())),
                    0x1 => Ok(Self::SetVxOrVy(x.into(), y.into())),
                    0x2 => Ok(Self::SetVxAndVy(x.into(), y.into())),
                    0x3 => Ok(Self::SetVxXorVy(x.into(), y.into())),
                    0x4 => Ok(Self::AddVyToVx(x.into(), y.into())),
                    0x5 => Ok(Self::SubVyFromVx(x.into(), y.into())),
                    0x6 => Ok(Self::RightShiftVx(x.into())),
                    0x7 => Ok(Self::SubVxFromVy(x.into(), y.into())),
                    0xE => Ok(Self::LeftShiftVx(x.into())),
                    _ => Err(InvalidOpcodeError(opcode)),
                }
            },
            0x9 => Ok(Self::IfVxNotEqVy(x.into(), y.into())),
            0xA => Ok(Self::SetI(nnn.into())),
            0xB => Ok(Self::JumpWithOffset(nnn.into())),
            0xC => Ok(Self::SetVxRand(x.into(), nn)),
            0xD => Ok(Self::Draw(x.into(), y.into(), n)),
            0xE => {
                match nn {
                    0x9E => Ok(Self::IfKeyPressed(x.into())),
                    0xA1 => Ok(Self::IfKeyNotPressed(x.into())),
                    _ => Err(InvalidOpcodeError(opcode)),
                }
            },
            0xF => {
                match nn {
                    0x07 => Ok(Self::SetVxToDelay(x.into())),
                    0x0A => Ok(Self::SetVxToKey(x.into())),
                    0x15 => Ok(Self::SetDelayToVx(x.into())),
                    0x18 => Ok(Self::SetSoundToVx(x.into())),
                    0x1E => Ok(Self::AddVxToI(x.into())),
                    0x29 => Ok(Self::SetIToCharInVx(x.into())),
                    0x33 => Ok(Self::StoreVxBCDAtI(x.into())),
                    0x55 => Ok(Self::VDump(x.into())),
                    0x65 => Ok(Self::VLoad(x.into())),
                    _ => Err(InvalidOpcodeError(opcode)),
                }
            },
            _ => Err(InvalidOpcodeError(opcode)),
        }
    }
}

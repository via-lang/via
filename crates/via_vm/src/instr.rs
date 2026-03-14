/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use via_macros::Opcode;

pub type RegId = u16;

#[derive(Copy, Clone, Debug)]
pub enum InstrFormat {
    Rx0,
    Rx1,
    Rx2,
    Rx3,
    RIm,
}

#[derive(Debug, Clone, Copy)]
pub struct Instr(u64);

impl Instr {
    #[inline]
    pub fn new_rx<const N: usize>(op: Op, operands: &[u16; N]) -> Self {
        assert!(
            N <= 3,
            "RxN instruction format must contain at most 3 operands"
        );

        let mut word = (op as u64) << 48;

        for (i, val) in operands.iter().enumerate().take(3) {
            let val: u16 = (*val).into();
            let shift = 32 - i * 16;

            word |= (val as u64) << shift;
        }

        Self(word)
    }

    #[inline]
    pub fn new_rim(op: Op, dst: u16, imm: u32) -> Self {
        let mut word = (op as u64) << 48;
        word |= (dst as u64) << 32;
        word |= imm as u64;

        Self(word)
    }

    #[inline]
    pub const fn op(&self) -> Op {
        unsafe { std::mem::transmute((self.0 & 0xFFFF) as u16) }
    }

    #[inline]
    pub const fn a(&self) -> u16 {
        (self.0 & u16::MAX as u64) as u16
    }

    #[inline]
    pub const fn b(&self) -> u16 {
        ((self.0 >> 16) & u16::MAX as u64) as u16
    }

    #[inline]
    pub const fn c(&self) -> u16 {
        ((self.0 >> 32) & u16::MAX as u64) as u16
    }

    #[inline]
    pub const fn imm(&self) -> u32 {
        ((self.0 >> 32) & u32::MAX as u64) as u32
    }
}

#[repr(u16)]
#[derive(Opcode)]
pub enum Op {
    #[layout(Rx0)]
    Halt,
    #[layout(Rx2)]
    Copy,
    #[layout(Rx1)]
    Free1,
    #[layout(Rx2)]
    Free2,
    #[layout(Rx3)]
    Free3,
    #[layout(Rx1)]
    ExtraArg1,
    #[layout(Rx2)]
    ExtraArg2,
    #[layout(Rx3)]
    ExtraArg3,
    #[layout(Rx1)]
    LoadNone,
    #[layout(Rx1)]
    LoadTrue,
    #[layout(Rx1)]
    LoadFalse,
    #[layout(RIm)]
    LoadI32,
    #[layout(RIm)]
    LoadI64,
    #[layout(RIm)]
    LoadU32,
    #[layout(RIm)]
    LoadU64,
    #[layout(RIm)]
    LoadF32,
    #[layout(RIm)]
    LoadF64,
    #[layout(Rx2)]
    Not,
    #[layout(Rx2)]
    INeg,
    #[layout(Rx2)]
    FNeg,
    #[layout(Rx2)]
    BitNot,
    #[layout(Rx3)]
    IAdd,
    #[layout(Rx3)]
    FAdd,
    #[layout(Rx3)]
    ISub,
    #[layout(Rx3)]
    FSub,
    #[layout(Rx3)]
    IMul,
    #[layout(Rx3)]
    FMul,
    #[layout(Rx3)]
    IDiv,
    #[layout(Rx3)]
    FDiv,
    #[layout(Rx3)]
    IPow,
    #[layout(Rx3)]
    FPow,
    #[layout(Rx3)]
    IMod,
    #[layout(Rx3)]
    FMod,
    #[layout(Rx3)]
    ILt,
    #[layout(Rx3)]
    FLt,
    #[layout(Rx3)]
    ILtEq,
    #[layout(Rx3)]
    FLtEq,
    #[layout(Rx3)]
    IGt,
    #[layout(Rx3)]
    FGt,
    #[layout(Rx3)]
    IGtEq,
    #[layout(Rx3)]
    FGtEq,
    #[layout(Rx3)]
    BEq,
    #[layout(Rx3)]
    IEq,
    #[layout(Rx3)]
    FEq,
    #[layout(Rx3)]
    And,
    #[layout(Rx3)]
    Or,
    #[layout(Rx3)]
    BitAnd,
    #[layout(Rx3)]
    BitOr,
    #[layout(Rx3)]
    BitShl,
    #[layout(Rx3)]
    BitShr,
}

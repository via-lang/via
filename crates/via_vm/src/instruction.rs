use std::fmt;

use via_macros::Opcode;

pub type Operand = u8;
pub type Immediate = u16;

type Word = u32;

#[derive(Clone, Copy)]
pub struct Instruction(Word);

const BITS: usize = 8;
const OP_SHIFT: usize = (size_of::<Word>() - size_of::<u8>()) * BITS;
const A_SHIFT: usize = (size_of::<Word>() - (size_of::<u8>() * 2)) * BITS;
const B_SHIFT: usize = (size_of::<Word>() - (size_of::<u8>() * 3)) * BITS;
const C_SHIFT: usize = 0;

impl Instruction {
    #[inline]
    pub fn new_rx<const N: usize>(op: OpCode, operands: &[Operand; N]) -> Self {
        assert!(N <= 3, "RxN format supports max 3 operands");

        let mut word = (op as Word) << OP_SHIFT;

        for (i, val) in operands.iter().enumerate().take(3) {
            let shift = (size_of::<Word>() - (size_of::<u8>() * (i + 2))) * BITS;
            word |= (*val as Word) << shift;
        }

        Self(word)
    }

    #[inline]
    pub fn new_rim(op: OpCode, dst: Operand, imm: Immediate) -> Self {
        let mut word = (op as Word) << OP_SHIFT;
        word |= (dst as Word) << A_SHIFT;
        word |= imm as Word;
        Self(word)
    }

    #[inline]
    pub const fn op(&self) -> OpCode {
        unsafe { std::mem::transmute(((self.0 >> OP_SHIFT) & 0xFF) as u8) }
    }

    pub const fn a(&self) -> Operand {
        ((self.0 >> A_SHIFT) & 0xFF) as u8
    }

    pub const fn b(&self) -> Operand {
        ((self.0 >> B_SHIFT) & 0xFF) as u8
    }

    pub const fn c(&self) -> Operand {
        ((self.0 >> C_SHIFT) & 0xFF) as u8
    }

    #[inline]
    pub const fn imm(&self) -> Immediate {
        (self.0 & (Immediate::MAX as Word)) as Immediate
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?} {} {} {}]",
            self.op(),
            self.a(),
            self.b(),
            self.c()
        )
    }
}

#[repr(u8)]
#[derive(Opcode, Debug)]
pub enum OpCode {
    /// Immediately halts execution with Interrupt::Halt
    #[layout(Rx0)]
    Halt,

    /// Deep-copies rA to rB.
    #[layout(Rx2)]
    Copy,

    /// Dereferences rA.
    #[layout(Rx1)]
    Free1,

    /// Dereferences rA and rB.
    #[layout(Rx2)]
    Free2,

    /// Dereferences rA, rB, and rC.
    #[layout(Rx3)]
    Free3,

    /// Pushes rA onto the stack.
    #[layout(Rx1)]
    Push,

    /// Sets local Imm to rA's value.
    #[layout(RIm)]
    SetLocal,

    /// Sets rA to local Imm's value.
    #[layout(RIm)]
    GetLocal,

    #[layout(RIm)]
    GetParam,

    /// Invokes native closure rA and return to rB.
    #[layout(Rx2)]
    InvokeNative,

    /// RESERVED: emitted after instructions that require 3+ operands.
    /// Encodes one extra operand into the previous instruction.
    #[layout(Rx1)]
    ExtraArg1,

    /// RESERVED: emitted after instructions that require 3+ operands.
    /// Encodes two extra operands into the previous instruction.
    #[layout(Rx2)]
    ExtraArg2,

    /// RESERVED: emitted after instructions that require 3+ operands.
    /// Encodes tree extra operands into the previous instruction.
    #[layout(Rx3)]
    ExtraArg3,

    /// Sets rA to ().
    #[layout(Rx1)]
    LoadNone,

    /// Sets rA to true.
    #[layout(Rx1)]
    LoadTrue,

    /// Sets rA to false.
    #[layout(Rx1)]
    LoadFalse,

    /// Sets rA to Imm interpreted as int16.
    #[layout(RIm)]
    LoadI16,

    /// Sets rA to Imm interpreted as int32.
    #[layout(RIm)]
    LoadI32,

    /// Sets rA to Imm + ExtraArg2 as int64.
    #[layout(RIm)]
    LoadI64,

    /// Sets rA to Imm + ExtraArg2 as float64.
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

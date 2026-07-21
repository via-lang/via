use std::fmt;

use via_macros::Opcode;

pub type Operand = u8;
pub type Immediate = u16;

type Word = u32;

#[derive(Clone, Copy)]
pub struct Instr(Word);

const BITS: usize = 8;
const OP_SHIFT: usize = (size_of::<Word>() - size_of::<u8>()) * BITS;
const A_SHIFT: usize = (size_of::<Word>() - (size_of::<u8>() * 2)) * BITS;
const B_SHIFT: usize = (size_of::<Word>() - (size_of::<u8>() * 3)) * BITS;
const C_SHIFT: usize = 0;

impl Instr {
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
    pub fn op(&self) -> OpCode {
        unsafe { std::mem::transmute(((self.0 >> OP_SHIFT) & 0xFF) as u8) }
    }

    #[inline]
    pub fn a(&self) -> Operand {
        ((self.0 >> A_SHIFT) & 0xFF) as u8
    }

    #[inline]
    pub fn b(&self) -> Operand {
        ((self.0 >> B_SHIFT) & 0xFF) as u8
    }

    #[inline]
    pub fn c(&self) -> Operand {
        ((self.0 >> C_SHIFT) & 0xFF) as u8
    }

    #[inline]
    pub fn imm(&self) -> Immediate {
        (self.0 & (Immediate::MAX as Word)) as Immediate
    }

    /// Encodes a 32-bit value into a base instruction and one EARG2 extension slot.
    #[inline]
    pub fn encode_32(dst: Operand, bits: u32, is_float: bool) -> (Self, Self) {
        let hi = (bits >> 16) as Immediate;
        let extra_a = ((bits >> 8) & 0xFF) as u8;
        let extra_b = (bits & 0xFF) as u8;

        let base = if is_float {
            Instr::LDF32(dst, hi)
        } else {
            Instr::LDI32(dst, hi)
        };

        let ext = Instr::EARG2(extra_a, extra_b);
        (base, ext)
    }

    /// Decodes a 32-bit value from a sequential 2-slot block.
    #[inline]
    pub unsafe fn decode_32(pc: *const Self) -> u32 {
        let base = unsafe { &*pc };
        let extra = unsafe { &*pc.add(1) };

        let hi = base.imm() as u32;
        let lo = ((extra.a() as u32) << 8) | (extra.b() as u32);

        (hi << 16) | lo
    }

    /// Encodes a 64-bit value into three sequential instructions using 3-byte extension slots.
    #[inline]
    pub fn encode_64(dst: Operand, bits: u64, is_float: bool) -> (Self, Self, Self) {
        let imm = (bits >> 48) as Immediate;

        let b1 = (bits >> 40) as u8;
        let b2 = (bits >> 32) as u8;
        let b3 = (bits >> 24) as u8;

        let b4 = (bits >> 16) as u8;
        let b5 = (bits >> 8) as u8;
        let b6 = bits as u8;

        let base = if is_float {
            Self::LDF64(dst, imm)
        } else {
            Self::LDI64(dst, imm)
        };

        let ext1 = Self::EARG3(b1, b2, b3);
        let ext2 = Self::EARG3(b4, b5, b6);

        (base, ext1, ext2)
    }

    /// Decodes a 64-bit value from a sequential 3-slot block by stitching the bytes back together.
    #[inline]
    pub unsafe fn decode_64(pc: *const Self) -> u64 {
        let base = unsafe { &*pc };
        let ext1 = unsafe { &*pc.add(1) };
        let ext2 = unsafe { &*pc.add(2) };

        let w1 = base.imm() as u64;
        let w2 = ((ext1.a() as u64) << 16) | ((ext1.b() as u64) << 8) | (ext1.c() as u64);
        let w3 = ((ext2.a() as u64) << 16) | ((ext2.b() as u64) << 8) | (ext2.c() as u64);

        (w1 << 48) | (w2 << 24) | w3
    }
}

impl fmt::Debug for Instr {
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

#[rustfmt::skip]
#[repr(u8)]
#[derive(Opcode, Debug)]
pub enum OpCode {
    /// Immediately halts execution with Interrupt::Halt
    #[layout(Rx0)] HLT,

    /// Deep-copies rA to rB.
    #[layout(Rx2)] CPY,

    /// Dereferences rA.
    #[layout(Rx1)] FR1,

    /// Dereferences rA and rB.
    #[layout(Rx2)] FR2,

    /// Dereferences rA, rB, and rC.
    #[layout(Rx3)] FR3,

    /// Pushes rA onto the stack.
    #[layout(Rx1)] PUSH,

    /// Sets rA to local Imm's value.
    #[layout(RIm)] GETLOC,
    #[layout(RIm)] SETLOC,

    #[layout(RIm)] GETPRM,

    /// RESERVED: emitted after instructions that require 3+ operands.
    /// Encodes one extra operand into the previous instruction.
    #[layout(Rx1)] EARG1,

    /// RESERVED: emitted after instructions that require 3+ operands.
    /// Encodes two extra operands into the previous instruction.
    #[layout(Rx2)] EARG2,

    /// RESERVED: emitted after instructions that require 3+ operands.
    /// Encodes tree extra operands into the previous instruction.
    #[layout(Rx3)] EARG3,

    /// Sets rA to ().
    #[layout(Rx1)] LDU,

    /// Sets rA to true.
    #[layout(Rx1)] LDT,

    /// Sets rA to false.
    #[layout(Rx1)] LDF,

    /// Sets rA to Imm interpreted as int16.
    #[layout(RIm)] LDI16,

    /// Sets rA to Imm + EARG2 interpreted as int32.
    #[layout(RIm)] LDI32,

    /// Sets rA to Imm + EARG3 + EARG3 as int64.
    #[layout(RIm)] LDI64,

    /// Sets rA to Imm + EARG2 as float32.
    #[layout(RIm)] LDF32,

    /// Sets rA to Imm + EARG3 + EARG3 as float64.
    #[layout(RIm)] LDF64,

    #[layout(Rx2)] IFCONV,
    #[layout(Rx2)] FICONV,

    #[layout(Rx3)] IADD,
    #[layout(Rx3)] FADD,
    #[layout(Rx3)] IFADD,

    #[layout(Rx3)] ISUB,
    #[layout(Rx3)] FSUB,
    #[layout(Rx3)] IFSUB,
    #[layout(Rx3)] FISUB,

    #[layout(Rx3)] IMUL,
    #[layout(Rx3)] FMUL,
    #[layout(Rx3)] IFMUL,

    #[layout(Rx3)] IDIV,
    #[layout(Rx3)] FDIV,
    #[layout(Rx3)] IFDIV,
    #[layout(Rx3)] FIDIV,

    #[layout(Rx3)] IEXP,
    #[layout(Rx3)] FEXP,
    #[layout(Rx3)] IFEXP,
    #[layout(Rx3)] FIEXP,

    #[layout(Rx3)] IREM,
    #[layout(Rx3)] FREM,
}

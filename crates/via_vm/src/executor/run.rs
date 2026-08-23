use std::ops::{Add, Div, Mul, Rem, Sub};

use super::{Executor, interrupt::Interrupt};
use crate::{
    heap::Alloc,
    instruction::{Instr, OpCode},
    stack::{Slot, SlotKind},
};

macro_rules! copy {
    ($a:expr, $regs:expr, $pc:expr) => {{
        let src = $regs[$pc.a() as usize];
        let copy = $a.clone(src);
        $regs[$pc.a() as usize] = copy;
    }};
}

macro_rules! free {
    ($a:expr, $regs:expr, $pc:expr, $($arg:ident),+) => {{
        $(
            $a.dec_ref($regs[$pc.$arg() as usize]);
        )+
    }};
}

macro_rules! ld16 {
    ($a:expr, $regs:expr, $pc:expr) => {{
        let lit = $pc.imm() as i16 as i64;
        $regs[$pc.a() as usize] = $a.alloc(lit);
    }};
}

macro_rules! ld32 {
    ($a:expr, $regs:expr, $pc:expr, $advance:expr) => {{
        let bits = unsafe { Instr::decode_32($pc) };
        $regs[(*$pc).a() as usize] = $a.alloc(bits as i32 as i64);
        $advance = 2;
    }};
}

macro_rules! ldf32 {
    ($a:expr, $regs:expr, $pc:expr, $advance:expr) => {{
        let bits = unsafe { Instr::decode_32($pc) };
        $regs[(*$pc).a() as usize] = $a.alloc(f32::from_bits(bits) as f64);
        $advance = 2;
    }};
}

macro_rules! ld64 {
    ($a:expr, $regs:expr, $pc:expr, $advance:expr, int) => {{
        let bits = unsafe { Instr::decode_64($pc) };
        $regs[(*$pc).a() as usize] = $a.alloc(bits as i64);
        $advance = 3;
    }};
    ($a:expr, $regs:expr, $pc:expr, $advance:expr, float) => {{
        let bits = unsafe { Instr::decode_64($pc) };
        $regs[(*$pc).a() as usize] = $a.alloc(f64::from_bits(bits));
        $advance = 3;
    }};
}

macro_rules! ibin {
    ($self:expr, $a:expr, $regs:expr, $pc:expr, $op:path) => {{
        let lhs = $a.get($regs[$pc.b() as usize]).as_int();
        let rhs = $a.get($regs[$pc.c() as usize]).as_int();
        $regs[$pc.a() as usize] = $a.alloc($op(lhs, rhs));
    }};
}

macro_rules! ibinf {
    ($self:expr, $a:expr, $regs:expr, $pc:expr, $op:path) => {{
        let lhs = $a.get($regs[$pc.b() as usize]).as_int();
        let rhs = $a.get($regs[$pc.c() as usize]).as_float();
        $regs[$pc.a() as usize] = $a.alloc($op(lhs as f64, rhs));
    }};
}

macro_rules! fbin {
    ($self:expr, $a:expr, $regs:expr, $pc:expr, $op:path) => {{
        let lhs = $a.get($regs[$pc.b() as usize]).as_float();
        let rhs = $a.get($regs[$pc.c() as usize]).as_float();
        $regs[$pc.a() as usize] = $a.alloc($op(lhs, rhs));
    }};
}

macro_rules! fbini {
    ($self:expr, $a:expr, $regs:expr, $pc:expr, $op:path) => {{
        let lhs = $a.get($regs[$pc.b() as usize]).as_float();
        let rhs = $a.get($regs[$pc.c() as usize]).as_int();
        $regs[$pc.a() as usize] = $a.alloc($op(lhs, rhs as f64));
    }};
}

macro_rules! cmp {
    ($self:expr, $a:expr, $regs:expr, $pc:expr, $as_ty:ident, $op:tt) => {{
        let lhs = $a.get($regs[$pc.b() as usize]).$as_ty();
        let rhs = $a.get($regs[$pc.c() as usize]).$as_ty();
        $regs[$pc.a() as usize] = $a.alloc(lhs $op rhs);
    }};
}

fn ipow(a: i64, b: i64) -> i64 {
    a.pow(b as u32)
}

impl Executor<'_> {
    pub fn run(&mut self) -> Interrupt {
        use OpCode::*;

        loop {
            let pc = *self.pc;

            let rg = &mut self.regs;
            let stk = unsafe { &mut *self.stack.get() };
            let heap = unsafe { &mut *self.heap.get() };

            let op = pc.op();
            let mut advance = 1;

            match op {
                HLT => break Interrupt::Halt,

                CPY => copy!(heap, rg, pc),
                FR1 => free!(heap, rg, pc, a),
                FR2 => free!(heap, rg, pc, a, b),
                FR3 => free!(heap, rg, pc, a, b, c),

                PUSH => {
                    let id = rg[pc.a() as usize];
                    heap.inc_ref(id);
                    stk.push(Slot {
                        kind: SlotKind::Value,
                        word: id.index() as usize,
                    });
                }

                LDU => rg[pc.a() as usize] = heap.alloc(()),
                LDT => rg[pc.a() as usize] = heap.alloc(true),
                LDF => rg[pc.a() as usize] = heap.alloc(false),

                LDI16 => ld16!(heap, rg, pc),
                LDI32 => ld32!(heap, rg, self.pc, advance),
                LDI64 => ld64!(heap, rg, self.pc, advance, int),
                LDF32 => ldf32!(heap, rg, self.pc, advance),
                LDF64 => ld64!(heap, rg, self.pc, advance, float),

                IFCONV => {
                    let ra = rg[pc.b() as usize];
                    let int = heap.get(ra).as_int();
                    rg[pc.a() as usize] = heap.alloc(int as f64);
                }

                FICONV => {
                    let ra = rg[pc.b() as usize];
                    let float = heap.get(ra).as_float();
                    rg[pc.a() as usize] = heap.alloc(float as i64);
                }

                IADD => ibin!(self, heap, rg, pc, i64::add),
                FADD => fbin!(self, heap, rg, pc, f64::add),
                IFADD => ibinf!(self, heap, rg, pc, f64::add),

                ISUB => ibin!(self, heap, rg, pc, i64::sub),
                FSUB => fbin!(self, heap, rg, pc, f64::sub),
                IFSUB => ibinf!(self, heap, rg, pc, f64::sub),
                FISUB => fbini!(self, heap, rg, pc, f64::sub),

                IMUL => ibin!(self, heap, rg, pc, i64::mul),
                FMUL => fbin!(self, heap, rg, pc, f64::mul),
                IFMUL => ibinf!(self, heap, rg, pc, f64::mul),

                IDIV => ibin!(self, heap, rg, pc, i64::div),
                FDIV => fbin!(self, heap, rg, pc, f64::div),
                IFDIV => ibinf!(self, heap, rg, pc, f64::div),
                FIDIV => fbini!(self, heap, rg, pc, f64::div),

                IEXP => ibin!(self, heap, rg, pc, ipow),
                FEXP => fbin!(self, heap, rg, pc, f64::powf),
                IFEXP => ibinf!(self, heap, rg, pc, f64::powf),
                FIEXP => fbini!(self, heap, rg, pc, f64::powf),

                IREM => ibin!(self, heap, rg, pc, i64::rem),
                FREM => fbin!(self, heap, rg, pc, f64::rem_euclid),

                EARG1 | EARG2 | EARG3 => {
                    panic!("executor fault: executed reserved opcode: {:?}", op)
                }
                _ => unimplemented!("unimplemented opcode: {:?}", op),
            }

            // Advance the PC by the determined amount (1 normally, 2 for multi-slot constants)
            self.pc = unsafe { &*(self.pc as *const Instr).add(advance) };
        }
    }
}

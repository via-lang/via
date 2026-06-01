use super::{Executor, interrupt::Interrupt};
use crate::{
    heap::Alloc,
    instruction::{Instruction, OpCode},
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

macro_rules! ibin {
    ($self:expr, $a:expr, $regs:expr, $pc:expr, $op:tt) => {{
        let lhs = $a.get($regs[$pc.b() as usize]).as_int();
        let rhs = $a.get($regs[$pc.c() as usize]).as_int();
        $regs[$pc.a() as usize] = $a.alloc(lhs $op rhs);
    }};
}

macro_rules! fbin {
    ($self:expr, $a:expr, $regs:expr, $pc:expr, $op:tt) => {{
        let lhs = $a.get($regs[$pc.b() as usize]).as_float();
        let rhs = $a.get($regs[$pc.c() as usize]).as_float();
        $regs[$pc.a() as usize] = $a.alloc(lhs $op rhs);
    }};
}

macro_rules! cmp {
    ($self:expr, $a:expr, $regs:expr, $pc:expr, $as_ty:ident, $op:tt) => {{
        let lhs = $a.get($regs[$pc.b() as usize]).$as_ty();
        let rhs = $a.get($regs[$pc.c() as usize]).$as_ty();
        $regs[$pc.a() as usize] = $a.alloc(lhs $op rhs);
    }};
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
                Halt => break Interrupt::Halt,

                Copy => copy!(heap, rg, pc),
                Free1 => free!(heap, rg, pc, a),
                Free2 => free!(heap, rg, pc, a, b),
                Free3 => free!(heap, rg, pc, a, b, c),

                Push => {
                    let id = rg[pc.a() as usize];
                    heap.inc_ref(id);
                    stk.push(Slot {
                        kind: SlotKind::Value,
                        word: id.index() as usize,
                    });
                }

                LoadNone => rg[pc.a() as usize] = heap.alloc(()),
                LoadTrue => rg[pc.a() as usize] = heap.alloc(true),
                LoadFalse => rg[pc.a() as usize] = heap.alloc(false),

                LoadI16 => {
                    let lit = pc.imm() as i16 as i64;
                    rg[pc.a() as usize] = heap.alloc(lit);
                }
                LoadI32 => {
                    let lit = pc.imm() as i16 as i32 as i64;
                    rg[pc.a() as usize] = heap.alloc(lit);
                }
                LoadI64 | LoadF64 => {
                    let extra = unsafe { &*(self.pc as *const Instruction).add(1) };

                    let hi = pc.imm() as u32;
                    let lo = (extra.a() as u32) << 8 | (extra.b() as u32);

                    let val = ((hi << 16) | lo) as u64;

                    if matches!(op, LoadI64) {
                        rg[pc.a() as usize] = heap.alloc(val as i64);
                    } else {
                        rg[pc.a() as usize] = heap.alloc(f64::from_bits(val));
                    }

                    advance = 2;
                }

                IAdd => ibin!(self, heap, rg, pc, +),
                ISub => ibin!(self, heap, rg, pc, -),
                IMul => ibin!(self, heap, rg, pc, *),
                IDiv => ibin!(self, heap, rg, pc, /),
                IMod => ibin!(self, heap, rg, pc, %),

                FAdd => fbin!(self, heap, rg, pc, +),
                FSub => fbin!(self, heap, rg, pc, -),
                FMul => fbin!(self, heap, rg, pc, *),
                FDiv => fbin!(self, heap, rg, pc, /),
                FMod => fbin!(self, heap, rg, pc, %),

                ILt => cmp!(self, heap, rg, pc, as_int, <),
                FLt => cmp!(self, heap, rg, pc, as_float, <),
                ILtEq => cmp!(self, heap, rg, pc, as_int, <=),
                FLtEq => cmp!(self, heap, rg, pc, as_float, <=),
                IGt => cmp!(self, heap, rg, pc, as_int, >),
                FGt => cmp!(self, heap, rg, pc, as_float, >),
                IGtEq => cmp!(self, heap, rg, pc, as_int, >=),
                FGtEq => cmp!(self, heap, rg, pc, as_float, >=),
                IEq => cmp!(self, heap, rg, pc, as_int, ==),
                FEq => cmp!(self, heap, rg, pc, as_float, ==),

                Not | BitNot => {
                    let val = heap.get(rg[pc.b() as usize]).as_int();
                    rg[pc.a() as usize] = heap.alloc(!val);
                }
                INeg => {
                    let val = heap.get(rg[pc.b() as usize]).as_int();
                    rg[pc.a() as usize] = heap.alloc(-val);
                }
                FNeg => {
                    let val = heap.get(rg[pc.b() as usize]).as_float();
                    rg[pc.a() as usize] = heap.alloc(-val);
                }

                ExtraArg1 | ExtraArg2 | ExtraArg3 => {
                    panic!("VM fault: executed reserved ExtraArg opcode: {:?}", op)
                }
                _ => unimplemented!("unimplemented opcode: {:?}", op),
            }

            // Advance the PC by the determined amount (1 normally, 2+ for extra args)
            self.pc = unsafe { &*(self.pc as *const Instruction).add(advance) };
        }
    }
}

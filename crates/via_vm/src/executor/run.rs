use super::{Executor, interrupt::Interrupt, macros::launder_mut};
use crate::{
    heap::{Alloc, Heap},
    instr::{Instr, Op},
    stack::{Slot, SlotKind},
};

macro_rules! copy {
    ($a:expr, $regs:expr, $pc:expr) => {{
        let src = unsafe { $regs[$pc.a() as usize].assume_init_read() };
        let copy = $a.clone(src);
        $regs[$pc.a() as usize].write(copy);
    }};
}

macro_rules! free {
    ($a:expr, $regs:expr, $pc:expr, $($arg:ident),+) => {{
        $(
            $a.dec_ref(unsafe { $regs[$pc.$arg() as usize].assume_init_read() });
        )+
    }};
}

macro_rules! ibin {
    ($self:expr, $a:expr, $regs:expr, $pc:expr, $op:tt) => {{
        let lhs = $a.get(unsafe { $regs[$pc.b() as usize].assume_init_read() });
        let rhs = $a.get(unsafe { $regs[$pc.c() as usize].assume_init_read() });

        let result = lhs.as_int() $op rhs.as_int();
        let value = $a.alloc(result);

        $regs[$pc.a() as usize].write(value);
    }};
}

impl Executor<'_> {
    pub fn run(&mut self) -> Interrupt {
        use Op::*;

        loop {
            let pc = *self.pc;

            dbg!(&pc);

            let rg = &mut self.regs;
            let stk = &mut self.stack;
            let heap: &mut Heap = launder_mut!(&mut self.heap);

            let op = pc.op();
            match op {
                Halt => break Interrupt::Halt,

                Copy => copy!(heap, rg, pc),
                Free1 => free!(heap, rg, pc, a),
                Free2 => free!(heap, rg, pc, a, b),
                Free3 => free!(heap, rg, pc, a, b, c),

                Push => {
                    let id = unsafe { rg[pc.a() as usize].assume_init_read() };
                    heap.inc_ref(id);
                    stk.push(Slot {
                        kind: SlotKind::Value,
                        word: id.0 as usize,
                    });
                }

                LoadNone => {
                    rg[pc.a() as usize].write(heap.alloc(()));
                }
                LoadTrue => {
                    rg[pc.a() as usize].write(heap.alloc(true));
                }
                LoadFalse => {
                    rg[pc.a() as usize].write(heap.alloc(false));
                }
                LoadI32 => {
                    let hi = pc.b() as u32;
                    let lo = pc.c() as u32;

                    let lit = ((hi << 16) | lo) as i32 as i64;
                    let id = heap.alloc(lit);

                    heap.inc_ref(id);
                    rg[pc.a() as usize].write(id);
                }

                IAdd => ibin!(self, heap, rg, pc, +),

                ExtraArg1 | ExtraArg2 | ExtraArg3 => panic!("reserved opcode: {:?}", op),
                _ => unimplemented!("unimplemented opcode: {:?}", op),
            }

            self.pc = unsafe { &*(self.pc as *const Instr).add(1) };
        }
    }
}

use super::{Executor, interrupt::Interrupt, macros::launder_mut};
use crate::{
    arena::{FALSE, NONE, TRUE, ValueArena},
    instr::{Instr, Op},
    stack::slot::{Slot, SlotKind},
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
        let value = $a.int(result);

        $regs[$pc.a() as usize].write(value);
    }};
}

impl Executor<'_> {
    pub fn run(&mut self) -> Interrupt {
        dbg!(dbg!(self).__run())
    }

    fn __run(&mut self) -> Interrupt {
        use Op::*;

        loop {
            let pc = *self.pc;
            let r = &mut self.regs;
            let s = &mut self.stack;
            let a: &mut ValueArena = launder_mut!(&mut self.arena);

            let op = pc.op();
            match op {
                Halt => break Interrupt::Halt,

                Copy => copy!(a, r, pc),
                Free1 => free!(a, r, pc, a),
                Free2 => free!(a, r, pc, a, b),
                Free3 => free!(a, r, pc, a, b, c),

                Push => {
                    let id = unsafe { r[pc.a() as usize].assume_init_read() };
                    a.inc_ref(id);
                    s.push(Slot {
                        kind: SlotKind::Value,
                        word: id.0 as usize,
                    });
                }

                LoadNone => {
                    r[pc.a() as usize].write(NONE);
                }
                LoadTrue => {
                    r[pc.a() as usize].write(TRUE);
                }
                LoadFalse => {
                    r[pc.a() as usize].write(FALSE);
                }
                LoadI32 => {
                    let hi = pc.b() as u32;
                    let lo = pc.c() as u32;

                    let lit = ((hi << 16) | lo) as i32 as i64;
                    let id = a.int(lit);

                    a.inc_ref(id);
                    r[pc.a() as usize].write(id);
                }

                IAdd => ibin!(self, a, r, pc, +),

                ExtraArg1 | ExtraArg2 | ExtraArg3 => panic!("reserved opcode: {:?}", op),
                _ => unimplemented!("unimplemented opcode: {:?}", op),
            }

            self.pc = unsafe { &*(self.pc as *const Instr).add(1) };
        }
    }
}

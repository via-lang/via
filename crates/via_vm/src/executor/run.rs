use std::mem::MaybeUninit;

use super::{Executor, interrupt::Interrupt, macros::launder_mut};
use crate::{
    arena::ValueArena,
    instr::{Instr, Op},
    value::ValueRef,
};

impl<'a> Executor<'a> {
    #[inline]
    fn gr(&mut self, regs: *mut MaybeUninit<ValueRef<'a>>, r: u16) -> &'a mut ValueRef<'a> {
        debug_assert!((r as usize) < self.reg_count);
        unsafe { (*regs.add(r as usize)).assume_init_mut() }
    }

    pub fn run(&'a mut self) -> Interrupt {
        use Op::*;

        loop {
            let instr = *self.pc;
            let regs = self.regs.as_mut_ptr();
            let arena: &mut ValueArena = launder_mut!(&mut self.arena);

            match instr.op() {
                Halt => break Interrupt::Halt,
                Copy => {
                    let src = self.gr(regs, instr.b()).clone();
                    let copy = arena.clone(src);

                    self.gr(regs, instr.a()).replace(copy);
                }
                Free1 => {
                    let a = self.gr(regs, instr.a()).clone();
                    drop(a);
                }
                ExtraArg1 | ExtraArg2 | ExtraArg3 => panic!("reserved opcode"),
                _ => unimplemented!("unimplemented opcode"),
            }

            let ptr = self.pc as *const Instr;
            self.pc = unsafe { &*ptr.add(1) };
        }
    }
}

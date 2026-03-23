use super::{
    Executor,
    interrupt::Interrupt,
    macros::{a, r},
};
use crate::{
    instr::{Instr, Op},
    value_arena::ValueArena,
};

impl<'a, const A: usize, const S: usize, const R: usize> Executor<'a, A, S, R> {
    pub fn run(&'a mut self) -> Interrupt {
        use Op::*;

        loop {
            let instr = *self.pc;
            let regs = self.regs.as_mut_ptr();
            let arena = &mut self.arena as *mut ValueArena<'a, _>;

            match instr.op() {
                Halt => break Interrupt::Halt,
                Copy => {
                    let src = r!(regs, instr.b()).clone();
                    let copy = a!(arena).clone(src);

                    r!(regs, instr.a()).replace(copy);
                }
                Free1 => {
                    let a = r!(regs, instr.a()).clone();
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

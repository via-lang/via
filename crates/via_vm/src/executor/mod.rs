mod error;
mod interrupt;
mod macros;
mod run;

use std::mem::MaybeUninit;

use crate::{instr::Instr, stack::Stack, value::ValueRef, value_arena::ValueArena};

pub const ARENA_SIZE: usize = 1024 * 512;
pub const STACK_SIZE: usize = 1024 * 1024;
pub const REGISTER_SIZE: usize = 256;

#[derive(Debug)]
pub struct Executor<
    'a,
    const A: usize = ARENA_SIZE,
    const S: usize = STACK_SIZE,
    const R: usize = REGISTER_SIZE,
> {
    pc: &'a Instr,
    regs: Box<[MaybeUninit<ValueRef<'a>>; R]>,
    stack: Stack<S>,
    arena: ValueArena<'a, A>,
}

impl<'a, const A: usize, const S: usize, const R: usize> Executor<'a, A, S, R> {
    pub fn new(code: &'a [Instr]) -> Self {
        Self {
            pc: code.first().expect("bytecode cannot be empty"),
            regs: Box::new(std::array::from_fn(|_| MaybeUninit::uninit())),
            stack: Stack::new(),
            arena: ValueArena::new(),
        }
    }
}

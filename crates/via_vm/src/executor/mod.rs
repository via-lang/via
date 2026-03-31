mod error;
mod interrupt;
mod macros;
mod run;

use std::mem::MaybeUninit;

use crate::{arena::ValueArena, instr::Instr, stack::Stack, value::ValueRef};

pub struct Config {
    pub reg_count: usize,
    pub stack_size: usize,
    pub arena_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            reg_count: 256,
            stack_size: 1024 * 1024,
            arena_size: 1024 * 512,
        }
    }
}

#[derive(Debug)]
pub struct Executor<'a> {
    pc: &'a Instr,
    regs: Box<[MaybeUninit<ValueRef<'a>>]>,
    reg_count: usize,
    stack: Stack,
    arena: ValueArena<'a>,
}

impl<'a> Executor<'a> {
    pub fn new(code: &'a [Instr], cfg: Option<Config>) -> Self {
        let cfg = cfg.unwrap_or_default();

        Self {
            pc: code.first().expect("bytecode cannot be empty"),
            regs: Box::new_uninit_slice(cfg.reg_count),
            reg_count: cfg.reg_count,
            stack: Stack::new(cfg.stack_size),
            arena: ValueArena::new(cfg.arena_size),
        }
    }
}

mod error;
mod interrupt;
mod macros;
mod run;

use std::mem::MaybeUninit;

pub use {error::*, interrupt::*};

use crate::{
    Executable,
    arena::{ValueArena, ValueId},
    instr::Instr,
    stack::Stack,
};

pub struct Config {
    pub reg_count: usize,
    pub stack_capacity: usize,
    pub arena_capacity: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // TEMPORARY VALUES
            reg_count: 32,
            stack_capacity: 128,
            arena_capacity: 64,
        }
    }
}

#[derive(Debug)]
pub struct Executor<'a> {
    pc: &'a Instr,
    regs: Box<[MaybeUninit<ValueId>]>,
    stack: Stack,
    arena: ValueArena,
}

impl<'a> Executor<'a> {
    pub fn new(exe: &'a Executable, cfg: Option<Config>) -> Self {
        let cfg = cfg.unwrap_or_default();
        Self {
            pc: exe.instrs.first().expect("empty bytecode"),
            regs: Box::new_uninit_slice(cfg.reg_count),
            stack: Stack::new(cfg.stack_capacity),
            arena: ValueArena::new(cfg.arena_capacity),
        }
    }
}

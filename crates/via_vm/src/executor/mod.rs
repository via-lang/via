mod error;
mod interrupt;
mod macros;
mod run;

use std::mem::MaybeUninit;

pub use {error::*, interrupt::*};

use crate::{
    Executable,
    heap::{Heap, ValueId},
    instr::Instr,
    stack::Stack,
    traits::Stats,
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
    heap: Heap,
}

impl<'a> Executor<'a> {
    pub fn new(exe: &'a Executable, cfg: Option<Config>) -> Self {
        let cfg = cfg.unwrap_or_default();
        Self {
            pc: exe.instrs.first().expect("empty bytecode"),
            regs: Box::new_uninit_slice(cfg.reg_count),
            stack: Stack::new(cfg.stack_capacity),
            heap: Heap::new(cfg.arena_capacity),
        }
    }

    pub fn heap(&mut self) -> &mut Heap {
        &mut self.heap
    }
}

impl Stats for Executor<'_> {
    fn reserved_bytes(&self) -> memsizes::Bytes {
        let heap_use = self.heap.reserved_bytes().count();
        let stk_use = self.stack.reserved_bytes().count();
        (heap_use + stk_use).into()
    }

    fn total_bytes(&self) -> memsizes::Bytes {
        let heap_use = self.heap.total_bytes().count();
        let stk_use = self.stack.total_bytes().count();
        let reg_use = (self.regs.len() * size_of::<MaybeUninit<ValueId>>()) as u64;
        ((heap_use + stk_use + reg_use) as u64).into()
    }

    fn used_bytes(&self) -> memsizes::Bytes {
        let heap_use = self.heap.used_bytes().count();
        let stk_use = self.stack.used_bytes().count();
        let reg_use = (self.regs.len() * size_of::<MaybeUninit<ValueId>>()) as u64;
        ((heap_use + stk_use + reg_use) as u64).into()
    }
}

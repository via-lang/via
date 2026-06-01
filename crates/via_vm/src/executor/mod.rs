mod error;
mod interrupt;
mod run;

use std::{cell::UnsafeCell, fmt, mem::MaybeUninit};

pub use {error::*, interrupt::*};

use crate::{
    Executable,
    heap::{Handle, Heap},
    instruction::Instruction,
    stack::Stack,
    stats::Stats,
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
            reg_count: 16,
            stack_capacity: 64,
            arena_capacity: 32,
        }
    }
}

pub struct Executor<'a> {
    pc: &'a Instruction,
    regs: Box<[Handle]>,
    stack: UnsafeCell<Stack>,
    heap: UnsafeCell<Heap>,
}

impl<'a> Executor<'a> {
    pub fn new(exe: &'a Executable, cfg: Option<Config>) -> Self {
        let cfg = cfg.unwrap_or_default();
        Self {
            pc: exe.instrs.first().expect("empty bytecode"),
            regs: (0..cfg.reg_count)
                .map(|_| Handle(u32::MAX))
                .collect::<Box<[_]>>(),
            stack: UnsafeCell::new(Stack::new(cfg.stack_capacity)),
            heap: UnsafeCell::new(Heap::new(cfg.arena_capacity)),
        }
    }

    pub fn stack(&self) -> *mut Stack {
        self.stack.get()
    }

    pub fn heap(&self) -> *mut Heap {
        self.heap.get()
    }
}

impl Stats for Executor<'_> {
    fn reserved_bytes(&self) -> memsizes::Bytes {
        let heap_use = unsafe { &*self.heap.get() }.reserved_bytes().count();
        let stk_use = unsafe { &*self.stack.get() }.reserved_bytes().count();
        (heap_use + stk_use).into()
    }

    fn total_bytes(&self) -> memsizes::Bytes {
        let heap_use = unsafe { &*self.heap.get() }.total_bytes().count();
        let stk_use = unsafe { &*self.stack.get() }.total_bytes().count();
        let reg_use = (self.regs.len() * size_of::<MaybeUninit<Handle>>()) as u64;
        ((heap_use + stk_use + reg_use) as u64).into()
    }

    fn used_bytes(&self) -> memsizes::Bytes {
        let heap_use = unsafe { &*self.heap.get() }.used_bytes().count();
        let stk_use = unsafe { &*self.stack.get() }.used_bytes().count();
        let reg_use = (self.regs.len() * size_of::<MaybeUninit<Handle>>()) as u64;
        ((heap_use + stk_use + reg_use) as u64).into()
    }
}

impl fmt::Debug for Executor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: Ensure this doesn't violate aliasing rules if fmt is called
        // during a mutable borrow of the heap.
        let heap = unsafe { &*self.heap() };

        writeln!(f, "┌── EXECUTOR STATE")?;
        writeln!(f, "│   PC: <{:?}@{:p}>", self.pc, self.pc as *const _)?;
        writeln!(f, "│")?;

        writeln!(f, "├───┬── REGISTERS")?;

        let mut omitted_count = 0;
        for (i, r) in self.regs.iter().enumerate() {
            if let Some(value) = heap.get_safe(*r) {
                writeln!(f, "│   │  R{i:02}: @{:02} ⮕  {value:?}", r.0)?;
            } else {
                omitted_count += 1;
            }
        }

        if omitted_count > 0 {
            writeln!(f, "│   │  ...({omitted_count} registers skipped)")?;
        }

        writeln!(f, "│   └──")?;
        writeln!(f, "└── END STATE")?;

        Ok(())
    }
}

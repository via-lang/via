/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::mem::MaybeUninit;

use crate::{
    instr::{
        Instr::{self, *},
        Op,
    },
    stack::Stack,
    value::Value,
};

pub const STACK_COUNT: usize = 1024 * 1024;
pub const REGISTER_COUNT: usize = 256;

type RegSlot = MaybeUninit<ValueRef>;

#[derive(Debug)]
pub struct ExecError {
    pub pc: *const Instr,
    pub err: Value,
}

#[derive(Debug)]
pub enum ExecResult {
    Ok,
}

#[derive(Debug)]
pub struct Executor<'a, const S: usize = STACK_COUNT, const R: usize = REGISTER_COUNT> {
    code: &'a [Instr],
    pc: *const Instr,
    regs: Box<[RegSlot; R]>,
    stack: Stack<S>,
}

impl<'a> Executor<'a> {
    pub fn new(code: &'a [Instr]) -> Self {
        Self {
            code,
            pc: code.as_ptr(),
            regs: Box::new([]),
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self) -> ExecResult {
        use Op::*;

        loop {
            match unsafe { *self.pc }.op {
                Halt => break,
                _ => unimplemented!(),
            }
        }

        ExecResult::Ok
    }
}

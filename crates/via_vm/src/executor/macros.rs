/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

macro_rules! r {
    ($regs:ident, $id:expr) => {{
        debug_assert!(($id as usize) < R);
        unsafe { (*$regs.add($id as usize)).assume_init_mut() }
    }};
}

macro_rules! a {
    ($arena:ident) => {
        unsafe { &mut *$arena }
    };
}

pub(super) use a;
pub(super) use r;

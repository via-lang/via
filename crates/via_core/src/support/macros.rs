/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

macro_rules! bug {
    () => {
        panic!(
            "internal compiler error at {}:{}:{}",
            file!(),
            line!(),
            column!(),
        )
    };
    ($($arg:tt)*) => {
        panic!(
            "internal compiler error at {}:{}:{}: {}",
            file!(),
            line!(),
            column!(),
            format_args!($($arg)*),
        )
    };
}

pub(crate) use bug;

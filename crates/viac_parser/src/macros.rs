/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

macro_rules! yes_or_no {
    ($vis:vis $name:ident) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        $vis enum $name {
            Yes,
            No,
        }
        impl From<$name> for bool {
            fn from(value: $name) -> Self {
                value == $name::Yes
            }
        }
    };
}

pub(crate) use yes_or_no;

/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

macro_rules! ast {
    (
        $vis:vis enum $enum:ident {
            $(
                $name:ident {
                    $($field:ident : $ty:ty),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        // structs
        $(
            #[derive(Debug)]
            $vis struct $name {
                $(pub $field : $ty),*
            }
        )*

        // apply attributes once via helper
        #[derive(Debug)]
        $vis enum $enum {
            $(
                $name($name),
            )*
        }

        // lifting impls
        $(
            impl From<$name> for $enum {
                fn from(v: $name) -> Self {
                    $enum::$name(v)
                }
            }
        )*
    };
}

pub(super) use ast;

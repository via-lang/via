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
        $enum:ident {
            $(
                $name:ident {
                    $($field:ident : $ty:ty),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        $(
            #[derive(Debug, PartialEq)]
            pub struct $name {
                $(pub $field : $ty),*
            }
            impl super::node::Marker for $name {}
        )*
        #[derive(derive_more::From, Debug, PartialEq)]
        pub enum $enum {
            $(
                $name($name),
            )*
        }
        impl super::node::Marker for $enum<> {}
    };
    (
        $enum:ident {
            $(
                $name:ident($field:ident)
            ),* $(,)?
        }
    ) => {
        #[derive(derive_more::From, Debug, PartialEq)]
        pub enum $enum {
            $(
                $name($name),
            )*
        }
        impl super::node::Marker for $enum {}
    };
}

pub(super) use ast;

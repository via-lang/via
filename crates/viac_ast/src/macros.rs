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
        $(
            #[derive(Debug, PartialEq)]
            $vis struct $name {
                $(pub $field : $ty),*
            }
            impl crate::node::Ast for $name {}
        )*
        #[derive(Debug, PartialEq)]
        $vis enum $enum {
            $(
                $name($name),
            )*
        }
        impl crate::node::Ast for $enum {}
        $(
            // impl From<crate::node::Node<$name>> for $enum {
            //     fn from(v: crate::node::Node<$name>) -> Self {
            //
            //     }
            // }
            impl From<$name> for $enum {
                fn from(v: $name) -> Self {
                    $enum::$name(v)
                }
            }
        )*
    };
    (
        $vis:vis enum $enum:ident {
            $(
                $name:ident($field:ident)
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, PartialEq)]
        $vis enum $enum {
            $(
                $name($name),
            )*
        }
        impl crate::node::Ast for $enum {}
    };
}

pub(super) use ast;

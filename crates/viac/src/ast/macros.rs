/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

macro_rules! ast_id {
    ($node:ident) => {
        paste::paste! {
            #[repr(transparent)]
            #[derive(
                derive_more::From,
                Debug, Clone, Copy, PartialEq, Eq
            )]
            pub struct [<$node Id>](u32);

            impl From<[<$node Id>]> for usize {
                fn from(value: [<$node Id>]) -> usize {
                    value.0 as usize
                }
            }

            impl From<usize> for [<$node Id>] {
                fn from(value: usize) -> Self {
                    Self(value as u32)
                }
            }
        }
    };
}

macro_rules! ast_traits {
    ($node:ident) => {
        paste::paste! {
            impl super::Node for $node {
                type Id = [<$node Id>];
            }

            impl super::Id for [<$node Id>] {
                type Node = $node;

                fn inner(self) -> u32 {
                    self.0
                }

                fn get(tree: &super::Tree) -> &Vec<Self::Node> {
                    &tree.[<$node _nodes>]
                }

                fn get_mut(tree: &mut super::Tree) -> &mut Vec<Self::Node> {
                    &mut tree.[<$node _nodes>]
                }
            }
        }
    };
}

macro_rules! ast_structs {
    ($(
        $name:ident {
            $($field:ident : $ty:ty),* $(,)?
        }
    ),* $(,)?) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                pub span: crate::source::SourceSpan,
                $(pub $field : $ty),*
            }
        )*
    };
}

macro_rules! ast_struct_enum {
    ($node:ident => $($name:ident),* $(,)?) => {
        #[derive(derive_more::From, Debug, Clone)]
        pub enum $node {
            $($name($name)),*
        }

        impl $node {
            pub fn span(&self) -> crate::source::SourceSpan {
                match &self {
                    $(Self::$name(inner) => inner.span.clone()),*
                }
            }
        }
    };
}

macro_rules! ast_expr_enum {
    ($node:ident => $($name:ident($ty:ty)),* $(,)?) => {
        #[derive(derive_more::From, Debug, Clone)]
        pub enum $node {
            $( $name($ty) ),*
        }

        impl $node {
            pub fn span(&self) -> crate::source::SourceSpan {
                match &self {
                    $(Self::$name(inner) => inner.span()),*
                }
            }
        }
    };
}

macro_rules! ast {
    (
        enum $node:ident {
            $(
                $name:ident {
                    $($field:ident : $ty:ty),* $(,)?
                }
            ),* $(,)?
        }
    ) => {
        paste::paste! {
            super::macros::ast_structs! {
                $(
                    $name { $($field : $ty),* }
                ),*
            }
            super::macros::ast_struct_enum! {
                $node => $($name),*
            }
            super::macros::ast_id!($node);
            super::macros::ast_traits!($node);
        }
    };
    (
        enum $node:ident {
            $(
                $name:ident($expr:expr)
            ),* $(,)?
        }
    ) => {
        paste::paste! {
            super::macros::ast_expr_enum! {
                $node => $($name($expr)),*
            }
            super::macros::ast_id!($node);
            super::macros::ast_traits!($node);
        }
    };
}

pub(super) use ast;
pub(super) use ast_expr_enum;
pub(super) use ast_id;
pub(super) use ast_struct_enum;
pub(super) use ast_structs;
pub(super) use ast_traits;

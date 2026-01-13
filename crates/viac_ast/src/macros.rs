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
            #[derive(Debug, Eq, PartialEq)]
            $vis struct $name {
                pub span: Span,
                $(pub $field : $ty),*
            }
        )*
        $(
            impl Node for $name {
                fn span(&self) -> Span {
                    self.span
                }
            }
        )*
        #[derive(Debug, Eq, PartialEq)]
        $vis enum $enum {
            $(
                $name($name),
            )*
        }
        impl crate::node::Node for $enum {
            fn span(&self) -> Span {
                match self {
                    $(
                        $enum::$name(inner) => inner.span(),
                    )*
                }
            }
        }
        $(
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
        #[derive(Debug, Eq, PartialEq)]
        $vis enum $enum {
            $(
                $name($name),
            )*
        }
        impl crate::node::Node for $enum {
            fn span(&self) -> viac_source::span::Span {
                match self {
                    $(
                        $enum::$name(inner) => inner.span(),
                    )*
                }
            }
        }
    };
}

pub(super) use ast;

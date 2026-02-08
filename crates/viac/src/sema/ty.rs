/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

//! Semantic type module.

use bitflags::bitflags;

use crate::module::symbol::SymbolId;

bitflags! {
    /// Canonical type qualifier flags.
    ///
    /// Currently, there are two implemented qualifiers:
    /// - `Option`: represents a generic optional type (`T?`).
    /// - `Reference`: represents a referenced type (`&T`).
    ///
    /// These qualifiers are fully canonical; meaning they cannot be stacked
    /// without delegating the pre-qualified type, which is prohibited by TODO[EXXXX].
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TyQuals: u8 {
        const None = 0;
        const Option = 1 << 1;
        const Reference = 1 << 2;
    }
}

/// A canonical ID for any instantiation of any type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TyId(u32);

/// Unqualified type constructs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind {
    /// Built-in none type. (monostate)
    ///
    /// Expressed as `none`
    None,

    /// Built-in boolean type.
    ///
    /// Expressed as `bool`.
    ///
    /// An internal union between `T` and `none`.
    /// This is **not** an alias for `T | none`.
    Bool,

    /// Built-in integer type.
    ///
    /// Expressed as `int`.
    Int,

    /// Built-in floating point type.
    ///
    /// Expressed as `float`.
    Float,

    /// Built-in string type.
    ///
    /// Expressed as `string`.
    String,

    /// Built-in generic array type.
    ///
    /// Expressed as `[T]` where `T` is the generic type parameter.
    ///
    /// The type parameter has the following invariants:
    /// - Cannot be *monostate*.
    /// - Cannot have an effect qualifier (e.g. `raise` clause).
    Array(TyId),

    /// Built-in generic map type.
    ///
    /// Expressed as `{T: U}` where `T` is the generic "key" type parameter
    /// and `U` is the "value" type parameter.
    ///
    /// The "key" type parameter has the following invariants:
    /// - Cannot be *monostate*.
    /// - Cannot be referencial.
    /// - Must implement the `Hash` trait.
    ///
    /// The "value" type parameter has the following invariants:
    /// - Cannot be *monostate*.
    Map {
        key: TyId,
        value: TyId,
    },

    Function {
        result: TyId,
        params: Vec<TyId>,
    },

    User(SymbolId),
}

/// Qualified type construct.
///
/// Primary abstraction unit for representing semantic types constructs.
/// Composed of [raw type] + [qualifiers].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
    pub kind: TyKind,
    pub quals: TyQuals,
}

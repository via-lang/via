use salsa::Update;

use crate::db::Symbol;

/// A utility macro that generates a qualified HIR path from lexical Rust paths.
///
/// # Example
///
/// ```rs
/// let add_trait_path = path!(::core::ops::Add);
/// ```
#[macro_export]
macro_rules! path {
    ($db:expr, :: $($seg:ident)::+ ) => {{
        let mut segments = ::std::vec::Vec::new();
        $(
            let sym = $crate::db::IntoSymbol::into_symbol(stringify!($seg), $db);
            segments.push($crate::hir::path::PathSegment::new($db, sym));
        )+
        $crate::hir::path::Path::new($db, Some($crate::hir::path::PathHead::Absolute), segments)
    }};

    ($db:expr, super :: $($seg:ident)::+ ) => {{
        let mut segments = $crate::std::vec::Vec::new();
        $(
            let sym = $crate::db::IntoSymbol::into_symbol(stringify!($seg), $db);
            segments.push($crate::hir::path::PathSegment::new($db, sym));
        )+
        $crate::hir::path::Path::new($db, Some($crate::hir::path::PathHead::Super), segments)
    }};

    ($db:expr, super) => {
        $crate::hir::path::Path::new($db, Some($crate::hir::path::PathHead::Super), $crate::std::vec::Vec::new())
    };

    ($db:expr, $($seg:ident)::+ ) => {{
        let mut segments = ::std::vec::Vec::new();
        $(
            let sym = $crate::db::IntoSymbol::into_symbol(stringify!($seg), $db);
            segments.push($crate::hir::path::PathSegment::new($db, sym));
        )+
        $crate::hir::path::Path::new($db, None, segments)
    }};

    ($db:expr, self :: $($seg:ident)::+ ) => {{
        let mut segments = ::std::vec::Vec::new();
        $(
            let sym = $crate::db::IntoSymbol::into_symbol(stringify!($seg), $db);
            segments.push($crate::hir::path::PathSegment::new($db, sym));
        )+
        // Assuming self-relative paths don't have a specific Head, or define PathHead::Self if needed
        $crate::hir::path::Path::new($db, None, segments)
    }};

    ($db:expr, $($seg:ident)::+ ) => {{
        let mut segments = ::std::vec::Vec::new();
        $(
            let sym = $crate::db::IntoSymbol::into_symbol(stringify!($seg), $db);
            segments.push($crate::hir::path::PathSegment::new($db, sym));
        )+
        $crate::hir::path::Path::new($db, None, segments)
    }};
}

/// Represents the heading of a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Update)]
pub enum PathHead {
    /// Refers to the root namespace `::`.
    Absolute,

    /// Refers to the parent of the current module.
    Super,
}

/// Represents the segment of a path, which is comprised of a symbol and optional generic arguments.
#[salsa::interned(debug)]
pub struct PathSegment<'db> {
    pub ident: Symbol<'db>,
}

/// Represents a fully qualified path.
#[salsa::interned(debug)]
pub struct Path<'db> {
    pub head: Option<PathHead>,
    #[returns(ref)]
    pub segments: Vec<PathSegment<'db>>,
}

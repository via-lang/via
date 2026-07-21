use ordered_float::OrderedFloat;
use salsa::Update;

use super::ops::{BinaryOp, UnaryOp};
use super::pat::Pat;
use super::path::Path;
use super::stat::Body;

#[salsa::tracked(debug)]
pub struct Expr<'db> {
    #[returns(ref)]
    pub kind: ExprKind<'db>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum ExprKind<'db> {
    Unit,
    Bool(bool),
    Int(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Range {
        inclusive: bool,
        lhs: Option<Expr<'db>>,
        rhs: Option<Expr<'db>>,
    },

    Array(Vec<Expr<'db>>),
    Map(Vec<(Expr<'db>, Expr<'db>)>),
    Path(Path<'db>),

    Assign {
        lhs: Expr<'db>,
        rhs: Expr<'db>,
    },

    Unary {
        op: UnaryOp,
        expr: Expr<'db>,
    },

    Binary {
        op: BinaryOp,
        lhs: Expr<'db>,
        rhs: Expr<'db>,
    },

    Index {
        lhs: Expr<'db>,
        rhs: Expr<'db>,
    },

    Call {
        callee: Expr<'db>,
        args: Vec<Expr<'db>>,
    },

    If {
        cond: Expr<'db>,
        then_body: Body<'db>,
        else_body: Option<Body<'db>>,
    },

    For {
        pat: Pat<'db>,
        iter: Expr<'db>,
        body: Body<'db>,
    },
}

/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::super::error::Result;
use crate::ast::expr::Expr;
use crate::ast::place::Place;
use crate::ast::ty::Ty;
use crate::ast::value::Value;
use assert_matches::assert_matches;

pub fn parse_expr(src: &str) -> Result<Expr> {
    super::parse(src, |p| p.parse_expr().map(|e| e.node))
}

#[test]
fn value_none() {
    assert_matches!(parse_expr("none"), Ok(Expr::Value(Value::None(_))));
}

#[test]
fn value_boolean() {
    assert_matches!(parse_expr("true"), Ok(Expr::Value(Value::True(_))));
    assert_matches!(parse_expr("false"), Ok(Expr::Value(Value::False(_))));
}

#[test]
fn value_integer() {
    assert_matches!(parse_expr("238940"), Ok(Expr::Value(Value::Integer(_))));

    assert_matches!(
        parse_expr("000000000000000000000000000000000"),
        Ok(Expr::Value(Value::Integer(_)))
    );
}

#[test]
fn value_float() {
    assert_matches!(parse_expr("23423.00142"), Ok(Expr::Value(Value::Float(_))));

    assert_matches!(parse_expr("23423.0014.2"), Err(_));
    assert_matches!(parse_expr("123123."), Err(_));
}

#[test]
fn value_string() {
    assert_matches!(parse_expr("\"Abcdefg\""), Ok(Expr::Value(Value::String(_))));
}

#[test]
fn value_range() {
    assert_matches!(
        parse_expr("1..2"),
        Ok(Expr::Value(Value::Range(r))) => {
            assert_matches!(*r.lhs.node, Expr::Value(Value::Integer(_)));
            assert_matches!(*r.rhs.node, Expr::Value(Value::Integer(_)));
            assert!(!r.inclusive);
        }
    );

    assert_matches!(
        parse_expr("1..=2"),
        Ok(Expr::Value(Value::Range(r))) => {
            assert_matches!(*r.lhs.node, Expr::Value(Value::Integer(_)));
            assert_matches!(*r.rhs.node, Expr::Value(Value::Integer(_)));
            assert!(r.inclusive);
        }
    );

    assert_matches!(
        parse_expr("a..b"),
        Ok(Expr::Value(Value::Range(r))) => {
            assert_matches!(*r.lhs.node, Expr::Place(Place::Symbol(_)));
            assert_matches!(*r.rhs.node, Expr::Place(Place::Symbol(_)));
            assert!(!r.inclusive);
        }
    );

    assert_matches!(
        parse_expr("a..=b"),
        Ok(Expr::Value(Value::Range(r))) => {
            assert_matches!(*r.lhs.node, Expr::Place(Place::Symbol(_)));
            assert_matches!(*r.rhs.node, Expr::Place(Place::Symbol(_)));
            assert!(r.inclusive);
        }
    );
}

#[test]
fn value_tuple() {
    assert_matches!(parse_expr("(1,)"), Ok(Expr::Value(Value::Tuple(t))) => {
        assert!(t.exprs.list.len() == 1);
    });

    assert_matches!(parse_expr("(1, 2)"), Ok(Expr::Value(Value::Tuple(t))) => {
        assert!(t.exprs.list.len() == 2);
    });

    assert_matches!(parse_expr("((1,), 2)"), Ok(Expr::Value(Value::Tuple(t))) => {
        assert_matches!(&t.exprs.list[..], [a, b] => {
            assert_matches!(&a.node, Expr::Value(Value::Tuple(ti)) if ti.exprs.list.len() == 1);
            assert_matches!(&b.node, Expr::Value(Value::Integer(_)));
        });
        assert!(t.exprs.list.len() == 2);
    });

    assert_matches!(parse_expr("((1,2,3), ((2,),))"), Ok(Expr::Value(Value::Tuple(t))) => {
        assert_matches!(&t.exprs.list[..], [a, b] => {
            assert_matches!(&a.node, Expr::Value(Value::Tuple(ti)) if ti.exprs.list.len() == 3);
            assert_matches!(&b.node, Expr::Value(Value::Tuple(ti)) => {
                assert_matches!(&ti.exprs.list[..], [a] => {
                    assert_matches!(&a.node, Expr::Value(Value::Tuple(ti)) if ti.exprs.list.len() == 1);
                });
            });
        });
        assert!(t.exprs.list.len() == 2);
    });
}

#[test]
fn value_array() {
    assert_matches!(parse_expr("[]"), Ok(Expr::Value(Value::Array(a))) => {
        assert!(a.exprs.list.is_empty());
    });

    assert_matches!(parse_expr("[1]"), Ok(Expr::Value(Value::Array(a))) => {
        assert!(a.exprs.list.len() == 1);
    });

    assert_matches!(parse_expr("[1,]"), Ok(Expr::Value(Value::Array(a))) => {
        assert!(a.exprs.list.len() == 1);
    });
}

#[test]
fn value_map() {
    assert_matches!(parse_expr("{}"), Ok(Expr::Value(Value::Map(m))) => {
        assert!(m.pairs.is_empty());
    });

    assert_matches!(parse_expr("{\"a\": 1}"), Ok(Expr::Value(Value::Map(m))) => {
        assert_matches!(&m.pairs[..], [a] => {
            assert_matches!(a.0.node, Expr::Value(Value::String(_)));
            assert_matches!(a.1.node, Expr::Value(Value::Integer(_)));
        })
    });
}

#[test]
fn value_lambda() {
    assert_matches!(parse_expr("fn {}"), Ok(Expr::Value(Value::Lambda(l))) => {
        assert!(l.params.list.is_empty());
        assert!(l.body.list.is_empty());
        assert_matches!(l.result, None);
    });

    assert_matches!(parse_expr("fn -> int {}"), Ok(Expr::Value(Value::Lambda(l))) => {
        assert!(l.params.list.is_empty());
        assert!(l.body.list.is_empty());
        assert_matches!(l.result, Some(r) => {
            assert_matches!(*r.node, Ty::Builtin(_));
        });
    });

    assert_matches!(parse_expr("fn (_: int) {}"), Ok(Expr::Value(Value::Lambda(l))) => {
        assert!(l.body.list.is_empty());
        assert_matches!(l.result, None);
        assert_matches!(&l.params.list[..], [a] => {
            assert_matches!(*a.node.ty.node, Ty::Builtin(_));
        });
    });

    assert_matches!(parse_expr("fn (_: int) -> int {}"), Ok(Expr::Value(Value::Lambda(l))) => {
        assert!(l.body.list.is_empty());
        assert_matches!(l.result, Some(r) => {
            assert_matches!(*r.node, Ty::Builtin(_));
        });
        assert_matches!(&l.params.list[..], [a] => {
            assert_matches!(*a.node.ty.node, Ty::Builtin(_));
        });
    });
}

#[test]
fn value_unary() {
    assert_matches!(parse_expr("-10"), Ok(Expr::Value(Value::Unary(u))) => {
        assert_matches!(*u.expr.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("~10"), Ok(Expr::Value(Value::Unary(u))) => {
        assert_matches!(*u.expr.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("!true"), Ok(Expr::Value(Value::Unary(u))) => {
        assert_matches!(*u.expr.node, Expr::Value(Value::True(_)));
    });

    assert_matches!(parse_expr("--10"), Err(_));
    assert_matches!(parse_expr("~--!!!!!10"), Err(_));
    assert_matches!(parse_expr("------~~~~~~!!!!!!!!!!10"), Err(_));
}

#[test]
fn value_binary() {
    assert_matches!(parse_expr("1 + 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("1 - 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("1 * 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("1 / 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("1 ** 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("1 % 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("1 & 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("1 | 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("false || true"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::False(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::True(_)));
    });

    assert_matches!(parse_expr("true && true"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::True(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::True(_)));
    });

    assert_matches!(parse_expr("1 -2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
    });

    assert_matches!(parse_expr("1 + 2 * 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.lhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.rhs.node, Expr::Value(Value::Binary(bb)) => {
            assert_matches!(*bb.lhs.node, Expr::Value(Value::Integer(_)));
            assert_matches!(*bb.rhs.node, Expr::Value(Value::Integer(_)));
        });
    });

    assert_matches!(parse_expr("(1 + 2) * 2"), Ok(Expr::Value(Value::Binary(b))) => {
        assert_matches!(*b.rhs.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*b.lhs.node, Expr::Value(Value::Binary(bb)) => {
            assert_matches!(*bb.lhs.node, Expr::Value(Value::Integer(_)));
            assert_matches!(*bb.rhs.node, Expr::Value(Value::Integer(_)));
        });
    });
}

#[test]
fn value_ternary() {
    assert_matches!(parse_expr("1 if true else 2"), Ok(Expr::Value(Value::Ternary(t))) => {
        assert_matches!(*t.cond.node, Expr::Value(Value::True(_)));
        assert_matches!(*t.iftrue.node, Expr::Value(Value::Integer(_)));
        assert_matches!(*t.iffalse.node, Expr::Value(Value::Integer(_)));
    });
}

#[test]
fn value_cast() {
    assert_matches!(parse_expr("\"10\" as int"), Ok(Expr::Value(Value::Cast(c))) => {
        assert_matches!(*c.expr.node, Expr::Value(Value::String(_)));
        assert_matches!(*c.ty.node, Ty::Builtin(_));
    });
}

/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod attr;
pub mod aux;
pub mod control;
pub mod decl;
pub mod expr;
pub mod macros;
pub mod place;
pub mod stmt;
pub mod ty;
pub mod value;

use attr::Attr;
use aux::Param;
use control::Control;
use decl::Decl;
use expr::Expr;
use place::Place;
use stmt::Stmt;
use ty::Ty;
use value::Value;

macro_rules! tree {
    ($($name:ident),*) => {
        paste::paste! {
            #[derive(Default, Debug)]
            #[allow(non_snake_case)]
            pub struct Tree {
                pub stmts: Vec<stmt::StmtId>,
                $(
                    [<$name _nodes>]: Vec<$name>
                ),*
            }
        }
    };
}

tree! { Attr, Control, Decl, Expr, Place, Stmt, Ty, Value, Param }

impl Tree {
    pub fn get<I: Id>(&self, id: I) -> &I::Node {
        <I as Id>::get(self).get(id.inner() as usize).unwrap()
    }

    // NOTE: This can implement interning behavior in the future (tuff)
    pub fn insert<I: Id>(&mut self, node: I::Node) -> I {
        let nodes = <I as Id>::get_mut(self);
        let index = nodes.len();
        nodes.insert(index, node);
        From::from(index)
    }
}

pub trait Node {
    type Id: Id;
}

pub trait Id: From<usize> {
    type Node: Node;

    fn inner(self) -> u32;
    fn get(tree: &Tree) -> &Vec<Self::Node>;
    fn get_mut(tree: &mut Tree) -> &mut Vec<Self::Node>;
}

/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use viac_source::span::Span;

pub trait Ast: PartialEq {}

pub trait IntoNode<T: Ast> {
    fn into_node(self) -> Node<T>;
}

#[derive(Debug, Eq)]
pub struct Node<T: Ast> {
    pub node: T,
    pub span: Span,
}

impl<T: Ast> Node<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }

    pub fn map<U: Ast>(self, f: impl FnOnce(T) -> U) -> Node<U> {
        Node {
            node: f(self.node),
            span: self.span,
        }
    }
}

impl<T: Ast, U: Ast + From<T>> IntoNode<U> for Node<T> {
    fn into_node(self) -> Node<U> {
        Node {
            node: self.node.into(),
            span: self.span,
        }
    }
}

impl<T: Ast> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl<T: Ast> Into<NodeRef<T>> for Node<T> {
    fn into(self) -> NodeRef<T> {
        NodeRef {
            node: Box::new(self.node),
            span: self.span,
        }
    }
}

#[derive(Debug, Eq)]
pub struct NodeRef<T: Ast> {
    pub node: Box<T>,
    pub span: Span,
}

impl<T: Ast> NodeRef<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self {
            node: Box::new(node),
            span,
        }
    }
}

impl<T: Ast> PartialEq for NodeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::attr::Attr;
use crate::source::SourceSpan;

pub trait Marker: PartialEq {}

#[derive(Debug)]
pub struct Node<T: Marker> {
    pub node: T,
    pub span: SourceSpan,
    pub attrs: Option<Nodes<Attr>>,
}

impl<T: Marker> Node<T> {
    pub fn new(node: impl Into<T>, span: SourceSpan, attrs: Option<Nodes<Attr>>) -> Self {
        Self {
            node: node.into(),
            span,
            attrs,
        }
    }

    pub fn map<U: Marker>(self, f: impl FnOnce(T) -> U) -> Node<U> {
        Node::<U>::new(f(self.node), self.span, self.attrs)
    }

    pub fn recast<U>(self) -> Node<U>
    where
        U: Marker + From<T>,
    {
        Node::<U>::new(self.node, self.span, self.attrs)
    }
}

impl<T: Marker> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

#[derive(Debug)]
pub struct NodeRef<T: Marker> {
    pub node: Box<T>,
    pub span: SourceSpan,
    pub attrs: Option<Nodes<Attr>>,
}

impl<T: Marker> NodeRef<T> {
    pub fn new(node: impl Into<T>, span: SourceSpan, attrs: Option<Nodes<Attr>>) -> Self {
        Self {
            node: Box::new(node.into()),
            span,
            attrs,
        }
    }
}

impl<T: Marker> PartialEq for NodeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl<T, U> From<Node<U>> for NodeRef<T>
where
    T: Marker + From<U>,
    U: Marker,
{
    fn from(value: Node<U>) -> Self {
        Self::new(value.node, value.span, value.attrs)
    }
}

#[derive(Debug, PartialEq)]
pub struct Nodes<T: Marker> {
    pub nodes: Vec<Node<T>>,
    pub span: SourceSpan,
}

impl<T: Marker> Marker for Nodes<T> {}

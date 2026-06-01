#![allow(unused)]

use std::{hash::Hash, ops::AddAssign};

pub trait Id: Copy + Clone + PartialEq + Eq + Hash {
    type Inner: Copy + Clone + PartialEq + Eq + Hash;

    fn from_inner(inner: Self::Inner) -> Self;
    fn inner(self) -> Self::Inner;
}

#[derive(Debug)]
pub struct Counter<T: Id>(T::Inner);

impl<T: Id> Counter<T>
where
    T::Inner: Default + AddAssign<u32>,
{
    pub fn new() -> Self {
        Self(T::Inner::default())
    }

    pub fn bump(&mut self) -> T {
        let id = T::from_inner(self.0);
        self.0 += 1;
        id
    }
}

impl<T: Id> Default for Counter<T>
where
    T::Inner: Default + AddAssign<u32>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct SnapCounter<T: Id> {
    inner: T::Inner,
    snapshots: Vec<T::Inner>,
}

impl<T: Id> Default for SnapCounter<T>
where
    T::Inner: Default + From<u8> + AddAssign<T::Inner>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Id> SnapCounter<T>
where
    T::Inner: Default + From<u8> + AddAssign<T::Inner>,
{
    pub fn new() -> Self {
        Self {
            inner: T::Inner::default(),
            snapshots: Vec::new(),
        }
    }

    pub fn bump(&mut self) -> T {
        let id = T::from_inner(self.inner);
        self.inner += 1.into();
        id
    }

    pub fn save(&mut self) {
        self.snapshots.push(self.inner);
    }

    pub fn restore(&mut self) {
        if let Some(snapshot) = self.snapshots.pop() {
            self.inner = snapshot;
        }
    }

    pub fn discard(&mut self) {
        self.snapshots.pop();
    }
}

use std::{hash::Hash, ops::AddAssign};

pub trait Id
where
    Self: Clone + Copy + PartialEq + Eq + Hash,
{
    type Inner: Clone + Copy + AddAssign<u32> + From<u32>;

    fn new(inner: Self::Inner) -> Self;
    fn new_inner() -> Self::Inner {
        0u32.into()
    }
}

#[derive(Debug)]
pub struct Counter<T: Id>(T::Inner);

impl<T: Id> Counter<T> {
    pub fn new() -> Self {
        Self(T::new_inner())
    }

    pub fn bump(&mut self) -> T {
        let id = T::new(self.0);
        self.0 += 1;
        id
    }
}

impl<T: Id> Default for Counter<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct SnapCounter<T: Id> {
    inner: T::Inner,
    snapshots: Vec<T::Inner>,
}

impl<T: Id> Default for SnapCounter<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Id> SnapCounter<T> {
    pub fn new() -> Self {
        Self {
            inner: T::new_inner(),
            snapshots: Vec::new(),
        }
    }

    pub fn bump(&mut self) -> T {
        let id = T::new(self.inner);
        self.inner += 1;
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

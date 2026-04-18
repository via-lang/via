use std::mem::MaybeUninit;

use crate::{ValueId, traits::Stats};

#[derive(Debug)]
pub enum SlotKind {
    Value,
}

#[derive(Debug)]
pub struct Slot {
    #[cfg(debug_assertions)]
    pub kind: SlotKind,
    pub word: usize,
}

#[derive(Debug)]
pub struct Stack {
    inner: Box<[MaybeUninit<Slot>]>,
    capacity: usize,
    pos: usize,
}

impl Slot {
    pub fn value(id: ValueId) -> Self {
        Self {
            #[cfg(debug_assertions)]
            kind: SlotKind::Value,
            word: id.0 as usize,
        }
    }
}

impl Stack {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Box::new_uninit_slice(capacity),
            capacity,
            pos: 0,
        }
    }

    pub fn push(&mut self, value: Slot) -> *mut Slot {
        debug_assert!(self.pos < self.capacity, "stack overflow");

        let inner = &mut self.inner[self.pos];
        self.pos += 1;

        inner.write(value);
        inner.as_mut_ptr()
    }

    pub fn pop(&mut self) -> Slot {
        debug_assert_ne!(self.pos, 0, "stack underflow");
        self.pos -= 1;
        unsafe { self.inner[self.pos].assume_init_read() }
    }
}

impl Stats for Stack {
    fn reserved_bytes(&self) -> memsizes::Bytes {
        let deficit = self.capacity - self.pos;
        ((deficit * size_of::<Slot>()) as u64).into()
    }

    fn used_bytes(&self) -> memsizes::Bytes {
        ((self.pos * size_of::<Slot>()) as u64).into()
    }

    fn total_bytes(&self) -> memsizes::Bytes {
        ((self.capacity * size_of::<Slot>()) as u64).into()
    }
}

use std::mem::MaybeUninit;

use crate::{Handle, NativeClosure, stats::Stats};

#[derive(Debug)]
pub enum SlotKind {
    Value,
    NativeFrame,
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
    stk_ptr: usize,
}

impl Slot {
    pub fn value(id: Handle) -> Self {
        Self {
            #[cfg(debug_assertions)]
            kind: SlotKind::Value,
            word: id.index() as usize,
        }
    }

    pub fn native_frame(id: *const NativeClosure) -> Self {
        Self {
            #[cfg(debug_assertions)]
            kind: SlotKind::NativeFrame,
            word: id as usize,
        }
    }
}

impl Stack {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Box::new_uninit_slice(capacity),
            capacity,
            stk_ptr: 0,
        }
    }

    pub fn push(&mut self, value: Slot) -> *mut Slot {
        debug_assert!(self.stk_ptr < self.capacity, "stack overflow");

        let inner = &mut self.inner[self.stk_ptr];
        self.stk_ptr += 1;

        inner.write(value);
        inner.as_mut_ptr()
    }

    pub fn pop(&mut self) -> Slot {
        debug_assert_ne!(self.stk_ptr, 0, "stack underflow");
        self.stk_ptr -= 1;
        unsafe { self.inner[self.stk_ptr].assume_init_read() }
    }
}

impl Stats for Stack {
    fn reserved_bytes(&self) -> memsizes::Bytes {
        let deficit = self.capacity - self.stk_ptr;
        ((deficit * size_of::<Slot>()) as u64).into()
    }

    fn used_bytes(&self) -> memsizes::Bytes {
        ((self.stk_ptr * size_of::<Slot>()) as u64).into()
    }

    fn total_bytes(&self) -> memsizes::Bytes {
        ((self.capacity * size_of::<Slot>()) as u64).into()
    }
}

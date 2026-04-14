pub mod slot;

use std::mem::MaybeUninit;

use slot::Slot;

#[derive(Debug)]
pub struct Stack {
    inner: Box<[MaybeUninit<Slot>]>,
    capacity: usize,
    pos: usize,
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

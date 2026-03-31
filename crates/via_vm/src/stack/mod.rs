pub mod slot;

use std::mem::MaybeUninit;

use slot::Slot;

#[derive(Debug)]
pub struct Stack {
    sp: usize,
    size: usize,
    data: Box<[MaybeUninit<Slot>]>,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        Self {
            sp: 0,
            size,
            data: Box::new_uninit_slice(size),
        }
    }

    pub fn push(&mut self, value: Slot) -> *mut Slot {
        debug_assert!(self.sp < self.size, "stack overflow");

        let data = &mut self.data[self.sp];
        self.sp += 1;

        data.write(value);
        data.as_mut_ptr()
    }

    pub fn pop(&mut self) -> Slot {
        debug_assert_ne!(self.sp, 0, "stack underflow");
        self.sp -= 1;
        unsafe { self.data[self.sp].assume_init_read() }
    }
}

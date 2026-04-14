use crate::macros::{ice_assert, ice_panic, ice_unimplemented};

pub struct RegisterAlloc {
    words: Box<[u64]>,
}

impl RegisterAlloc {
    pub fn new(size: usize) -> Self {
        ice_assert!(
            size < u16::MAX as usize,
            "Register allocator can only be as big as maximum register space, which is u16::MAX"
        );

        Self {
            words: vec![0u64; size.div_ceil(64)].into_boxed_slice(),
        }
    }

    pub fn alloc(&mut self) -> u16 {
        for (i, w) in self.words.iter_mut().enumerate() {
            let inv = !*w;
            if inv != 0 {
                let bit = inv.trailing_zeros();
                *w |= 1u64 << bit;
                return (i * 64 + bit as usize) as u16;
            }
        }
        ice_unimplemented!("spilling not yet implemented (semantic register exhaustion)")
    }

    pub fn free(&mut self, reg: u16) {
        let offset = reg % 64;
        let index = ((reg - offset) / 64) as usize;

        match self.words.get_mut(index) {
            Some(word) => *word &= !(1u64 << offset),
            None => ice_panic!("register {reg} is invalid"),
        }
    }
}

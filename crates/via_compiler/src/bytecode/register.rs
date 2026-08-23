use via_vm::Operand;

pub struct RegisterAlloc {
    words: Box<[u64]>,
}

impl RegisterAlloc {
    pub fn new(size: usize) -> Self {
        debug_assert!(
            size <= (Operand::MAX as usize) + 1,
            "Register allocator can only be as big as maximum register space, which is Operand::MAX"
        );

        Self {
            words: vec![0u64; size.div_ceil(64)].into_boxed_slice(),
        }
    }

    pub fn alloc(&mut self) -> Operand {
        for (i, w) in self.words.iter_mut().enumerate() {
            let inv = !*w;
            if inv != 0 {
                let bit = inv.trailing_zeros();
                *w |= 1u64 << bit;
                return (i * 64 + bit as usize) as Operand;
            }
        }
        unimplemented!("spilling not yet implemented (semantic register exhaustion)")
    }

    pub fn free(&mut self, reg: Operand) {
        let offset = reg % 64;
        let index = ((reg - offset) / 64) as usize;

        match self.words.get_mut(index) {
            Some(word) => *word &= !(1u64 << offset),
            None => panic!("register {reg} is invalid"),
        }
    }
}

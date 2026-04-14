use crate::arena::ValueId;

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

impl Slot {
    pub fn value(id: ValueId) -> Self {
        Self {
            #[cfg(debug_assertions)]
            kind: SlotKind::Value,
            word: id.0 as usize,
        }
    }
}

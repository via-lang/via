use crate::value::ValueRef;

#[derive(Debug)]
pub enum SlotKind {
    Value,
    Frame,
}

#[derive(Debug)]
pub struct Slot {
    #[cfg(debug_assertions)]
    pub kind: SlotKind,
    pub ptr: usize,
}

impl Slot {
    pub fn value(ptr: *mut ValueRef) -> Self {
        Self {
            #[cfg(debug_assertions)]
            kind: SlotKind::Value,
            ptr: ptr as usize,
        }
    }

    pub fn frame(ptr: *mut ()) -> Self {
        Self {
            #[cfg(debug_assertions)]
            kind: SlotKind::Frame,
            ptr: ptr as usize,
        }
    }
}

use std::ptr::drop_in_place;

use crate::{Alloc, Heap, ValueId};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tag {
    /// Sentinel tombstone value state that signals
    /// the value heap that the slot is unoccupied.
    Dead = 0,
    /// Represents a unit value.
    None,
    /// Represents a boolean value.
    Bool,
    /// Represents a signed 64-bit integer value.
    Int,
    /// Represents an IEEE-754 double precision floating point value.
    Float,
    /// Represents a mutable heap-allocated string value.
    String,
}

#[derive(Debug)]
pub struct Value {
    control: u64,
    payload: u64,
}

pub trait IntoVia {
    fn into_via(self, a: &mut Heap) -> ValueId;
}

pub trait Cached: IntoVia {}
pub trait Allocated: IntoVia {}

const TAG_MASK: u64 = 0xFF00_0000_0000_0000;
const RC_MASK: u64 = 0x00FF_FFFF_FFFF_FFFF;

/// Packs the value tag (8 bits) and reference counter (56 bits) into a
/// control block, an 8-byte memory region which allows for 16-byte wide values.
#[inline]
fn control_block(rc: u64, tag: Tag) -> u64 {
    (rc & RC_MASK) | ((tag as u64) << 56)
}

impl Value {
    pub fn tag(&self) -> Tag {
        let raw = (self.control >> 56) as u8;
        unsafe { std::mem::transmute(raw) }
    }

    pub fn as_bool(&self) -> bool {
        debug_assert_eq!(self.tag(), Tag::Bool, "invalid as_bool");
        self.payload != 0
    }

    pub fn as_int(&self) -> i64 {
        debug_assert_eq!(self.tag(), Tag::Int, "invalid as_int");
        self.payload.cast_signed()
    }

    pub fn as_float(&self) -> f64 {
        debug_assert_eq!(self.tag(), Tag::Float, "invalid as_float");
        f64::from_bits(self.payload)
    }

    pub fn as_string(&self) -> &String {
        debug_assert_eq!(self.tag(), Tag::String, "invalid as_string");
        unsafe { &*(self.payload as *const String) }
    }

    pub fn as_string_mut(&mut self) -> &mut String {
        debug_assert_eq!(self.tag(), Tag::String, "invalid as_string");
        unsafe { &mut *(self.payload as *mut String) }
    }

    pub fn deep_clone(&self) -> Self {
        match self.tag() {
            Tag::Dead => panic!("clone called on dead value"),
            Tag::None => ().into(),
            Tag::Bool => self.as_bool().into(),
            Tag::Int => self.as_int().into(),
            Tag::Float => self.as_float().into(),
            Tag::String => self.as_string().as_str().into(),
        }
    }

    pub(crate) unsafe fn reset(&mut self) {
        debug_assert_ne!(self.tag(), Tag::Dead, "reset(): called on dead value");

        // Non-primitive types require manual destruction
        unsafe {
            #[allow(clippy::single_match)]
            match self.tag() {
                Tag::String => drop_in_place(self.payload as *mut String),
                _ => {} // Primitive; do nothing
            }
        }

        self.control = control_block(0, Tag::Dead);
    }

    pub(crate) fn inc_ref(&mut self) {
        let rc = self.control & RC_MASK;
        let tag = self.control & TAG_MASK;

        let new_rc = rc + 1;
        debug_assert!(new_rc <= RC_MASK, "RC overflow");

        self.control = new_rc | tag;
    }

    pub(crate) fn dec_ref(&mut self) -> bool {
        let rc = self.control & RC_MASK;
        let tag = self.control & TAG_MASK;

        debug_assert!(rc > 0, "RC underflow");

        let new_rc = rc - 1;
        let destruct = new_rc == 0;

        self.control = new_rc | tag;

        destruct
    }
}

impl Default for Value {
    fn default() -> Self {
        Self {
            control: control_block(0, Tag::Dead),
            payload: 0,
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self {
            control: control_block(0, Tag::None),
            payload: 0,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self {
            control: control_block(0, Tag::Bool),
            payload: value as u64,
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            control: control_block(0, Tag::Int),
            payload: value.cast_unsigned(),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self {
            control: control_block(0, Tag::Float),
            payload: value.to_bits(),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            control: control_block(0, Tag::String),
            payload: Box::into_raw(Box::new(value)) as u64,
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            control: control_block(0, Tag::String),
            payload: Box::into_raw(Box::new(value.to_string())) as u64,
        }
    }
}

impl Cached for () {}
impl Cached for bool {}
impl Allocated for i64 {}
impl Allocated for f64 {}
impl Allocated for String {}
impl Allocated for &str {}

impl<T> IntoVia for T
where
    Heap: Alloc<T>,
    Value: From<T>,
{
    fn into_via(self, heap: &mut Heap) -> ValueId {
        <Heap as Alloc<T>>::alloc(heap, self)
    }
}

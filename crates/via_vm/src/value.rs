use std::{fmt, ptr::drop_in_place};

use crate::{IntoVia, NativeClosure};

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
    NativeFn,
}

/// Packs the value tag (8 bits) and reference counter (56 bits) into a
/// control block, an 8-byte memory region which allows for 16-byte wide values.
#[derive(Debug)]
pub struct ControlBlock(u64);

pub struct Payload(pub u64);

pub struct Value {
    control: ControlBlock,
    payload: Payload,
}

pub trait Cached: IntoVia {}
pub trait Allocated: IntoVia {}

const TAG_MASK: u64 = 0xFF00_0000_0000_0000;
const RC_MASK: u64 = 0x00FF_FFFF_FFFF_FFFF;

impl ControlBlock {
    #[inline]
    fn new(tag: Tag) -> Self {
        Self((tag as u64) << 56)
    }

    #[inline]
    #[allow(unused)]
    fn with_rc(rc: u64, tag: Tag) -> Self {
        Self((rc & RC_MASK) | ((tag as u64) << 56))
    }

    #[inline]
    pub fn tag(&self) -> Tag {
        let raw = (self.0 >> 56) as u8;
        unsafe { std::mem::transmute(raw) }
    }

    #[inline]
    pub fn rc(&self) -> u64 {
        self.0 & RC_MASK
    }

    #[inline]
    pub(crate) fn inc(&mut self) {
        let rc = (self.0 & RC_MASK) + 1;
        debug_assert!(rc <= RC_MASK, "RC overflow");
        self.0 = rc | (self.0 & TAG_MASK);
    }

    #[inline]
    pub(crate) fn dec(&mut self) -> bool {
        let tag = self.0 & TAG_MASK;
        let rc = self.0 & RC_MASK;
        debug_assert!(rc > 0, "RC underflow");

        let new_rc = rc - 1;
        let destruct = new_rc == 0;

        self.0 = new_rc | tag;

        destruct
    }
}

impl Value {
    pub fn new() -> Self {
        Self {
            control: ControlBlock::new(Tag::Dead),
            payload: Payload(0),
        }
    }

    #[inline]
    pub fn tag(&self) -> Tag {
        self.control.tag()
    }

    #[inline]
    pub fn as_bool(&self) -> bool {
        debug_assert_eq!(self.tag(), Tag::Bool, "invalid as_bool");
        self.payload.0 != 0
    }

    #[inline]
    pub fn as_int(&self) -> i64 {
        debug_assert_eq!(self.tag(), Tag::Int, "invalid as_int");
        self.payload.0.cast_signed()
    }

    #[inline]
    pub fn as_float(&self) -> f64 {
        debug_assert_eq!(self.tag(), Tag::Float, "invalid as_float");
        f64::from_bits(self.payload.0)
    }

    #[inline]
    pub fn as_string(&self) -> &String {
        debug_assert_eq!(self.tag(), Tag::String, "invalid as_string");
        unsafe { &*(self.payload.0 as *const String) }
    }

    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn as_string_mut(&self) -> &mut String {
        debug_assert_eq!(self.tag(), Tag::String, "invalid as_string");
        unsafe { &mut *(self.payload.0 as *mut String) }
    }

    #[inline]
    pub fn as_nativefn(&self) -> &NativeClosure<'_> {
        debug_assert_eq!(self.tag(), Tag::NativeFn, "invalid as_nativefn");
        unsafe { &mut *(self.payload.0 as *mut &NativeClosure) }
    }

    #[inline]
    pub fn deep_clone(&self) -> Self {
        match self.tag() {
            Tag::Dead => panic!("clone called on dead value"),
            Tag::None => ().into(),
            Tag::Bool => self.as_bool().into(),
            Tag::Int => self.as_int().into(),
            Tag::Float => self.as_float().into(),
            Tag::String => self.as_string().as_str().into(),
            _ => panic!("failed to clone"),
        }
    }

    #[inline]
    pub(crate) fn inc_ref(&mut self) {
        self.control.inc();
    }

    #[inline]
    pub(crate) fn dec_ref(&mut self) -> bool {
        self.control.dec()
    }

    #[inline]
    pub(crate) unsafe fn reset(&mut self) {
        debug_assert_ne!(self.tag(), Tag::Dead, "reset(): called on dead value");

        // Non-primitive types require manual destruction
        unsafe {
            #[allow(clippy::single_match)]
            match self.tag() {
                Tag::String => drop_in_place(self.payload.0 as *mut String),
                Tag::NativeFn => drop_in_place(self.payload.0 as *mut NativeClosure),
                _ => {} // Primitive; do nothing
            }
        }

        self.control = ControlBlock::new(Tag::Dead);
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::new()
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self {
            control: ControlBlock::new(Tag::None),
            payload: Payload(0),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self {
            control: ControlBlock::new(Tag::Bool),
            payload: Payload(value as u64),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            control: ControlBlock::new(Tag::Int),
            payload: Payload(value.cast_unsigned()),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self {
            control: ControlBlock::new(Tag::Float),
            payload: Payload(value.to_bits()),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            control: ControlBlock::new(Tag::String),
            payload: Payload(Box::into_raw(Box::new(value)) as u64),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            control: ControlBlock::new(Tag::String),
            payload: Payload(Box::into_raw(Box::new(value.to_string())) as u64),
        }
    }
}

impl From<NativeClosure<'_>> for Value {
    fn from(value: NativeClosure) -> Self {
        Self {
            control: ControlBlock::new(Tag::NativeFn),
            payload: Payload(Box::into_raw(Box::new(value)) as u64),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = self.tag();
        write!(
            f,
            "{tag:?}[{} rc={}]",
            match tag {
                Tag::Dead => "Dead".to_owned(),
                Tag::None => "Unit".to_owned(),
                Tag::Bool => format!("{}", self.as_bool()),
                Tag::Int => format!("{}", self.as_int()),
                Tag::Float => format!("{}", self.as_float()),
                Tag::String => self.as_string().clone(),
                Tag::NativeFn => format!("{:?}", self.as_nativefn()),
            },
            self.control.rc()
        )
    }
}

impl Cached for () {}
impl Cached for bool {}
impl Allocated for i64 {}
impl Allocated for f64 {}
impl Allocated for String {}
impl Allocated for &str {}

#![allow(unused)]

// SAFETY: `launder!` and `launder_mut!` exist to bypass the borrow checker's
// covariance restrictions where aliasing is statically impossible by construction
// (e.g. distinct fields of a struct) even when it is sound.
// Only use when you own the aliasing proof.
// Undisciplined use is catastrophic UB. You have been warned.

/// Launder an immutable reference to bypass covariance restrictions.
/// Only use when you have a proof that the resulting reference is not aliased.
macro_rules! launder {
    ($thing:expr) => {
        unsafe { &*($thing as *const _) }
    };
}

/// Launder a mutable reference to bypass covariance restrictions.
/// Only use when you have a proof that the resulting reference is not aliased.
macro_rules! launder_mut {
    ($thing:expr) => {
        unsafe { &mut *($thing as *mut _) }
    };
}

pub(super) use {launder, launder_mut};

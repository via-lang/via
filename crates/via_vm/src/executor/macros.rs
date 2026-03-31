macro_rules! launder {
    ($thing:expr) => {
        unsafe { &*($thing as *const _) }
    };
}

macro_rules! launder_mut {
    ($thing:expr) => {
        unsafe { &mut *($thing as *mut _) }
    };
}

pub(super) use launder;
pub(super) use launder_mut;

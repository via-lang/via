#![allow(unused)]

macro_rules! ice_panic {
    ($($arg:tt)*) => {{
        eprintln!("internal compiler error: {}", format_args!($($arg)*));
        eprintln!("this is a bug in viac, please report it at: https://github.com/via-lang/via/issues");
        unsafe { std::env::set_var("RUST_BACKTRACE", "1") };
        panic!("aborting due to ICE");
    }};
}

macro_rules! ice_assert {
    ($cond:expr) => {
        if !$cond {
            crate::macros::ice_panic!("assertion failed: {}", stringify!($cond))
        }
    };
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            crate::macros::ice_panic!($($arg)*)
        }
    };
}

macro_rules! ice_assert_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            crate::macros::ice_panic!(
                "assertion failed: `{} == {}`\n  left:  {:?}\n  right: {:?}",
                stringify!($left), stringify!($right), $left, $right
            )
        }
    };
    ($left:expr, $right:expr, $($arg:tt)*) => {
        if $left != $right {
            crate::macros::ice_panic!($($arg)*)
        }
    };
}

macro_rules! ice_assert_ne {
    ($left:expr, $right:expr) => {
        if $left == $right {
            crate::macros::ice_panic!(
                "assertion failed: `{} != {}`\n  left:  {:?}\n  right: {:?}",
                stringify!($left), stringify!($right), $left, $right
            )
        }
    };
    ($left:expr, $right:expr, $($arg:tt)*) => {
        if $left == $right {
            crate::macros::ice_panic!($($arg)*)
        }
    };
}

macro_rules! ice_unreachable {
    () => {
        crate::macros::ice_panic!("reached unreachable code")
    };
    ($($arg:tt)*) => {
        crate::macros::ice_panic!("reached unreachable code: {}", format_args!($($arg)*))
    };
}

macro_rules! ice_unimplemented {
    () => {
        crate::macros::ice_panic!("reached unimplemented code")
    };
    ($($arg:tt)*) => {
        crate::macros::ice_panic!("unimplemented: {}", format_args!($($arg)*))
    };
}

pub(crate) use {
    ice_assert, ice_assert_eq, ice_assert_ne, ice_panic, ice_unimplemented, ice_unreachable,
};

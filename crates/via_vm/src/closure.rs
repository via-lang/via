use std::fmt;

use crate::{Executor, IntoVia, ValueId};

pub type NativeFn = dyn Fn(&mut Executor, Vec<ValueId>) -> ValueId;

#[allow(clippy::type_complexity)]
pub struct NativeClosure {
    ptr: Box<NativeFn>,
    upvs: Box<[ValueId]>,
}

pub trait IntoNativeFn<R> {
    fn into_native(self) -> Box<NativeFn>;
}

impl NativeClosure {
    pub fn new<R, F>(f: F, upvs: &[ValueId]) -> Self
    where
        R: IntoVia,
        F: IntoNativeFn<R>,
    {
        Self {
            upvs: Box::from(upvs),
            ptr: f.into_native(),
        }
    }
}

impl<R, F> IntoNativeFn<R> for F
where
    R: IntoVia,
    F: for<'a, 'b> Fn(&'a mut Executor<'b>, Vec<ValueId>) -> R + 'static,
{
    fn into_native(self) -> Box<NativeFn> {
        Box::new(move |e, args| -> ValueId {
            let result = self(e, args);
            result.into_via(e.heap())
        })
    }
}

impl fmt::Debug for NativeClosure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native-closure@{:p}>", self.ptr.as_ref() as *const _)
    }
}

use std::{fmt, sync::Arc};

use crate::{Executor, FromVia, Handle, Heap, IntoVia};

pub trait FromArgs {
    fn from_args(heap: &mut Heap, args: &[Handle]) -> Self;
}

pub type FastFn = fn(e: &Executor, args: &[Handle]) -> Handle;

pub type HostFn = Arc<dyn Fn(&Executor, &[Handle]) -> Handle + Send + Sync>;

pub trait NativeCallback: Fn(&Executor, &[Handle]) -> Handle {}

pub struct NativeClosure<'e> {
    e: &'e Executor<'e>,
    ptr: Box<dyn NativeCallback>,
    upvs: Box<[Handle]>,
}

pub fn create_function<'e, F, A, R>(f: F) -> Box<dyn NativeCallback + 'e>
where
    F: Fn(&Executor, A) -> R + 'e,
    A: FromArgs,
    R: IntoVia,
{
    Box::new(move |e: &Executor, args: &[Handle]| {
        let args = A::from_args(unsafe { &mut *e.heap() }, args);
        let result = f(e, args);
        result.into_via(unsafe { &mut *e.heap() })
    })
}

impl<T> FromArgs for T
where
    T: FromVia,
{
    fn from_args(heap: &mut Heap, args: &[Handle]) -> Self {
        debug_assert_eq!(args.len(), 1);
        T::from_via(heap, args[0])
    }
}

impl<T, U> FromArgs for (T, U)
where
    T: FromVia,
    U: FromVia,
{
    fn from_args(heap: &mut Heap, args: &[Handle]) -> Self {
        debug_assert_eq!(args.len(), 2);
        debug_assert_ne!(args[0], args[1]);
        (
            T::from_via(unsafe { &mut *(heap as *mut _) }, args[0]),
            U::from_via(unsafe { &mut *(heap as *mut _) }, args[1]),
        )
    }
}

impl<T> NativeCallback for T where T: Fn(&Executor, &[Handle]) -> Handle {}

impl<'e> NativeClosure<'e> {
    pub fn new(e: &'e Executor, ptr: Box<dyn NativeCallback>, upvs: &[Handle]) -> Self {
        let heap = unsafe { &mut *e.heap() };
        for upv in upvs {
            heap.dec_ref(*upv);
        }

        Self {
            e,
            ptr,
            upvs: Box::from(upvs),
        }
    }

    pub fn upvalues(&self) -> &[Handle] {
        &self.upvs
    }
}

impl Drop for NativeClosure<'_> {
    fn drop(&mut self) {
        let heap = unsafe { &mut *self.e.heap() };
        for upv in &self.upvs {
            heap.dec_ref(*upv);
        }
    }
}

impl fmt::Debug for NativeClosure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native-closure@{:p}>", self.ptr)
    }
}

use crate::{Alloc, Allocated, Handle, Heap, Tag, Value};

pub trait IntoVia {
    fn into_via(self, heap: &mut Heap) -> Handle;
}

pub trait FromVia {
    fn from_via(heap: &mut Heap, id: Handle) -> Self;
}

impl IntoVia for () {
    fn into_via(self, heap: &mut Heap) -> Handle {
        heap.alloc(())
    }
}

impl IntoVia for bool {
    fn into_via(self, heap: &mut Heap) -> Handle {
        heap.alloc(self)
    }
}

impl<T> IntoVia for T
where
    T: Into<Value> + Allocated,
    Heap: Alloc<T>,
{
    fn into_via(self, heap: &mut Heap) -> Handle {
        heap.alloc(self)
    }
}

impl FromVia for () {
    fn from_via(heap: &mut Heap, id: Handle) -> Self {
        debug_assert_eq!(heap.get(id).tag(), Tag::None)
    }
}

impl FromVia for bool {
    fn from_via(heap: &mut Heap, id: Handle) -> Self {
        heap.get(id).as_bool()
    }
}

impl FromVia for i64 {
    fn from_via(heap: &mut Heap, id: Handle) -> Self {
        heap.get(id).as_int()
    }
}

impl FromVia for f64 {
    fn from_via(heap: &mut Heap, id: Handle) -> Self {
        heap.get(id).as_float()
    }
}

impl FromVia for String {
    fn from_via(heap: &'_ mut Heap, id: Handle) -> Self {
        heap.get(id).as_string().clone()
    }
}

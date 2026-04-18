use crate::{Allocated, traits::Stats, value::Value};

mod sealed {
    pub trait Sealed {}
}

type Index = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub(crate) Index);

#[derive(Debug)]
pub struct Heap {
    inner: Vec<Value>,
    free_list: Vec<usize>,
}

pub trait Alloc<T>: sealed::Sealed
where
    Value: From<T>,
{
    fn alloc(&mut self, value: T) -> ValueId;
}

const NONE: usize = 0;
const TRUE: usize = 1;
const FALSE: usize = 2;

impl Heap {
    pub fn new(initial_capacity: usize) -> Self {
        let mut inner = Vec::with_capacity(initial_capacity);

        inner.insert(NONE, ().into());
        inner.insert(TRUE, true.into());
        inner.insert(FALSE, false.into());

        Self {
            inner,
            free_list: Vec::new(),
        }
    }

    #[inline]
    fn alloc_raw(&mut self, value: impl Into<Value>) -> ValueId {
        let value = value.into();

        if let Some(i) = self.free_list.pop() {
            self.inner[i] = value;
            return ValueId(i as u32);
        }

        let i = self.inner.len();
        self.inner.push(value);
        ValueId(i as u32)
    }

    #[inline]
    pub fn free(&mut self, id: ValueId) {
        let i = id.0 as usize;

        debug_assert_ne!(i, 0);

        let value = &mut self.inner[i];
        unsafe { value.reset() };
        *value = Value::default();

        self.free_list.push(i);
    }

    #[inline]
    pub fn clone(&mut self, value: ValueId) -> ValueId {
        let value = self.inner[value.0 as usize].deep_clone();
        self.alloc_raw(value)
    }

    #[inline]
    pub fn get(&self, id: ValueId) -> &Value {
        &self.inner[id.0 as usize]
    }

    #[inline]
    pub fn get_mut(&mut self, id: ValueId) -> &mut Value {
        &mut self.inner[id.0 as usize]
    }

    #[inline]
    pub(crate) fn inc_ref(&mut self, id: ValueId) {
        self.get_mut(id).inc_ref();
    }

    #[inline]
    pub(crate) fn dec_ref(&mut self, id: ValueId) {
        self.get_mut(id).dec_ref().then(|| self.free(id));
    }
}

impl sealed::Sealed for Heap {}

impl Alloc<()> for Heap {
    #[inline]
    fn alloc(&mut self, _: ()) -> ValueId {
        ValueId(NONE as u32)
    }
}

impl Alloc<bool> for Heap {
    #[inline]
    fn alloc(&mut self, value: bool) -> ValueId {
        ValueId(if value { TRUE } else { FALSE } as u32)
    }
}

impl<T> Alloc<T> for Heap
where
    T: Allocated,
    Value: From<T>,
{
    #[inline]
    fn alloc(&mut self, value: T) -> ValueId {
        self.alloc_raw(value)
    }
}

impl Stats for Heap {
    fn reserved_bytes(&self) -> memsizes::Bytes {
        let deficit = self.inner.capacity() - self.inner.len();
        let free_count = self.free_list.len();
        (((deficit + free_count) * size_of::<Value>()) as u64).into()
    }

    fn used_bytes(&self) -> memsizes::Bytes {
        let free_count = self.free_list.len();
        (((self.inner.len() + free_count) * size_of::<Value>()) as u64).into()
    }

    fn total_bytes(&self) -> memsizes::Bytes {
        ((self.inner.capacity() * size_of::<Value>()) as u64).into()
    }
}

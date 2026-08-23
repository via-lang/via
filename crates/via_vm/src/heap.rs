use crate::{Allocated, stats::Stats, value::Value};

mod sealed {
    pub trait Sealed {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(pub(crate) u32);

#[derive(Debug)]
pub struct Heap {
    inner: Vec<Value>,
    free_list: Vec<usize>,
}

pub trait Alloc<T: Into<Value>>: sealed::Sealed {
    fn alloc(&mut self, value: T) -> Handle;
}

const NONE: usize = 0;
const TRUE: usize = 1;
const FALSE: usize = 2;

impl Handle {
    pub(super) fn new(index: impl Into<u32>) -> Self {
        Self(index.into())
    }

    pub(crate) fn index(&self) -> u32 {
        self.0
    }
}

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

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn size(&self) -> usize {
        self.inner.len() - self.free_list.len()
    }

    #[inline]
    fn alloc_raw(&mut self, value: impl Into<Value>) -> Handle {
        let mut value = value.into();
        value.inc_ref();

        if let Some(i) = self.free_list.pop() {
            self.inner[i] = value;
            return Handle::new(i as u32);
        }

        let i = self.inner.len();
        self.inner.push(value);
        Handle::new(i as u32)
    }

    #[inline]
    pub fn free(&mut self, id: Handle) {
        let i = id.0 as usize;

        debug_assert_ne!(i, 0);

        let value = &mut self.inner[i];
        unsafe { value.reset() };
        *value = Value::default();

        self.free_list.push(i);
    }

    #[inline]
    pub fn clone(&mut self, value: Handle) -> Handle {
        let value = self.inner[value.0 as usize].deep_clone();
        self.alloc_raw(value)
    }

    #[inline]
    pub fn get(&self, id: Handle) -> &Value {
        debug_assert!(self.get_safe(id).is_some(), "handle uninitialized");
        &self.inner[id.0 as usize]
    }

    #[inline]
    pub fn get_safe(&self, id: Handle) -> Option<&Value> {
        if self.free_list.contains(&(id.0 as usize)) {
            return None;
        }
        self.inner.get(id.0 as usize)
    }

    #[inline]
    pub fn get_mut(&mut self, id: Handle) -> &mut Value {
        &mut self.inner[id.0 as usize]
    }

    #[inline]
    pub(crate) fn inc_ref(&mut self, id: Handle) {
        self.get_mut(id).inc_ref();
    }

    #[inline]
    pub(crate) fn dec_ref(&mut self, id: Handle) {
        self.get_mut(id).dec_ref().then(|| self.free(id));
    }
}

impl sealed::Sealed for Heap {}

impl Alloc<()> for Heap {
    #[inline]
    fn alloc(&mut self, _: ()) -> Handle {
        Handle::new(NONE as u32)
    }
}

impl Alloc<bool> for Heap {
    #[inline]
    fn alloc(&mut self, value: bool) -> Handle {
        Handle::new(if value { TRUE } else { FALSE } as u32)
    }
}

impl<T> Alloc<T> for Heap
where
    T: Allocated,
    Value: From<T>,
{
    #[inline]
    fn alloc(&mut self, value: T) -> Handle {
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

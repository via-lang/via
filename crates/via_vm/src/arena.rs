use crate::value::Value;

type Index = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub(crate) Index);

const NONE_INDEX: usize = 0;
const TRUE_INDEX: usize = 1;
const FALSE_INDEX: usize = 2;

pub const NONE: ValueId = ValueId(NONE_INDEX as u32);
pub const TRUE: ValueId = ValueId(TRUE_INDEX as u32);
pub const FALSE: ValueId = ValueId(FALSE_INDEX as u32);

#[derive(Debug)]
pub struct ValueArena {
    inner: Vec<Value>,
    free_list: Vec<usize>,
}

impl ValueArena {
    pub fn new(initial_capacity: usize) -> Self {
        let mut inner = Vec::with_capacity(initial_capacity);

        inner.insert(NONE_INDEX, Value::none());
        inner.insert(TRUE_INDEX, Value::bool(true));
        inner.insert(FALSE_INDEX, Value::bool(false));

        Self {
            inner,
            free_list: Vec::new(),
        }
    }

    fn alloc(&mut self, value: Value) -> ValueId {
        if let Some(i) = self.free_list.pop() {
            self.inner[i] = value;
            return ValueId(i as u32);
        }

        let i = self.inner.len();
        self.inner.push(value);
        ValueId(i as u32)
    }

    pub fn free(&mut self, id: ValueId) {
        let i = id.0 as usize;

        debug_assert_ne!(i, 0);

        self.inner[i] = Value::dead();
        self.free_list.push(i);
    }

    pub fn inc_ref(&mut self, id: ValueId) {
        self.get_mut(id).inc_ref();
    }

    pub fn dec_ref(&mut self, id: ValueId) {
        self.get_mut(id).dec_ref().then(|| self.free(id));
    }

    pub fn clone(&mut self, value: ValueId) -> ValueId {
        let value = self.inner[value.0 as usize].clone();
        self.alloc(value)
    }

    pub fn get(&self, id: ValueId) -> &Value {
        &self.inner[id.0 as usize]
    }

    pub fn get_mut(&mut self, id: ValueId) -> &mut Value {
        &mut self.inner[id.0 as usize]
    }

    pub fn int(&mut self, v: i64) -> ValueId {
        self.alloc(Value::int(v))
    }

    pub fn float(&mut self, v: f64) -> ValueId {
        self.alloc(Value::float(v))
    }

    pub fn string(&mut self, v: &str) -> ValueId {
        self.alloc(Value::string(v))
    }
}

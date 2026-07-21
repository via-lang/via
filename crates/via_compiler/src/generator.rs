pub struct Generator<T> {
    current: u32,
    _marker: std::marker::PhantomData<T>,
}

impl<T: From<u32>> Generator<T> {
    pub fn new() -> Self {
        Self {
            current: 0,
            _marker: std::marker::PhantomData,
        }
    }
    pub fn next_id(&mut self) -> T {
        let id = T::from(self.current);
        self.current += 1;
        id
    }
}

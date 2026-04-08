pub trait Access<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

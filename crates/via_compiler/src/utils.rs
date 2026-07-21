pub trait IntoType {
    fn into_type<T>(self) -> T
    where
        Self: Into<T>,
    {
        Into::<T>::into(self)
    }
}

impl<T> IntoType for T {}

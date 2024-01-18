pub trait PbVec<T> {
    fn push(&mut self, elem: T) -> Result<(), ()>;

    fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), ()>;

    fn write_slice(&mut self, slice: &[T]) -> Result<(), ()>;
}

pub trait PbString {
    fn write_str(&mut self, s: &str) -> Result<(), ()>;
}

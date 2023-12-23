pub trait Collection<T> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn insert(&mut self, data: T);
    fn contains(&self, data: T) -> bool;
}
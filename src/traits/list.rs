#[derive(Debug)]
pub enum ListResults {
    IndexTooBig,
    ListIsEmpty,
}

pub trait List<T: Clone> {
    fn remove(&mut self, index: usize) -> Result<T, ListResults>;
    fn get(&self, index: usize) -> Result<T, ListResults>;
}
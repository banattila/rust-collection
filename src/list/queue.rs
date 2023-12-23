use crate::{nodes::queue_node::QueueNode, traits::collection::Collection};

#[derive(Debug, Clone)]
pub struct Queue<T: Clone + PartialEq> {
    head: Option<Box<QueueNode<T>>>,
    tail: Option<Box<QueueNode<T>>>,
    len: usize,
}

impl<T: Clone + PartialEq> Queue<T> {

    pub fn new() -> Self {
        Self {head: None, tail: None, len: 0}
    }
}

impl<T: Clone + PartialEq> Collection<T> for Queue<T> {

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn contains(&self, data: T) -> bool {
        todo!()
    }

    fn insert(&mut self, data: T) {
        todo!()
    }
}

#[cfg(test)]

mod queue_tests {
    use super::Queue;


    #[test]
    fn create_new_queue() {
        let queue = Queue::<i8>::new();
    }
}
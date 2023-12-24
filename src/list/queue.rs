use crate::traits::{collection::Collection, list::{List, ListResults}};

use super::linked_list::LinkedList;

#[derive(Debug, Clone)]
pub struct Queue<T: Clone + PartialEq> {
    list: LinkedList<T>,
}

impl<T: Clone + PartialEq> Queue<T> {

    pub fn new() -> Self {
        Self {list: LinkedList::<T>::new()}
    }

    pub fn dequeue(&mut self) -> Result<T, ListResults> {
        let result = self.list.remove(0);
        match result {
            Err(err) => return Err(err),
            Ok(item) => {
                return Ok(item);
            },
        }
    }
}

impl<T: Clone + PartialEq> Collection<T> for Queue<T> {

    fn is_empty(&self) -> bool {
        self.list.len() == 0
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    fn contains(&self, data: T) -> bool {
        self.list.contains(data)
    }

    fn insert(&mut self, data: T) {
        self.list.append(data);
    }
}

#[cfg(test)]

mod queue_tests {
    use crate::traits::collection::Collection;
    use super::Queue;

    #[test]
    fn create_new_queue() {
        let queue = Queue::<i8>::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn insert_test_if_queue_is_empty() {
        let data = 1i8;
        let mut queue = Queue::<i8>::new();

        assert!(queue.is_empty());

        queue.insert(data);

        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn insert_test_with_many_items() {

        let mut queue = Queue::<i8>::new();
        assert!(queue.is_empty());

        for i in 0..10 {
            queue.insert(i);
        }

        assert_eq!(queue.len(), 10);


    }

    #[test]
    fn dequeue_test() {
        let mut queue = Queue::<i8>::new();

        for i in 0..10 {
            queue.insert(i);
        }

        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 10);

        match queue.dequeue() {
            Ok(item) => {
                assert_eq!(item, 0);
            },
            Err(_) => panic!("Result must be some"),
        }

        assert_eq!(queue.len(), 9);
    }

    #[test]
    fn dequeue_test_if_queue_is_empty() {
        let mut queue = Queue::<i8>::new();

        assert!(queue.is_empty());

        match queue.dequeue() {
            Ok(_) => panic!("Must be ListIsEmpty"),
            Err(_) => (),
        }
    }

    #[test]
    fn queue_contains_test_if_item_contained() {
        let data = 1i8;
        let mut queue = Queue::<i8>::new();
        queue.insert(data);
        assert!(queue.contains(data));
    }

    #[test]
    fn queue_contains_test_if_item_does_not_contained() {
        let mut queue = Queue::<i8>::new();

        queue.insert(1i8);

        assert!(!queue.contains(2i8));
    }
}
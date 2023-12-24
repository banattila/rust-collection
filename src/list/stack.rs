use crate::traits::{list::{ListResults, List}, collection::Collection};

use super::linked_list::LinkedList;

#[derive(Debug, Clone)]
pub struct Stack<T: Clone + PartialEq> {
    list: LinkedList<T>,
}

impl<T: Clone + PartialEq> Stack<T> {
    pub fn new() -> Self {
        Self {list: LinkedList::<T>::new(),}
    }

    pub fn pop(&mut self) -> Result<T, ListResults> {
        let result = self.list.remove(0);

        match result {
            Err(err) => return Err(err),
            Ok(item) => {
                return Ok(item);
            }
        }
    }
}

impl<T: Clone + PartialEq> Collection<T> for Stack<T> {
    fn contains(&self, data: T) -> bool {
        self.list.contains(data)
    }

    fn insert(&mut self, data: T) {
        self.list.insert(data);
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    fn len(&self) -> usize {
        self.list.len()
    }
}

#[cfg(test)]
mod stack_tests {
    use crate::{list::stack::Stack, traits::collection::Collection};

    
    #[test]
    fn create_stack() {
        let stack = Stack::<i8>::new();

        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_insert() {
        let data = 1i8;
        let mut stack = Stack::<i8>::new();
        assert!(stack.is_empty());

        stack.insert(data);

        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn pop_test_if_stack_is_empty() {
        let mut stack = Stack::<i8>::new();
        assert!(stack.is_empty());
        match stack.pop() {
            Ok(_) => panic!("Must be return error"),
            Err(_) => (),
        }

        assert!(stack.is_empty());
    }

    #[test]
    fn pop_if_stack_is_not_empty() {
        let data = 1i8;
        let mut stack = Stack::<i8>::new();

        assert!(stack.is_empty());

        stack.insert(data);
        assert!(!stack.is_empty());
        assert!(stack.contains(data));

        match stack.pop() {
            Err(_) => panic!("Must return with a value"),
            Ok(item) => assert_eq!(item, data),
        }

        assert!(stack.is_empty());
    }
}
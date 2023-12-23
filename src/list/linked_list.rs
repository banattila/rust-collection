use crate::{nodes::list_node::ListNode, traits::{collection::Collection, list::{List, ListResults}}};

use super::linked_list_iterator::LinkedListIterator;

#[derive(Debug, Clone)]
pub struct LinkedList<T: Clone + PartialEq> {
    head: Option<Box<ListNode<T>>>,
    len: usize,
}

impl<T: Clone + PartialEq> LinkedList<T> {

    pub fn new() -> Self {
        Self {head: None, len: 0}
    }

    pub fn iter<'a>(&self) -> LinkedListIterator<T> {
        LinkedListIterator::new(&self.head)
    }

    pub fn append(&mut self, data: T) {

        let mut current = &mut self.head;

        loop {
            match current {
                None => {
                    let new_node = ListNode::new(data.clone());
                    *current = Some(Box::new(new_node));
                    self.len += 1;
                    break;
                },
                Some(node) => {
                    current = node.mut_next();
                },
            }
        }
        
    }

    pub fn create_from_vector(vector: &Vec<T>) -> Self {
        let mut ll = LinkedList::<T>::new();

        for item in vector {
            ll.append(item.clone());
        }
        ll
    }

    pub fn create_from_other_list(other_list: &LinkedList<T>) -> Self {
        let mut ll = LinkedList::<T>::new();

        for item in other_list.iter() {
            ll.append(item);
        }

        ll
    }
}

impl<T: Clone + PartialEq> Collection<T> for LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn len(&self) -> usize {
        self.len.clone()
    }

    fn insert(&mut self, data: T) {
        let mut new_node = ListNode::new(data);
        if self.is_empty() {
            self.head = Some(Box::new(new_node));
            self.len += 1;
            return;
        }

        new_node.set_next(self.head.take());
        self.head = Some(Box::new(new_node));
        self.len += 1;
    }

    fn contains(&self, data: T) -> bool {

        let mut current = self.head.clone();
        loop {
            match current {
                None => return false,
                Some(node) if node.get_data() == data => {
                    return true;
                    
                },
                Some(node) => {
                    current = node.next_clone();

                },
            }
        }
    }
}

impl<T: Clone + PartialEq> List<T> for LinkedList<T> {
    fn remove(&mut self, index: usize) -> Result<T, ListResults> {

        if self.len == 0 {
            return Err(ListResults::ListIsEmpty);
        } else if self.len <= index {
            return Err(ListResults::IndexTooBig);
        }
        else {
            let mut current = &mut self.head;

            for _ in 0..index {
                match current {
                    None => return Err(ListResults::IndexTooBig),
                    Some(node) => {
                        current = node.mut_next();
                    },
                }
            }

            if let Some(mut node) = current.clone() {
                let next = node.mut_next();
                *current = next.take();
                self.len -= 1;
                return Ok(node.get_data());
            }
            else {
                return Err(ListResults::IndexTooBig);
            }
        }        
    }

    fn get(&self, index: usize) -> Result<T, ListResults> {
        if self.len == 0 {
            return Err(ListResults::ListIsEmpty);
        }
        else if self.len <= index {
            return Err(ListResults::IndexTooBig);
        }
        else {
            let mut current = &self.head;

            for _ in 0..index {
                match current {
                    None => return Err(ListResults::IndexTooBig),
                    Some(node) => current = node.next_ref(),
                }
            }
            
            if let Some(node) = current {
                return Ok(node.get_data());
            }
            else {
                return Err(ListResults::IndexTooBig);
            }
        }
    }
}

impl<T: Clone + PartialEq> FromIterator<T> for LinkedList<T>{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let elements: Vec<T> = iter.into_iter().collect();
        let mut list = LinkedList::<T>::new();

        for item in elements.into_iter() {
            list.insert(item);
        }
        list
    }
}

#[cfg(test)]
mod linked_list_tests {
    use crate::traits::{collection::Collection, list::{List, ListResults}};
    use super::LinkedList;

    #[test]
    fn create_list() {
        let ll = LinkedList::<i8>::new();

        assert!(ll.head.is_none());
        assert!(ll.is_empty());

        assert_eq!(ll.len(), 0);
    }

    #[test]
    fn insert_test() {
        let data_one = 1i8;
        let data_two = 2i8;
        let mut ll = LinkedList::<i8>::new();

        assert!(ll.is_empty());

        ll.insert(data_one);

        assert!(!ll.is_empty());
        assert_eq!(ll.len(), 1);


        let mut head = &ll.head;

        match &head {
            None => panic!("Must be not None"),
            Some(_node) => {
                assert!(_node.next_clone().is_none());
                assert_eq!(_node.get_data(), data_one);
            }
        }

        ll.insert(data_two);
        assert!(!ll.is_empty());
        assert_eq!(ll.len(), 2);

        head = &&ll.head;

        match &head {
            None => panic!("Must be not None"),
            Some(node) => {
                assert!(node.next_clone().is_some());
                assert_eq!(node.get_data(), data_two);
            }
        }
    }

    #[test]
    fn insert_more_data() {
        let mut ll = LinkedList::<i8>::new();
        assert!(ll.is_empty());

        for i in 0..10 {
            ll.insert(i);
        }

        assert!(!ll.is_empty());
        assert_eq!(ll.len(), 10);

        let head = ll.head;

        match head {
            None => panic!("It is must ListNode"),
            Some(mut node) => {
                assert!(node.mut_next().is_some());
                assert_eq!(node.get_data(), 9);
            }
        }
    }

    #[test]
    fn contains_if_not_contains() {
        let ll = LinkedList::<i8>::new();
        assert!(ll.is_empty());

        assert!(!ll.contains(2i8));
    }

    #[test]
    fn contains_if_contains() {
        let data = 2i8;
        let mut ll = LinkedList::<i8>::new();
        assert!(ll.is_empty());
        assert!(!ll.contains(data));

        ll.insert(data);

        assert!(!ll.is_empty());
        assert!(ll.contains(data));
    }

    #[test]
    fn remove_if_list_is_empty() {
        let mut ll = LinkedList::<i8>::new();

        assert!(ll.is_empty());

        match ll.remove(0) {
            Ok(_) => panic!("List is empty"),
            Err(err) => match err {
                ListResults::ListIsEmpty => (),
                _ => panic!("List is empty")
            }
        }
    }

    #[test]
    fn remove_if_index_too_big() {
        let data = 1i8;
        let mut ll = LinkedList::<i8>::new();

        ll.insert(data);
        assert!(!ll.is_empty());
        assert!(ll.contains(data));

        match ll.remove(1) {
            Ok(_) => panic!("Index is too big"),
            Err(err) => match err {
                ListResults::IndexTooBig => (),
                _ => panic!("Index is too big"),
            },
        }
    }

    #[test]
    fn remove_if_list_not_empty_and_index_is_ok() {
        let data = 1i8;
        let data_two = 2i8;
        let data_three = 3i8;
        let mut ll = LinkedList::<i8>::new();

        ll.insert(data);
        ll.insert(data_two);
        ll.insert(data_three);
        assert_eq!(ll.len(), 3);

        match ll.remove(1) {
            Err(_) => panic!("Must return {} value", data),
            Ok(item) => {
                assert_eq!(item, data_two);
            },
        }

        assert!(!ll.is_empty());
        assert_eq!(ll.len(), 2);
        let head = &ll.head;

        match head {
            None => panic!("Must be Some"),
            Some(item) => {
                assert_eq!(item.get_data(), data_three);
                let next = item.next_clone();

                if let Some(node) = next {
                    assert_eq!(node.get_data(), data);
                }
                else {
                    panic!("Must be not None");
                }
            },
        }
    }

    #[test]
    fn get_test_if_list_is_empty() {
        let ll = LinkedList::<i8>::new();

        assert!(ll.is_empty());

        match ll.get(0) {
            Ok(_) => panic!("Result must be ListIsEmpty error not T value"),
            Err(err) => match err {
                ListResults::ListIsEmpty => (),
                _ => panic!("Result must be ListIsEmpty error")
            }
        }
    }

    #[test]
    fn get_test_if_index_too_big() {
        let mut ll = LinkedList::<i8>::new();
        ll.insert(1i8);
        assert!(ll.contains(1i8));

        match ll.get(2) {
            Ok(_) => panic!("Result must be IndexTooBig error not T value"),
            Err(err) => match err {
                ListResults::IndexTooBig => (),
                _ => panic!("Result must be IndexTooBig error"),
            },
        }
    }

    #[test]
    fn get_test_if_list_does_not_empty_and_index_is_ok() {
        let mut ll = LinkedList::<i8>::new();
        let data = 2i8;
        ll.insert(data);
        ll.insert(1i8);

        assert!(ll.contains(data));

        match ll.get(1) {
            Err(_) => panic!("Result must be T value not error"),
            Ok(result) => assert_eq!(result, data),
        }
    }

    #[test]
    fn iterator_test() {
        let mut ll = LinkedList::<i8>::new();

        for i in 0..20 {
            ll.insert(i);
        }

        for (index, item) in ll.iter().enumerate() {
            let i: i8 = index.try_into().unwrap();
            let value = 19 - i;
            assert_eq!(item, value);
        }
    }

    #[test]
    fn iterator_test_with_filter() {
        let mut ll = LinkedList::<i8>::new();

        for i in 0..20 {
            ll.insert(i);
        }

        assert_eq!(ll.len(), 20);

        let filtered: LinkedList<i8> = ll.iter().filter(|number| number % 2 == 0).collect();

        for item in filtered.iter() {
            assert!(item % 2 == 0);
        }
    }

    #[test]
    fn append_test() {
        let mut ll = LinkedList::<i8>::new();

        assert!(ll.is_empty());

        for i in 0..20 {
            ll.append(i);
        }

        assert_eq!(ll.len(), 20);

        for (index, item) in ll.iter().enumerate() {

            assert_eq!(item, index.try_into().unwrap());
        }
    }

    #[test]
    fn create_from_vectro_test() {

        let vector: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let ll = LinkedList::create_from_vector(&vector);

        assert!(!ll.is_empty());
        assert_eq!(ll.len(), 10);

        for (index, item) in ll.iter().enumerate() {
            let mut i: i8 = index.try_into().unwrap();
            i += 1;
            assert_eq!(item, i);
        }
    }

    #[test]
    fn create_from_other_list() {
        let mut original = LinkedList::<i8>::new();

        for i in 0..20 {
            original.insert(i);
        }

        assert_eq!(original.len(), 20);

        let copied = LinkedList::create_from_other_list(&original);

        for index in 0..20 {
            let original_item = original.get(index).unwrap();
            let copied_item = copied.get(index).unwrap();

            assert_eq!(original_item, copied_item);
        }
    }
}
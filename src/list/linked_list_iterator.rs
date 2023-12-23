use crate::nodes::node::Node;

#[derive(Clone, Debug)]
pub struct LinkedListIterator<'a, T: Clone> {
    current: &'a Option<Box<Node<T>>>
}

impl<'a, T: Clone> LinkedListIterator<'a, T> {
    pub fn new(head: &'a Option<Box<Node<T>>>) -> Self {
        Self {
            current: head,
        }
    }
}

impl<'a, T: Clone> Iterator for LinkedListIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(node) => {
                let next = node.next_ref();
                self.current = &next;
                Some(node.get_data())
            }
        }
    }
}
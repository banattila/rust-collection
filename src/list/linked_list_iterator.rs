use crate::nodes::list_node::ListNode;

#[derive(Clone, Debug)]
pub struct LinkedListIterator<'a, T: Clone> {
    current: &'a Option<Box<ListNode<T>>>
}

impl<'a, T: Clone> LinkedListIterator<'a, T> {
    pub fn new(head: &'a Option<Box<ListNode<T>>>) -> Self {
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
#[derive(Debug, Clone)]
pub struct ListNode<T: Clone> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T: Clone> ListNode<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub fn get_data(&self) -> T {
        self.data.clone()
    }

    pub fn mut_next(&mut self) -> &mut Option<Box<ListNode<T>>> {
        &mut self.next
    }

    pub fn next_clone(&self) -> Option<Box<ListNode<T>>> {
        self.next.clone()
    }

    pub fn next_ref(&self) -> &Option<Box<ListNode<T>>> {
        &self.next
    }

    pub fn set_next(&mut self, other_node: Option<Box<ListNode<T>>>) {
        self.next = other_node;
    }
}

#[cfg(test)]
mod node_tests {
    use super::ListNode;

    #[test]
    fn create_node() {
        let data = 1i8;
        let data_two = 2i8;
        let mut node = ListNode::new(data);
        let node_two = ListNode::new(data_two);
        assert_eq!(node.data, data);
        assert!(node.next.is_none());

        node.next = Some(Box::new(node_two));
        assert!(node.next.is_some());

        let next = node.next;

        match next {
            None => panic!("Must be ListNode"),
            Some(node) => {
                assert!(node.next.is_none());
                assert_eq!(node.data, data_two);
            }
        }
    }

    #[test]
    fn set_next_test() {
        let data_one = 1i8;
        let data_two = 2i8;

        let mut node_one = ListNode::new(data_one);
        let node_two = ListNode::new(data_two);
        assert!(node_one.next.is_none());
        assert!(node_two.next.is_none());


        node_one.set_next(Some(Box::new(node_two)));

        assert!(node_one.next.is_some());

        let next = node_one.mut_next();

        match next {
            None => panic!("It is must ListNode"),
            Some(node) => {
                assert!(node.next.is_none());
                assert_eq!(node.data, data_two);
            }
        }
    }

    #[test]
    fn get_data_test() {
        let data = 1i8;
        let node = ListNode::new(data);

        assert_eq!(node.get_data(), data);
    }
}

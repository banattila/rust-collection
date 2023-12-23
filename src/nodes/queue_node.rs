#[derive(Debug, Clone)]
pub struct QueueNode<T: Clone + PartialEq> {
    data: T,
    next: Option<Box<QueueNode<T>>>,
    previous: Option<Box<QueueNode<T>>>,
}

impl<T: Clone + PartialEq> QueueNode<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None, previous: None,}
    }

    pub fn get_data(&self) -> T {
        self.data.clone()
    }

    pub fn next_mut(&mut self) -> &mut Option<Box<QueueNode<T>>> {
        &mut self.next
    }

    pub fn next_ref(&self) -> &Option<Box<QueueNode<T>>> {
        &self.next
    }

    pub fn next_clone(&self) -> Option<Box<QueueNode<T>>> {
        self.next.clone()
    }

    pub fn prev_mut(&mut self) -> &mut Option<Box<QueueNode<T>>> {
        &mut self.previous
    }

    pub fn prev_ref(&self) -> &Option<Box<QueueNode<T>>> {
        &self.previous
    }

    pub fn prev_clone(&self) -> Option<Box<QueueNode<T>>> {
        self.previous.clone()
    }

    pub fn set_next(&mut self, next_node: &mut Option<Box<QueueNode<T>>>) {
        if let Some(ref mut node) = next_node {
            node.previous = Some(Box::new(self.clone()));
            self.next = next_node.take();
        }
    }

    pub fn set_prev(&mut self, prev_node: &mut Option<Box<QueueNode<T>>>) {
        if let Some(ref mut node) = prev_node {
            node.next = Some(Box::new(self.clone()));
            self.previous = prev_node.take();
        }
    }
}

#[cfg(test)]
mod queue_node_tests {
    use super::QueueNode;


    #[test]
    fn create_queue_node() {
        let data = 1i8;
        let node = QueueNode::<i8>::new(data);

        assert_eq!(node.get_data(), data);
        assert!(node.next.is_none());
        assert!(node.previous.is_none());
    }

    #[test]
    fn test_set_next_node() {
        let data_one = 1i8;
        let data_two = 2i8;
        let mut node_one = QueueNode::new(data_one);
        let node_two = QueueNode::new(data_two);
        assert_eq!(node_one.get_data(), data_one);
        assert!(node_one.next_clone().is_none());
        assert!(node_one.prev_clone().is_none());

        assert_eq!(node_two.get_data(), data_two);
        assert!(node_two.next_clone().is_none());
        assert!(node_two.prev_clone().is_none());

        node_one.set_next(&mut Some(Box::new(node_two)));

        assert_eq!(node_one.get_data(), data_one);
        assert!(node_one.next_clone().is_some());
        assert!(node_one.prev_clone().is_none());

        let next = node_one.next_clone();

        if let Some(node) = next {
            assert!(node.prev_clone().is_some());
            assert_eq!(node.get_data(), data_two);
            assert!(node.next_clone().is_none());
        }
        else {
            panic!("Node must be some");
        }

    }

    #[test]
    fn test_set_previous_node() {
        let data_one = 1i8;
        let data_two = 2i8;

        let mut node_one = QueueNode::new(data_one);
        let node_two = QueueNode::new(data_two);

        assert_eq!(node_one.get_data(), data_one);
        assert!(node_one.next_clone().is_none());
        assert!(node_one.prev_clone().is_none());

        assert_eq!(node_two.get_data(), data_two);
        assert!(node_two.next_clone().is_none());
        assert!(node_two.prev_clone().is_none());

        node_one.set_prev(&mut Some(Box::new(node_two)));
        
        assert_eq!(node_one.get_data(), data_one);
        assert!(node_one.next_clone().is_none());
        assert!(node_one.prev_clone().is_some());

        let prev = node_one.prev_ref();

        if let Some(node) = prev {
            assert!(node.prev_clone().is_none());
            assert!(node.next_clone().is_some());
            assert_eq!(node.get_data(), data_two);
        }
        else {
            panic!("Node must be some");
        }
    }
}
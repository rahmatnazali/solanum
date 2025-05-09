//! Implementation of various unidirectional node

use std::rc::Rc;

/// Implementation of an immutable node with [Rc]
#[derive(Debug, PartialEq)]
pub struct ImmutableNode<T> {
    /// Holds generic value
    pub value: T,

    /// Holds optional link to the next node
    pub next: Option<Rc<ImmutableNode<T>>>,
}

impl<T> ImmutableNode<T> {
    /// Create a Node with a value and empty next reference.
    pub fn new(value: T) -> ImmutableNode<T> {
        Self { value, next: None }
    }

    /// Create a Node with a value and next reference.
    pub fn new_with_next(value: T, next_node: Rc<ImmutableNode<T>>) -> ImmutableNode<T> {
        Self {
            value,
            next: Some(Rc::clone(&next_node)),
        }
    }
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn initialize_tail_node() {
        let node = ImmutableNode::new(1);
        assert_eq!(node.value, 1);
        assert!(node.next.is_none());
    }

    #[test]
    fn initialize_node_with_next_reference() {
        let tail_node = Rc::new(ImmutableNode::new(1));
        let node = ImmutableNode::new_with_next(2, Rc::clone(&tail_node));
        assert_eq!(node.value, 2);
        assert!(node.next.is_some());
        assert_eq!(node.next.as_ref().unwrap().value, 1);
        assert_eq!(node.next.unwrap(), tail_node);
    }

    #[test]
    fn primitive_node() {
        let integer_node = ImmutableNode::new(1);
        assert_eq!(integer_node.value, 1);

        let float_node = ImmutableNode::new(0.1);
        assert_eq!(float_node.value, 0.1);

        let boolean_node = ImmutableNode::new(true);
        assert!(boolean_node.value);

        let str_node = ImmutableNode::new("hello");
        assert_eq!(str_node.value, "hello");
    }

    #[test]
    fn complex_node() {
        #[allow(dead_code)]
        struct Point {
            x: u32,
            y: u32,
        }
        let _point_node = ImmutableNode::new(Point { x: 1, y: 2 });
    }

    #[test]
    fn reference_count_in_node_next() {
        let node_1 = Rc::new(ImmutableNode::new(1));
        let node_2 = Rc::new(ImmutableNode::new_with_next(2, Rc::clone(&node_1)));

        assert_eq!(Rc::strong_count(&node_1), 2); // node_1 & being referenced by node_2.next
        assert_eq!(Rc::strong_count(&node_2), 1); // node_2
    }

    #[test]
    fn reference_count_is_reduced_after_unlink() {
        let node_1 = Rc::new(ImmutableNode::new(1));
        assert_eq!(Rc::strong_count(&node_1), 1); // node_1 itself

        {
            let _node_2 = Rc::new(ImmutableNode::new_with_next(2, Rc::clone(&node_1)));
            assert_eq!(Rc::strong_count(&node_1), 2); // node_1 & being referenced by node_2.next
        }
        // here, node_2 is dropped

        assert_eq!(Rc::strong_count(&node_1), 1); // node_1 only, as node_2 has been dropped
    }
}

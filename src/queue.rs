//! Implementation of mutable Queue with `enqueue()` and `dequeue()`.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node {
    value: Option<u32>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    /// Create an empty Node
    fn empty() -> Node {
        Self {
            value: None,
            next: None,
        }
    }

    /// Create a Node with a value and empty next reference.
    fn new(value: u32) -> Node {
        Self {
            value: Some(value),
            next: None,
        }
    }

    /// Create a Node with a value and next reference.
    fn new_with_next(value: u32, next_node: Option<Rc<RefCell<Node>>>) -> Node {
        Self {
            value: Some(value),
            next: next_node,
        }
    }

    /// Indicate whether this Node doesn't contain any value
    fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

pub struct Queue {
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn initialize_empty_node() {
        let node = Node::empty();
        assert!(node.is_empty());
        assert!(node.value.is_none());
        assert!(node.next.is_none());

        let sophisticated_node = Rc::new(RefCell::new(Node::empty()));
        let sophisticated_node_ref = sophisticated_node.borrow();
        assert!(sophisticated_node_ref.is_empty());
        assert!(sophisticated_node_ref.value.is_none());
        assert!(sophisticated_node_ref.next.is_none());
    }

    #[test]
    fn initialize_single_node() {
        let node = Node::new(1);
        assert!(node.value.is_some());
        assert_eq!(node.value.unwrap(), 1);
        assert!(node.next.is_none());

        let sophisticated_node = Rc::new(RefCell::new(Node::new(2)));
        let sophisticated_node_ref = sophisticated_node.borrow();
        assert!(sophisticated_node_ref.value.is_some());
        assert_eq!(sophisticated_node_ref.value.unwrap(), 2);
        assert!(sophisticated_node_ref.next.is_none());
    }

    #[test]
    fn initialize_node_with_next_reference() {
        let tail_node = Rc::new(RefCell::new(Node::new(1)));
        let head_node = Rc::new(RefCell::new(Node::new_with_next(2, Some(tail_node))));

        // evaluate that the queue order is as intended
        let head_node_ref = head_node.borrow();
        assert!(!head_node_ref.is_empty());
        assert!(head_node_ref.next.is_some());
        assert!(head_node_ref.value.is_some());
        assert_eq!(head_node_ref.value.unwrap(), 2);

        let tail_node_ref = head_node_ref.next.as_ref().unwrap().borrow();
        assert!(!tail_node_ref.is_empty());
        assert!(tail_node_ref.value.is_some());
        assert_eq!(tail_node_ref.value.unwrap(), 1);
        assert!(tail_node_ref.next.is_none());
    }

    #[test]
    fn borrow_next_node_to_evaluate_or_traverse() {
        let node = Rc::new(RefCell::new(Node::new(1)));

        // node.next can be borrowed many times
        assert!(node.borrow().next.is_none());
        assert!(node.borrow().next.is_none());
        assert!(node.borrow().next.is_none());

        // even as other variable
        let borrowed_reference = node.borrow();
        assert!(borrowed_reference.next.is_none());
        assert!(borrowed_reference.next.is_none());
        assert!(borrowed_reference.next.is_none());
    }

    #[test]
    fn borrow_mutable_next_node_to_modify() {
        let node = Rc::new(RefCell::new(Node::new(1)));
        assert!(node.borrow().next.is_none());

        // node.next can be modified with borrow_mut
        node.borrow_mut()
            .next
            .replace(Rc::new(RefCell::new(Node::new(2))));

        assert!(node.borrow().next.is_some());
        let node_ref = node.borrow();
        let next_node_ref = node_ref.next.as_ref().unwrap().borrow();
        assert_eq!(next_node_ref.value.as_ref().unwrap(), &2);
    }

    #[test]
    fn node_next_reference_is_removable() {
        let tail_node = Rc::new(RefCell::new(Node::new(1)));
        let head_node = Rc::new(RefCell::new(Node::new_with_next(2, Some(tail_node))));

        let mut head_node_ref = head_node.borrow_mut();
        assert!(head_node_ref.next.is_some());

        // assert head.next.value without variable assignment
        assert!(
            head_node_ref
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value
                .is_some()
        );
        assert_eq!(
            head_node_ref
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value
                .as_ref()
                .unwrap(),
            &1
        );

        // set head_node.next as None
        head_node_ref.next.take();

        // head_node.next is now None
        assert!(head_node_ref.next.is_none());
    }

    #[test]
    fn node_next_reference_is_changeable() {
        let tail_node = Rc::new(RefCell::new(Node::new(1)));
        let head_node = Rc::new(RefCell::new(Node::new(2)));

        // each node is independent
        assert!(head_node.borrow().next.is_none());
        assert!(tail_node.borrow().next.is_none());

        // head_node.next is linked to tail_node
        let mut head_node_ref = head_node.borrow_mut();
        head_node_ref.next.replace(tail_node);

        // head_node.next is now tail_node
        assert!(head_node_ref.next.is_some());
        assert!(head_node_ref.next.as_ref().unwrap().borrow().next.is_none());
        assert!(
            head_node_ref
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value
                .is_some()
        );
        assert_eq!(
            head_node_ref
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value
                .as_ref()
                .unwrap(),
            &1
        );
    }

    #[test]
    fn primitive_node() {
        let integer_node = Node::new(1);
        assert!(integer_node.value.is_some());
        assert_eq!(integer_node.value.unwrap(), 1);

        // let float_node = Node::new(0.1);
        // assert_eq!(float_node.value, 0.1);
        //
        // let boolean_node = Node::new(true);
        // assert!(boolean_node.value);
        //
        // let str_node = Node::new("hello");
        // assert_eq!(str_node.value, "hello");
    }

    // #[test]
    // fn complex_node() {
    //     #[allow(dead_code)]
    //     struct Point {
    //         x: u32,
    //         y: u32,
    //     }
    //     let _point_node = Node::new(Point { x: 1, y: 2 });
    // }

    // #[test]
    // fn reference_count_in_node_next() {
    //     let node_1 = Rc::new(Node::new(1));
    //     let node_2 = Rc::new(Node::new_with_next(2, Rc::clone(&node_1)));
    //
    //     assert_eq!(Rc::strong_count(&node_1), 2); // node_1 & being referenced by node_2.next
    //     assert_eq!(Rc::strong_count(&node_2), 1); // node_2
    // }
    //
    // #[test]
    // fn reference_count_is_reduced_after_unlink() {
    //     let node_1 = Rc::new(Node::new(1));
    //     assert_eq!(Rc::strong_count(&node_1), 1); // node_1 itself
    //
    //     {
    //         let _node_2 = Rc::new(Node::new_with_next(2, Rc::clone(&node_1)));
    //         assert_eq!(Rc::strong_count(&node_1), 2); // node_1 & being referenced by node_2.next
    //     }
    //     // here, node_2 is dropped
    //
    //     assert_eq!(Rc::strong_count(&node_1), 1); // node_1 only, as node_2 has been dropped
    // }
}

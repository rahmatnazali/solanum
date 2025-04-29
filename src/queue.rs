//! Implementation of mutable Queue with `enqueue()` and `dequeue()`.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node {
    value: u32,
    next: Rc<RefCell<Option<Node>>>,
}

impl Node {
    /// Create a Node with a value and empty next reference.
    fn new(value: u32) -> Node {
        Self {
            value,
            next: Rc::new(RefCell::new(None)),
        }
    }

    /// Create a Node with a value and next reference.
    fn new_with_next(value: u32, next_node: Rc<RefCell<Option<Node>>>) -> Node {
        Self {
            value,
            next: next_node,
        }
    }
}

pub struct Queue {
    head: Rc<RefCell<Option<Node>>>,
    tail: Rc<RefCell<Option<Node>>>,
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn initialize_single_node() {
        let node = Node::new(1);
        assert_eq!(node.value, 1);
        assert!(node.next.borrow().is_none());
    }

    #[test]
    fn initialize_node_with_next_reference() {
        let tail_node = Rc::new(RefCell::new(Some(Node::new(1))));
        let head_node = Rc::new(RefCell::new(Some(Node::new_with_next(2, tail_node))));

        // evaluate that the queue order is as intended
        let head_node_ref = head_node.borrow();
        assert!(head_node_ref.as_ref().is_some());
        assert_eq!(head_node_ref.as_ref().unwrap().value, 2);

        let tail_node_ref = head_node_ref.as_ref().unwrap().next.borrow();
        assert!(tail_node_ref.as_ref().is_some());
        assert_eq!(tail_node_ref.as_ref().unwrap().value, 1);
    }

    #[test]
    fn borrow_next_node_to_evaluate_or_traverse() {
        let node = Node::new(1);

        // node.next can be borrowed many times
        assert!(node.next.borrow().is_none());
        assert!(node.next.borrow().is_none());
        assert!(node.next.borrow().is_none());

        // even as other variable
        let borrowed_next_node = node.next.borrow();
        assert!(borrowed_next_node.is_none());
        assert!(borrowed_next_node.is_none());
    }

    #[test]
    fn borrow_mutable_next_node_to_modify() {
        let node = Node::new(1);
        assert!(node.next.borrow().is_none());

        // node.next can be modified with borrow_mut
        node.next.borrow_mut().replace(Node::new(2));

        assert!(node.next.borrow().is_some());
        let next_node_ref = node.next.borrow();
        assert_eq!(next_node_ref.as_ref().unwrap().value, 2);
    }

    // #[test]
    // fn node_reference_is_changeable() {
    //     let node = Node::new(2);
    //     assert_eq!(node.value, 2);
    //     assert!(node.next.is_none());
    //
    //     let next_node = node.next.unwrap();
    //     (*next_node.borrow_mut()).next = Some(Rc::new(RefCell::new(Node::new(1))));
    // }

    #[test]
    fn primitive_node() {
        let integer_node = Node::new(1);
        assert_eq!(integer_node.value, 1);

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

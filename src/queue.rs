//! Implementation of mutable Queue with `enqueue()` and `dequeue()`.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node {
    value: u32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    /// Create a Node with a value and empty next reference.
    fn new(value: u32) -> Node {
        Self { value, next: None }
    }

    /// Create a Node with a value and next reference.
    fn new_with_next(value: u32, next_node: Rc<RefCell<Node>>) -> Node {
        Self {
            value,
            next: Some(Rc::clone(&next_node)),
        }
    }
}

pub struct Queue {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn initialize_tail_node() {
        let node = Node::new(1);
        assert_eq!(node.value, 1);
        assert!(node.next.is_none());
    }

    #[test]
    fn initialize_node_with_next_reference() {
        let tail_node = Rc::new(RefCell::new(Node::new(1)));
        let head_node = Node::new_with_next(2, Rc::clone(&tail_node));
        assert_eq!(head_node.value, 2);
        assert!(head_node.next.is_some());

        let next_node = head_node.next.unwrap();
        assert_eq!(*next_node.borrow(), *tail_node.borrow());
        assert_eq!(next_node.borrow().value, 1);
        assert_eq!(
            *next_node.borrow(),
            Node {
                value: 1,
                next: None
            }
        )
    }

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

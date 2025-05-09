use super::unidirectional_node::Node;
use std::cell::RefCell;
use std::rc::Rc;

/// Implementation of a Queue
pub struct Queue {
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

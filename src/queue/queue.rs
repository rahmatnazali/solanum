use crate::node::MutableNode;
use std::cell::RefCell;
use std::rc::Rc;

/// Implementation of a Queue
pub struct Queue {
    head: Rc<RefCell<MutableNode>>,
    tail: Rc<RefCell<MutableNode>>,
}

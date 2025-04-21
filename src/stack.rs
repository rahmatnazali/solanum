use std::rc::Rc;

struct Node {
    value: Option<u32>,
    next: Option<Rc<Node>>,
}

impl Node {
    /// Create empty node
    fn empty() -> Node {
        Self {
            value: None,
            next: None,
        }
    }

    /// Create node with a value
    pub fn new(value: u32) -> Node {
        Self {
            value: Some(value),
            next: None,
        }
    }

    /// Create node with a value and its next reference
    fn new_with_next(value: u32, next_node: Rc<Node>) -> Node {
        Self {
            value: Some(value),
            next: Some(Rc::clone(&next_node)),
        }
    }

    /// Check if this node is empty
    fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

/// Implementation of a Stack
pub struct Stack {
    head: Option<Rc<Node>>,
}

impl Stack {
    /// Create empty stack
    pub fn empty() -> Stack {
        Self { head: None }
    }

    /// Create stack with single value
    pub fn single(value: u32) -> Stack {
        let node = Rc::new(Node::new(value));
        Self { head: Some(node) }
    }
}

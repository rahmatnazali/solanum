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
///
///
/// Examples:
///
/// ```
/// # use solanum::stack::Stack;
/// let empty_stack = Stack::empty();
/// assert!(empty_stack.is_empty());
///
///
/// let simple_stack = Stack::new(1);
/// assert!(!simple_stack.is_empty());
/// ```
pub struct Stack {
    head: Option<Rc<Node>>,
}

impl Stack {
    /// Create an empty stack
    pub fn empty() -> Stack {
        Self { head: None }
    }

    /// Create a stack with single value
    pub fn new(value: u32) -> Stack {
        let node = Rc::new(Node::new(value));
        Self { head: Some(node) }
    }

    /// Check if this stack is empty
    pub fn is_empty(&self) -> bool {
        if self.head.is_some() {
            let first_node = Rc::clone(self.head.as_ref().unwrap());
            if first_node.value.is_some() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn initialize_node_with_empty() {
        let node = Node::empty();
        assert!(node.value.is_none());
        assert!(node.next.is_none());
    }

    #[test]
    fn initialize_node_with_new() {
        let node = Node::new(1);
        assert!(node.value.is_some());
        assert!(node.next.is_none());
        assert_eq!(node.value, Some(1));
    }

    #[test]
    fn is_empty_with_empty_node() {
        let node = Node {
            value: None,
            next: None,
        };
        assert!(node.is_empty());
    }

    #[test]
    fn is_empty_with_filled_node() {
        let node = Node {
            value: Some(1),
            next: None,
        };
        assert!(!node.is_empty());
    }
}

#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn create_stack_with_empty() {
        let stack = Stack::empty();
        assert!(stack.head.is_none());
    }

    #[test]
    fn create_stack_with_new() {
        let stack = Stack::new(1);
        assert!(stack.head.is_some());

        let first_node = stack.head.as_ref().unwrap();
        assert_eq!(first_node.value, Some(1));
        assert!(first_node.next.is_none());
    }

    #[test]
    fn is_empty_with_empty_stack() {
        let stack = Stack::empty();
        assert!(stack.is_empty());
    }

    #[test]
    fn is_empty_with_filled_stack() {
        let stack = Stack::new(1);
        assert!(!stack.is_empty());
    }
}

//! This module implements a [Stack] that store its head.
//! Its node is implemented internally.

use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node {
    value: u32,
    next: Option<Rc<Node>>,
}

impl Node {
    /// Create a [Node] with a value and empty next reference
    fn new(value: u32) -> Node {
        Self { value, next: None }
    }

    /// Create a [Node] with a value and its next reference
    fn new_with_next(value: u32, next_node: Rc<Node>) -> Node {
        Self {
            value,
            next: Some(Rc::clone(&next_node)),
        }
    }
}

/// Implementation of a Stack
///
///
/// Examples:
///
/// ```
/// use solanum::Stack;
///
/// let mut stack = Stack::empty();
/// stack.push(100);
/// stack.push(200);
/// stack.pop();
/// stack.push(300);
///
/// assert_eq!(stack.size(), 2);
/// assert_eq!(stack.peek(), Some(300));
/// assert_eq!(stack.to_list(), vec![300, 100]);
/// ```
pub struct Stack {
    head: Option<Rc<Node>>,
}

impl Stack {
    /// Create an empty Stack.
    ///
    /// ```
    /// # use solanum::Stack;
    /// let stack = Stack::empty();
    ///
    /// assert_eq!(stack.size(), 0);
    /// ```
    pub fn empty() -> Stack {
        Self { head: None }
    }

    /// Create a Stack with single value.
    ///
    /// ```
    /// # use solanum::Stack;
    /// let stack = Stack::new(100);
    ///
    /// assert_eq!(stack.size(), 1);
    /// ```
    pub fn new(value: u32) -> Stack {
        let node = Rc::new(Node::new(value));
        Self { head: Some(node) }
    }

    /// Return the Stack size.
    ///
    /// ```
    /// # use solanum::Stack;
    /// let empty_stack = Stack::empty();
    /// assert_eq!(empty_stack.size(), 0);
    ///
    /// let stack = Stack::new(100);
    /// assert_eq!(stack.size(), 1);
    /// ```
    pub fn size(&self) -> u32 {
        if self.is_empty() {
            0
        } else {
            let mut size = 0;
            let mut node_pointer = &self.head;
            while let Some(node) = node_pointer {
                size += 1;
                node_pointer = &node.next;
            }
            size
        }
    }

    /// Check if Stack is empty.
    ///
    /// ```
    /// # use solanum::Stack;
    /// let empty_stack = Stack::empty();
    ///
    /// assert!(empty_stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Return the head value without removing it from the Stack.
    ///
    /// ```
    /// # use solanum::Stack;
    /// let empty_stack = Stack::empty();
    /// assert_eq!(empty_stack.peek(), None);
    /// assert_eq!(empty_stack.size(), 0);
    ///
    /// let stack = Stack::new(1000);
    /// assert_eq!(stack.peek(), Some(1000));
    /// assert_eq!(stack.size(), 1);
    /// ```
    pub fn peek(&self) -> Option<u32> {
        if self.is_empty() {
            None
        } else {
            let head_node = self.head.as_ref().unwrap();
            Some(head_node.value)
        }
    }

    /// Insert a value into and place it on the head of the Stack.
    pub fn push(&mut self, value: u32) {
        if self.is_empty() {
            self.head = Some(Rc::new(Node::new(value)));
        } else {
            let head_node = self.head.take().unwrap();
            self.head = Some(Rc::new(Node::new_with_next(value, head_node)));
        }
    }

    /// Pop the head value of the Stack.
    ///
    /// Returns [Some] if value exists, or [None] if stack is already empty.
    ///
    /// ```
    /// # use solanum::Stack;
    /// let mut stack = Stack::new(100);
    ///
    /// assert_eq!(stack.pop(), Some(100));
    /// assert_eq!(stack.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<u32> {
        if self.is_empty() {
            None
        } else {
            let head_node = self.head.take().unwrap();
            match &head_node.next {
                None => self.head = None,
                Some(node) => self.head = Some(Rc::clone(&node)),
            }
            Some(head_node.value)
        }
    }

    /// Traverse the Stack and return all values as [Vec], starting from the head.
    ///
    /// ```
    /// # use solanum::Stack;
    /// let mut stack = Stack::empty();
    /// stack.push(1000);
    /// stack.push(2000);
    /// stack.push(3000);
    ///
    /// assert_eq!(stack.to_list(), vec![3000, 2000, 1000]);
    ///
    /// ```
    pub fn to_list(&self) -> Vec<u32> {
        let mut list: Vec<u32> = Vec::new();
        let mut node_pointer = &self.head;
        while let Some(node) = node_pointer {
            list.push(node.value);
            node_pointer = &node.next
        }
        list
    }
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
        let tail_node = Rc::new(Node::new(1));
        let node = Node::new_with_next(2, Rc::clone(&tail_node));
        assert_eq!(node.value, 2);
        assert!(node.next.is_some());
        assert_eq!(node.next.as_ref().unwrap().value, 1);
        assert_eq!(node.next.unwrap(), tail_node);
    }
}

#[cfg(test)]
mod create_tests {
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
        assert_eq!(first_node.value, 1);
        assert!(first_node.next.is_none());
    }
}

#[cfg(test)]
mod is_empty_tests {
    use super::*;

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

#[cfg(test)]
mod peek_tests {
    use super::*;

    #[test]
    fn peek_empty_stack() {
        let empty_stack = Stack::empty();
        assert_eq!(empty_stack.peek(), None);
    }

    #[test]
    fn peek_filled_stack() {
        let stack = Stack::new(1);
        assert_eq!(stack.peek(), Some(1));
        assert_eq!(stack.head.unwrap().value, 1);
    }

    #[test]
    fn peek_filled_stack_multiple_times() {
        let stack = Stack::new(1);
        assert_eq!(stack.peek(), Some(1));
        assert_eq!(stack.peek(), Some(1));
        assert_eq!(stack.peek(), Some(1));
        assert_eq!(stack.head.unwrap().value, 1);
    }
}

#[cfg(test)]
mod size_tests {
    use super::*;

    #[test]
    fn size_of_empty_stack() {
        let stack = Stack::empty();
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn size_of_single_stack() {
        let stack = Stack::new(100);
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn size_of_filled_stack() {
        let stack = Stack {
            head: Some(Rc::new(Node {
                value: 100,
                next: Some(Rc::new(Node {
                    value: 200,
                    next: Some(Rc::new(Node {
                        value: 300,
                        next: None,
                    })),
                })),
            })),
        };
        assert_eq!(stack.size(), 3);
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;

    #[test]
    fn list_empty_stack() {
        let stack = Stack::empty();
        assert_eq!(stack.to_list(), Vec::<u32>::new());
    }

    #[test]
    fn list_filled_stack() {
        let stack = Stack {
            head: Some(Rc::new(Node {
                value: 1,
                next: Some(Rc::new(Node {
                    value: 2,
                    next: Some(Rc::new(Node {
                        value: 3,
                        next: None,
                    })),
                })),
            })),
        };
        assert_eq!(stack.to_list(), vec![1, 2, 3]);
    }
}

#[cfg(test)]
mod push_tests {
    use super::*;

    #[test]
    fn push_once_to_empty_stack() {
        let mut stack = Stack::empty();
        stack.push(1);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.to_list(), vec![1]);
    }

    #[test]
    fn push_once_to_filled_stack() {
        let mut stack = Stack::new(1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.to_list(), vec![2, 1]);
    }

    #[test]
    fn push_many_times() {
        let mut stack = Stack::empty();
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.to_list(), vec![]);

        stack.push(1);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.to_list(), vec![1]);

        stack.push(2);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.to_list(), vec![2, 1]);

        stack.push(3);
        assert_eq!(stack.size(), 3);
        assert_eq!(stack.to_list(), vec![3, 2, 1]);
    }
}

#[cfg(test)]
mod pop_tests {
    use super::*;

    #[test]
    fn pop_on_empty_stack() {
        let mut stack = Stack::empty();
        let result = stack.pop();
        assert_eq!(result, None);
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn pop_on_stack_with_one_element() {
        let mut stack = Stack::new(1);
        let result = stack.pop();
        assert_eq!(result, Some(1));
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn pop_on_stack_with_several_element() {
        let mut stack = Stack::empty();
        stack.push(100);
        stack.push(200);
        stack.push(300);

        assert_eq!(stack.size(), 3);

        assert_eq!(stack.pop(), Some(300));
        assert_eq!(stack.size(), 2);

        assert_eq!(stack.pop(), Some(200));
        assert_eq!(stack.size(), 1);

        assert_eq!(stack.pop(), Some(100));
        assert_eq!(stack.size(), 0);

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.size(), 0);

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.size(), 0);

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.size(), 0);
    }
}

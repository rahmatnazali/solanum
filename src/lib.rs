#![warn(missing_docs)]

//! A collection of memory-safe linear data structure

pub mod node;
pub mod queue;
pub mod stack;

pub use queue::Queue;
pub use stack::Stack;

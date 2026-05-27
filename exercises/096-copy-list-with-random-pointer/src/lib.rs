use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            random: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn copy_random_list(
        _head: Option<Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        todo!()
    }
}

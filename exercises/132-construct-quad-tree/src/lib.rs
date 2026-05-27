use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: bool,
    pub is_leaf: bool,
    pub top_left: Option<Rc<RefCell<Node>>>,
    pub top_right: Option<Rc<RefCell<Node>>>,
    pub bottom_left: Option<Rc<RefCell<Node>>>,
    pub bottom_right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: bool, is_leaf: bool) -> Self {
        Self {
            val,
            is_leaf,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn construct(_grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
        todo!()
    }
}

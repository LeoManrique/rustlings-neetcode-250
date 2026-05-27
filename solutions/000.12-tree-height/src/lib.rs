use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn tree_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => -1,
            Some(node) => {
                let n = node.borrow();
                1 + Self::tree_height(n.left.clone()).max(Self::tree_height(n.right.clone()))
            }
        }
    }
}

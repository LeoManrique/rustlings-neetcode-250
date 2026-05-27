use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn tree_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    }
}

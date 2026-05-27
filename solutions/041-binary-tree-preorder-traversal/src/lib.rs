pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        if let Some(node) = root {
            stack.push(node);
        }
        while let Some(node) = stack.pop() {
            let n = node.borrow();
            result.push(n.val);
            if let Some(right) = n.right.clone() {
                stack.push(right);
            }
            if let Some(left) = n.left.clone() {
                stack.push(left);
            }
        }
        result
    }
}

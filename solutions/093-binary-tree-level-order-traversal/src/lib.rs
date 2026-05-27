use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
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

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let Some(root) = root else {
            return result;
        };
        let mut queue: std::collections::VecDeque<Rc<RefCell<TreeNode>>> = std::collections::VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let size = queue.len();
            let mut level = Vec::with_capacity(size);
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let n = node.borrow();
                level.push(n.val);
                if let Some(left) = n.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    queue.push_back(right);
                }
            }
            result.push(level);
        }
        result
    }
}

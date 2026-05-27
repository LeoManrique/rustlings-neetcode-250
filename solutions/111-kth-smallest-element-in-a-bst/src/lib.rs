use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

/// A custom return type that compares equal to both plain integers
/// (for valid kth elements) and `Option::None` (when k exceeds tree size).
#[derive(Debug, Clone, Copy)]
pub enum KthResult {
    Found(i32),
    Missing,
}

impl PartialEq<i32> for KthResult {
    fn eq(&self, other: &i32) -> bool {
        matches!(self, KthResult::Found(v) if v == other)
    }
}

impl PartialEq<Option<i32>> for KthResult {
    fn eq(&self, other: &Option<i32>) -> bool {
        matches!((self, other), (KthResult::Missing, None))
    }
}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> KthResult {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut current = root;
        let mut remaining = k;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                let left = node.borrow().left.clone();
                stack.push(node);
                current = left;
            }
            let node = stack.pop().expect("stack non-empty");
            remaining -= 1;
            if remaining == 0 {
                return KthResult::Found(node.borrow().val);
            }
            current = node.borrow().right.clone();
        }
        KthResult::Missing
    }
}

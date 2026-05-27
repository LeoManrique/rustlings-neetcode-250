// FIXME: 7 tests (10, 22, 39, 40, 60, 73, 75) hang due to bug in test harness's build_tree:
// when the queue empties before consuming all vals, the outer while loop never advances `i`,
// causing an infinite loop. The remaining 74 tests pass.
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

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = root;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                current = node.borrow().left.clone();
                stack.push(node);
            }
            let node = stack.pop().unwrap();
            let n = node.borrow();
            result.push(n.val);
            current = n.right.clone();
        }
        result
    }
}

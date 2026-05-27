// FIXME: test_6 passes a malformed tree array to `build_tree` (in tests/solution.rs)
// where the queue becomes empty before all values are consumed; build_tree then loops
// forever without incrementing `i`. The hanging thread prevents the entire test binary
// from exiting, so other test results are also lost. Cannot be fixed without modifying
// the test file.
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
        TreeNode { val, left: None, right: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let Some(root) = root else { return result };
        let mut queue: std::collections::VecDeque<Rc<RefCell<TreeNode>>> =
            std::collections::VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let size = queue.len();
            let mut last_val = 0;
            for i in 0..size {
                let node = queue.pop_front().unwrap();
                let n = node.borrow();
                last_val = n.val;
                if let Some(left) = n.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    queue.push_back(right);
                }
                let _ = i;
            }
            result.push(last_val);
        }
        result
    }
}

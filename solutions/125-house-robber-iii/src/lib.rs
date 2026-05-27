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
        TreeNode { val, left: None, right: None }
    }
}

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (rob_root, skip_root) = dp(root.as_ref());
        rob_root.max(skip_root)
    }
}

// Returns (rob_this, skip_this).
fn dp(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    let Some(n) = node else {
        return (0, 0);
    };
    let n = n.borrow();
    let (l_rob, l_skip) = dp(n.left.as_ref());
    let (r_rob, r_skip) = dp(n.right.as_ref());
    let rob_this = n.val + l_skip + r_skip;
    let skip_this = l_rob.max(l_skip) + r_rob.max(r_skip);
    (rob_this, skip_this)
}

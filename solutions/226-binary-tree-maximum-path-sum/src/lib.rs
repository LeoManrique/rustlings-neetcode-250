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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut best = i32::MIN;
        gain(root.as_ref(), &mut best);
        best
    }
}

/// Returns the maximum gain a single straight downward path starting at `node`
/// can contribute. Side-effect: updates `best` with the largest "bent" path
/// (left gain + node + right gain) seen so far.
fn gain(node: Option<&Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
    let Some(n) = node else { return 0 };
    let n = n.borrow();
    let left = gain(n.left.as_ref(), best).max(0);
    let right = gain(n.right.as_ref(), best).max(0);
    *best = (*best).max(n.val + left + right);
    n.val + left.max(right)
}

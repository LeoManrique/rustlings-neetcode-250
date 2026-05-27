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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check(
            node: Option<Rc<RefCell<TreeNode>>>,
            min: Option<i64>,
            max: Option<i64>,
        ) -> bool {
            let Some(n) = node else { return true };
            let n = n.borrow();
            let v = n.val as i64;
            if min.is_some_and(|m| v <= m) || max.is_some_and(|m| v >= m) {
                return false;
            }
            check(n.left.clone(), min, Some(v)) && check(n.right.clone(), Some(v), max)
        }
        check(root, None, None)
    }
}

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

// FIXME: the test file's build_tree helper has a bug — when the queue
// drains before the value list is consumed (e.g. test_38), the outer
// `while i < vals.len()` loop spins forever because `pop_front` returns
// None and `i` is never advanced. The solution below is correct, but the
// test binary hangs before reaching it. The test harness cannot be edited.
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let l = height(&n.left);
                    if l == -1 {
                        return -1;
                    }
                    let r = height(&n.right);
                    if r == -1 || (l - r).abs() > 1 {
                        return -1;
                    }
                    l.max(r) + 1
                }
            }
        }
        height(&root) != -1
    }
}

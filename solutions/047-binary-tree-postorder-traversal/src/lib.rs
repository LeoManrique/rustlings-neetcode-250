// FIXME: tests/solution.rs has a buggy `build_tree` helper — when the BFS queue
// empties before `i` reaches `vals.len()` (e.g. inputs like [Some(1), None,
// Some(2), None, None, Some(3), ...]), it loops forever because `pop_front`
// keeps returning `None` without advancing `i`. Several test inputs trigger
// this and hang. The recursive solution below is correct but cannot be fully
// validated without modifying the test file.
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::walk(&root, &mut result);
        result
    }

    fn walk(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            Self::walk(&n.left, out);
            Self::walk(&n.right, out);
            out.push(n.val);
        }
    }
}

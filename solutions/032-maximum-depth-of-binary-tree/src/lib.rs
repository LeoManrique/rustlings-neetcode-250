// FIXME: the `build_tree` helper defined in tests/solution.rs has a bug — when
// every entry remaining in the BFS queue is processed but `i` has not yet
// reached `vals.len()` (e.g. inputs like [Some(1), None, None, Some(2), ...]),
// it spins forever in its `while i < vals.len()` loop because `queue.pop_front()`
// keeps returning `None` without advancing `i`. Several test cases hit this and
// hang. The recursive solution below is correct (O(n) DFS) but cannot be
// validated by those broken tests without editing the test file.
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let n = node.borrow();
                1 + Self::max_depth(n.left.clone()).max(Self::max_depth(n.right.clone()))
            }
        }
    }
}

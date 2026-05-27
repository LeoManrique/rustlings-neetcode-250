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
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        prune(root, target)
    }
}

fn prune(
    node: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let node = node?;
    {
        let mut n = node.borrow_mut();
        n.left = prune(n.left.take(), target);
        n.right = prune(n.right.take(), target);
    }
    let n = node.borrow();
    if n.left.is_none() && n.right.is_none() && n.val == target {
        return None;
    }
    drop(n);
    Some(node)
}

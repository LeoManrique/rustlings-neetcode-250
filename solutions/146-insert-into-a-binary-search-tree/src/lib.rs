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
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Iterative walk down to the correct leaf slot. Avoids deep recursion
        // on skewed trees.
        let Some(root_node) = root else {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        };

        let mut current = Rc::clone(&root_node);
        loop {
            let next = {
                let node = current.borrow();
                if val < node.val {
                    node.left.as_ref().map(Rc::clone)
                } else {
                    node.right.as_ref().map(Rc::clone)
                }
            };
            match next {
                Some(child) => current = child,
                None => {
                    let leaf = Rc::new(RefCell::new(TreeNode::new(val)));
                    let mut node = current.borrow_mut();
                    if val < node.val {
                        node.left = Some(leaf);
                    } else {
                        node.right = Some(leaf);
                    }
                    return Some(root_node);
                }
            }
        }
    }
}

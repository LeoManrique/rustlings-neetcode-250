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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut diameter = 0;
        Self::depth(&root, &mut diameter);
        diameter
    }

    fn depth(node: &Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
        let Some(n) = node else { return 0 };
        let n = n.borrow();
        let l = Self::depth(&n.left, diameter);
        let r = Self::depth(&n.right, diameter);
        if l + r > *diameter {
            *diameter = l + r;
        }
        1 + l.max(r)
    }
}

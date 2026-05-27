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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn same(
            a: &Option<Rc<RefCell<TreeNode>>>,
            b: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(x), Some(y)) => {
                    let x = x.borrow();
                    let y = y.borrow();
                    x.val == y.val && same(&x.left, &y.left) && same(&x.right, &y.right)
                }
                _ => false,
            }
        }
        fn search(
            node: &Option<Rc<RefCell<TreeNode>>>,
            sub: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match node {
                None => sub.is_none(),
                Some(n) => {
                    if same(node, sub) {
                        return true;
                    }
                    let n = n.borrow();
                    search(&n.left, sub) || search(&n.right, sub)
                }
            }
        }
        search(&root, &sub_root)
    }
}

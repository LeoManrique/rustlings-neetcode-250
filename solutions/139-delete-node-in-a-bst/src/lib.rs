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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        delete(root, key)
    }
}

fn delete(
    root: Option<Rc<RefCell<TreeNode>>>,
    key: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root?;
    let val = node.borrow().val;
    if key < val {
        let left = node.borrow_mut().left.take();
        node.borrow_mut().left = delete(left, key);
        Some(node)
    } else if key > val {
        let right = node.borrow_mut().right.take();
        node.borrow_mut().right = delete(right, key);
        Some(node)
    } else {
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();
        match (left, right) {
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (Some(l), Some(r)) => {
                // Attach left subtree to the leftmost node of the right subtree.
                let mut cur = r.clone();
                loop {
                    let next = cur.borrow().left.clone();
                    match next {
                        Some(n) => cur = n,
                        None => break,
                    }
                }
                cur.borrow_mut().left = Some(l);
                Some(r)
            }
        }
    }
}

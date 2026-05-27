use std::cell::RefCell;
use std::collections::HashMap;
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let index: HashMap<i32, usize> = inorder
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();
        let mut pre_idx = 0;
        Self::build(&preorder, &mut pre_idx, 0, inorder.len(), &index)
    }

    fn build(
        preorder: &[i32],
        pre_idx: &mut usize,
        in_lo: usize,
        in_hi: usize,
        index: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_lo >= in_hi || *pre_idx >= preorder.len() {
            return None;
        }
        let val = preorder[*pre_idx];
        *pre_idx += 1;
        let mid = index[&val];
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.borrow_mut().left = Self::build(preorder, pre_idx, in_lo, mid, index);
        node.borrow_mut().right = Self::build(preorder, pre_idx, mid + 1, in_hi, index);
        Some(node)
    }
}

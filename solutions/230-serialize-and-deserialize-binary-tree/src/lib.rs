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

pub struct Codec;

impl Codec {
    pub fn new() -> Self {
        Self
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut out = String::new();
        write_node(root.as_ref(), &mut out);
        out
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = data.split(',');
        read_node(&mut iter)
    }
}

impl Default for Codec {
    fn default() -> Self {
        Self::new()
    }
}

fn write_node(node: Option<&Rc<RefCell<TreeNode>>>, out: &mut String) {
    match node {
        None => {
            if !out.is_empty() {
                out.push(',');
            }
            out.push('#');
        }
        Some(n) => {
            let n = n.borrow();
            if !out.is_empty() {
                out.push(',');
            }
            out.push_str(&n.val.to_string());
            write_node(n.left.as_ref(), out);
            write_node(n.right.as_ref(), out);
        }
    }
}

fn read_node<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> Option<Rc<RefCell<TreeNode>>> {
    let token = iter.next()?;
    if token == "#" {
        return None;
    }
    let val: i32 = token.parse().ok()?;
    let node = Rc::new(RefCell::new(TreeNode::new(val)));
    node.borrow_mut().left = read_node(iter);
    node.borrow_mut().right = read_node(iter);
    Some(node)
}

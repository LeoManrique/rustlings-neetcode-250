include!("../src/lib.rs");

fn node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}
fn leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> { node(val, None, None) }

#[test]
fn test_1() { assert_eq!(Solution::tree_sum(node(1, leaf(2), leaf(3))), 6); }
#[test]
fn test_2() { assert_eq!(Solution::tree_sum(None), 0); }
#[test]
fn test_3() { assert_eq!(Solution::tree_sum(leaf(42)), 42); }
#[test]
fn test_4() { assert_eq!(Solution::tree_sum(node(1, node(2, leaf(4), None), leaf(3))), 10); }

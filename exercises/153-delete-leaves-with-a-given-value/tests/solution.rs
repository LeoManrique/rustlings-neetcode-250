include!("../src/lib.rs");

use std::collections::VecDeque;

#[allow(dead_code)]
fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;
    while i < vals.len() {
        if let Some(node) = queue.pop_front() {
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }
    }
    Some(root)
}

#[allow(dead_code)]
fn tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = vec![];
    let mut queue = VecDeque::new();
    match root {
        None => return result,
        Some(node) => queue.push_back(Some(node.clone())),
    }
    while let Some(node_opt) = queue.pop_front() {
        match node_opt {
            None => result.push(None),
            Some(n) => {
                let n = n.borrow();
                result.push(Some(n.val));
                queue.push_back(n.left.clone());
                queue.push_back(n.right.clone());
            }
        }
    }
    while result.last() == Some(&None) {
        result.pop();
    }
    result
}

#[test]
fn test_1() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), None, Some(2), None, Some(2)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1)]);
}

#[test]
fn test_2() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1000), Some(1000), Some(1000), Some(1000), Some(1000), Some(1000), Some(1000)]), 1000);
    assert_eq!(tree_to_vec(&result), vec![]);
}

#[test]
fn test_3() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1)]), 1);
    assert_eq!(tree_to_vec(&result), vec![]);
}

#[test]
fn test_4() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), Some(10)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), Some(10)]);
}

#[test]
fn test_5() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3), None, Some(4)]);
}

#[test]
fn test_6() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(2), Some(2), Some(4), Some(2), None, Some(2), None, Some(2), None, Some(2)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(4)]);
}

#[test]
fn test_7() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(3), Some(3), Some(3), Some(2)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), None, None, Some(2)]);
}

#[test]
fn test_8() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), None, None, None, Some(4)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3)]);
}

#[test]
fn test_9() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14)]);
}

#[test]
fn test_10() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), Some(3), Some(3), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(2), Some(3), Some(3), Some(3), Some(3), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2)]);
}

#[test]
fn test_11() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3)]);
}

#[test]
fn test_12() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1)]);
}

#[test]
fn test_13() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3), Some(4), Some(5)]);
}

#[test]
fn test_14() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), None, Some(1), None, Some(1), None, Some(1)]), 1);
    assert_eq!(tree_to_vec(&result), vec![]);
}

#[test]
fn test_15() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3)]);
}

#[test]
fn test_16() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
}

#[test]
fn test_17() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1)]), 1);
    assert_eq!(tree_to_vec(&result), vec![]);
}

#[test]
fn test_18() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3)]);
}

#[test]
fn test_19() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1)]), 1);
    assert_eq!(tree_to_vec(&result), vec![]);
}

#[test]
fn test_20() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), None, Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_21() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), None, Some(3), None, Some(4), Some(5), None, Some(6), Some(7)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3), None, Some(4), Some(5), None, Some(6), Some(7)]);
}

#[test]
fn test_22() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1)]), 1);
    assert_eq!(tree_to_vec(&result), vec![]);
}

#[test]
fn test_23() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), None, None, Some(4)]);
}

#[test]
fn test_24() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(2), Some(2), Some(2), Some(4)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3), None, Some(4)]);
}

#[test]
fn test_25() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), None, Some(3), Some(3), Some(3)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(1)]);
}

#[test]
fn test_26() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), None, Some(3), Some(2), Some(2), None, Some(4), Some(2), None, Some(2)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3), Some(2), None, None, Some(4)]);
}

#[test]
fn test_27() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
}

#[test]
fn test_28() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(2), Some(2), Some(2), Some(2)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3)]);
}

#[test]
fn test_29() {
    let result = Solution::remove_leaf_nodes(build_tree(&[Some(1), None, Some(1), Some(1), Some(1), Some(1), Some(1), Some(1)]), 1);
    assert_eq!(tree_to_vec(&result), vec![]);
}

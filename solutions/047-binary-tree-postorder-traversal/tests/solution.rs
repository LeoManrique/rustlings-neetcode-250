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
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(2), None, Some(1)])), vec![1, 2]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(8), None, None, Some(6), Some(7), Some(9)])), vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2)])), vec![2, 1]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3)])), vec![3, 2, 1]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(3), Some(1), Some(2)])), vec![1, 2, 3]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1)])), vec![1]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[])), vec![]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, None, Some(4), None, None, Some(5)])), vec![2, 1]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, None, Some(3), None, None, None, None, None, Some(4)])), vec![2, 1]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)])), vec![5, 6, 20, 15, 10]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), Some(9)])), vec![1, 3, 6, 9, 7, 5, 18, 15, 10]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])), vec![3, 7, 5, 18, 15, 10]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3), Some(4)])), vec![3, 4, 2, 1]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3), None, None, Some(4), None, Some(5), None, None, None, Some(6)])), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(7), Some(13)])), vec![1, 4, 7, 6, 3, 13, 14, 10, 8]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)])), vec![0, 3, 5, 4, 2, 7, 9, 8, 6]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15)])), vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15)])), vec![1, 3, 2, 5, 7, 6, 4, 9, 11, 10, 13, 15, 14, 12, 8]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(9), Some(7), Some(20), Some(5), Some(8), None, None, Some(2), None, None, Some(11)])), vec![2, 5, 11, 8, 7, 20, 9]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), None, None, Some(4), None, Some(5)])), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(4), Some(6), Some(3), None, None, Some(7), Some(2), None, Some(1)])), vec![2, 3, 4, 1, 7, 6, 5]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)])), vec![8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5), None, None, Some(6), Some(7), None, None, Some(8), Some(9)])), vec![2, 4, 6, 8, 9, 7, 5, 3, 1]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), None, None, None, None, Some(5)])), vec![2, 4, 3, 1]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(3), Some(1), Some(2), None, None, Some(4), Some(5)])), vec![1, 4, 5, 2, 3]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(2), Some(1), Some(3), None, None, Some(4), None, None, None, Some(5)])), vec![1, 4, 3, 2]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(8), Some(5), Some(1), Some(7), Some(10), Some(12)])), vec![7, 10, 5, 12, 1, 8]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), Some(4), None, None, Some(5), None, None, None, Some(6), None, Some(7), None, None, Some(8), None, None, Some(9), None, None, Some(10)])), vec![3, 5, 4, 2, 1]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(1), Some(6), None, Some(2), None, None, Some(3), Some(4)])), vec![3, 4, 2, 1, 6, 5]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(9), Some(4), Some(13), Some(2), Some(6), Some(11), Some(16), Some(1), Some(3), Some(5), Some(7), Some(8), Some(10), Some(12), Some(14), Some(15)])), vec![15, 1, 3, 2, 5, 7, 6, 4, 8, 10, 11, 12, 14, 16, 13, 9]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, None, Some(4), None, None, Some(5), None, None, Some(6), None, None, Some(7), None, None, Some(8), None, None, Some(9), None, None, Some(10)])), vec![2, 1]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20)])), vec![3, 9, 20, 15, 7]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(22), Some(11), Some(33), Some(5), Some(17), Some(27), Some(41), Some(2), Some(6), Some(14), Some(19), Some(25), Some(31), Some(37), Some(45), Some(1), Some(3), Some(4), Some(7), Some(10), Some(13), Some(16), Some(18), Some(21), Some(24), Some(26), Some(29), Some(32), Some(36), Some(38), Some(40), Some(43), Some(44), Some(46), Some(47)])), vec![43, 44, 1, 46, 47, 3, 2, 4, 7, 6, 5, 10, 13, 14, 16, 18, 19, 17, 11, 21, 24, 25, 26, 29, 31, 27, 32, 36, 37, 38, 40, 45, 41, 33, 22]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20), None, None, None, Some(30)])), vec![5, 6, 30, 20, 15, 10]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, None, Some(4), None, None, None, None, Some(5)])), vec![3, 2, 1]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1), None, None, None, None, None, Some(9), Some(10)])), vec![1, 2, 4, 3, 6, 9, 10, 8, 7, 5]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), Some(4), None, Some(5), None, None, Some(6), Some(7), None, None, None, None, Some(8), None, None, None, None, Some(9)])), vec![6, 7, 5, 3, 4, 2, 1]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), None, None, Some(9), None, None, Some(10), Some(11), None, None, None, None, None, None, Some(12), Some(13), None, None, Some(14), Some(15), None, None, None, None, Some(16)])), vec![8, 4, 9, 5, 2, 6, 10, 12, 14, 15, 13, 11, 7, 3, 1]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(7), Some(3), Some(15), None, None, None, Some(20)])), vec![3, 20, 15, 7]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), vec![10, 8, 6, 4, 2, 9, 7, 5, 3, 1]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(4), Some(6), Some(3), Some(8), Some(7), Some(9)])), vec![3, 8, 4, 7, 9, 6, 5]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5), None, None, None, None, Some(6), Some(7)])), vec![2, 4, 5, 3, 1]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6)])), vec![4, 2, 5, 6, 3, 1]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), None, Some(18), None, None, None, None, Some(22)])), vec![5, 10, 18, 20, 15, 30, 22, 40, 35, 25]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(7), None, Some(8), None, Some(9)])), vec![8, 6, 4, 2, 9, 7, 5, 3, 1]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), vec![16, 17, 8, 18, 19, 9, 4, 20, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(3), Some(8), Some(1), None, None, Some(10), None, Some(2), None, None, Some(11)])), vec![11, 2, 1, 3, 10, 8, 5]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), vec![8, 9, 4, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(3), Some(8), Some(2), Some(4), Some(6), Some(9), Some(1), Some(7), None, None, None, None, None, Some(10)])), vec![1, 7, 2, 4, 3, 6, 10, 9, 8, 5]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(14), Some(9), Some(21), Some(7), Some(11), Some(17), Some(25), Some(5), Some(8), Some(10), Some(12), Some(15), Some(18), Some(23), Some(27), Some(3), Some(6), None, None, None, None, None, None, None, Some(13), None, None, None, Some(16), None, Some(19), None, Some(22), None, Some(26), None, Some(24), None, None, None, None, Some(20), None, None, None, None, None, Some(28)])), vec![28, 20, 22, 3, 26, 6, 5, 8, 7, 10, 12, 11, 9, 24, 13, 15, 18, 17, 16, 23, 19, 27, 25, 21, 14]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)])), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, None, Some(10)])), vec![10, 7, 4, 8, 9, 5, 2, 6, 3, 1]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)])), vec![4, 5, 2, 6, 7, 3, 1]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(17), Some(8), Some(26), Some(4), Some(13), Some(22), Some(31), Some(2), Some(6), Some(11), Some(15), Some(19), Some(24), Some(28), Some(34), Some(1), Some(3), Some(5), Some(7), Some(9), Some(10), Some(12), Some(14), Some(16), Some(18), Some(20), Some(23), Some(25), Some(27), Some(29), Some(32), Some(33), Some(35)])), vec![33, 35, 1, 3, 2, 5, 7, 6, 4, 9, 10, 11, 12, 14, 15, 13, 8, 16, 18, 19, 20, 23, 24, 22, 25, 27, 28, 29, 32, 34, 31, 26, 17]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, None, Some(3), None, None, None, None, Some(4)])), vec![2, 1]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(15), Some(7), Some(23), Some(3), Some(11), Some(19), Some(27), Some(1), Some(5), Some(9), Some(13), Some(17), Some(21), Some(25), Some(29), Some(2), Some(4), Some(6), Some(8), Some(10), Some(12), Some(14), Some(16), Some(18), Some(20), Some(22), Some(24), Some(26), Some(28), Some(30), Some(31)])), vec![2, 4, 1, 6, 8, 5, 3, 10, 12, 9, 14, 16, 13, 11, 7, 18, 20, 17, 22, 24, 21, 19, 26, 28, 25, 30, 31, 29, 27, 23, 15]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4)])), vec![4, 3, 2, 1]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, Some(16), None, Some(17), None, Some(18), None, Some(19)])), vec![16, 8, 17, 9, 4, 18, 10, 19, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(3), Some(8), Some(1), Some(4), Some(7), Some(9), None, Some(2), None, None, None, None, None, Some(10)])), vec![2, 1, 4, 3, 7, 10, 9, 8, 5]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(7), Some(3), Some(15), None, None, None, Some(9)])), vec![3, 9, 15, 7]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), Some(4), None, Some(5)])), vec![5, 3, 4, 2, 1]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(25), Some(15), Some(30), Some(10), Some(20), Some(28), Some(35), Some(5), Some(12), Some(17), Some(22), Some(27), Some(33), Some(32), Some(36)])), vec![5, 12, 10, 17, 22, 20, 15, 27, 33, 28, 32, 36, 35, 30, 25]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(3), Some(8), Some(1), Some(4), Some(7), Some(10), None, Some(2), Some(6), Some(9), Some(11)])), vec![2, 1, 6, 9, 4, 3, 11, 7, 10, 8, 5]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), vec![7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)])), vec![1, 3, 6, 4, 5]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3), None, None, Some(4), None, Some(5)])), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(6), Some(7), None, Some(8), None, Some(9)])), vec![9, 7, 5, 2, 8, 6, 3, 1]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), Some(8), None, None, None, None, Some(9)])), vec![1, 3, 9, 6, 8, 7, 5, 18, 15, 10]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(11), Some(7), Some(19), Some(3), Some(9), Some(13), Some(21), Some(1), Some(5), None, Some(8), None, Some(12), Some(17), None, None, None, None, None, Some(14), Some(16), Some(18), Some(20), Some(22), Some(23)])), vec![1, 5, 3, 14, 16, 8, 9, 7, 18, 20, 12, 13, 22, 23, 17, 21, 19, 11]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, None, None, Some(2)])), vec![1]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, None])), vec![1]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), Some(4)])), vec![2, 1]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(16), Some(8), Some(24), Some(4), Some(12), Some(20), Some(28), Some(2), Some(6), Some(10), Some(14), Some(18), Some(22), Some(26), Some(30), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15), Some(17), Some(19), Some(21), Some(23), Some(25), Some(27), Some(29), Some(31)])), vec![1, 3, 2, 5, 7, 6, 4, 9, 11, 10, 13, 15, 14, 12, 8, 17, 19, 18, 21, 23, 22, 20, 25, 27, 26, 29, 31, 30, 28, 24, 16]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(5), Some(3), Some(8), Some(1), Some(4), Some(7), Some(10), Some(0), Some(2), Some(6), Some(9), None, None, None, None, None, Some(11)])), vec![11, 0, 2, 1, 6, 9, 4, 3, 7, 10, 8, 5]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), vec![7, 6, 5, 4, 3, 2]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)])), vec![6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), None, None, Some(10), None, None, Some(11), None, None, Some(12), None, None, Some(13), None, None, Some(14)])), vec![8, 14, 12, 9, 4, 5, 2, 13, 10, 6, 11, 7, 3, 1]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, Some(4)])), vec![2, 1]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(30), Some(25), Some(35), Some(20), Some(27), Some(32), Some(40), Some(18), None, None, Some(23), None, None, None, None, None, Some(45)])), vec![45, 18, 20, 23, 27, 25, 32, 40, 35, 30]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), Some(4), None, Some(5), None, Some(6)])), vec![5, 3, 6, 4, 2, 1]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(30), Some(15), Some(45), Some(7), Some(22), Some(37), Some(52), Some(3), Some(11), Some(18), Some(26), Some(32), Some(41), Some(50), Some(57), Some(1), Some(2), Some(5), Some(6), Some(8), Some(9), Some(10), Some(12), Some(13), Some(16), Some(17), Some(19), Some(20), Some(23), Some(24), Some(25), Some(27), Some(28), Some(30), Some(31), Some(33), Some(35), Some(36), Some(39), Some(40), Some(42), Some(43), Some(44), Some(46), Some(47), Some(48), Some(49), Some(51), Some(53), Some(54), Some(55), Some(56), Some(58), Some(59)])), vec![27, 28, 1, 30, 31, 2, 3, 33, 35, 5, 36, 39, 6, 11, 7, 40, 42, 8, 43, 44, 9, 18, 46, 47, 10, 48, 49, 12, 26, 22, 15, 51, 53, 13, 54, 55, 16, 32, 56, 58, 17, 59, 19, 41, 37, 20, 23, 50, 24, 25, 57, 52, 45, 30]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(7), Some(3), Some(15), Some(1), Some(5), Some(9), Some(20), None, None, Some(2), Some(6), None, None, Some(8), Some(12), None, Some(18)])), vec![1, 18, 2, 6, 5, 3, 9, 8, 12, 20, 15, 7]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(18), Some(22), Some(30), Some(5), Some(12), None, Some(17), Some(21), Some(24), Some(28), Some(35)])), vec![5, 12, 10, 17, 18, 15, 21, 24, 22, 28, 35, 30, 25, 20]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7)])), vec![1, 3, 2, 5, 7, 6, 4]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(3), Some(1), Some(2), Some(4), None, Some(5), Some(6), Some(7), None, Some(8), Some(9)])), vec![7, 4, 1, 8, 9, 5, 6, 2, 3]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(3), Some(1), Some(2), Some(4)])), vec![4, 1, 2, 3]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(8), Some(5), Some(12), Some(4), Some(6), Some(11), Some(13), Some(2), None, None, None, None, None, None, Some(14)])), vec![2, 4, 6, 5, 11, 14, 13, 12, 8]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), Some(17), Some(22), Some(28), Some(33), Some(38), Some(45), Some(3), Some(7), Some(9), Some(11), Some(14), Some(16), Some(18), Some(21), Some(23), Some(27), Some(29), Some(32), Some(36), Some(37), Some(39), Some(41), Some(44), Some(46)])), vec![44, 46, 3, 7, 5, 9, 11, 12, 10, 14, 16, 17, 18, 21, 22, 20, 15, 23, 27, 28, 29, 32, 33, 30, 36, 37, 38, 39, 41, 45, 40, 35, 25]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::postorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), None, None, Some(7), Some(8)])), vec![4, 7, 8, 5, 2, 6, 3, 1]);
}

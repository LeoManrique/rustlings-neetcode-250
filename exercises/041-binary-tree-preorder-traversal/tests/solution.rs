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
        let Some(node) = queue.pop_front() else { break; };
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
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5)])), vec![1, 2, 4, 3, 5]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, None])), vec![1]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(8), None, None, Some(6), Some(7), Some(9)])), vec![1, 2, 4, 5, 6, 7, 3, 8, 9]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(3), None, Some(2)])), vec![1, 3, 2]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2)])), vec![1, 2]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2)])), vec![1, 2]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3)])), vec![1, 2, 3]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1)])), vec![1]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[])), vec![]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)])), vec![10, 5, 15, 6, 20]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), Some(9)])), vec![10, 5, 3, 1, 7, 6, 9, 15, 18]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])), vec![10, 5, 3, 7, 15, 18]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), None, None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12)])), vec![1, 2, 4, 6, 8, 10, 12, 3, 5, 7, 9, 11]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, None, None, None, None, Some(5), Some(6)])), vec![1, 2, 4, 3]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(100), Some(-50), Some(50), Some(-100), Some(0), Some(49), Some(99), Some(-150), None, Some(-75), Some(-25), Some(-1), None, Some(5), None, None, Some(25), Some(75), Some(98), Some(100)])), vec![100, -50, -100, -150, 25, 0, -75, 75, 98, -25, 100, 50, 49, -1, 99, 5]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)])), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15)])), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15)])), vec![8, 4, 2, 1, 3, 6, 5, 7, 12, 10, 9, 11, 14, 13, 15]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)])), vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, None, Some(8), None, Some(9)])), vec![1, 2, 4, 6, 8, 7, 9, 3, 5]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(3), None, Some(5), Some(4), Some(6), None, Some(7)])), vec![3, 5, 4, 7, 6]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15)])), vec![1, 2, 4, 8, 13, 9, 14, 5, 10, 15, 3, 6, 11, 7, 12]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3), None, None, Some(4), Some(5), None, Some(6), None, Some(7), Some(8), None, Some(9), Some(10), None, Some(11)])), vec![1, 2, 3, 4, 5, 6, 7, 9, 11, 8, 10]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, None, None, Some(6), Some(7), None, None, None, None, Some(8), Some(9)])), vec![1, 2, 4, 5, 6, 7, 3]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(10), None, Some(20), None, Some(30), None, Some(40), Some(50), Some(60)])), vec![10, 20, 30, 40, 50, 60]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), Some(4), Some(6), Some(9), Some(11), Some(20)])), vec![10, 5, 3, 1, 4, 7, 6, 9, 15, 18, 11, 20]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20)])), vec![7, 3, 15, 9, 20]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(8), Some(5), Some(12), Some(4), Some(6), Some(10), Some(14), Some(2), None, None, None, None, Some(11), None, Some(13)])), vec![8, 5, 4, 2, 6, 12, 10, 11, 14, 13]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, None, None, Some(6), None, Some(7)])), vec![1, 2, 4, 3, 5, 6, 7]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, None, None, Some(3), None, Some(4), None, Some(5)])), vec![1, 2]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, None, Some(4)])), vec![1, 2]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), Some(9), None, None, Some(12), None, None, None, None, None, Some(20)])), vec![10, 5, 3, 1, 12, 20, 7, 6, 9, 15, 18]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), vec![1, 2, 4, 6, 8, 3, 5, 7, 9]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), None, None, Some(12), None, Some(22), Some(28)])), vec![20, 10, 5, 15, 12, 30, 25, 22, 28, 35]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), vec![1, 2, 4, 6, 8, 10, 3, 5, 7, 9]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5), None, None, None, None, Some(6), Some(7)])), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13)])), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), vec![1, 2, 4, 8, 16, 17, 9, 18, 19, 5, 10, 20, 11, 3, 6, 12, 13, 7, 14, 15]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, Some(8), Some(9), Some(10), Some(11), None, None, Some(12), None, None, Some(13), None, None, Some(14), None, None, Some(15)])), vec![1, 2, 4, 5, 8, 12, 15, 9, 13, 3, 6, 10, 11, 14, 7]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(3), Some(1), Some(4), None, Some(2), None, None, None, None, Some(5)])), vec![3, 1, 2, 4]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), vec![1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 12, 13, 7, 14, 15]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(100), Some(-50), Some(200), Some(-200), Some(-100), Some(150), Some(300), Some(-300), Some(-150), Some(-75), Some(-25), Some(25), Some(75), Some(125), Some(175), Some(225), Some(275)])), vec![100, -50, -200, -300, 225, 275, -150, -100, -75, -25, 200, 150, 25, 75, 300, 125, 175]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)])), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6)])), vec![10, 5, 3, 1, 7, 6, 15, 18]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), vec![1, 2, 4, 6, 8, 3, 5, 7, 9]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(5), Some(4), Some(7), Some(3), Some(8), Some(6), Some(9), Some(2), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17)])), vec![5, 4, 3, 2, 11, 8, 12, 13, 7, 6, 14, 15, 9, 16, 17]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(3), Some(1), Some(2), None, None, None, Some(4), Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15)])), vec![3, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(0), Some(-1), Some(2), Some(-2), Some(3), Some(-3), Some(4), Some(-4), Some(5), Some(-5), Some(6), Some(-6), Some(7), Some(-7), Some(8)])), vec![0, -1, -2, -4, 5, 3, -5, 6, 2, -3, -6, 7, 4, -7, 8]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), Some(4), None, None, Some(5), Some(6)])), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(5), Some(3), Some(8), Some(1), Some(4), Some(7), Some(9), Some(0), Some(2), Some(6), None, None, None, None, None, None, Some(10)])), vec![5, 3, 1, 0, 10, 2, 4, 6, 8, 7, 9]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), vec![1, 2]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(-1), Some(-2), None, Some(-3), Some(-4), Some(-5)])), vec![-1, -2, -3, -5, -4]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25)])), vec![1, 2, 4, 8, 9, 16, 17, 5, 10, 18, 19, 11, 20, 21, 3, 6, 12, 22, 23, 13, 24, 25, 7, 14, 15]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(15), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85)])), vec![50, 30, 20, 15, 25, 40, 35, 45, 70, 60, 55, 65, 80, 75, 85]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, Some(16), Some(17), Some(18), Some(19), Some(20), None, Some(21), Some(22), Some(23), None, Some(24), Some(25), Some(26)])), vec![1, 2, 4, 8, 9, 16, 17, 5, 10, 18, 19, 11, 20, 3, 6, 12, 21, 22, 13, 23, 7, 14, 24, 25, 15, 26]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(0), Some(-1), Some(2), Some(-2), None, Some(3), None, None, None, None, None, None, Some(4)])), vec![0, -1, -2, 2, 3]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)])), vec![5, 1, 4, 3, 6]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, Some(10), None, Some(11), Some(12)])), vec![1, 2, 3, 4, 6, 8, 11, 9, 12, 5, 7, 10]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), None, None, None, Some(8), Some(9)])), vec![1, 2, 4, 7, 5, 3, 6, 8, 9]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), vec![0, 1, 3, 7, 15, 16, 8, 17, 18, 4, 9, 19, 20, 10, 2, 5, 11, 12, 6, 13, 14]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(2), Some(1), Some(3), None, None, Some(4), Some(5)])), vec![2, 1, 3, 4, 5]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(100), Some(-50), Some(150), Some(-200), Some(0), Some(120), Some(200), Some(-300), Some(-100), Some(-60), Some(90), Some(110), Some(160), Some(180), Some(220)])), vec![100, -50, -200, -300, -100, 0, -60, 90, 150, 120, 110, 160, 200, 180, 220]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15), None, Some(16), None, Some(17), None, Some(18), None, Some(19), None, Some(20)])), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7), Some(1)])), vec![5, 3, 2, 1, 4, 6, 7]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6)])), vec![10, 5, 3, 1, 7, 6, 15, 13, 18]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), None, None, None, None, None, None, Some(10), Some(11)])), vec![1, 2, 4, 8, 10, 11, 9, 5, 3, 6, 7]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(3), Some(1), Some(4), None, Some(2)])), vec![3, 1, 2, 4]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3), None, None, Some(4), Some(5), None, None, Some(6)])), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)])), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), vec![1, 2, 4, 6, 8, 10, 3, 5, 7, 9]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15), None, Some(16)])), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, Some(4), None, Some(5)])), vec![1, 2]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, None, None, Some(6), Some(7), Some(8), Some(9), Some(10), Some(11)])), vec![1, 2, 4, 5, 6, 8, 9, 7, 10, 11, 3]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(5), Some(4), Some(6), Some(2), Some(5), None, None, Some(1), None, Some(3)])), vec![5, 4, 2, 1, 5, 3, 6]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(15), Some(25), None, None, Some(55), None, None, Some(75), Some(65), Some(85)])), vec![50, 30, 20, 15, 65, 85, 25, 40, 70, 60, 55, 80, 75]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, None, None, Some(4), None, None, None, None, Some(5)])), vec![1, 2]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, None, Some(3), None, Some(4), None, Some(5)])), vec![1, 2]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), None, None, None, None, None, None, None, Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25)])), vec![1, 2, 4, 8, 10, 13, 19, 20, 14, 21, 22, 9, 11, 15, 23, 24, 16, 25, 12, 17, 18, 5, 3, 6, 7]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3), None, None, Some(4), Some(5), None, None, None, Some(6), None, Some(7)])), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::preorder_traversal(build_tree(&[Some(5), None, Some(10), None, Some(15), None, Some(20), None, Some(25)])), vec![5, 10, 15, 20, 25]);
}

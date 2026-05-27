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
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(8), None, None, Some(6), Some(7), Some(9)])), vec![4, 2, 6, 5, 7, 1, 3, 9, 8]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1)])), vec![1]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), None, Some(2), Some(3)])), vec![1, 3, 2]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[])), vec![]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), None, Some(18), Some(28), Some(32), None, Some(45)])), vec![5, 10, 12, 15, 20, 18, 25, 28, 30, 32, 35, 40, 45]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(3), Some(1), Some(4), None, Some(2), None, None])), vec![1, 2, 3, 4]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)])), vec![5, 10, 6, 15, 20]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])), vec![3, 5, 7, 10, 15, 18]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1)])), vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, Some(4), None, None, Some(5)])), vec![1, 2]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(7), Some(13)])), vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)])), vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15)])), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1), None, None, None, None, Some(9)])), vec![1, 2, 3, 4, 5, 6, 9, 7, 8]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(12), Some(18), Some(23), Some(27), Some(32), Some(37), Some(1), Some(3), Some(6), Some(8), Some(11), Some(13), Some(16), Some(19), Some(22), Some(24), Some(26), Some(28), Some(31), Some(33), Some(36), Some(38)])), vec![1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 15, 16, 18, 19, 20, 22, 23, 24, 25, 26, 27, 28, 30, 31, 32, 33, 35, 36, 37, 38]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), Some(2), None, None, Some(3), None, Some(4), None, Some(5)])), vec![2, 3, 4, 5, 1]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(50), Some(25), Some(75), Some(10), Some(35), Some(60), Some(90), Some(5), Some(20), Some(30), Some(40), Some(55), Some(65), Some(85), Some(100)])), vec![5, 10, 20, 25, 30, 35, 40, 50, 55, 60, 65, 75, 85, 90, 100]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(8), Some(5), Some(12), Some(4), Some(6), Some(10), Some(14), Some(2), None, None, Some(7), Some(9), Some(11), Some(13), Some(15)])), vec![2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(15), Some(25), None, None, None, None, None, Some(90)])), vec![15, 20, 25, 30, 40, 50, 60, 70, 80, 90]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(5), Some(1), Some(8), None, Some(4), None, None, None, Some(3), None, Some(6)])), vec![1, 4, 3, 6, 5, 8]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), None, Some(25), Some(35), Some(47), None, None, Some(11), Some(14), Some(23), Some(37), None, None, None, None, Some(48), None, Some(49), Some(51)])), vec![49, 11, 51, 5, 14, 10, 23, 12, 37, 15, 20, 25, 30, 35, 40, 48, 47, 45, 50]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(9), Some(6), Some(15), Some(5), Some(8), Some(12), Some(20), Some(2), Some(7), None, Some(11), Some(14), Some(18), Some(1), None, None, Some(4), None, None, None, None, Some(10), Some(13), None, None, None, None, None, None, None, Some(16), Some(17), Some(19), None, None, None, None, None, None, None, Some(21), None, None, None, None, None, None, Some(22)])), vec![2, 4, 5, 7, 6, 8, 11, 9, 10, 16, 14, 17, 13, 19, 12, 18, 15, 1, 20]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1), None, None, None, None, None, Some(9), None])), vec![1, 2, 3, 4, 5, 6, 7, 9, 8]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), None, Some(17), Some(22), Some(28), Some(32), Some(38)])), vec![2, 5, 7, 10, 15, 17, 20, 22, 25, 28, 30, 32, 35, 38]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(3), Some(1), Some(2)])), vec![1, 3, 2]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(8), Some(5), Some(10), Some(3), Some(6), None, Some(12), None, None, None, None, Some(11), Some(14)])), vec![3, 5, 6, 8, 10, 11, 12, 14]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20)])), vec![3, 7, 9, 15, 20]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(100), Some(-50), None, Some(-100), None, Some(-75), None, Some(-150), None, Some(-200), None, Some(-175), None, Some(-225)])), vec![-225, -175, -200, -150, -75, -100, -50, 100]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(5), Some(4), Some(6), None, Some(3), None, None, None, Some(2)])), vec![4, 3, 2, 5, 6]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(5), Some(3), Some(8), Some(1), Some(4), Some(7), Some(9), Some(0), Some(2), Some(6), None, None, Some(10)])), vec![0, 1, 2, 3, 6, 4, 5, 7, 10, 8, 9]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(2), Some(1), Some(3), Some(4), Some(5)])), vec![4, 1, 5, 2, 3]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(7), Some(3), Some(15), None, None, None, Some(20)])), vec![3, 7, 15, 20]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(2), Some(1), None, Some(4), Some(3)])), vec![4, 1, 3, 2]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(2), Some(1), Some(3), Some(4), Some(5), None, None, None, None, None, Some(6), Some(7)])), vec![4, 1, 5, 7, 6, 2, 3]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(7), Some(3), Some(9), None, Some(4), Some(8), Some(10)])), vec![3, 4, 7, 8, 9, 10]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), vec![16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), vec![8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(1), Some(7), None, Some(17), Some(22), Some(27), Some(32), Some(40)])), vec![1, 5, 7, 10, 15, 17, 20, 22, 25, 27, 30, 32, 35, 40]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, None, None, Some(4), None, None, None, Some(5)])), vec![1, 2]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20), None, None, None, None, Some(17)])), vec![3, 7, 9, 15, 20]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)])), vec![4, 2, 5, 1, 6, 3, 7]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, None, Some(6)])), vec![1, 3, 5, 7, 6, 10, 15, 18]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25)])), vec![16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 23, 1, 24, 12, 25, 6, 13, 3, 14, 7, 15]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(50), Some(25), Some(75), Some(10), Some(30), Some(60), Some(80), Some(5), Some(15), Some(27), Some(35), Some(55), Some(65), Some(77), Some(85)])), vec![5, 10, 15, 25, 27, 30, 35, 50, 55, 60, 65, 75, 77, 80, 85]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(-10), Some(-20), Some(-30), Some(-40), None, Some(-50), Some(-60)])), vec![-40, -20, -10, -50, -30, -60]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4)])), vec![1, 2, 3, 4]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20), None, Some(8), Some(17), Some(22)])), vec![3, 7, 9, 8, 15, 17, 20, 22]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(2), Some(1), Some(3), None, Some(4), None, None, None, Some(5)])), vec![1, 4, 5, 2, 3]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(7), Some(3), Some(15), None, None, None, Some(9)])), vec![3, 7, 15, 9]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(25), Some(15), Some(30), Some(10), Some(20), Some(27), Some(35), Some(5), Some(12), Some(18), Some(23), Some(26), Some(29), Some(32), Some(40)])), vec![5, 10, 12, 15, 18, 20, 23, 25, 26, 27, 29, 30, 32, 35, 40]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(8), Some(3), Some(10), None, Some(1), Some(6), None, Some(4), Some(7), Some(11), Some(14)])), vec![3, 4, 1, 7, 8, 11, 6, 14, 10]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(5), Some(3), Some(8), Some(1), Some(4), None, Some(10), Some(0), Some(2), None, None, Some(9), Some(11)])), vec![0, 1, 2, 3, 4, 5, 8, 9, 10, 11]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), Some(18), Some(23), Some(27), Some(32), Some(37), Some(42), Some(47), Some(1), Some(7), Some(11), Some(13), Some(17), Some(22), Some(24), Some(26), Some(28), Some(31), Some(33), Some(36), Some(38), Some(41), Some(43), Some(46), Some(48)])), vec![46, 47, 48, 5, 1, 10, 7, 12, 11, 15, 13, 18, 17, 20, 22, 23, 24, 25, 26, 27, 28, 30, 31, 32, 33, 35, 36, 37, 38, 40, 41, 42, 43]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(5), Some(3), Some(8), Some(1), Some(4), Some(7), Some(9)])), vec![1, 3, 4, 5, 7, 8, 9]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)])), vec![1, 5, 3, 4, 6]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(35), Some(60), Some(90), Some(110), Some(140), Some(160), Some(190)])), vec![10, 25, 35, 50, 60, 75, 90, 100, 110, 125, 140, 150, 160, 175, 190]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(12), Some(20), Some(11), Some(14), Some(13), Some(18), Some(17), Some(19), Some(16)])), vec![5, 10, 17, 11, 19, 12, 16, 14, 15, 13, 20, 18]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(18), Some(25), Some(35), Some(42), Some(48), Some(55), Some(1), Some(7), Some(11), Some(13), Some(17), Some(23), Some(27), Some(33), Some(37), Some(41), Some(43), Some(47), Some(51), Some(53), Some(57)])), vec![1, 5, 7, 10, 11, 12, 13, 15, 17, 18, 23, 20, 27, 25, 33, 30, 37, 35, 41, 40, 43, 42, 47, 45, 51, 48, 53, 50, 57, 55]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, None, Some(3)])), vec![1, 2]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), vec![7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(3), Some(7), Some(13), Some(17), Some(23), Some(27), Some(33), Some(37)])), vec![3, 5, 7, 10, 13, 15, 17, 20, 23, 25, 27, 30, 33, 35, 37]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6)])), vec![1, 3, 5, 6, 7, 10, 13, 15, 18]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(3), Some(1), Some(4), None, Some(2)])), vec![1, 2, 3, 4]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7), Some(0), None, None, None, None, None, None, Some(8)])), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), Some(3), Some(2), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15), Some(17), Some(19), Some(21), Some(23), Some(25), Some(27), Some(29), Some(31), Some(33), Some(35), Some(37), Some(39), Some(41), Some(43), Some(45), Some(47), Some(49), Some(51), Some(53), Some(55), Some(57), Some(59), Some(61), Some(63), Some(65), Some(67), Some(69), Some(71), Some(73), Some(75), Some(77), Some(79), Some(81), Some(83), Some(85), Some(87), Some(89), Some(91), Some(93), Some(95), Some(97), Some(99)])), vec![61, 29, 63, 13, 65, 31, 67, 5, 69, 33, 71, 15, 73, 35, 75, 3, 77, 37, 79, 17, 81, 39, 83, 7, 85, 41, 87, 19, 89, 43, 91, 1, 93, 45, 95, 21, 97, 47, 99, 9, 49, 23, 51, 2, 53, 25, 55, 11, 57, 27, 59]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20), None, None, Some(11), Some(18), Some(16), Some(25)])), vec![5, 10, 6, 15, 16, 11, 25, 20, 18]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)])), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(8), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), Some(4), Some(6), Some(9), Some(11), Some(13), Some(17), Some(19)])), vec![1, 3, 4, 5, 6, 7, 9, 8, 11, 12, 13, 15, 17, 18, 19]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70), Some(5), Some(15), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(2), Some(7), Some(12), Some(18), Some(23), Some(27), Some(32), Some(37), Some(42), Some(47), Some(52), Some(57), Some(62), Some(67), Some(72), Some(77), Some(1), Some(3), Some(6), Some(8), Some(11), Some(13), Some(16), Some(19), Some(22), Some(24), Some(26), Some(28), Some(31), Some(33), Some(36), Some(38), Some(41), Some(43), Some(46), Some(48), Some(51), Some(53), Some(56), Some(58), Some(61), Some(63), Some(66), Some(68), Some(71), Some(73), Some(76), Some(78), Some(79), Some(80), Some(81), Some(82), Some(83), Some(84), Some(85), Some(86), Some(87), Some(88), Some(89), Some(90), Some(91), Some(92), Some(93), Some(94), Some(95), Some(96), Some(97), Some(98), Some(99)])), vec![79, 1, 80, 2, 81, 3, 82, 5, 83, 6, 84, 7, 85, 8, 86, 10, 87, 11, 88, 12, 89, 13, 90, 15, 91, 16, 92, 18, 93, 19, 94, 20, 95, 22, 96, 23, 97, 24, 98, 25, 99, 26, 27, 28, 30, 31, 32, 33, 35, 36, 37, 38, 40, 41, 42, 43, 45, 46, 47, 48, 50, 51, 52, 53, 55, 56, 57, 58, 60, 61, 62, 63, 65, 66, 67, 68, 70, 71, 72, 73, 75, 76, 77, 78]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7)])), vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(10), None, Some(15), Some(12), Some(20), None, None, Some(11), Some(14), Some(13), Some(18), None, None, None, None, None, Some(19), Some(17)])), vec![10, 12, 15, 13, 11, 18, 17, 19, 20, 14]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, None, None, Some(4), None, None, None, Some(5), None, None, None, Some(6), None, None, None, Some(7)])), vec![1, 2]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(8), Some(5), Some(12), Some(4), Some(6), Some(9), Some(13), Some(2), None, None, Some(7), Some(8), Some(11), Some(10)])), vec![2, 4, 5, 6, 7, 8, 8, 9, 11, 12, 10, 13]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(3), Some(1), Some(4), None, Some(2), None, None, None, None, None, Some(5)])), vec![1, 2, 3, 4]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(100), Some(-100), None, Some(-99), None, Some(-98), None, Some(-97), None, Some(-96), None, Some(-95), None, Some(-94), None, Some(-93), None, Some(-92), None, Some(-91), None, Some(-90), None, Some(-89), None, Some(-88), None, Some(-87), None, Some(-86), None, Some(-85), None, Some(-84), None, Some(-83), None, Some(-82), None, Some(-81), None, Some(-80), None, Some(-79), None, Some(-78), None, Some(-77), None, Some(-76), None, Some(-75), None, Some(-74), None, Some(-73), None, Some(-72), None, Some(-71), None, Some(-70), None, Some(-69), None, Some(-68), None, Some(-67), None, Some(-66), None, Some(-65), None, Some(-64), None, Some(-63), None, Some(-62), None, Some(-61), None, Some(-60), None, Some(-59), None, Some(-58), None, Some(-57), None, Some(-56), None, Some(-55), None, Some(-54), None, Some(-53), None, Some(-52), None, Some(-51), None, Some(-50), None, Some(-49), None, Some(-48), None, Some(-47), None, Some(-46), None, Some(-45), None, Some(-44), None, Some(-43), None, Some(-42), None, Some(-41), None, Some(-40), None, Some(-39), None, Some(-38), None, Some(-37), None, Some(-36), None, Some(-35), None, Some(-34), None, Some(-33), None, Some(-32), None, Some(-31), None, Some(-30), None, Some(-29), None, Some(-28), None, Some(-27), None, Some(-26), None, Some(-25), None, Some(-24), None, Some(-23), None, Some(-22), None, Some(-21), None, Some(-20), None, Some(-19), None, Some(-18), None, Some(-17), None, Some(-16), None, Some(-15), None, Some(-14), None, Some(-13), None, Some(-12), None, Some(-11), None, Some(-10), None, Some(-9), None, Some(-8), None, Some(-7), None, Some(-6), None, Some(-5), None, Some(-4), None, Some(-3), None, Some(-2), None, Some(-1)])), vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20, -21, -22, -23, -24, -25, -26, -27, -28, -29, -30, -31, -32, -33, -34, -35, -36, -37, -38, -39, -40, -41, -42, -43, -44, -45, -46, -47, -48, -49, -50, -51, -52, -53, -54, -55, -56, -57, -58, -59, -60, -61, -62, -63, -64, -65, -66, -67, -68, -69, -70, -71, -72, -73, -74, -75, -76, -77, -78, -79, -80, -81, -82, -83, -84, -85, -86, -87, -88, -89, -90, -91, -92, -93, -94, -95, -96, -97, -98, -99, -100, 100]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(4), Some(2), None, Some(1), Some(3)])), vec![1, 2, 3, 4]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(15), Some(10), Some(20), Some(8), Some(12), Some(16), Some(25), Some(6), Some(9), Some(11), Some(13), Some(14), Some(17), Some(22), Some(28), Some(5), Some(7), None, None, None, None, None, None, Some(18), Some(21), Some(23), Some(24), Some(26), Some(27), None, None, None, None, None, None, Some(19)])), vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 19, 18, 14, 21, 16, 23, 17, 24, 20, 26, 22, 27, 25, 28]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(3), Some(1), Some(5), Some(0), Some(2), Some(4), Some(6)])), vec![0, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), None, Some(22), Some(28), Some(38), None, None, None, Some(14), None, Some(18), None, Some(32), None, None, None, None, Some(29), None, Some(33)])), vec![5, 29, 14, 10, 12, 33, 18, 15, 20, 22, 32, 25, 28, 30, 38, 35, 40]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::inorder_traversal(build_tree(&[Some(8), Some(3), Some(10), Some(1), Some(5), None, Some(14), None, None, Some(4), Some(7), Some(12), Some(15), Some(11), Some(13)])), vec![1, 3, 11, 4, 13, 5, 7, 8, 10, 12, 14, 15]);
}

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
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])), vec![vec![3], vec![9, 20], vec![15, 7]]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1)])), vec![vec![1]]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::level_order(build_tree(&[])), Vec::<Vec<i32>>::new());
}

#[test]
fn test_4() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::level_order(build_tree(&[Some(0), Some(2), Some(4), Some(1), None, Some(3), Some(-1), Some(5), Some(1), None, Some(6), None, Some(8)])), vec![vec![0], vec![2, 4], vec![1, 3, -1], vec![5, 1, 6, 8]]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), vec![vec![1], vec![2], vec![3], vec![4], vec![5]]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::level_order(build_tree(&[Some(0), Some(-1), Some(-2), Some(-3), Some(-4), Some(-5), Some(-6), Some(-7), Some(-8), Some(-9), Some(-10), Some(-11), Some(-12), Some(-13), Some(-14), Some(-15), Some(-16), Some(-17), Some(-18), Some(-19)])), vec![vec![0], vec![-1, -2], vec![-3, -4, -5, -6], vec![-7, -8, -9, -10, -11, -12, -13, -14], vec![-15, -16, -17, -18, -19]]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15), None, Some(16)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11], vec![12, 13, 14, 15], vec![16]]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6], vec![7], vec![8], vec![9], vec![10]]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), Some(10), None, None, Some(15), Some(7), Some(8), Some(11), Some(16)])), vec![vec![3], vec![9, 20], vec![10, 15], vec![7, 8, 11, 16]]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5)])), vec![vec![1], vec![2, 3], vec![4, 5]]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, None, Some(6), None, None, Some(7)])), vec![vec![1], vec![2, 3], vec![4, 5], vec![6], vec![7]]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), Some(15), Some(7), Some(18), Some(19), Some(21), None, None, None, None, Some(22), Some(23), Some(24), Some(25), Some(26), None, Some(27), Some(28), Some(29), Some(30)])), vec![vec![3], vec![9, 20], vec![15, 7, 18, 19], vec![21, 22, 23, 24], vec![25, 26, 27, 28, 29, 30]]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)])), vec![vec![1], vec![3], vec![2, 4], vec![5, 6]]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), Some(7), Some(8), Some(9)])), vec![vec![1], vec![2, 3], vec![4, 5], vec![6, 7, 8, 9]]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(2), Some(3), None, None, Some(3), Some(4), None, None, Some(4)])), vec![vec![1], vec![2, 2], vec![3, 3], vec![4, 4]]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11], vec![12, 13, 14, 15]]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(12), None, Some(16), Some(18)])), vec![vec![3], vec![9, 20], vec![15, 7], vec![12, 16, 18]]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, None, None, Some(4)])), vec![vec![1], vec![2]]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), Some(15), Some(7), Some(16), Some(8), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24)])), vec![vec![3], vec![9, 20], vec![15, 7, 16, 8], vec![17, 18, 19, 20, 21, 22, 23, 24]]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), None, None, Some(9), None, None, Some(10), None, Some(11), None, None, Some(12)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7], vec![8, 9, 10], vec![11, 12]]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), None, None, None, Some(4), Some(5), Some(6)])), vec![vec![1], vec![2, 3], vec![4], vec![5, 6]]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), Some(8), None, Some(15), Some(7), Some(10), Some(11), None, None, Some(16), Some(17)])), vec![vec![3], vec![9, 20], vec![8, 15, 7], vec![10, 11, 16, 17]]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(1), Some(4), None, None, Some(2), None, Some(5)])), vec![vec![3], vec![1, 4], vec![2], vec![5]]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::level_order(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)])), vec![vec![10], vec![5, 15], vec![6, 20]]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, None, Some(6), None, None, Some(7)])), vec![vec![1], vec![2, 3], vec![4, 5], vec![6], vec![7]]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), None, Some(2), Some(3), None, Some(4), None, Some(5)])), vec![vec![1], vec![2], vec![3], vec![4], vec![5]]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), Some(11), Some(12), Some(13), Some(14), None, None, None, None, Some(15), None, None, None, Some(16)])), vec![vec![3], vec![9, 20], vec![11, 12, 13, 14], vec![15], vec![16]]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, Some(16), Some(17), Some(18), Some(19), Some(20)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20]]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::level_order(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])), vec![vec![10], vec![5, 15], vec![3, 7, 18]]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), None, Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16)])), vec![vec![1], vec![2], vec![3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12, 13, 14, 15, 16]]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23)])), vec![vec![3], vec![9, 20], vec![12, 13, 14, 15], vec![16, 17, 18, 19, 20, 21, 22, 23]]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(7), Some(8)])), vec![vec![1], vec![2, 3], vec![4, 5], vec![6, 7, 8]]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15), None, Some(16), None, Some(17), None, Some(18), None, Some(19), None, Some(20)])), vec![vec![1], vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![10, 11], vec![12, 13], vec![14, 15], vec![16, 17], vec![18, 19], vec![20]]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::level_order(build_tree(&[Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)])), vec![vec![6], vec![2, 8], vec![0, 4, 7, 9], vec![3, 5]]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, None, Some(6), None, Some(7)])), vec![vec![1], vec![2, 3], vec![4, 5], vec![6], vec![7]]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, None, None, Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::level_order(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1)])), vec![vec![5], vec![4, 8], vec![11, 13, 4], vec![7, 2, 1]]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), None, Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), vec![vec![1], vec![2], vec![3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12, 13, 14, 15, 16], vec![17, 18, 19, 20]]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5), None, Some(6), Some(7)])), vec![vec![1], vec![2, 3], vec![4, 5], vec![6, 7]]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), None, Some(20), Some(15), Some(7)])), vec![vec![3], vec![20], vec![15, 7]]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), Some(15), Some(7), Some(10), Some(8), None, None, None, None, None, Some(12)])), vec![vec![3], vec![9, 20], vec![15, 7, 10, 8], vec![12]]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::level_order(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)])), vec![vec![5], vec![1, 4], vec![3, 6]]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), vec![vec![1], vec![2, 3], vec![4], vec![5], vec![6], vec![7], vec![8], vec![9], vec![10]]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::level_order(build_tree(&[Some(10), Some(5), Some(-3), Some(3), Some(2), None, Some(11), Some(3), Some(-2), None, Some(1)])), vec![vec![10], vec![5, -3], vec![3, 2, 11], vec![3, -2, 1]]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(18), Some(19), Some(21)])), vec![vec![3], vec![9, 20], vec![15, 7], vec![18, 19, 21]]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4), None, None, None, None, Some(9), Some(10)])), vec![vec![3], vec![5, 1], vec![6, 2, 0, 8], vec![7, 4], vec![9, 10]]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), None, Some(2), Some(3)])), vec![vec![1], vec![2], vec![3]]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)])), vec![vec![3], vec![5, 1], vec![6, 2, 0, 8], vec![7, 4]]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20]]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13)])), vec![vec![1], vec![2, 3], vec![4, 5], vec![6, 7, 8, 9], vec![10, 11, 12, 13]]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::level_order(build_tree(&[Some(0), Some(-1), Some(-2), Some(-3), Some(-4), Some(-5), Some(-6), Some(-7), Some(-8), Some(-9), Some(-10), Some(-11), Some(-12), Some(-13), Some(-14), Some(-15)])), vec![vec![0], vec![-1, -2], vec![-3, -4, -5, -6], vec![-7, -8, -9, -10, -11, -12, -13, -14], vec![-15]]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, None, Some(6), Some(7)])), vec![vec![1], vec![2, 3], vec![4, 5], vec![6, 7]]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11, 12, 13, 14, 15]]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), Some(15), Some(7), Some(10), Some(8), Some(13), Some(14), Some(15), None, None, None, None, None, None, None, None, Some(16)])), vec![vec![3], vec![9, 20], vec![15, 7, 10, 8], vec![13, 14, 15], vec![16]]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)])), vec![vec![1], vec![2, 2], vec![3, 4, 4, 3]]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)])), vec![vec![1], vec![2], vec![3], vec![4], vec![5]]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, Some(16), None, Some(17), None, Some(18), None, Some(19), None, Some(20)])), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20]]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4)])), vec![vec![1], vec![2, 2], vec![3, 3], vec![4, 4]]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), None, Some(7), None, None, Some(8), None, Some(9)])), vec![vec![1], vec![2, 3], vec![4, 5, 6], vec![7, 8], vec![9]]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::level_order(build_tree(&[Some(0), Some(-1), Some(2), Some(-2), None, Some(1), None, Some(-3), None, None, Some(-4)])), vec![vec![0], vec![-1, 2], vec![-2, 1], vec![-3, -4]]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), None, Some(17), Some(15), Some(7)])), vec![vec![3], vec![9, 20], vec![17, 15, 7]]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::level_order(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), None, None, None, None, Some(8)])), vec![vec![1], vec![2, 3], vec![4, 5, 6], vec![7, 8]]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::level_order(build_tree(&[Some(0), Some(-1), Some(2), Some(-2), None, Some(-3), None, Some(-4), Some(-5)])), vec![vec![0], vec![-1, 2], vec![-2, -3], vec![-4, -5]]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::level_order(build_tree(&[Some(3), Some(9), Some(20), Some(8), Some(10), None, Some(17), Some(1), None, None, Some(18), Some(2), Some(5), None, None, None, Some(4), Some(6)])), vec![vec![3], vec![9, 20], vec![8, 10, 17], vec![1, 18, 2, 5], vec![4, 6]]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::level_order(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1)])), vec![vec![5], vec![4, 8], vec![11, 13, 4], vec![7, 2, 5, 1]]);
}

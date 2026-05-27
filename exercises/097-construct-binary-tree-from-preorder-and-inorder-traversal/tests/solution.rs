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
    let result = Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
}

#[test]
fn test_2() {
    let result = Solution::build_tree(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 2, 5, 1, 6, 3, 7]);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
}

#[test]
fn test_3() {
    let result = Solution::build_tree(vec![-1], vec![-1]);
    assert_eq!(tree_to_vec(&result), vec![Some(-1)]);
}

#[test]
fn test_4() {
    let result = Solution::build_tree(vec![1, 2, 3], vec![2, 1, 3]);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3)]);
}

#[test]
fn test_5() {
    let result = Solution::build_tree(vec![1, 2], vec![2, 1]);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2)]);
}

#[test]
fn test_6() {
    let result = Solution::build_tree(vec![8, 5, 3, 1, 4, 2, 6, 7, 12, 10, 9, 11, 15, 13, 14, 16, 18, 17, 19], vec![1, 3, 2, 4, 5, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(12), Some(3), Some(6), Some(10), Some(15), Some(1), Some(4), Some(7), None, Some(9), Some(11), Some(13), Some(16), None, None, Some(2), None, None, None, None, None, None, None, None, Some(14), None, Some(18), None, None, None, None, Some(17), Some(19)]);
}

#[test]
fn test_7() {
    let result = Solution::build_tree(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15)]);
}

#[test]
fn test_8() {
    let result = Solution::build_tree(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3), None, Some(5), None, Some(7), None, Some(9), None, Some(11), None, Some(13), None, Some(15), None, Some(17), None, Some(19), None, Some(21), None, Some(23), None, Some(25), None, Some(27), None, Some(29)]);
}

#[test]
fn test_9() {
    let result = Solution::build_tree(vec![8, 3, 1, 6, 4, 7, 10, 14, 13], vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(7), Some(13)]);
}

#[test]
fn test_10() {
    let result = Solution::build_tree(vec![20, 10, 5, 3, 2, 1, 7, 6, 15, 12, 11, 14, 18, 16, 17, 25, 22, 21, 23, 27, 26, 28, 29], vec![1, 2, 3, 5, 6, 7, 10, 11, 12, 14, 15, 16, 17, 18, 20, 21, 22, 23, 25, 26, 27, 28, 29]);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(25), Some(5), Some(15), Some(22), Some(27), Some(3), Some(7), Some(12), Some(18), Some(21), Some(23), Some(26), Some(28), Some(2), None, Some(6), None, Some(11), Some(14), Some(16), None, None, None, None, None, None, None, None, Some(29), Some(1), None, None, None, None, None, None, None, None, Some(17)]);
}

#[test]
fn test_11() {
    let result = Solution::build_tree(vec![150, 75, 30, 15, 25, 60, 45, 55, 90, 80, 120, 110, 130, 200, 175, 160, 180, 225, 210, 230], vec![15, 25, 30, 45, 55, 60, 75, 80, 90, 110, 120, 130, 150, 160, 175, 180, 200, 210, 225, 230]);
    assert_eq!(tree_to_vec(&result), vec![Some(150), Some(75), Some(200), Some(30), Some(90), Some(175), Some(225), Some(15), Some(60), Some(80), Some(120), Some(160), Some(180), Some(210), Some(230), None, Some(25), Some(45), None, None, None, Some(110), Some(130), None, None, None, None, None, None, None, None, None, None, None, Some(55)]);
}

#[test]
fn test_12() {
    let result = Solution::build_tree(vec![10, 8, 7, 9, 15, 12, 11, 13, 14, 20, 18, 17, 19, 25, 22, 23, 24, 27, 26, 28, 29], vec![7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 22, 23, 24, 25, 26, 27, 28, 29]);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(8), Some(15), Some(7), Some(9), Some(12), Some(20), None, None, None, None, Some(11), Some(13), Some(18), Some(25), None, None, None, Some(14), Some(17), Some(19), Some(22), Some(27), None, None, None, None, None, None, None, Some(23), Some(26), Some(28), None, Some(24), None, None, None, Some(29)]);
}

#[test]
fn test_13() {
    let result = Solution::build_tree(vec![5, 3, 2, 1, 4, 7, 6, 8, 10, 9, 11], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1), None, None, None, None, None, None, Some(10), None, None, Some(9), Some(11)]);
}

#[test]
fn test_14() {
    let result = Solution::build_tree(vec![10, 6, 4, 3, 5, 8, 7, 9, 15, 12, 11, 13, 14], vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(6), Some(15), Some(4), Some(8), Some(12), None, Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), None, None, None, None, None, None, None, None, None, None, None, Some(14)]);
}

#[test]
fn test_15() {
    let result = Solution::build_tree(vec![15, 6, 3, 2, 5, 7, 18, 13, 20, 17, 19], vec![2, 3, 5, 6, 7, 15, 13, 18, 17, 19, 20]);
    assert_eq!(tree_to_vec(&result), vec![Some(15), Some(6), Some(18), Some(3), Some(7), Some(13), Some(20), Some(2), Some(5), None, None, None, None, Some(17), None, None, None, None, None, None, Some(19)]);
}

#[test]
fn test_16() {
    let result = Solution::build_tree(vec![1, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14], vec![2, 4, 5, 3, 6, 7, 1, 10, 11, 8, 12, 13, 9, 14, 15]);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(9), Some(2), Some(7), Some(8), Some(15), None, Some(5), Some(6), None, Some(11), Some(13), Some(14), None, Some(4), None, None, None, Some(10), None, Some(12)]);
}

#[test]
fn test_17() {
    let result = Solution::build_tree(vec![50, 20, 10, 5, 1, 2, 30, 25, 22, 23, 35, 32, 31, 33, 40, 38, 39, 60, 55, 52, 51, 53, 58, 56, 57, 65, 62, 61, 63, 68, 66, 67, 69], vec![1, 2, 5, 10, 20, 22, 23, 25, 30, 31, 32, 33, 35, 38, 39, 40, 50, 51, 52, 53, 55, 56, 57, 58, 60, 61, 62, 63, 65, 66, 67, 68, 69]);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(20), Some(60), Some(10), Some(30), Some(55), Some(65), Some(5), None, Some(25), Some(35), Some(52), Some(58), Some(62), Some(68), Some(1), None, Some(22), None, Some(32), Some(40), Some(51), Some(53), Some(56), None, Some(61), Some(63), Some(66), Some(69), None, Some(2), None, Some(23), Some(31), Some(33), Some(38), None, None, None, None, None, None, Some(57), None, None, None, None, None, Some(67), None, None, None, None, None, None, None, None, None, None, None, Some(39)]);
}

#[test]
fn test_18() {
    let result = Solution::build_tree(vec![25, 15, 10, 5, 3, 2, 4, 12, 11, 13, 30, 28, 27, 29, 35, 33, 32, 34, 37, 36, 38], vec![2, 3, 4, 5, 10, 12, 11, 13, 15, 30, 27, 28, 29, 25, 32, 33, 34, 35, 36, 38, 37]);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(15), Some(35), Some(10), Some(30), Some(33), Some(37), Some(5), Some(12), None, Some(28), Some(32), Some(34), Some(36), None, Some(3), None, None, Some(11), Some(27), Some(29), None, None, None, None, None, Some(38), Some(2), Some(4), None, Some(13)]);
}

#[test]
fn test_19() {
    let result = Solution::build_tree(vec![10, 5, 1, 3, 7, 6, 8, 15, 12, 11, 13, 18, 17, 20, 19], vec![1, 3, 5, 6, 7, 8, 10, 11, 12, 13, 15, 17, 18, 19, 20]);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(1), Some(7), Some(12), Some(18), None, Some(3), Some(6), Some(8), Some(11), Some(13), Some(17), Some(20), None, None, None, None, None, None, None, None, None, None, None, None, Some(19)]);
}

#[test]
fn test_20() {
    let result = Solution::build_tree(vec![10, 6, 2, 1, 3, 8, 5, 7, 12, 11, 13], vec![1, 2, 3, 6, 5, 7, 8, 10, 11, 12, 13]);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(6), Some(12), Some(2), Some(8), Some(11), Some(13), Some(1), Some(3), Some(5), None, None, None, None, None, None, None, None, None, None, Some(7)]);
}

#[test]
fn test_21() {
    let result = Solution::build_tree(vec![8, 5, 9, 7, 3, 6, 2, 4, 1, 10, 11], vec![9, 5, 7, 8, 6, 3, 2, 4, 1, 10, 11]);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(3), Some(9), Some(7), Some(6), Some(2), None, None, None, None, None, None, None, Some(4), None, Some(1), None, Some(10), None, Some(11)]);
}

#[test]
fn test_22() {
    let result = Solution::build_tree(vec![1, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3), Some(2), Some(5), None, None, Some(4), Some(7), None, None, Some(6), Some(9), None, None, Some(8), Some(11), None, None, Some(10), Some(13), None, None, Some(12), Some(15), None, None, Some(14)]);
}

#[test]
fn test_23() {
    let result = Solution::build_tree(vec![8, 5, 3, 1, 4, 7, 6, 9, 12, 10, 11, 14, 13, 15], vec![1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(9), Some(3), Some(7), None, Some(12), Some(1), Some(4), Some(6), None, Some(10), Some(14), None, None, None, None, None, None, None, Some(11), Some(13), Some(15)]);
}

#[test]
fn test_24() {
    let result = Solution::build_tree(vec![20, 10, 5, 3, 7, 15, 13, 18, 25, 22, 27], vec![3, 5, 7, 10, 13, 15, 18, 20, 22, 25, 27]);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(25), Some(5), Some(15), Some(22), Some(27), Some(3), Some(7), Some(13), Some(18)]);
}

#[test]
fn test_25() {
    let result = Solution::build_tree(vec![8, 5, 3, 2, 4, 7, 6, 9, 12, 10, 11, 14, 13, 15, 17, 16, 18], vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(9), Some(3), Some(7), None, Some(12), Some(2), Some(4), Some(6), None, Some(10), Some(14), None, None, None, None, None, None, None, Some(11), Some(13), Some(15), None, None, None, None, None, Some(17), Some(16), Some(18)]);
}

#[test]
fn test_26() {
    let result = Solution::build_tree(vec![20, 10, 5, 1, 6, 15, 12, 18, 30, 25, 35], vec![1, 5, 6, 10, 12, 15, 18, 20, 25, 30, 35]);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(1), Some(6), Some(12), Some(18)]);
}

#[test]
fn test_27() {
    let result = Solution::build_tree(vec![8, 5, 3, 1, 4, 7, 6, 9, 12, 10, 11, 13], vec![1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(9), Some(3), Some(7), None, Some(12), Some(1), Some(4), Some(6), None, Some(10), Some(13), None, None, None, None, None, None, None, Some(11)]);
}

#[test]
fn test_28() {
    let result = Solution::build_tree(vec![50, 30, 20, 10, 15, 40, 35, 37, 45, 60, 55, 52, 57, 70, 65, 75], vec![10, 15, 20, 30, 35, 37, 40, 45, 50, 52, 55, 57, 60, 65, 70, 75]);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(60), Some(20), Some(40), Some(55), Some(70), Some(10), None, Some(35), Some(45), Some(52), Some(57), Some(65), Some(75), None, Some(15), None, Some(37)]);
}

#[test]
fn test_29() {
    let result = Solution::build_tree(vec![7, 3, 1, 0, -1, 2, 5, 4, 6, 9, 8, 10, 11, 13, 12, 15, 14, 16, 18, 17, 20, 19, 22, 21, 23], vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]);
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(3), Some(9), Some(1), Some(5), Some(8), Some(10), Some(0), Some(2), Some(4), Some(6), None, None, None, Some(11), Some(-1), None, None, None, None, None, None, None, None, Some(13), None, None, Some(12), Some(15), None, None, Some(14), Some(16), None, None, None, Some(18), Some(17), Some(20), None, None, Some(19), Some(22), None, None, Some(21), Some(23)]);
}

#[test]
fn test_30() {
    let result = Solution::build_tree(vec![34, 23, 12, 9, 7, 8, 15, 13, 14, 26, 25, 27, 36, 35, 37, 45, 40, 42, 41, 43, 47, 46, 48], vec![7, 8, 9, 12, 13, 14, 15, 23, 25, 26, 27, 34, 35, 36, 37, 40, 41, 42, 43, 45, 46, 47, 48]);
    assert_eq!(tree_to_vec(&result), vec![Some(34), Some(23), Some(36), Some(12), Some(26), Some(35), Some(37), Some(9), Some(15), Some(25), Some(27), None, None, None, Some(45), Some(7), None, Some(13), None, None, None, None, None, Some(40), Some(47), None, Some(8), None, Some(14), None, Some(42), Some(46), Some(48), None, None, None, None, Some(41), Some(43)]);
}

#[test]
fn test_31() {
    let result = Solution::build_tree(vec![10, 5, 1, 3, 7, 6, 9, 8, 20, 15, 12, 18, 25], vec![1, 3, 5, 6, 7, 8, 9, 10, 12, 15, 18, 20, 25]);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(20), Some(1), Some(7), Some(15), Some(25), None, Some(3), Some(6), Some(9), Some(12), Some(18), None, None, None, None, None, None, Some(8)]);
}

#[test]
fn test_32() {
    let result = Solution::build_tree(vec![20, 10, 5, 3, 7, 15, 12, 18, 25, 22, 27], vec![3, 5, 7, 10, 12, 15, 18, 20, 22, 25, 27]);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(25), Some(5), Some(15), Some(22), Some(27), Some(3), Some(7), Some(12), Some(18)]);
}

#[test]
fn test_33() {
    let result = Solution::build_tree(vec![30, 15, 10, 5, 7, 25, 20, 18, 27, 40, 35, 32, 38, 50, 45, 55], vec![5, 7, 10, 15, 18, 20, 25, 27, 30, 32, 35, 38, 40, 45, 50, 55]);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(15), Some(40), Some(10), Some(25), Some(35), Some(50), Some(5), None, Some(20), Some(27), Some(32), Some(38), Some(45), Some(55), None, Some(7), Some(18)]);
}

#[test]
fn test_34() {
    let result = Solution::build_tree(vec![100, 75, 60, 40, 50, 90, 80, 120, 110, 130], vec![40, 50, 60, 75, 80, 90, 100, 110, 120, 130]);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(75), Some(120), Some(60), Some(90), Some(110), Some(130), Some(40), None, Some(80), None, None, None, None, None, None, Some(50)]);
}

#[test]
fn test_35() {
    let result = Solution::build_tree(vec![50, 30, 20, 10, 40, 70, 60, 90, 80, 110], vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 110]);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(90), Some(10), None, None, None, None, None, Some(80), Some(110)]);
}

#[test]
fn test_36() {
    let result = Solution::build_tree(vec![20, 10, 5, 3, 1, 7, 15, 12, 9, 14, 18, 17, 25, 23, 22, 24, 30, 28, 27, 29, 32, 31], vec![1, 3, 5, 7, 10, 9, 12, 14, 15, 20, 22, 23, 24, 25, 17, 18, 27, 28, 29, 30, 31, 32]);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(18), Some(5), Some(15), Some(17), Some(30), Some(3), Some(7), Some(12), None, Some(25), None, Some(28), Some(32), Some(1), None, None, None, Some(9), Some(14), Some(23), None, Some(27), Some(29), Some(31), None, None, None, None, None, None, None, Some(22), Some(24)]);
}

#[test]
fn test_37() {
    let result = Solution::build_tree(vec![5, 3, 1, 4, 8, 6, 7, 10, 9], vec![1, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(8), Some(1), Some(4), Some(6), Some(10), None, None, None, None, None, Some(7), Some(9)]);
}

#[test]
fn test_38() {
    let result = Solution::build_tree(vec![8, 5, 3, 1, 4, 7, 6, 9, 10, 11], vec![1, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(9), Some(3), Some(7), None, Some(10), Some(1), Some(4), Some(6), None, None, Some(11)]);
}

#[test]
fn test_39() {
    let result = Solution::build_tree(vec![100, 50, 25, 10, 30, 75, 60, 80, 150, 125, 175], vec![10, 25, 30, 50, 60, 75, 80, 100, 125, 150, 175]);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80)]);
}

#[test]
fn test_40() {
    let result = Solution::build_tree(vec![30, 15, 7, 3, 9, 22, 18, 20, 25, 40, 35, 37, 45, 50], vec![3, 7, 9, 15, 18, 20, 22, 25, 30, 35, 37, 40, 45, 50]);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(15), Some(40), Some(7), Some(22), Some(35), Some(45), Some(3), Some(9), Some(18), Some(25), None, Some(37), None, Some(50), None, None, None, None, None, Some(20)]);
}

#[test]
fn test_41() {
    let result = Solution::build_tree(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15), None, Some(16), None, Some(17), None, Some(18), None, Some(19), None, Some(20), None, Some(21), None, Some(22), None, Some(23), None, Some(24), None, Some(25), None, Some(26), None, Some(27), None, Some(28), None, Some(29), None, Some(30)]);
}

#[test]
fn test_42() {
    let result = Solution::build_tree(vec![8, 5, 3, 2, 4, 7, 6, 9, 11, 10, 12], vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(9), Some(3), Some(7), None, Some(11), Some(2), Some(4), Some(6), None, Some(10), Some(12)]);
}

#[test]
fn test_43() {
    let result = Solution::build_tree(vec![10, 5, 1, 2, 6, 3, 4, 8, 7, 9, 11, 12], vec![1, 2, 5, 3, 4, 6, 10, 7, 8, 9, 11, 12]);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(8), Some(1), Some(6), Some(7), Some(9), None, Some(2), Some(3), None, None, None, None, Some(11), None, None, None, Some(4), None, Some(12)]);
}

#[test]
fn test_44() {
    let result = Solution::build_tree(vec![5, 2, 1, 3, 4, 8, 6, 7, 9], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(2), Some(8), Some(1), Some(3), Some(6), Some(9), None, None, None, Some(4), None, Some(7)]);
}

#[test]
fn test_45() {
    let result = Solution::build_tree(vec![21, 15, 10, 8, 12, 17, 16, 18, 25, 23, 24, 28, 27, 29, 30], vec![8, 10, 12, 15, 16, 17, 18, 21, 23, 24, 25, 27, 28, 29, 30]);
    assert_eq!(tree_to_vec(&result), vec![Some(21), Some(15), Some(25), Some(10), Some(17), Some(23), Some(28), Some(8), Some(12), Some(16), Some(18), None, Some(24), Some(27), Some(29), None, None, None, None, None, None, None, None, None, None, None, None, None, Some(30)]);
}

#[test]
fn test_46() {
    let result = Solution::build_tree(vec![7, 3, 1, 5, 4, 6, 10, 9, 11, 14, 12, 13, 15], vec![1, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(3), Some(10), Some(1), Some(5), Some(9), Some(11), None, None, Some(4), Some(6), None, None, None, Some(14), None, None, None, None, Some(12), Some(15), None, Some(13)]);
}

#[test]
fn test_47() {
    let result = Solution::build_tree(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]);
}

#[test]
fn test_48() {
    let result = Solution::build_tree(vec![15, 5, 3, 2, 1, 4, 7, 6, 8, 10, 9, 11, 20, 18, 17, 19, 25, 23, 22, 24, 30, 28, 27, 29, 32, 31], vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 9, 11, 15, 17, 18, 19, 20, 22, 23, 24, 25, 27, 28, 29, 30, 31, 32]);
    assert_eq!(tree_to_vec(&result), vec![Some(15), Some(5), Some(20), Some(3), Some(7), Some(18), Some(25), Some(2), Some(4), Some(6), Some(8), Some(17), Some(19), Some(23), Some(30), Some(1), None, None, None, None, None, None, Some(10), None, None, None, None, Some(22), Some(24), Some(28), Some(32), None, None, None, Some(9), None, None, None, None, Some(27), Some(29), Some(31), None, None, Some(11)]);
}

#[test]
fn test_49() {
    let result = Solution::build_tree(vec![10, 5, 1, 7, 6, 11, 15, 12, 20, 18, 25], vec![1, 5, 6, 7, 10, 11, 12, 15, 18, 20, 25]);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(11), Some(1), Some(7), None, Some(15), None, None, Some(6), None, Some(12), Some(20), None, None, None, None, Some(18), Some(25)]);
}

#[test]
fn test_50() {
    let result = Solution::build_tree(vec![50, 30, 20, 10, 25, 40, 35, 45, 70, 60, 80], vec![10, 20, 25, 30, 35, 40, 45, 50, 60, 70, 80]);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45)]);
}

#[test]
fn test_51() {
    let result = Solution::build_tree(vec![40, 20, 10, 5, 3, 8, 15, 13, 18, 30, 25, 28, 50, 45, 42, 47, 55, 52, 58], vec![3, 5, 8, 10, 13, 15, 18, 20, 25, 28, 30, 40, 42, 45, 47, 50, 52, 55, 58]);
    assert_eq!(tree_to_vec(&result), vec![Some(40), Some(20), Some(50), Some(10), Some(30), Some(45), Some(55), Some(5), Some(15), Some(25), None, Some(42), Some(47), Some(52), Some(58), Some(3), Some(8), Some(13), Some(18), None, Some(28)]);
}

#[test]
fn test_52() {
    let result = Solution::build_tree(vec![8, 5, 3, 1, 4, 6, 7, 12, 10, 9, 11, 14, 13, 15], vec![1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(12), Some(3), Some(6), Some(10), Some(14), Some(1), Some(4), None, Some(7), Some(9), Some(11), Some(13), Some(15)]);
}

#[test]
fn test_53() {
    let result = Solution::build_tree(vec![60, 30, 15, 5, 25, 45, 40, 50, 90, 80, 100, 95, 110], vec![5, 15, 25, 30, 40, 45, 50, 60, 80, 90, 95, 100, 110]);
    assert_eq!(tree_to_vec(&result), vec![Some(60), Some(30), Some(90), Some(15), Some(45), Some(80), Some(100), Some(5), Some(25), Some(40), Some(50), None, None, Some(95), Some(110)]);
}

#[test]
fn test_54() {
    let result = Solution::build_tree(vec![25, 15, 10, 5, 3, 7, 12, 20, 18, 17, 19, 30, 28, 27, 29, 32, 31], vec![3, 5, 7, 10, 12, 15, 17, 18, 19, 20, 25, 27, 28, 29, 30, 31, 32]);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(15), Some(30), Some(10), Some(20), Some(28), Some(32), Some(5), Some(12), Some(18), None, Some(27), Some(29), Some(31), None, Some(3), Some(7), None, None, Some(17), Some(19)]);
}

#[test]
fn test_55() {
    let result = Solution::build_tree(vec![25, 15, 10, 5, 12, 20, 17, 30, 27, 32, 35], vec![5, 10, 12, 15, 17, 20, 25, 27, 30, 32, 35]);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(15), Some(30), Some(10), Some(20), Some(27), Some(32), Some(5), Some(12), Some(17), None, None, None, None, Some(35)]);
}

#[test]
fn test_56() {
    let result = Solution::build_tree(vec![50, 20, 10, 5, 15, 40, 25, 30, 35, 70, 60, 55, 65, 80, 75, 85], vec![5, 10, 15, 20, 25, 30, 35, 40, 50, 55, 60, 65, 70, 75, 80, 85]);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(20), Some(70), Some(10), Some(40), Some(60), Some(80), Some(5), Some(15), Some(25), None, Some(55), Some(65), Some(75), Some(85), None, None, None, None, None, Some(30), None, None, None, None, None, None, None, None, None, Some(35)]);
}

#[test]
fn test_57() {
    let result = Solution::build_tree(vec![7, 3, 1, 2, 5, 4, 6, 15, 9, 8, 10, 20, 18, 19, 25], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 18, 19, 20, 25]);
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(3), Some(15), Some(1), Some(5), Some(9), Some(20), None, Some(2), Some(4), Some(6), Some(8), Some(10), Some(18), Some(25), None, None, None, None, None, None, None, None, None, None, None, Some(19)]);
}

#[test]
fn test_58() {
    let result = Solution::build_tree(vec![50, 25, 10, 5, 3, 7, 20, 15, 30, 27, 35, 40, 45, 55], vec![3, 5, 7, 10, 15, 20, 25, 27, 30, 35, 40, 45, 50, 55]);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(25), Some(55), Some(10), Some(30), None, None, Some(5), Some(20), Some(27), Some(35), Some(3), Some(7), Some(15), None, None, None, None, Some(40), None, None, None, None, None, None, None, Some(45)]);
}

#[test]
fn test_59() {
    let result = Solution::build_tree(vec![10, 5, 1, 2, 3, 8, 6, 7, 15, 13, 12, 14, 20, 18, 17, 19, 25, 23, 22, 24, 28, 27, 26, 29], vec![1, 2, 3, 5, 6, 7, 8, 10, 12, 13, 14, 15, 17, 18, 19, 20, 22, 23, 24, 25, 26, 27, 28, 29]);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(1), Some(8), Some(13), Some(20), None, Some(2), Some(6), None, Some(12), Some(14), Some(18), Some(25), None, Some(3), None, Some(7), None, None, None, None, Some(17), Some(19), Some(23), Some(28), None, None, None, None, None, None, None, None, Some(22), Some(24), Some(27), Some(29), None, None, None, None, Some(26)]);
}

#[test]
fn test_60() {
    let result = Solution::build_tree(vec![25, 15, 10, 4, 12, 6, 8, 20, 16, 22, 28, 27, 30, 32], vec![4, 10, 6, 8, 12, 15, 16, 20, 22, 25, 27, 28, 30, 32]);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(15), Some(28), Some(10), Some(20), Some(27), Some(30), Some(4), Some(12), Some(16), Some(22), None, None, None, Some(32), None, None, Some(6), None, None, None, None, None, None, None, None, Some(8)]);
}

#[test]
fn test_61() {
    let result = Solution::build_tree(vec![10, 5, 2, 1, 3, 8, 6, 7, 15, 12, 11, 13, 20, 18, 19, 25], vec![1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 15, 18, 19, 20, 25]);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(2), Some(8), Some(12), Some(20), Some(1), Some(3), Some(6), None, Some(11), Some(13), Some(18), Some(25), None, None, None, None, None, Some(7), None, None, None, None, None, Some(19)]);
}

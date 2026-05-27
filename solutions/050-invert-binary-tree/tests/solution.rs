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
    let result = Solution::invert_tree(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]));
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)]);
}

#[test]
fn test_2() {
    let result = Solution::invert_tree(build_tree(&[Some(2), Some(1), Some(3)]));
    assert_eq!(tree_to_vec(&result), vec![Some(2), Some(3), Some(1)]);
}

#[test]
fn test_3() {
    let result = Solution::invert_tree(build_tree(&[]));
    assert_eq!(tree_to_vec(&result), vec![]);
}

#[test]
fn test_4() {
    let result = Solution::invert_tree(build_tree(&[Some(3), Some(1), Some(4), None, Some(2)]));
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(4), Some(1), None, None, Some(2)]);
}

#[test]
fn test_5() {
    let result = Solution::invert_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)]);
}

#[test]
fn test_6() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), None, Some(10), Some(11), Some(12)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(7), None, Some(5), Some(4), None, Some(12), Some(11), Some(10), None, Some(8)]);
}

#[test]
fn test_7() {
    let result = Solution::invert_tree(build_tree(&[Some(50), Some(20), Some(70), Some(10), Some(30), Some(60), Some(80), Some(5), Some(15), Some(25), Some(35), Some(55), Some(65), Some(75), Some(85)]));
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(70), Some(20), Some(80), Some(60), Some(30), Some(10), Some(85), Some(75), Some(65), Some(55), Some(35), Some(25), Some(15), Some(5)]);
}

#[test]
fn test_8() {
    let result = Solution::invert_tree(build_tree(&[Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)]));
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(4), Some(1), Some(5), Some(1), None, Some(3)]);
}

#[test]
fn test_9() {
    let result = Solution::invert_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)]);
}

#[test]
fn test_10() {
    let result = Solution::invert_tree(build_tree(&[Some(8), Some(5), Some(9), Some(3), Some(7), Some(12), Some(15), Some(1), Some(4), Some(6), Some(10), Some(13), Some(17)]));
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(9), Some(5), Some(15), Some(12), Some(7), Some(3), None, None, Some(17), Some(13), Some(10), Some(6), Some(4), Some(1)]);
}

#[test]
fn test_11() {
    let result = Solution::invert_tree(build_tree(&[Some(5), Some(15), Some(1), None, Some(30), Some(20), None]));
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(1), Some(15), None, Some(20), Some(30)]);
}

#[test]
fn test_12() {
    let result = Solution::invert_tree(build_tree(&[Some(8), Some(15), Some(3), Some(7), Some(19), Some(1), Some(2)]));
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(3), Some(15), Some(2), Some(1), Some(19), Some(7)]);
}

#[test]
fn test_13() {
    let result = Solution::invert_tree(build_tree(&[Some(6), Some(3), Some(8), Some(1), Some(4), Some(7), Some(9), None, None, Some(2), Some(5)]));
    assert_eq!(tree_to_vec(&result), vec![Some(6), Some(8), Some(3), Some(9), Some(7), Some(4), Some(1), None, None, None, None, Some(5), Some(2)]);
}

#[test]
fn test_14() {
    let result = Solution::invert_tree(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(3), Some(7), Some(13), Some(17), Some(23), Some(27), Some(33), Some(37)]));
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(30), Some(10), Some(35), Some(25), Some(15), Some(5), Some(37), Some(33), Some(27), Some(23), Some(17), Some(13), Some(7), Some(3)]);
}

#[test]
fn test_15() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(2)]);
}

#[test]
fn test_16() {
    let result = Solution::invert_tree(build_tree(&[Some(10), None, Some(20), None, Some(30), None, Some(40)]));
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(20), None, Some(30), None, Some(40)]);
}

#[test]
fn test_17() {
    let result = Solution::invert_tree(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]));
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(20), Some(9), Some(7), Some(15)]);
}

#[test]
fn test_18() {
    let result = Solution::invert_tree(build_tree(&[Some(7), Some(3), Some(15), Some(1), Some(5), Some(9), Some(20), None, None, None, Some(6), None, Some(18)]));
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(15), Some(3), Some(20), Some(9), Some(5), Some(1), None, None, Some(18), None, Some(6)]);
}

#[test]
fn test_19() {
    let result = Solution::invert_tree(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)]));
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(15), Some(5), Some(20), Some(6)]);
}

#[test]
fn test_20() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(3), Some(2), Some(5), Some(4)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]);
}

#[test]
fn test_21() {
    let result = Solution::invert_tree(build_tree(&[Some(8), Some(5), Some(9), Some(2), Some(7), None, None, None, Some(3), None, Some(4)]));
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(9), Some(5), None, None, Some(7), Some(2), Some(4), None, Some(3)]);
}

#[test]
fn test_22() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(5), None, Some(4)]);
}

#[test]
fn test_23() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, Some(16), Some(17), Some(18), Some(19), Some(20)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4), Some(15), Some(14), Some(13), Some(12), Some(11), Some(10), Some(9), Some(8), None, None, None, None, None, None, None, None, None, Some(20), Some(19), Some(18), Some(17), Some(16)]);
}

#[test]
fn test_24() {
    let result = Solution::invert_tree(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]));
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(15), Some(5), Some(18), None, Some(7), Some(3)]);
}

#[test]
fn test_25() {
    let result = Solution::invert_tree(build_tree(&[Some(-10), Some(-5), Some(-15), Some(-3), Some(-8), None, Some(-20)]));
    assert_eq!(tree_to_vec(&result), vec![Some(-10), Some(-15), Some(-5), Some(-20), None, Some(-8), Some(-3)]);
}

#[test]
fn test_26() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(3), Some(2), None, Some(6), Some(5), Some(4), None, None, None, None, None, Some(7)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), None, Some(7)]);
}

#[test]
fn test_27() {
    let result = Solution::invert_tree(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1)]));
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(7), Some(3), Some(8), Some(6), Some(4), Some(2), None, None, None, None, None, None, None, Some(1)]);
}

#[test]
fn test_28() {
    let result = Solution::invert_tree(build_tree(&[Some(100), Some(-50), Some(-20), Some(-60), Some(-80), Some(-30), Some(-10)]));
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(-20), Some(-50), Some(-10), Some(-30), Some(-80), Some(-60)]);
}

#[test]
fn test_29() {
    let result = Solution::invert_tree(build_tree(&[Some(5), Some(1), Some(8), None, None, Some(6), Some(9)]));
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(8), Some(1), Some(9), Some(6)]);
}

#[test]
fn test_30() {
    let result = Solution::invert_tree(build_tree(&[Some(-1), Some(-2), Some(-3), Some(-4), Some(-5), Some(-6), Some(-7)]));
    assert_eq!(tree_to_vec(&result), vec![Some(-1), Some(-3), Some(-2), Some(-7), Some(-6), Some(-5), Some(-4)]);
}

#[test]
fn test_31() {
    let result = Solution::invert_tree(build_tree(&[Some(7), Some(-3), Some(9), Some(-5), Some(-4), Some(8), Some(-10), None, None, None, Some(-9), Some(-8), Some(-6)]));
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(9), Some(-3), Some(-10), Some(8), Some(-4), Some(-5), None, None, Some(-6), Some(-8), Some(-9)]);
}

#[test]
fn test_32() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(8), None, Some(9), None, Some(10)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(5), None, None, Some(4), None, Some(8), None, Some(6), None, Some(10), None, Some(9)]);
}

#[test]
fn test_33() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]);
}

#[test]
fn test_34() {
    let result = Solution::invert_tree(build_tree(&[Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)]));
    assert_eq!(tree_to_vec(&result), vec![Some(6), Some(8), Some(2), Some(9), Some(7), Some(4), Some(0), None, None, None, None, Some(5), Some(3)]);
}

#[test]
fn test_35() {
    let result = Solution::invert_tree(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1)]));
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(8), Some(4), Some(4), Some(13), None, Some(11), Some(1), Some(5), None, None, Some(2), Some(7)]);
}

#[test]
fn test_36() {
    let result = Solution::invert_tree(build_tree(&[Some(1), None, Some(2), None, None, None, Some(3), None, None, None, None, None, None, Some(4)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2)]);
}

#[test]
fn test_37() {
    let result = Solution::invert_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)]);
}

#[test]
fn test_38() {
    let result = Solution::invert_tree(build_tree(&[Some(8), Some(5), Some(10), None, None, Some(6), Some(12), None, None, Some(7), Some(9)]));
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(10), Some(5), Some(12), Some(6), None, None, Some(9), Some(7)]);
}

#[test]
fn test_39() {
    let result = Solution::invert_tree(build_tree(&[Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, Some(3), Some(5), None, None, None, None, None, None]));
    assert_eq!(tree_to_vec(&result), vec![Some(6), Some(8), Some(2), Some(9), Some(7), Some(4), Some(0), None, None, None, None, None, Some(5), Some(3)]);
}

#[test]
fn test_40() {
    let result = Solution::invert_tree(build_tree(&[Some(5), Some(4), Some(7), Some(3), None, Some(6), Some(9)]));
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(7), Some(4), Some(9), Some(6), None, Some(3)]);
}

#[test]
fn test_41() {
    let result = Solution::invert_tree(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(18), None, Some(30), None, Some(16), Some(22), Some(27), None, None, Some(19)]));
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(25), Some(15), Some(30), None, Some(18), Some(10), None, None, Some(27), Some(22), Some(16), None, None, None, None, None, None, Some(19)]);
}

#[test]
fn test_42() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), None, Some(10), None, None, Some(11), Some(12), None, None, Some(13), None, Some(14), Some(15)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4), Some(12), Some(11), None, None, Some(10), None, Some(9), Some(8), None, None, None, None, Some(15), Some(14), None, Some(13)]);
}

#[test]
fn test_43() {
    let result = Solution::invert_tree(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]));
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(4), Some(1), Some(6), Some(3)]);
}

#[test]
fn test_44() {
    let result = Solution::invert_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)]);
}

#[test]
fn test_45() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(3), Some(2), None, None, Some(5), Some(4)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
}

#[test]
fn test_46() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), None, Some(7), Some(8), None, None, None, None, Some(9)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), None, Some(6), Some(5), Some(4), None, None, None, None, Some(8), Some(7), None, None, None, Some(9)]);
}

#[test]
fn test_47() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)]);
}

#[test]
fn test_48() {
    let result = Solution::invert_tree(build_tree(&[Some(100), Some(-100), Some(50), Some(25), Some(75), Some(-25), Some(-50)]));
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(-100), Some(-50), Some(-25), Some(75), Some(25)]);
}

#[test]
fn test_49() {
    let result = Solution::invert_tree(build_tree(&[Some(5), Some(3), Some(8), Some(1), Some(4), Some(7), Some(9), None, None, Some(2), Some(6)]));
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(8), Some(3), Some(9), Some(7), Some(4), Some(1), None, None, None, None, Some(6), Some(2)]);
}

#[test]
fn test_50() {
    let result = Solution::invert_tree(build_tree(&[Some(0), Some(0), Some(0), Some(0), Some(0), None, None, Some(0), Some(0)]));
    assert_eq!(tree_to_vec(&result), vec![Some(0), Some(0), Some(0), None, None, Some(0), Some(0), None, None, Some(0), Some(0)]);
}

#[test]
fn test_51() {
    let result = Solution::invert_tree(build_tree(&[Some(5), Some(4), Some(7), Some(3), None, Some(2), None, Some(-1)]));
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(7), Some(4), None, Some(2), None, Some(3), None, None, None, Some(-1)]);
}

#[test]
fn test_52() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), None, None, None, None, None, Some(14)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4), None, None, Some(13), Some(12), Some(11), Some(10), Some(9), Some(8), None, None, None, None, None, None, None, None, Some(14)]);
}

#[test]
fn test_53() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4), Some(15), Some(14), Some(13), Some(12), Some(11), Some(10), Some(9), Some(8), None, None, None, None, None, None, None, None, None, None, None, Some(20), Some(19), Some(18), Some(17), Some(16)]);
}

#[test]
fn test_54() {
    let result = Solution::invert_tree(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2)]);
}

#[test]
fn test_55() {
    let result = Solution::invert_tree(build_tree(&[Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)]));
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(1), Some(5), Some(8), Some(0), Some(2), Some(6), None, None, None, None, Some(4), Some(7)]);
}

#[test]
fn test_56() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5), None, None, Some(6), Some(7), None, None, Some(8), Some(9)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(5), Some(4), None, None, Some(7), Some(6), None, None, Some(9), Some(8)]);
}

#[test]
fn test_57() {
    let result = Solution::invert_tree(build_tree(&[Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)]));
    assert_eq!(tree_to_vec(&result), vec![Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6)]);
}

#[test]
fn test_58() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4), Some(15), Some(14), Some(13), Some(12), Some(11), Some(10), Some(9), Some(8)]);
}

#[test]
fn test_59() {
    let result = Solution::invert_tree(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2)]);
}

#[test]
fn test_60() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)]);
}

#[test]
fn test_61() {
    let result = Solution::invert_tree(build_tree(&[Some(100), Some(-50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(15), Some(35), Some(65), Some(85), Some(115), Some(145), Some(165), Some(185)]));
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(150), Some(-50), Some(175), Some(125), Some(75), Some(25), Some(185), Some(165), Some(145), Some(115), Some(85), Some(65), Some(35), Some(15)]);
}

#[test]
fn test_62() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4), Some(15), Some(14), Some(13), Some(12), Some(11), Some(10), Some(9), Some(8), Some(31), Some(30), Some(29), Some(28), Some(27), Some(26), Some(25), Some(24), Some(23), Some(22), Some(21), Some(20), Some(19), Some(18), Some(17), Some(16)]);
}

#[test]
fn test_63() {
    let result = Solution::invert_tree(build_tree(&[Some(8), Some(5), Some(1), Some(7), Some(10), Some(12)]));
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(1), Some(5), None, Some(12), Some(10), Some(7)]);
}

#[test]
fn test_64() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)]);
}

#[test]
fn test_65() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(3), None, None, Some(2)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3), Some(2)]);
}

#[test]
fn test_66() {
    let result = Solution::invert_tree(build_tree(&[Some(1), None, Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), None, Some(4), Some(3), Some(8), Some(7), Some(6), Some(5), None, Some(15), Some(14), Some(13), Some(12), Some(11), Some(10), Some(9), Some(28), Some(27), Some(26), Some(25), Some(24), Some(23), Some(22), Some(21), Some(20), Some(19), Some(18), Some(17), Some(16), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(30), Some(29)]);
}

#[test]
fn test_67() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(7), Some(8), None, None, Some(14)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(7), None, None, Some(4), Some(14), None, None, Some(8)]);
}

#[test]
fn test_68() {
    let result = Solution::invert_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), None, None, None, None, Some(8)]));
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(2), Some(6), None, Some(5), Some(4), Some(8), None, None, None, None, Some(7)]);
}

#[test]
fn test_69() {
    let result = Solution::invert_tree(build_tree(&[Some(10), None, Some(20), None, None, Some(15), Some(25)]));
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(20)]);
}

#[test]
fn test_70() {
    let result = Solution::invert_tree(build_tree(&[Some(5), Some(1), None, None, Some(4), Some(3), None, None, Some(2)]));
    assert_eq!(tree_to_vec(&result), vec![Some(5), None, Some(1), Some(4), None, None, Some(3), Some(2)]);
}

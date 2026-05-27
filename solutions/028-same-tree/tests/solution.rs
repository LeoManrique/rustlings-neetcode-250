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
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3)]), build_tree(&[Some(1), Some(2), Some(3), Some(4)])), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2)]), build_tree(&[Some(1), None, Some(2)])), false);
}

#[test]
fn test_3() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(0)]), build_tree(&[Some(1), None])), false);
}

#[test]
fn test_4() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(3)]), build_tree(&[Some(1), None, Some(3)])), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3)]), build_tree(&[Some(1), Some(2), Some(3)])), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)])), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3)]), build_tree(&[Some(1), None, Some(2), None, Some(3)])), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2)]), build_tree(&[Some(1), Some(2), None])), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(1)]), build_tree(&[Some(1), Some(1), Some(2)])), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::is_same_tree(build_tree(&[]), build_tree(&[])), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(3)]), build_tree(&[Some(1), Some(2), Some(3)])), false);
}

#[test]
fn test_12() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]), build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)])), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1)]), build_tree(&[Some(1)])), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(6)])), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), None, Some(3)]), build_tree(&[Some(1), Some(2), None, Some(3)])), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), None, Some(3)]), build_tree(&[Some(1), Some(2), None, Some(4)])), false);
}

#[test]
fn test_17() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)])), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15)]), build_tree(&[Some(10), Some(5), None, None, Some(15)])), false);
}

#[test]
fn test_19() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(16)])), false);
}

#[test]
fn test_20() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), None, None, None, Some(12)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), None, None, None, Some(13)])), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(7)])), false);
}

#[test]
fn test_22() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), Some(9)])), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), None, Some(10), Some(11)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), None, Some(10), Some(11)])), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]), build_tree(&[Some(5), Some(1), Some(4), None, None, Some(2), Some(6)])), false);
}

#[test]
fn test_26() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14)])), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(14)])), false);
}

#[test]
fn test_29() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), None, None, Some(6), Some(9)]), build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), None, None, Some(6), Some(9)])), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6)])), false);
}

#[test]
fn test_31() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(7), Some(8), None, None, Some(9), Some(10)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(7), Some(8), None, None, Some(10), Some(9)])), false);
}

#[test]
fn test_32() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(21)])), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), None, Some(18), Some(19)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), None, Some(18), Some(19)])), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(17)])), false);
}

#[test]
fn test_35() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), None, None, Some(6), Some(9)]), build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(19), None, None, Some(6), Some(9)])), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(9)])), false);
}

#[test]
fn test_38() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15)]), build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(16)])), false);
}

#[test]
fn test_39() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, Some(16)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, Some(17)])), false);
}

#[test]
fn test_40() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, None, None, Some(6), Some(7)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, None, None, Some(6), Some(8)])), false);
}

#[test]
fn test_41() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(10)])), false);
}

#[test]
fn test_42() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7)])), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6), Some(7)]), build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6), None])), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, Some(6), None, None, Some(7), None, Some(8)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, Some(6), None, None, Some(7), None, Some(9)])), false);
}

#[test]
fn test_45() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, None, Some(6), Some(7), Some(8)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, None, Some(6), Some(7), Some(9)])), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13)])), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), Some(7), None, Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), Some(7), None, Some(8), Some(10)])), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(11)])), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), None, Some(18), Some(19)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), None, Some(18), None])), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(10)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(10)])), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(6)])), false);
}

#[test]
fn test_52() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(9)])), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5)])), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9), Some(10), Some(11)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9), Some(10), Some(11)])), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None])), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), None, Some(8), Some(9), Some(10)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), None, Some(8), Some(9), Some(10)])), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), None, Some(8), Some(9), Some(10)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), None, Some(8), None, Some(10)])), false);
}

#[test]
fn test_58() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), None, Some(8)]), build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), None, Some(8)])), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(6)])), false);
}

#[test]
fn test_61() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(10)])), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(3), Some(5), Some(1), Some(6), Some(2), Some(9), Some(8), None, None, Some(7), Some(4)]), build_tree(&[Some(3), Some(5), Some(1), Some(6), Some(2), Some(9), Some(8), None, None, Some(7), Some(3)])), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), None, None, None, Some(12)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), None, None, None, Some(12)])), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13), Some(15), Some(14)])), false);
}

#[test]
fn test_65() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, Some(10), Some(11)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, Some(10), Some(12)])), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6)])), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(3), Some(5), Some(1), Some(6), Some(2), Some(9), Some(8), None, None, Some(7), Some(4)]), build_tree(&[Some(3), Some(5), Some(1), Some(6), Some(2), Some(9), Some(8), None, None, Some(7), Some(4)])), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6), None, None, Some(7), Some(8)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6), None, None, Some(8), Some(7)])), false);
}

#[test]
fn test_69() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9), Some(10), Some(11)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9), Some(10), Some(12)])), false);
}

#[test]
fn test_70() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31)])), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), None, Some(7)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), None, Some(8)])), false);
}

#[test]
fn test_72() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(7), Some(8), Some(9)])), false);
}

#[test]
fn test_73() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6)])), false);
}

#[test]
fn test_74() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]), build_tree(&[Some(5), Some(1), Some(4), None, None, Some(6), Some(3)])), false);
}

#[test]
fn test_75() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]), build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(19)])), false);
}

#[test]
fn test_76() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, Some(6), None, None, Some(7)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, Some(6), None, None, Some(8)])), false);
}

#[test]
fn test_77() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4)])), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), Some(17)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), Some(17)])), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(10)])), false);
}

#[test]
fn test_80() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(6)])), false);
}

#[test]
fn test_81() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), true);
}

#[test]
fn test_82() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6)]), build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(2), None, Some(6)])), false);
}

#[test]
fn test_83() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11)])), true);
}

#[test]
fn test_84() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, None, Some(6)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, None, Some(6)])), false);
}

#[test]
fn test_85() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), None])), false);
}

#[test]
fn test_86() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(6), Some(5)])), false);
}

#[test]
fn test_87() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]), build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])), true);
}

#[test]
fn test_88() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13), Some(14), Some(15)])), true);
}

#[test]
fn test_89() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(7), Some(8)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(7), Some(8)])), true);
}

#[test]
fn test_90() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16)])), true);
}

#[test]
fn test_91() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13), Some(14), Some(16)])), false);
}

#[test]
fn test_92() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6)]), build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6)])), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, Some(10), Some(11), None, Some(12), Some(13), None, Some(14), Some(15), None, Some(16), Some(17), None, Some(18), Some(19), None, Some(20), Some(21)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, Some(10), Some(11), None, Some(12), Some(13), None, Some(14), Some(15), None, Some(16), Some(17), None, Some(18), Some(19), None, Some(20), Some(21)])), true);
}

#[test]
fn test_94() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), None, Some(18)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), None, Some(19)])), false);
}

#[test]
fn test_95() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), Some(17)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), None])), false);
}

#[test]
fn test_96() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6), None, None, Some(7), Some(8)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6), None, None, Some(7), Some(8)])), true);
}

#[test]
fn test_97() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(6), Some(7), Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), None, None, Some(6), Some(7), Some(8), Some(9)])), true);
}

#[test]
fn test_98() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), Some(8), None, None, Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), Some(8), None, None, Some(9)])), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, None])), false);
}

#[test]
fn test_100() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)])), true);
}

#[test]
fn test_101() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(10), None, None, Some(12), Some(13)])), false);
}

#[test]
fn test_102() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)]), build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)])), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(10)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(11)])), false);
}

#[test]
fn test_104() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9)])), true);
}

#[test]
fn test_105() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), Some(6)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), Some(6)])), true);
}

#[test]
fn test_106() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), None, Some(8)]), build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), None, Some(9)])), false);
}

#[test]
fn test_107() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), Some(3)]), build_tree(&[Some(1), Some(2), None, Some(3)])), false);
}

#[test]
fn test_108() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(8)])), false);
}

#[test]
fn test_109() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), false);
}

#[test]
fn test_110() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), None, Some(18)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), None, Some(18)])), true);
}

#[test]
fn test_111() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)]), build_tree(&[Some(10), Some(5), Some(15), None, None, Some(7), Some(20)])), false);
}

#[test]
fn test_112() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(7), Some(8), None, None, Some(9), Some(10)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(7), Some(8), None, None, Some(9), Some(11)])), false);
}

#[test]
fn test_113() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9)])), true);
}

#[test]
fn test_114() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(10)])), false);
}

#[test]
fn test_115() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6)])), true);
}

#[test]
fn test_116() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4)]), build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(5)])), false);
}

#[test]
fn test_117() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, Some(10), Some(11), None, Some(12), Some(13), None, Some(14), Some(15), None, Some(16), Some(17), None, Some(18), Some(19), None, Some(20), Some(21)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, Some(10), Some(11), None, Some(12), Some(13), None, Some(14), Some(15), None, Some(16), Some(17), None, Some(18), Some(19), None, Some(20), Some(22)])), false);
}

#[test]
fn test_118() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(12)])), false);
}

#[test]
fn test_119() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), true);
}

#[test]
fn test_120() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, Some(10), Some(11)]), build_tree(&[Some(1), Some(2), Some(3), None, Some(4), Some(5), None, Some(6), Some(7), None, Some(8), Some(9), None, Some(10), Some(11)])), true);
}

#[test]
fn test_121() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(16)])), false);
}

#[test]
fn test_122() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, Some(16), Some(17), Some(18), Some(19)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, Some(16), Some(17), Some(18), Some(20)])), false);
}

#[test]
fn test_123() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(6), Some(7), Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), None, None, Some(6), Some(7), None, Some(9)])), false);
}

#[test]
fn test_124() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None])), false);
}

#[test]
fn test_125() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8)])), false);
}

#[test]
fn test_126() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None])), true);
}

#[test]
fn test_127() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1)]), build_tree(&[Some(2)])), false);
}

#[test]
fn test_128() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(32)])), false);
}

#[test]
fn test_129() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(7), Some(8)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(7), Some(9)])), false);
}

#[test]
fn test_130() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9), None, None, Some(12), Some(13)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7), Some(8), Some(9)])), false);
}

#[test]
fn test_131() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(15), Some(14)])), false);
}

#[test]
fn test_132() {
    assert_eq!(Solution::is_same_tree(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]), build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), true);
}

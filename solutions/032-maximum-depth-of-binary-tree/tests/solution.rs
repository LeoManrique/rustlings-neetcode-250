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
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)])), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4), Some(5), Some(5)])), 4);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), 5);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), 4);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(3), Some(2), None, Some(4), Some(5)])), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5)])), 3);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), None, None, None, Some(8)])), 4);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4)])), 4);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2)])), 2);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, None, Some(2), None, None, Some(3), None, None, Some(4), None, None, Some(5)])), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(0)])), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])), 3);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1)])), 1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_depth(build_tree(&[])), 0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(8), None, Some(6), None, Some(4), None, Some(1), None])), 5);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), 10);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(2), Some(3), None, None, Some(3), Some(4), None, None, Some(4)])), 4);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), Some(15), Some(7), Some(16), Some(8), Some(13), Some(14), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23)])), 4);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, None, Some(6), None, None, Some(7)])), 5);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])), 3);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4), Some(4), Some(4)])), 4);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9)])), 4);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(-2), Some(-3), Some(4), Some(5), Some(-6), Some(-7), Some(8), Some(9), Some(-10), Some(-11), Some(-12), Some(-13), Some(-14), Some(-15)])), 4);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), None, Some(16), None, Some(17), None, None, None, None, Some(18)])), 4);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15)])), 15);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), 9);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, Some(6), Some(7), None, Some(8), None, None, None, None, None, Some(9)])), 5);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)])), 8);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), None, Some(3), Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14)])), 8);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(10), Some(11), None, Some(12)])), 4);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5), None, None, Some(6), None, None, Some(7), Some(8), Some(9)])), 6);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)])), 8);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(16), Some(17), Some(18), Some(19), None, None, Some(20), None])), 5);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31)])), 5);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(8), None, Some(6), None, Some(4), None, Some(1), None, Some(2)])), 6);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(0), Some(-1), Some(-2), Some(-3), Some(-4), Some(-5), Some(-6)])), 3);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3), Some(5), Some(6), None, None, Some(6), Some(5)])), 4);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, None, Some(16)])), 5);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), Some(3), None, None, Some(4), Some(5), None, None, None, Some(6), Some(7), None, None, Some(8), Some(9)])), 5);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, None, Some(6), None, Some(7)])), 5);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9), Some(10), Some(11), Some(12), Some(13)])), 5);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), None, None, None, None, Some(17), Some(20)])), 4);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(2), Some(3), None, None, Some(3), Some(4), Some(5), None, None, Some(4), Some(5)])), 5);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(8), None, Some(9), None, None, Some(10)])), 5);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(8), Some(9), Some(10), None, None, Some(11), Some(12)])), 5);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(2), Some(-1), Some(-2), None, Some(-3), None, Some(-4), None, Some(-5)])), 4);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5), None, None, Some(6), Some(7)])), 4);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15), None, Some(16), None, Some(17), None, Some(18), None, Some(19), None, Some(20), None, Some(21), None, Some(22), None, Some(23), None, Some(24), None, Some(25), None, Some(26), None, Some(27), None, Some(28), None, Some(29), None, Some(30), None, Some(31), None, Some(32)])), 32);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(10), Some(9), Some(20), None, None, Some(15), Some(7), Some(12), Some(13), Some(14), Some(15)])), 4);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(8), Some(10), Some(11), None, Some(12)])), 5);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, Some(6), None, Some(7), None, None, Some(8), None, Some(9)])), 6);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30)])), 5);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), None, Some(7), None, Some(8), None, Some(9)])), 5);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), 5);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), None, Some(15), Some(7), Some(25), Some(30), Some(35), None, None, None, Some(40), None, None, None, None, Some(45)])), 6);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(2), Some(1), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), 6);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)])), 5);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(5), Some(4), Some(7), Some(3), None, Some(2), None, Some(-1), None, Some(9)])), 4);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, None, None, Some(6), Some(7), Some(8), Some(9)])), 5);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, None, None, Some(3), None, None, None, None, Some(4)])), 2);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), Some(40), Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), Some(48), Some(49), Some(50)])), 6);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(8), Some(9)])), 4);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, Some(6), None, None, Some(7), None, Some(8)])), 6);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, None, None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11)])), 9);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, None, None, None, Some(6), None, None, Some(7), None, None, Some(8), None, None, Some(9)])), 3);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7), Some(8), None, None, None, None, Some(9)])), 4);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12)])), 5);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(1), None, Some(18), None, None, Some(21)])), 5);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), 7);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), None, None, None, None, None, None, None, None, Some(9), None, None, None, None, Some(10)])), 5);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, None, Some(6), Some(7), None, Some(8), None, Some(9)])), 5);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), None, None, None, None, None, None, None, None, Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30)])), 8);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15)])), 15);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), None, None, Some(16), Some(17), None, None, None, None, Some(18)])), 4);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(2), Some(3), None, None, Some(3), Some(4), Some(4), None, None, Some(4), Some(4)])), 5);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), None, None, Some(7), Some(8), Some(9), Some(10), None, None, Some(11), Some(12)])), 5);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(50), Some(25), Some(75), Some(10), Some(35), Some(65), Some(100), None, None, Some(30), Some(40), Some(60), Some(80), None, None, None, None, Some(90), Some(110)])), 5);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15), None, Some(16), None, Some(17), None, Some(18), None, Some(19), None, Some(20)])), 20);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(0), Some(2), Some(4), Some(1), None, Some(3), Some(-1), Some(5), Some(1), None, Some(6), None, Some(8)])), 4);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), Some(8), Some(10), Some(15), Some(7), Some(6), None, None, None, None, Some(18)])), 4);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), Some(32)])), 6);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)])), 6);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), None, None, Some(16), Some(17), None, None, Some(18), Some(19)])), 5);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), Some(40), Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), Some(55), Some(56), Some(57), Some(58), Some(59), Some(60), Some(61), Some(62), Some(63), Some(64), Some(65), Some(66), Some(67), Some(68), Some(69), Some(70), Some(71), Some(72), Some(73), Some(74), Some(75), Some(76), Some(77), Some(78), Some(79), Some(80), Some(81), Some(82), Some(83), Some(84), Some(85), Some(86), Some(87), Some(88), Some(89), Some(90), Some(91), Some(92), Some(93), Some(94), Some(95), Some(96), Some(97), Some(98), Some(99), Some(100)])), 7);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), 6);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, Some(8), Some(9), None, None, Some(10), Some(11), None, None, Some(12), Some(13), None, None, Some(14), Some(15)])), 5);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12)])), 12);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), Some(3), Some(3), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), None, None, Some(5), Some(5)])), 5);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13)])), 13);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), 9);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(3), Some(9), Some(20), Some(15), Some(7), None, None, Some(8), None, None, None, None, Some(10)])), 5);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11)])), 11);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, Some(8), None, Some(9), None, None, Some(10)])), 4);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), None, None, None, None, None, None])), 1);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), None, None, Some(9), None, None, Some(10), Some(11), Some(12), None, None, Some(13), Some(14), Some(15), None, None, None, None, None, None, Some(16)])), 6);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)])), 3);
}

#[test]
fn test_97() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9)])), 4);
}

#[test]
fn test_98() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)])), 3);
}

#[test]
fn test_99() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, Some(6), Some(7), Some(8), Some(9), None, None, None, Some(10), None, Some(11)])), 5);
}

#[test]
fn test_100() {
    assert_eq!(Solution::max_depth(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1)])), 4);
}

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
    assert_eq!(Solution::good_nodes(build_tree(&[Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)])), 4);
}

#[test]
fn test_2() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-10), Some(-20), Some(-5), Some(-15), Some(-30)])), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(10), Some(20), Some(15), Some(25)])), 5);
}

#[test]
fn test_4() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-10), None, Some(-20), None, Some(-30)])), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(3), Some(3), None, Some(4), Some(2)])), 3);
}

#[test]
fn test_6() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(4), Some(6), Some(3), Some(8), Some(5), Some(7)])), 4);
}

#[test]
fn test_7() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)])), 3);
}

#[test]
fn test_8() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-10000), Some(-5000), Some(-2500)])), 3);
}

#[test]
fn test_9() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1)])), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), 5);
}

#[test]
fn test_11() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(20), Some(10), Some(25), Some(5), Some(15), Some(22), Some(30), Some(2), Some(7), Some(12), Some(18), Some(21), Some(24), Some(28), Some(32)])), 4);
}

#[test]
fn test_12() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), 10);
}

#[test]
fn test_13() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7), Some(1), None, None, None, None, Some(8)])), 4);
}

#[test]
fn test_14() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25)])), 25);
}

#[test]
fn test_15() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9), Some(10), Some(11)])), 11);
}

#[test]
fn test_16() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(7), Some(6), Some(8), Some(5), None, Some(9), Some(10), Some(4), None, None, None, Some(1), Some(11), None, Some(12)])), 6);
}

#[test]
fn test_17() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5), Some(2), None, None, Some(3), None, None, Some(1), Some(6)])), 5);
}

#[test]
fn test_18() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(2), Some(2), Some(3), None, None, Some(4), None, Some(5), Some(6)])), 8);
}

#[test]
fn test_19() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(8), Some(1), None, None, None, Some(7), Some(9)])), 4);
}

#[test]
fn test_20() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-10), Some(-20), Some(-30), Some(-40), Some(-50), None, Some(-60)])), 1);
}

#[test]
fn test_21() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(9), Some(8), Some(7), Some(6), Some(5), Some(4), Some(3), Some(2), Some(1)])), 1);
}

#[test]
fn test_22() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10)])), 10);
}

#[test]
fn test_23() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(15), Some(14), Some(13), Some(12), Some(11), Some(10), Some(9), Some(8), Some(7), Some(6), Some(5), Some(4), Some(3), Some(2), Some(1)])), 1);
}

#[test]
fn test_24() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9), Some(4), None, Some(6), None, Some(7), Some(8), Some(10), Some(11)])), 9);
}

#[test]
fn test_25() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])), 3);
}

#[test]
fn test_26() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7), Some(-2), None, None, None, None, None, None, Some(-3)])), 3);
}

#[test]
fn test_27() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(3), Some(7), Some(13), Some(18), Some(23), Some(27), Some(33), Some(37)])), 4);
}

#[test]
fn test_28() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5)])), 52);
}

#[test]
fn test_29() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-5), Some(-4), Some(-3), Some(-6), Some(-2), Some(-4), Some(-2), Some(-8), Some(-1), Some(-3), Some(-1), Some(-5), Some(-1), Some(-7), Some(-1)])), 9);
}

#[test]
fn test_30() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(100), Some(50), Some(200), Some(25), Some(75), Some(150), Some(250), Some(10), Some(30), Some(60), Some(80), Some(125), Some(175), Some(225), Some(275)])), 4);
}

#[test]
fn test_31() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), Some(7), Some(8), Some(1), Some(9), None, None, None, None, None, None, None, None])), 5);
}

#[test]
fn test_32() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(50), Some(25), Some(75), Some(10), Some(30), Some(60), Some(80), None, Some(20), Some(28), Some(35), None, Some(55), Some(70), None, Some(15), None, Some(22), Some(26), Some(29), Some(33), Some(38), Some(52), Some(57), Some(72), None, Some(13), Some(17), None, None, Some(21), None, Some(24), Some(27), None, None, Some(31), Some(36), None, None, Some(51), Some(54), Some(56), Some(58), Some(71), None, None, None, Some(12), None, None, None, Some(16), None, Some(19), None, None, Some(23), None, None, Some(32), None, Some(34), Some(37), None, Some(50), None, Some(53), None, Some(59), None, Some(69), Some(73), None, None, Some(11), None, Some(14), None, Some(18), None, Some(21), None, None, Some(25), None, Some(30), None, None, None, None, None, Some(49), None, Some(52), None, Some(68), None, Some(72), None, Some(10), None, Some(13), None, None, None, Some(17), None, Some(19), None, None, Some(25), None, Some(29), None, Some(33), None, None, Some(48), None, None, Some(51), None, Some(67), None, Some(71), None, None, None, None, Some(9), None, Some(12), None, None, None, Some(16), None, Some(18), None, None, Some(24), None, None, Some(28), None, None, Some(32), None, Some(34), None, None, Some(47), None, None, Some(50), None, Some(66), None, Some(70), None, None, None, None, Some(8), None, Some(11), None, None, None, Some(15), None, None, None, Some(17), None, Some(19), None, None, Some(23), None, None, Some(27), None, None, Some(31), None, Some(34), None, None, Some(46), None, None, Some(49), None, Some(65), None, Some(69), None, None, None, None, Some(7), None, Some(10), None, None, None, Some(14), None, None, None, Some(17), None, Some(19), None, None, Some(23), None, None, Some(27), None, None, Some(31), None, Some(33), None, None, Some(45), None, None, Some(48), None, Some(64), None, Some(68), None, None, None, None, Some(6), None, Some(9), None, None, None, Some(13), None, None, None, Some(16), None, Some(18), None, None, Some(22), None, None, Some(26), None, None, Some(30), None, Some(33), None, None, Some(44), None, None, Some(47), None, Some(63), None, Some(67), None, None, None, None, Some(5), None, Some(8), None, None, None, Some(12), None, None, None, Some(15), None, None, None, Some(17), None, Some(19), None, None, Some(22), None, None, Some(26), None, None, Some(30), None, Some(32), None, None, Some(43), None, None, Some(46), None, Some(62), None, Some(66), None, None, None, None, Some(4), None, Some(7), None, None, None, Some(11), None, None, None, Some(14), None, None, None, Some(16), None, Some(18), None, None, Some(21), None, None, Some(25), None, None, Some(29), None, Some(32), None, None, Some(42), None, None, Some(45), None, Some(61), None, Some(65), None, None, None, None, Some(3), None, Some(6), None, None, None, Some(10), None, None, None, Some(13), None, None, None, Some(15), None, None, None, Some(18), None, Some(20), None, None, Some(24), None, None, Some(28), None, Some(31), None, Some(33), None, None, Some(41), None, None, Some(44), None, Some(60), None, Some(64), None, None, None, None, Some(2), None, Some(5), None, None, None, Some(9), None, None, None, Some(12), None, None, None, Some(14), None, None, None, Some(17), None, Some(19), None, None, Some(23), None, None, Some(27), None, None, Some(31), None, Some(33), None, None, Some(40), None, None, Some(43), None, Some(59), None, Some(63), None, None, None, None, Some(1), None, Some(4), None, None, None, Some(8), None, None, None, Some(11), None, None, None, Some(13), None, None, None, Some(16), None, Some(18), None, None, Some(22), None, None, Some(26), None, None, Some(30), None, Some(32), None, None, Some(39), None, None, Some(42), None, Some(58), None, Some(62), None, None, None, None])), 10);
}

#[test]
fn test_33() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), Some(4), Some(6), Some(8), Some(12), Some(14), Some(17), Some(20)])), 4);
}

#[test]
fn test_34() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(7), Some(13)])), 3);
}

#[test]
fn test_35() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(8), Some(5), Some(9), Some(7), Some(6), Some(12), None, None, None, None, Some(15), None, Some(18)])), 5);
}

#[test]
fn test_36() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(7), Some(12), Some(16)])), 4);
}

#[test]
fn test_37() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), None, Some(2), None, None, Some(3), None, None, None, None, Some(4), None, None, None, None, Some(5)])), 2);
}

#[test]
fn test_38() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6)])), 3);
}

#[test]
fn test_39() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(100), Some(-20), Some(-30), Some(-40), Some(-50), Some(-60), Some(-70), Some(-80), Some(-90), Some(-100), Some(101), Some(102), Some(103), Some(104), Some(105)])), 6);
}

#[test]
fn test_40() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), 9);
}

#[test]
fn test_41() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(100), Some(-50), Some(50), Some(-100), Some(0), Some(25), Some(75), Some(-200), Some(-100), Some(-75), Some(-25), Some(0), Some(100), Some(150), Some(200)])), 4);
}

#[test]
fn test_42() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(20), Some(10), Some(20), Some(5), Some(15), Some(15), Some(25), Some(3), Some(7), Some(12), Some(18), Some(22), Some(30)])), 5);
}

#[test]
fn test_43() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(1), Some(6), Some(0), Some(2), Some(5), Some(8), Some(-1), None, None, Some(3), None, Some(4), Some(7), Some(9)])), 4);
}

#[test]
fn test_44() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9), Some(2), None, Some(8), None, None, None, None, Some(4)])), 7);
}

#[test]
fn test_45() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1)])), 4);
}

#[test]
fn test_46() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, Some(6), Some(7), None, None, Some(8), Some(9), None, None, Some(10), Some(11), None, None, None, None, Some(12), Some(13), None, None, None, None, None, Some(14), Some(15)])), 13);
}

#[test]
fn test_47() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180)])), 4);
}

#[test]
fn test_48() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5), Some(2), None, None, None, None, None, Some(6), None])), 5);
}

#[test]
fn test_49() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), Some(7), Some(8), None, Some(9), None, Some(10), Some(11), Some(12)])), 12);
}

#[test]
fn test_50() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-10), None, Some(-5), Some(-6), None, Some(-8), None, Some(-7)])), 2);
}

#[test]
fn test_51() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(15), Some(10), Some(20), Some(8), Some(12), Some(16), Some(25), None, None, None, None, None, Some(19), Some(22), Some(30)])), 4);
}

#[test]
fn test_52() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(3), Some(2), Some(4), None, None, Some(3), Some(5), None, None, Some(2), None, None, None, Some(6)])), 3);
}

#[test]
fn test_53() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(20), Some(10), Some(25), Some(5), Some(15), Some(22), Some(30), None, None, None, None, Some(21), Some(27), Some(28), Some(35)])), 5);
}

#[test]
fn test_54() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(2), Some(8)])), 3);
}

#[test]
fn test_55() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7), None, None, None, Some(12)])), 3);
}

#[test]
fn test_56() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), 9);
}

#[test]
fn test_57() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), None, Some(5), None, Some(4), None, None, None, Some(6)])), 6);
}

#[test]
fn test_58() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(7), Some(3), Some(15), Some(1), Some(5), Some(10), Some(20), Some(-1), Some(2), None, Some(6), Some(8), Some(12), Some(18), Some(25)])), 4);
}

#[test]
fn test_59() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(4), Some(6), Some(3), Some(7), None, None, Some(2), None, None, None, None, None, Some(8)])), 3);
}

#[test]
fn test_60() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(10), Some(10), Some(10), Some(10), None, None, Some(10), Some(10)])), 7);
}

#[test]
fn test_61() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(9), Some(8), Some(7), Some(6), Some(5), Some(4), Some(3), Some(2), Some(1), None, None, None, None, None, None, None, None, None])), 1);
}

#[test]
fn test_62() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180)])), 4);
}

#[test]
fn test_63() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7), Some(1), None, Some(3), Some(4)])), 3);
}

#[test]
fn test_64() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(-10), Some(-5), Some(6), Some(5), None, Some(-8), Some(3), Some(12), Some(-9), None, None, Some(-3), None, None, None, None])), 4);
}

#[test]
fn test_65() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-1), Some(-2), Some(-3), Some(-4), Some(-5), None, None, Some(-6), Some(-7)])), 1);
}

#[test]
fn test_66() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(1), Some(4), Some(2), Some(3), None, None, None, None, Some(4), Some(5)])), 2);
}

#[test]
fn test_67() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), 20);
}

#[test]
fn test_68() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-10), Some(-20), Some(-30), Some(-40), Some(-50), Some(-60), Some(-70), Some(-80), Some(-90), Some(-100)])), 1);
}

#[test]
fn test_69() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(4), Some(6), Some(3), None, None, Some(7), Some(2), None, None, None, Some(8)])), 4);
}

#[test]
fn test_70() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6), None, Some(14), None, None, None, Some(8)])), 3);
}

#[test]
fn test_71() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-10), Some(-5), Some(20), None, Some(0), Some(15), Some(25), Some(-8), None, None, Some(5), None, None, None, None])), 5);
}

#[test]
fn test_72() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), Some(3), Some(3), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5), Some(5)])), 30);
}

#[test]
fn test_73() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), 15);
}

#[test]
fn test_74() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(5), Some(4), Some(6), Some(3), Some(8), Some(5), Some(7), Some(2), Some(9), Some(10), None, None, Some(11), Some(12), Some(13)])), 9);
}

#[test]
fn test_75() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(40), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180)])), 4);
}

#[test]
fn test_76() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9), Some(2), Some(8), Some(2), None, None, None, None, Some(4), Some(4), None, None, Some(5)])), 8);
}

#[test]
fn test_77() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(7), Some(7), Some(7), Some(7), Some(7), Some(7), Some(7), Some(7), Some(7), Some(7), Some(7), Some(7), Some(7), Some(7), Some(7)])), 15);
}

#[test]
fn test_78() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), Some(3), Some(3), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4), Some(4)])), 15);
}

#[test]
fn test_79() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(8), Some(3), Some(10), Some(1), Some(6), Some(9), Some(14), None, Some(2), Some(4), Some(7), None, Some(11), Some(13), Some(15)])), 5);
}

#[test]
fn test_80() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11)])), 11);
}

#[test]
fn test_81() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), Some(6), Some(8), Some(11), Some(13), Some(17), Some(19), Some(20)])), 7);
}

#[test]
fn test_82() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(20), Some(1), Some(4), Some(6), Some(8), Some(11), Some(13), Some(18), Some(25)])), 4);
}

#[test]
fn test_83() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), 15);
}

#[test]
fn test_84() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(-1), Some(-2), Some(-3), Some(-4), Some(-5), Some(-6), Some(-7), Some(-8), Some(-9)])), 1);
}

#[test]
fn test_85() {
    assert_eq!(Solution::good_nodes(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11)])), 11);
}

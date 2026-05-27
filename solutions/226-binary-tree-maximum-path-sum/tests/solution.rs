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
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), 15);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(2), Some(-1)])), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-3)])), -3);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(2), Some(3)])), 6);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(-2), Some(-3)])), 1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-2), Some(1)])), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)])), 42);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(2)])), 3);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), None, Some(2)])), 3);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(0)])), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1)])), -1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1)])), 48);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1)])), 1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(-2), Some(-3), Some(1), Some(3), Some(-2), None, Some(-1)])), 3);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1)])), 48);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7), Some(-1), Some(-2), Some(1), Some(1), Some(-3), Some(-4), Some(-5), Some(-6)])), 43);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), 55);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(20), Some(30), Some(40), Some(50), Some(60), Some(70), Some(80), Some(90), Some(100), Some(110), Some(120), Some(130), Some(140), Some(150)])), 440);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), Some(2), Some(-3), Some(4), Some(-5), Some(6), Some(-7), Some(8), Some(-9), None, None, None, None, None, Some(-10)])), 16);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(50), Some(20), Some(30), Some(15), Some(25), Some(10), Some(35), Some(-10), Some(-5), None, None, None, None, None, None])), 160);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8)])), 36);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-10), Some(5), Some(15), Some(-5), Some(3), None, Some(7), Some(-1), None, Some(-2), Some(4), None, None, Some(8)])), 24);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(9), Some(6), Some(-3), None, None, Some(-6), Some(2), None, None, Some(2), Some(2), None, Some(-1), Some(-6), Some(-6), Some(-6)])), 16);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(50), Some(20), Some(30), Some(10), Some(40), Some(25), Some(35), Some(5), Some(15), Some(32), Some(45), Some(22), Some(28), Some(33), Some(42), Some(1), Some(9), Some(11), Some(19), Some(31), Some(37), Some(43), Some(48), Some(0), Some(4), Some(8), Some(14), Some(16), Some(18), Some(21), Some(24), Some(27), Some(29), Some(34), Some(36), Some(39), Some(41), Some(44), Some(46), Some(47), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None])), 351);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), None, Some(-2), None, Some(-3), None, Some(-4), None, Some(-5), None, Some(-6), None, Some(-7), None, Some(-8), None, Some(-9)])), 1);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), None, Some(-2), None, Some(-3), None, Some(-4), Some(-5), None])), -1);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(2), Some(-5), Some(7), Some(-4), None, None, Some(6), Some(-3), Some(-1), Some(8), Some(9), None, None, None, None, None, Some(10)])), 33);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(9), Some(-10), None, None, Some(15), Some(7), None, None, None, Some(1), None, None, Some(-1), None, Some(-5)])), 24);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(2), Some(-1), Some(-2), Some(1), Some(-3), Some(-4), Some(-5), Some(-6), Some(-7)])), 2);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1), None, None, Some(-2), Some(9), Some(3), Some(1), None, None, None, None, None, None, None, Some(-10)])), 52);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), 27);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(100), Some(-50), Some(-50), Some(40), Some(20), None, Some(-60), Some(30), None, Some(10), None, Some(-5)])), 120);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1), None, Some(-5), Some(6), Some(8), Some(-4), Some(-6), None, Some(7), Some(9)])), 58);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), Some(2), Some(2), None, Some(3), None, Some(3), None, Some(4), None, Some(4)])), 17);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(0), Some(-10), Some(20), None, None, Some(15), Some(6), None, None, Some(-5), Some(12), Some(11), Some(13), Some(-14)])), 53);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-10), Some(-20), Some(-30), Some(-40), None, Some(-50), Some(-60), None, None, Some(-70), Some(-80), Some(-90), None, None, Some(-100)])), -10);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(100), Some(50), Some(200), Some(25), Some(75), Some(150), Some(300), Some(10), Some(40), Some(60), Some(90), Some(125), Some(175), Some(250), Some(350)])), 1175);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), Some(-2), Some(-3), Some(1), Some(2), Some(-1), Some(-1), Some(-2), None, Some(-3), None, Some(-2), Some(-2)])), 2);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(9), Some(20), None, None, Some(15), Some(7), Some(6), Some(12), Some(3), Some(8), Some(99), Some(-100), Some(5), None, Some(4), Some(2), Some(1), None, None, None, None, None, None, None, Some(-99)])), 159);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(100), Some(-50), Some(50), Some(-25), Some(25), Some(-12), Some(12), Some(-6), Some(6), Some(-3), Some(3), Some(-1), Some(1), Some(0), Some(-100), Some(100), None, None, None, None, None, None, None, None, None, None, None, None, None, None])), 181);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1), None, None, None, Some(9)])), 52);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(-1), Some(-2), Some(-3), Some(-4), Some(-5), Some(-6), Some(-7), Some(-8), Some(-9), Some(-10), Some(-11), Some(-12), Some(-13), Some(-14), Some(-15)])), 44);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(-10), Some(20), Some(15), Some(10), None, Some(-25), Some(6), None, None, Some(-3), None, Some(4)])), 32);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), Some(-2), Some(-3), Some(-4), Some(-5)])), -1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1), None, None, Some(-1), None, Some(-5), Some(6), Some(9)])), 50);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(-2), Some(3), Some(4), None, None, Some(-6), None, Some(-7), None, None, Some(8), Some(9)])), 10);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-10), Some(5), Some(20), Some(15), Some(25), None, None, None, Some(-5), Some(30), None])), 75);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(-2), Some(15), Some(20), Some(-5), None, Some(7), None, Some(1), None, Some(3), Some(9)])), 60);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30)])), 101);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(2), Some(-5), Some(-1), None, Some(-9), Some(-4), None, None, Some(-3), None, Some(-8)])), 2);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), 69);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(3), Some(10), Some(-1), None, Some(-15), Some(9), None, None, None, Some(-1)])), 59);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(0), Some(-10), Some(20), Some(15), Some(7), Some(-2), None, None, None, None, None, Some(5), Some(1), None, None, None, Some(-1)])), 28);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(2), Some(10), Some(8), Some(34), Some(7), Some(6), Some(1), None, None, None, None, None, Some(9), Some(8), Some(7)])), 71);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1), None, Some(5)])), 53);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1), Some(-14), Some(16), Some(18), None, None, None, None, None, None, None, Some(-19)])), 64);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), 44);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(0), Some(2), Some(5), None, None, Some(6), Some(3), None, None, Some(9), None, None, Some(12), Some(15), Some(7)])), 50);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(0), Some(-10), Some(7), Some(8), None, Some(15), None, Some(-8), None, Some(5), None, Some(9), Some(6), Some(11), None, None, None, None, None, None, None, None, None, None, None])), 38);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1), Some(-1)])), 1);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), Some(4), Some(6), Some(8), Some(12), Some(14), Some(17), Some(19), None, Some(2), None, Some(0), None, None, None, None, None, None, None, None, None, None, None, None, None, None])), 82);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), None, Some(9), Some(-6), Some(3), None, None, None, Some(-2)])), 12);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(-16), Some(-17), Some(-18), Some(-19), Some(-20), Some(-21), Some(-22), Some(-23), Some(-24), Some(-25), Some(-26), Some(-27), Some(-28), Some(-29), Some(-30)])), 44);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(-2), Some(3), Some(-4), Some(5), Some(-6), Some(7), Some(-8), Some(9), Some(-10), Some(11), Some(-12), Some(13), Some(-14), Some(15), Some(-16), Some(17), Some(-18), Some(19), Some(-20)])), 48);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7), None, None, None, None, Some(2), None, Some(1), None, Some(-1), None, None, None, Some(-2)])), 42);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)])), 0);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), Some(-2), Some(-3), Some(-4), Some(-5), Some(-6), Some(-7), Some(-8), Some(-9), Some(-10), Some(-11), Some(-12), Some(-13), Some(-14), Some(-15), Some(-16), Some(-17), Some(-18), Some(-19), Some(-20), Some(-21), Some(-22), Some(-23), Some(-24), Some(-25), Some(-26), Some(-27), Some(-28), Some(-29), Some(-30), Some(-31), Some(-32)])), -1);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1), None, None, Some(-1), None, Some(-1), None, Some(-2)])), 48);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(100), Some(-50), Some(200), Some(-75), Some(25), Some(150), Some(250), Some(-100), None, Some(20), None, Some(-50), None, Some(30), Some(40)])), 640);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), None, Some(-2), None, Some(-3), None, Some(-4), None, Some(-5)])), -1);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(9), Some(4), Some(0), Some(-1), Some(-3), None, Some(-10), None, None, Some(-5)])), 13);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(0), Some(-10), Some(5), None, None, Some(-5), Some(6)])), 11);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(-10), Some(20), Some(-20), Some(30), Some(-30), Some(40), None, None, None, Some(-40), None, None, Some(-50), Some(-60), None, None, Some(-70), Some(-80)])), 90);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), Some(-2), Some(-3), Some(-4), Some(-5), Some(-6), Some(-7), Some(-8), Some(-9), Some(-10), Some(-11), Some(-12), Some(-13), Some(-14), Some(-15)])), -1);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13), None, Some(14), None, Some(15), None, Some(16), None, Some(17), None, Some(18), None, Some(19), None, Some(20), None, Some(21), None, Some(22), None, Some(23), None, Some(24), None, Some(25), None, Some(26), None, Some(27), None, Some(28), None, Some(29), None, Some(30), None, Some(31), None, Some(32), None, Some(33), None, Some(34), None, Some(35), None, Some(36), None, Some(37), None, Some(38), None, Some(39), None, Some(40)])), 820);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-1), None, Some(-2), None, Some(-3), None, Some(-4), None, Some(-5), None, Some(-6), None, Some(-7)])), -1);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(9), Some(20), None, None, Some(15), Some(7), Some(12), None, Some(2), Some(1), None, None, Some(-5)])), 66);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(-4), Some(-8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1), None, None, Some(-1), None, Some(-5), Some(6), Some(9), Some(-3)])), 26);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(2), Some(-1), Some(-2), Some(1), Some(-4), Some(-5), Some(-6)])), 2);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), Some(32)])), 119);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(2), Some(-5), Some(6), None, Some(5), None, Some(7)])), 15);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(100), Some(50), Some(50), Some(25), Some(25), Some(25), Some(25), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19)])), 284);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(3), Some(-2), Some(5), Some(-6), None, None, Some(4), None, None, Some(-8), None, None, Some(7)])), 12);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(50), Some(20), Some(30), Some(10), Some(40), Some(25), Some(35), Some(5), Some(15), Some(32), Some(45), Some(23), Some(27), Some(33), Some(47), Some(1), Some(9), Some(11), Some(19), Some(31), Some(34), Some(39), Some(44), Some(46), Some(48)])), 325);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), 45);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1), None, None, None, Some(-1), None, Some(-9)])), 48);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(9), Some(-10), None, None, Some(15), Some(20), Some(1), None, Some(-1), None, Some(-5), None, Some(4), None, Some(-6)])), 32);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(5), Some(-3), Some(3), Some(2), None, Some(11), Some(3), Some(-2), None, Some(1)])), 29);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(-2), Some(3), Some(-4), None, Some(5), Some(-6), Some(7), None, None, Some(8), None, Some(9)])), 19);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(-2), Some(3), Some(4), None, Some(5), Some(-6), None, None, Some(7), Some(-8), Some(9), Some(-10), None, None, None, None, Some(11)])), 29);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(-10), Some(20), Some(15), Some(7), None, Some(-5), Some(20), Some(30)])), 65);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(1), Some(-2), Some(-3), Some(4), Some(5), None, Some(6), None, None, Some(-8), Some(-9)])), 7);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(-10), Some(5), Some(9), None, Some(-3), None, Some(-8), None, Some(-2), None, Some(7), Some(-5), None, Some(-1), None, Some(6), None, None, None, None, None, None, Some(8)])), 9);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(9), Some(6), Some(-3), None, None, Some(-6), Some(2), None, None, Some(2), None, Some(-6), Some(-6), Some(-6)])), 16);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(100), Some(-50), Some(-50), None, Some(-100), Some(-100), None, Some(-50), Some(-100), None, Some(-150), Some(-150)])), 100);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_path_sum(build_tree(&[Some(10), Some(9), Some(20), None, None, Some(15), Some(7), None, None, Some(30), Some(40), None, Some(50)])), 127);
}

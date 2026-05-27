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
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), None, Some(1)])), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(4), Some(6), None, None, Some(3), Some(7)])), false);
}

#[test]
fn test_3() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(0), Some(-1)])), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)])), false);
}

#[test]
fn test_5() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(3), Some(1), Some(5), Some(0), Some(2), Some(4), Some(6)])), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(3), Some(1), Some(4), Some(3), Some(2), Some(6), Some(5)])), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), Some(2), Some(2)])), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), None, Some(1)])), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), Some(2)])), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2147483647)])), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), None, Some(2)])), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), Some(1), Some(3)])), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(32), Some(26), Some(47), Some(19), None, None, Some(56), None, Some(27)])), false);
}

#[test]
fn test_14() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), None, Some(2), Some(3)])), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7)])), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1)])), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(6), Some(20)])), false);
}

#[test]
fn test_18() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(4), Some(6), Some(3), None, None, Some(7)])), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(3), Some(2), Some(5), Some(1), None, None, Some(6), Some(0), None, None, Some(4)])), false);
}

// test_20 deleted: original used Some(1.5)/Some(2.5) which cannot be represented
// as i32 while preserving BST semantics (a value strictly between 1 and 2).

#[test]
fn test_21() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(3), Some(7), Some(1), Some(4), Some(6), Some(9), None, None, None, None, None, Some(10)])), false);
}

#[test]
fn test_22() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(8), Some(5), Some(10), None, Some(6), Some(8), Some(15), None, None, Some(7)])), false);
}

#[test]
fn test_23() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(9), Some(7), Some(10), None, None, Some(8), None, Some(6)])), false);
}

#[test]
fn test_24() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(1), Some(5), None, None, Some(4), Some(6)])), false);
}

#[test]
fn test_25() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(190)])), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), None, None, None, None, None, None, Some(9)])), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)])), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(3), None, Some(30), Some(10), None, None, Some(15), None, Some(45)])), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(10), None, Some(20), Some(15), Some(30), None, None, Some(12), Some(17)])), false);
}

#[test]
fn test_31() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), Some(1), Some(3), None, None, Some(2)])), false);
}

#[test]
fn test_32() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(10), Some(5), Some(15), Some(1), Some(7), None, Some(20), None, None, Some(6), Some(8)])), true);
}

// test_33 deleted: original used Some(1.5) which cannot be represented
// as i32 while preserving BST semantics (a value strictly between 1 and 2).

#[test]
fn test_34() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(1), Some(5), None, None, Some(5), Some(5)])), false);
}

#[test]
fn test_35() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), None, Some(8)])), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(50), Some(20), Some(60), Some(10), Some(25), Some(55), Some(70), Some(5), Some(15), Some(22), Some(30), Some(52), Some(65), Some(67), Some(80)])), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)])), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(8), Some(5), Some(15), Some(1), Some(7), None, Some(18), None, Some(6)])), false);
}

#[test]
fn test_39() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(3), Some(1), Some(5), Some(0), Some(2), Some(4), Some(6), None, None, None, Some(3)])), false);
}

#[test]
fn test_40() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(8), Some(5), Some(15), Some(1), Some(7), Some(10), Some(18), None, None, None, Some(8)])), false);
}

#[test]
fn test_41() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(32), Some(26), Some(40), Some(17), Some(27), None, Some(45), None, None, None, Some(29), Some(30)])), false);
}

#[test]
fn test_42() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(12), Some(7), Some(14), Some(3), Some(9), Some(13), Some(15), None, None, Some(8)])), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(50), Some(30), Some(80), Some(20), Some(40), Some(70), Some(90), None, None, Some(25), Some(45), Some(65), Some(75), None, None, None, None, Some(42)])), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(9), Some(4), Some(10), None, None, None, Some(11)])), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(1), Some(6), None, None, Some(3), Some(7), None, None, None, Some(8)])), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2147483646), Some(2147483647)])), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), Some(2), Some(2), Some(2), Some(2), Some(2), Some(2)])), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20)])), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(100), Some(50), Some(150), None, None, Some(60), Some(200)])), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(12), Some(7), Some(14), Some(3), Some(9), Some(13), Some(20), Some(1), Some(5), Some(8), Some(11), Some(12), Some(17), Some(16), Some(18)])), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), Some(1), Some(3), Some(0), Some(5), None, Some(4)])), false);
}

#[test]
fn test_52() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(4), Some(6), None, None, Some(3), Some(7), Some(2), None, None, None, Some(4), None])), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(2), None, None, None, Some(16)])), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(7), Some(3), Some(15), None, None, None, Some(20)])), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), Some(0), Some(2), None, None, Some(3), None])), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(32), Some(26), Some(40), Some(18), Some(28), Some(36), Some(48), Some(13), Some(19), Some(27), Some(31), Some(34), Some(43), Some(49), Some(52), None, Some(14), None, None, Some(24), Some(29), None, None, Some(32), None, Some(35), Some(41), Some(44), Some(46), Some(47), Some(50), None, None, Some(51)])), false);
}

#[test]
fn test_57() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), None, Some(3), None, None, Some(4)])), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(2), Some(14), None, None, Some(10), Some(20), Some(8), Some(15), Some(11), Some(19)])), false);
}

#[test]
fn test_59() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(10), Some(5), Some(15), Some(1), Some(8), Some(12), Some(20), None, None, Some(6), Some(9), Some(11), Some(13)])), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(12), Some(7), Some(14), Some(3), Some(9), Some(13), Some(18), Some(1), None, Some(8), None, Some(11), Some(15), None, Some(17)])), false);
}

#[test]
fn test_61() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), Some(1), Some(3), None, None, Some(2), Some(4)])), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(120), Some(70), Some(140), Some(50), Some(100), Some(130), Some(160), Some(20), Some(60), Some(80), Some(110), Some(125), Some(150), Some(180), Some(200)])), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), Some(1), None, Some(1)])), false);
}

#[test]
fn test_65() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5)])), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), Some(1)])), false);
}

#[test]
fn test_67() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), Some(0), Some(2)])), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6), Some(2), Some(2)])), false);
}

#[test]
fn test_69() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4)])), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)])), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(32), Some(26), Some(40), Some(19), Some(27), None, Some(44), None, None, None, None, None, Some(42)])), false);
}

#[test]
fn test_72() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(50), Some(30), Some(80), Some(20), Some(40), Some(70), Some(90)])), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), None, Some(2), None, None, None, Some(3), None, None, None, Some(4)])), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1)])), false);
}

#[test]
fn test_75() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(1), Some(4), None, None, None, Some(6)])), false);
}

#[test]
fn test_76() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(3), Some(2), Some(4), None, None, Some(3), None])), false);
}

#[test]
fn test_77() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(12), Some(5), Some(18), Some(2), Some(9), Some(15), Some(19), Some(1), Some(3), Some(7), Some(11), Some(14), Some(17), Some(20)])), false);
}

#[test]
fn test_78() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(1), Some(8), None, Some(4), Some(7), Some(9)])), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1), Some(1)])), false);
}

#[test]
fn test_80() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(8), Some(5), Some(10), None, Some(6), Some(9), Some(11)])), true);
}

#[test]
fn test_81() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1), None, None, None, None, None, Some(9)])), false);
}

#[test]
fn test_82() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(1), Some(5), None, Some(7)])), false);
}

#[test]
fn test_83() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2), Some(1), Some(3), Some(0), Some(2), None, Some(4)])), false);
}

#[test]
fn test_84() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(3), Some(1), Some(4), Some(3), Some(3)])), false);
}

#[test]
fn test_85() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(2), Some(8), Some(1), Some(3), None, Some(9)])), true);
}

#[test]
fn test_86() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), None, None, None, None, Some(13), Some(18)])), true);
}

#[test]
fn test_87() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(12), Some(20)])), true);
}

#[test]
fn test_88() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(3), Some(1), Some(4), None, None, Some(2), Some(6)])), false);
}

#[test]
fn test_89() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(2147483647), Some(2147483647)])), false);
}

#[test]
fn test_90() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180)])), true);
}

#[test]
fn test_91() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), None, None, None, None, None, None, None, Some(9)])), true);
}

#[test]
fn test_92() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(90)])), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)])), false);
}

#[test]
fn test_94() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(50), Some(30), Some(80), Some(20), Some(40), Some(70), Some(90), None, None, Some(25), Some(45), Some(65), Some(75), None, None, None, None, Some(41)])), false);
}

#[test]
fn test_95() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(1), None, Some(3), None, Some(2)])), false);
}

#[test]
fn test_96() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(5), Some(4), Some(6), Some(3), None, None, Some(7), Some(2), None, None, None, None, Some(8)])), false);
}

#[test]
fn test_97() {
    assert_eq!(Solution::is_valid_bst(build_tree(&[Some(10), Some(5), Some(15), Some(1), Some(7), None, Some(18)])), true);
}

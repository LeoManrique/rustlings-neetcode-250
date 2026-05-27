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
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)])), 23);
}

#[test]
fn test_2() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(2), None, Some(1), Some(3), None, Some(1)])), 7);
}

#[test]
fn test_3() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)])), 9);
}

#[test]
fn test_4() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(0), Some(0)])), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)])), 38);
}

#[test]
fn test_6() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(4), Some(5), Some(1), Some(3), None, Some(1)])), 9);
}

#[test]
fn test_7() {
    assert_eq!(Solution::rob(build_tree(&[Some(2), Some(1), Some(3), None, Some(4), None, Some(7)])), 13);
}

#[test]
fn test_8() {
    assert_eq!(Solution::rob(build_tree(&[Some(2), Some(1), Some(3), None, Some(4)])), 7);
}

#[test]
fn test_9() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)])), 14);
}

#[test]
fn test_10() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(2), Some(3), None, Some(3), None, Some(1)])), 7);
}

#[test]
fn test_11() {
    assert_eq!(Solution::rob(build_tree(&[Some(0)])), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), None, Some(4), None, Some(5)])), 8);
}

#[test]
fn test_13() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(9), Some(20), None, None, Some(15), Some(7), None, Some(3), Some(18), None, Some(6), Some(2), Some(0), Some(1), Some(5), None, None, None, None, Some(4)])), 59);
}

#[test]
fn test_14() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), 30);
}

#[test]
fn test_15() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(20), Some(1), None, Some(6), Some(8), None, None, None, Some(9)])), 55);
}

#[test]
fn test_16() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6), Some(8), Some(12), Some(14), Some(17), Some(19)])), 97);
}

#[test]
fn test_17() {
    assert_eq!(Solution::rob(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(13), Some(17), Some(23), Some(27), Some(33), Some(37), Some(1), Some(3), Some(6), Some(8), Some(12), Some(14), Some(16), Some(18), Some(22), Some(24), Some(26), Some(28), Some(32), Some(34), Some(36), Some(38)])), 418);
}

#[test]
fn test_18() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), Some(4), None, None, Some(5), None, None, Some(6), None, None, Some(7)])), 17);
}

#[test]
fn test_19() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4), Some(5), Some(5), None, None, Some(6), Some(6), Some(7), Some(7), Some(8), Some(8), Some(9), Some(9)])), 63);
}

#[test]
fn test_20() {
    assert_eq!(Solution::rob(build_tree(&[Some(2), Some(1), Some(3), None, Some(4), None, Some(5), None, Some(6)])), 13);
}

#[test]
fn test_21() {
    assert_eq!(Solution::rob(build_tree(&[Some(5), Some(2), Some(13), None, Some(5), None, Some(14), Some(20), None, None, Some(9), Some(10), Some(3)])), 44);
}

#[test]
fn test_22() {
    assert_eq!(Solution::rob(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(190)])), 980);
}

#[test]
fn test_23() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(20), Some(1), None, Some(6), None, Some(11), Some(13), None, None, Some(18), Some(22)])), 104);
}

#[test]
fn test_24() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(2), Some(3), Some(3), Some(3), None, Some(2), None, Some(1)])), 11);
}

#[test]
fn test_25() {
    assert_eq!(Solution::rob(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15)])), 80);
}

#[test]
fn test_26() {
    assert_eq!(Solution::rob(build_tree(&[Some(3000), Some(1000), Some(2000), Some(500), Some(700), None, Some(3000), None, Some(600), None, Some(800), None, Some(900)])), 7400);
}

#[test]
fn test_27() {
    assert_eq!(Solution::rob(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1), Some(9), None, None, Some(11), Some(13), None, None, Some(15), Some(17)])), 82);
}

#[test]
fn test_28() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(2), Some(3), Some(2), None, Some(1), Some(1), Some(1), Some(1), Some(2), Some(2), Some(2), Some(2), Some(1), Some(1)])), 16);
}

#[test]
fn test_29() {
    assert_eq!(Solution::rob(build_tree(&[Some(100), Some(90), Some(80), Some(70), Some(60), Some(50), Some(40), Some(30), Some(20), Some(10)])), 320);
}

#[test]
fn test_30() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)])), 12);
}

#[test]
fn test_31() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)])), 18);
}

#[test]
fn test_32() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), Some(8), Some(13), None, None, None, None, Some(19)])), 61);
}

#[test]
fn test_33() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(20), Some(1), None, Some(6), Some(8), None, None, None, None, Some(18)])), 58);
}

#[test]
fn test_34() {
    assert_eq!(Solution::rob(build_tree(&[Some(5), Some(1), Some(5), None, Some(7), Some(6), None, Some(4), None, Some(3), None, Some(2), Some(8), None, None, None, Some(9)])), 35);
}

#[test]
fn test_35() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), Some(12), Some(18), Some(13), Some(17), Some(16), Some(19), Some(14), Some(21), Some(10), Some(11), None, Some(15)])), 137);
}

#[test]
fn test_36() {
    assert_eq!(Solution::rob(build_tree(&[Some(6), Some(7), Some(8), Some(2), Some(7), Some(1), Some(3), Some(9), None, Some(1), Some(4), None, None, None, Some(5)])), 34);
}

#[test]
fn test_37() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), Some(8), None, Some(20), Some(17), Some(21)])), 92);
}

#[test]
fn test_38() {
    assert_eq!(Solution::rob(build_tree(&[Some(6), Some(3), Some(7), Some(2), Some(5), None, Some(8), Some(1), Some(4), None, None, None, None, None, Some(9)])), 32);
}

#[test]
fn test_39() {
    assert_eq!(Solution::rob(build_tree(&[Some(8), Some(4), Some(13), Some(2), Some(6), Some(10), Some(14), Some(1), None, Some(5), Some(7), None, None, Some(9), Some(12)])), 53);
}

#[test]
fn test_40() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(2), Some(3), None, Some(3), None, Some(1), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12)])), 63);
}

#[test]
fn test_41() {
    assert_eq!(Solution::rob(build_tree(&[Some(15), Some(10), Some(20), Some(8), None, None, Some(25), None, None, None, Some(30)])), 60);
}

#[test]
fn test_42() {
    assert_eq!(Solution::rob(build_tree(&[Some(50), Some(20), Some(50), Some(10), Some(30), Some(40), Some(60), Some(5), Some(15), None, None, None, None, None, Some(70)])), 210);
}

#[test]
fn test_43() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30)])), 368);
}

#[test]
fn test_44() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)])), 162);
}

#[test]
fn test_45() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)])), 97);
}

#[test]
fn test_46() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), Some(6), Some(8), Some(9), Some(12), Some(14), Some(17), Some(19)])), 106);
}

#[test]
fn test_47() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), None, Some(1), Some(7), None, Some(12), Some(0), Some(2), Some(6), Some(8), None, None, Some(11), Some(13), None, None, Some(3), Some(4), None, None, None, None, Some(9), Some(10)])), 76);
}

#[test]
fn test_48() {
    assert_eq!(Solution::rob(build_tree(&[Some(5), Some(4), Some(7), Some(3), None, Some(2), Some(9), None, None, None, Some(10)])), 27);
}

#[test]
fn test_49() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25)])), 259);
}

#[test]
fn test_50() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(2), Some(3), None, Some(3), None, Some(1), None, None, Some(5), Some(6)])), 17);
}

#[test]
fn test_51() {
    assert_eq!(Solution::rob(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180), Some(5), Some(15), Some(20), Some(35), Some(65), Some(70), Some(85), Some(105), Some(135), Some(145), Some(155), Some(175), Some(185), Some(190)])), 1890);
}

#[test]
fn test_52() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), Some(6), Some(8), Some(12), Some(14), Some(17), Some(9), Some(11), Some(2), Some(4), Some(16), Some(19), Some(20)])), 144);
}

#[test]
fn test_53() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7), None, None, Some(8), Some(12), None, None, Some(11), None, Some(10)])), 59);
}

#[test]
fn test_54() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(2), Some(3), None, Some(3), None, Some(1), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10)])), 45);
}

#[test]
fn test_55() {
    assert_eq!(Solution::rob(build_tree(&[Some(9), Some(4), Some(17), Some(3), Some(6), Some(11), Some(20), Some(1), Some(5), Some(8), Some(13), None, Some(22), None, Some(2), None, None, Some(0), Some(2), None, Some(9), None, None, Some(7), Some(12), None, None, None, Some(2)])), 89);
}

#[test]
fn test_56() {
    assert_eq!(Solution::rob(build_tree(&[Some(2), Some(1), Some(3), None, Some(4), None, Some(5), Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), 32);
}

#[test]
fn test_57() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, Some(6), Some(7), Some(8), Some(9), None, None, None, None, Some(10), Some(11), None, None, None, None, None, Some(12)])), 49);
}

#[test]
fn test_58() {
    assert_eq!(Solution::rob(build_tree(&[Some(50), Some(20), Some(60), Some(10), Some(30), Some(55), Some(70), Some(5), Some(15), Some(25), Some(35), Some(51), Some(59), Some(65), Some(75), Some(1), Some(4), Some(9), Some(16), Some(24), Some(26), Some(34), Some(36), Some(50), Some(58), Some(61), Some(64), Some(66), Some(73), Some(76), Some(78)])), 891);
}

#[test]
fn test_59() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), 16);
}

#[test]
fn test_60() {
    assert_eq!(Solution::rob(build_tree(&[Some(5), Some(4), Some(5), None, Some(1), Some(4), None, None, Some(2), Some(3)])), 14);
}

#[test]
fn test_61() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), Some(13), Some(18), Some(1), None, Some(6), Some(8), Some(9), Some(14), None, None, None, Some(19), None, Some(20)])), 100);
}

#[test]
fn test_62() {
    assert_eq!(Solution::rob(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(5), None, Some(8), Some(1), None, Some(4), None, Some(6), None, Some(3), Some(5)])), 28);
}

#[test]
fn test_63() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(20), Some(1), None, Some(6), None, Some(11), Some(13), None, None, Some(18), Some(22), Some(19), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30)])), 283);
}

#[test]
fn test_64() {
    assert_eq!(Solution::rob(build_tree(&[Some(50), Some(20), Some(60), Some(10), Some(30), Some(55), Some(70), Some(5), Some(15), Some(25), Some(35), Some(52), Some(58), Some(65), Some(75), Some(2), Some(7), Some(12), Some(17), Some(23), Some(27), Some(32), Some(37), Some(51), Some(56), Some(59), Some(64), Some(67), Some(72), Some(76), Some(78)])), 895);
}

#[test]
fn test_65() {
    assert_eq!(Solution::rob(build_tree(&[Some(100), Some(50), Some(50), Some(25), Some(25), Some(25), Some(25), Some(10), Some(10), Some(10), Some(10), Some(10), Some(10), Some(10), Some(10)])), 200);
}

#[test]
fn test_66() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, None, Some(5), None, None, Some(6), None, None, None, Some(7)])), 13);
}

#[test]
fn test_67() {
    assert_eq!(Solution::rob(build_tree(&[Some(2), Some(1), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)])), 17);
}

#[test]
fn test_68() {
    assert_eq!(Solution::rob(build_tree(&[Some(9), Some(4), Some(7), Some(3), Some(6), Some(5), Some(11), Some(2), Some(5), None, None, Some(8), None, Some(10), Some(12), None, None, None, None, None, Some(14)])), 63);
}

#[test]
fn test_69() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(4), Some(5), Some(1), Some(3), None, Some(1), Some(5), Some(6), None, Some(2), Some(8), None, None, None, Some(9), None, None, Some(10)])), 41);
}

#[test]
fn test_70() {
    assert_eq!(Solution::rob(build_tree(&[Some(8), Some(4), Some(13), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(12), Some(15)])), 80);
}

#[test]
fn test_71() {
    assert_eq!(Solution::rob(build_tree(&[Some(7), Some(3), Some(8), Some(1), Some(4), Some(3), Some(9), Some(0), Some(2), Some(5), Some(6), None, None, None, None, None, Some(11)])), 43);
}

#[test]
fn test_72() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), Some(8)])), 45);
}

#[test]
fn test_73() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), Some(13), Some(18), Some(1), None, Some(6), Some(8), Some(9), Some(14), Some(19), Some(20)])), 97);
}

#[test]
fn test_74() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), Some(8), Some(14), Some(19), None, None, Some(13), None, None, None, None, None, None, Some(12)])), 75);
}

#[test]
fn test_75() {
    assert_eq!(Solution::rob(build_tree(&[Some(5), Some(2), Some(8), Some(1), Some(3), None, Some(9), Some(0), None, None, Some(4), None, None, Some(6), None, Some(10)])), 34);
}

#[test]
fn test_76() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), Some(13), Some(18), Some(1), None, Some(6), Some(8), None, Some(14), None, None, Some(19)])), 77);
}

#[test]
fn test_77() {
    assert_eq!(Solution::rob(build_tree(&[Some(50), Some(20), Some(30), Some(10), Some(40), None, None, Some(5), None, None, Some(55)])), 115);
}

#[test]
fn test_78() {
    assert_eq!(Solution::rob(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), Some(2), None, None, None, Some(20)])), 45);
}

#[test]
fn test_79() {
    assert_eq!(Solution::rob(build_tree(&[Some(5), Some(2), Some(6), Some(1), Some(3), None, Some(8), None, None, Some(4), Some(7), None, None, None, Some(9)])), 30);
}

#[test]
fn test_80() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)])), 32);
}

#[test]
fn test_81() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None, None, None, Some(8), Some(9), Some(10), Some(11), Some(12)])), 56);
}

#[test]
fn test_82() {
    assert_eq!(Solution::rob(build_tree(&[Some(7), Some(3), Some(8), Some(1), Some(4), Some(5), Some(9), Some(0), Some(2), Some(6), None, None, None, None, None, None, Some(10)])), 39);
}

#[test]
fn test_83() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10), None, Some(11), None, Some(12), None, Some(13)])), 49);
}

#[test]
fn test_84() {
    assert_eq!(Solution::rob(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)])), 25);
}

#[test]
fn test_85() {
    assert_eq!(Solution::rob(build_tree(&[Some(20), Some(10), Some(10), Some(5), Some(15), None, None, Some(3), Some(7), Some(12), Some(18)])), 60);
}

#[test]
fn test_86() {
    assert_eq!(Solution::rob(build_tree(&[Some(8), Some(4), Some(13), Some(2), Some(6), Some(10), Some(17), Some(1), None, Some(5), Some(7), None, Some(12), None, None, None, Some(14)])), 65);
}

#[test]
fn test_87() {
    assert_eq!(Solution::rob(build_tree(&[Some(7), Some(3), Some(8), Some(1), Some(4), Some(6), Some(9), Some(0), Some(2), Some(5), None, None, None, None, None, None, None, Some(10), Some(11), None, Some(12), None, Some(13), Some(14), Some(15), None, None, None, None, Some(16), Some(17)])), 101);
}

#[test]
fn test_88() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(2), Some(3), None, Some(3), None, Some(1), Some(2), None, Some(4)])), 11);
}

#[test]
fn test_89() {
    assert_eq!(Solution::rob(build_tree(&[Some(50), Some(20), Some(50), None, Some(30), None, Some(60), Some(10), Some(40), None, Some(70)])), 190);
}

#[test]
fn test_90() {
    assert_eq!(Solution::rob(build_tree(&[Some(3), Some(1), Some(5), Some(0), Some(2), Some(4), Some(6)])), 15);
}

#[test]
fn test_91() {
    assert_eq!(Solution::rob(build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1)])), 35);
}

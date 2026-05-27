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
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)]), 3), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(3), Some(1), Some(4), None, Some(2)]), 1), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(3), Some(2), Some(4), Some(1)]), 2), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(1)]), 1), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(5), Some(1), Some(7), None, Some(2), None, Some(8), None, Some(3)]), 4), 5);
}

#[test]
fn test_6() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(3), Some(1), Some(4), None, Some(2), None, None, None, None, None]), 2), 2);
}

#[test]
fn test_7() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]), 4), 10);
}

#[test]
fn test_8() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15)]), 15), 15);
}

#[test]
fn test_9() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), Some(4), Some(6), Some(8), Some(11), Some(13), Some(17), Some(19)]), 5), 6);
}

#[test]
fn test_10() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(50), Some(25), Some(75), Some(10), Some(30), Some(60), Some(80), Some(5), Some(15), Some(27), Some(35), Some(55), Some(65), Some(70), Some(90), Some(1), None, Some(8), Some(12), Some(20), Some(28), Some(32), Some(38), Some(40), Some(52), Some(58), Some(62), Some(68), Some(72), Some(78), Some(85), Some(95)]), 15), 38);
}

#[test]
fn test_11() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(7), Some(13), Some(17), Some(11), Some(15)]), 7), 7);
}

#[test]
fn test_12() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20), Some(8), Some(10), Some(11), Some(19)]), 4), 9);
}

#[test]
fn test_13() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(60), Some(30), Some(90), Some(20), Some(40), Some(70), Some(100), Some(15), Some(25), Some(35), Some(45), Some(65), Some(80), Some(95), Some(110)]), 17), None);
}

#[test]
fn test_14() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(1), Some(7), Some(12), Some(18), Some(23), Some(27), Some(32), Some(37)]), 10), 25);
}

#[test]
fn test_15() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(7), Some(3), Some(9), Some(2), Some(5), Some(8), Some(10), Some(1), Some(4), Some(6), None, None, None, None, None, None, None]), 5), 6);
}

#[test]
fn test_16() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1)]), 8), 8);
}

#[test]
fn test_17() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(13), Some(17), Some(23), Some(27), Some(32), Some(37)]), 10), 25);
}

#[test]
fn test_18() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20), None, None, None, Some(22)]), 5), 20);
}

#[test]
fn test_19() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1)]), 1), 1);
}

#[test]
fn test_20() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15)]), 7), 7);
}

#[test]
fn test_21() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6)]), 4), 6);
}

#[test]
fn test_22() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(18), None, Some(30), Some(5), Some(13), None, None, None, None, Some(17), Some(19)]), 10), 30);
}

#[test]
fn test_23() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180)]), 15), 180);
}

#[test]
fn test_24() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(20), Some(1), None, Some(6), None, None, None, None, None, None, Some(3)]), 6), 7);
}

#[test]
fn test_25() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(18), Some(25), Some(35), Some(42), Some(48), Some(55)]), 18), None);
}

#[test]
fn test_26() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), Some(18), Some(22), Some(28), Some(32), Some(38), Some(42)]), 18), None);
}

#[test]
fn test_27() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(18), Some(22), Some(35), Some(42), Some(48), Some(55)]), 7), 22);
}

#[test]
fn test_28() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), Some(17), Some(22), Some(28), Some(32), Some(38), Some(45), Some(2), Some(7), Some(11), Some(14), Some(18), Some(21), Some(26), Some(29), Some(31), Some(33), Some(36), Some(42), Some(44), Some(47), None, None, None, Some(9), None, Some(13), None, Some(16), None, None, Some(23), None, None, None, None, None, None, None, None, None, None, Some(39), None, None, None, None, None, None, None, None, Some(41), Some(43), None, Some(46), None, None, Some(48), None, None, Some(49)]), 22), 22);
}

#[test]
fn test_29() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), Some(18), Some(22), Some(28), Some(32), Some(38), Some(45)]), 15), 45);
}

#[test]
fn test_30() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), None, None, Some(3), Some(7), Some(12), Some(18)]), 8), 20);
}

#[test]
fn test_31() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(50), Some(20), Some(80), Some(10), Some(30), Some(70), Some(90), Some(5), Some(15), Some(25), Some(35), Some(60), Some(75), Some(85), Some(95)]), 10), 70);
}

#[test]
fn test_32() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(30), Some(10), Some(50), Some(5), Some(20), Some(40), Some(60), Some(1), Some(7), Some(15), Some(25), Some(35), Some(45), Some(55), Some(65)]), 20), None);
}

#[test]
fn test_33() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20)]), 3), 9);
}

#[test]
fn test_34() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(60), Some(40), Some(80), Some(30), Some(50), Some(70), Some(90), Some(20), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85), Some(95)]), 22), None);
}

#[test]
fn test_35() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), Some(4), Some(6), Some(8), None, None, None, None, Some(11), Some(14), Some(16), None, None, Some(13), Some(17), Some(19)]), 7), 5);
}

#[test]
fn test_36() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6)]), 5), 7);
}

#[test]
fn test_37() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(12), Some(20), Some(11), None, None, Some(13), Some(14), Some(16), None, None, None, Some(18), Some(19), Some(21)]), 4), 18);
}

#[test]
fn test_38() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(12), Some(17), Some(22), Some(27), Some(32), Some(37), Some(1), Some(3), Some(6), Some(8), Some(11), Some(13), Some(16), Some(18), Some(21), Some(23), Some(26), Some(28), Some(31), Some(33), Some(36), Some(38), Some(4), Some(9), Some(14), Some(19), Some(24), Some(29), Some(34), Some(39)]), 20), 15);
}

#[test]
fn test_39() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20), Some(8), None, None, None, None, Some(25)]), 6), 15);
}

#[test]
fn test_40() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(50), Some(25), Some(75), Some(15), Some(35), Some(60), Some(85), Some(10), Some(20), Some(30), Some(40), Some(55), Some(65), Some(80), Some(90)]), 12), 75);
}

#[test]
fn test_41() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(15), Some(10), Some(20), Some(5), Some(12), Some(18), Some(25), None, Some(7), Some(11), Some(14), Some(16), Some(19), Some(22), Some(27)]), 8), 16);
}

#[test]
fn test_42() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(40), Some(60), Some(90), Some(110), Some(140), Some(160), Some(190)]), 15), 190);
}

#[test]
fn test_43() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), None, Some(18), Some(28), Some(32), Some(38), Some(45)]), 15), None);
}

#[test]
fn test_44() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(12), Some(18), Some(22), Some(28), Some(32), Some(38), Some(1), None, Some(6), None, Some(11), Some(13), None, Some(17), Some(19), None, Some(21), Some(23), None, Some(27), Some(29), None, Some(31), Some(33), None, Some(37), Some(39)]), 10), 39);
}

#[test]
fn test_45() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(60), Some(30), Some(90), Some(20), Some(40), Some(70), Some(110), Some(10), Some(25), Some(35), Some(50), Some(65), Some(85), Some(100), Some(120)]), 9), 65);
}

#[test]
fn test_46() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(1), None, Some(12), Some(18), Some(22), Some(28), None, None, None, None, Some(14)]), 10), 25);
}

#[test]
fn test_47() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20), Some(8), None, None, None, None, Some(22)]), 12), None);
}

#[test]
fn test_48() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(18), Some(25), Some(35), Some(45), Some(48), Some(55)]), 7), 25);
}

#[test]
fn test_49() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(40), Some(60), Some(85), Some(110), Some(140), Some(160), Some(185), Some(5), Some(15), Some(35), Some(45), Some(55), Some(70), Some(80), Some(90), Some(105), Some(115), Some(130), Some(135), Some(155), Some(170), Some(190), Some(3), Some(7), Some(13), Some(17), Some(20), Some(30), Some(43), Some(47), Some(53), Some(65), Some(72), Some(87), Some(88), Some(95), Some(103), Some(108), Some(112), Some(127), Some(132), Some(138), Some(148), Some(157), Some(165), Some(168), Some(172), Some(182), Some(187), Some(192), Some(197), Some(2), Some(4), Some(6), Some(8), Some(11), Some(12), Some(14), Some(16), Some(19), Some(21), Some(22), Some(23), Some(24), Some(26), Some(27), Some(28), Some(29), Some(31), Some(32), Some(33), Some(34), Some(36), Some(37), Some(38), Some(39), Some(41), Some(42), Some(44), Some(46), Some(48), Some(49), Some(51), Some(52), Some(54), Some(56), Some(57), Some(58), Some(59), Some(61), Some(62), Some(63), Some(64), Some(66), Some(67), Some(68), Some(69), Some(71), Some(73), Some(74), Some(76), Some(77), Some(78), Some(79), Some(81), Some(82), Some(83), Some(84), Some(86), Some(89), Some(91), Some(92), Some(93), Some(94), Some(96), Some(97), Some(98), Some(99), Some(100), Some(101), Some(102), Some(104), Some(106), Some(107), Some(109), Some(111), Some(113), Some(114), Some(116), Some(117), Some(118), Some(119), Some(120), Some(121), Some(122), Some(123), Some(124), Some(126), Some(128), Some(129), Some(131), Some(133), Some(134), Some(136), Some(137), Some(139), Some(141), Some(142), Some(143), Some(144), Some(145), Some(146), Some(147), Some(149), Some(150), Some(151), Some(152), Some(153), Some(154), Some(156), Some(158), Some(159), Some(161), Some(162), Some(163), Some(164), Some(166), Some(167), Some(169), Some(171), Some(173), Some(174), Some(175), Some(176), Some(177), Some(178), Some(179), Some(180), Some(181), Some(183), Some(184), Some(186), Some(188), Some(189), Some(191), Some(193), Some(194), Some(195), Some(196), Some(198), Some(199), Some(200)]), 150), 66);
}

#[test]
fn test_50() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), None, Some(18), Some(28), Some(32), None, None, None, None, None, None, Some(13), None, Some(16), None, None, Some(27), Some(29), Some(31), Some(33), None, None, None, None, None, Some(34)]), 18), 35);
}

#[test]
fn test_51() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(75), Some(50), Some(100), Some(25), Some(60), Some(85), Some(125), Some(15), Some(35), Some(55), Some(65), Some(90), Some(110), Some(130), Some(150)]), 13), 130);
}

#[test]
fn test_52() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(4), Some(2), Some(5), Some(1), Some(3)]), 2), 2);
}

#[test]
fn test_53() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(90), Some(110), Some(140), Some(160), Some(180)]), 20), None);
}

#[test]
fn test_54() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(55), Some(30), Some(80), Some(15), Some(40), Some(70), Some(90), Some(10), Some(20), Some(35), Some(45), Some(65), Some(75), Some(85), Some(95)]), 9), 65);
}

#[test]
fn test_55() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(50), Some(25), Some(75), Some(12), Some(37), Some(62), Some(87), Some(6), Some(18), Some(31), Some(43), Some(56), Some(70), Some(81), Some(93), Some(3), Some(9), Some(15), Some(21), Some(28), Some(34), Some(40), Some(47), Some(53), Some(59), Some(65), Some(68), Some(73), Some(79), Some(84), Some(89), Some(91), Some(96)]), 25), 68);
}

#[test]
fn test_56() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), Some(18), Some(22), Some(28), Some(32), Some(38), Some(45)]), 18), None);
}

#[test]
fn test_57() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(7), Some(3), Some(9), Some(1), Some(5), None, Some(10), None, Some(2), Some(4), Some(6), Some(8), None, None, None, None, None, None, None]), 8), 9);
}

#[test]
fn test_58() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(90), Some(110), Some(140), Some(160), Some(180)]), 15), 180);
}

#[test]
fn test_59() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(7), Some(13)]), 7), 10);
}

#[test]
fn test_60() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(1), Some(2), None, None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7)]), 7), 1);
}

#[test]
fn test_61() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15)]), 13), 13);
}

#[test]
fn test_62() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(17), Some(25), Some(35), Some(47), Some(55), Some(60)]), 20), None);
}

#[test]
fn test_63() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(2), Some(1), Some(3), None, None, Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13)]), 7), 12);
}

#[test]
fn test_64() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(15), Some(10), Some(20), Some(8), Some(12), None, Some(25), Some(7), Some(9), Some(11), Some(13), Some(22), Some(30)]), 15), None);
}

#[test]
fn test_65() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(8), Some(3), Some(10), None, Some(6), Some(9), Some(14), None, Some(7), Some(8), Some(11), Some(12), Some(15)]), 7), 11);
}

#[test]
fn test_66() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(35), Some(60), Some(90), Some(110), Some(140), Some(160), Some(190)]), 10), 125);
}

#[test]
fn test_67() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(3), Some(7), Some(13), Some(17), Some(23), Some(27), Some(33), Some(37)]), 25), None);
}

#[test]
fn test_68() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15)]), 10), 10);
}

#[test]
fn test_69() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(1), Some(7), Some(12), Some(18), Some(22), Some(28), Some(32), Some(40)]), 15), 40);
}

#[test]
fn test_70() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(37), Some(23), Some(49), Some(10), Some(30), Some(44), Some(55), Some(5), Some(15), Some(25), Some(33), Some(41), Some(47), Some(52), Some(60)]), 9), 41);
}

#[test]
fn test_71() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(50), Some(25), Some(75), Some(10), Some(40), Some(60), Some(90), Some(5), Some(15), Some(35), Some(45), Some(55), Some(65), Some(85), Some(100), Some(3), Some(7), Some(13), Some(17), Some(32), Some(37), Some(43), Some(47), Some(52), Some(57), Some(62), Some(67), Some(80), Some(87), Some(95), Some(105), None, None, Some(8), Some(12), Some(16), Some(18), Some(33), Some(36), Some(41), Some(44), Some(46), Some(48), Some(51), Some(53), Some(56), Some(58), Some(61), Some(63), Some(66), Some(68), Some(78), Some(83), Some(86), Some(92), Some(98), None, None, Some(6), None, Some(9), None, None, Some(11), None, None, Some(14), None, None, Some(19), None, Some(24), None, Some(27), None, None, Some(30), None, Some(34), None, None, Some(38), None, Some(42), None, None, Some(45), None, None, Some(50), None, None, Some(54), None, None, Some(59), None, None, Some(64), None, None, Some(69), None, None, Some(74), None, None, Some(79), None, None, Some(82), None, None, Some(84), None, None, Some(89), None, None, Some(91), None, None, Some(93), None, None, Some(96), None, None, Some(99), None, None, Some(102), None, None, Some(104)]), 30), 35);
}

#[test]
fn test_72() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7), Some(0)]), 3), 2);
}

#[test]
fn test_73() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(18), Some(25), Some(35), Some(48), Some(2), None, Some(11), Some(14), Some(16), Some(19), Some(22), Some(28), Some(33), Some(38), Some(47), Some(52), None, Some(8), None, None, None, Some(13), None, Some(17), None, None, Some(21), Some(23), None, Some(27), None, None, Some(32), Some(36), None, None, Some(41), None, Some(46), Some(51), Some(55), None, None, None, Some(53), Some(54), None, Some(56)]), 12), 56);
}

#[test]
fn test_74() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(9), Some(5), Some(13), Some(2), Some(7), Some(11), Some(15), Some(1), Some(3), Some(6), Some(8), Some(10), Some(12), Some(14), Some(16)]), 12), 13);
}

#[test]
fn test_75() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)]), 5), 5);
}

#[test]
fn test_76() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(100), Some(75), Some(125), Some(50), Some(85), Some(110), Some(140), Some(30), Some(60), Some(80), Some(90), Some(105), Some(120), Some(135), Some(150)]), 28), None);
}

#[test]
fn test_77() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(20), Some(1), None, Some(6)]), 4), 6);
}

#[test]
fn test_78() {
    assert_eq!(Solution::kth_smallest(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(20), None, None, Some(6), Some(8)]), 4), 7);
}

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
    let result = Solution::insert_into_bst(build_tree(&[Some(1)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(2)]);
}

#[test]
fn test_2() {
    let result = Solution::insert_into_bst(build_tree(&[]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(5)]);
}

#[test]
fn test_3() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)]);
}

#[test]
fn test_4() {
    let result = Solution::insert_into_bst(build_tree(&[Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70)]), 25);
    assert_eq!(tree_to_vec(&result), vec![Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70), None, None, Some(25)]);
}

#[test]
fn test_5() {
    let result = Solution::insert_into_bst(build_tree(&[]), 1);
    assert_eq!(tree_to_vec(&result), vec![Some(1)]);
}

#[test]
fn test_6() {
    let result = Solution::insert_into_bst(build_tree(&[Some(5), None, Some(14), Some(11), Some(15)]), 12);
    assert_eq!(tree_to_vec(&result), vec![Some(5), None, Some(14), Some(11), Some(15), None, Some(12)]);
}

#[test]
fn test_7() {
    let result = Solution::insert_into_bst(build_tree(&[Some(1)]), 0);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(0)]);
}

#[test]
fn test_8() {
    let result = Solution::insert_into_bst(build_tree(&[Some(3), Some(1), Some(4), None, Some(2)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(1), Some(4), None, Some(2), None, Some(5)]);
}

#[test]
fn test_9() {
    let result = Solution::insert_into_bst(build_tree(&[Some(5), Some(2), Some(7), Some(1), Some(3), Some(6), Some(8)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(2), Some(7), Some(1), Some(3), Some(6), Some(8), None, None, None, Some(4)]);
}

#[test]
fn test_10() {
    let result = Solution::insert_into_bst(build_tree(&[Some(2), Some(1)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(2), Some(1), Some(3)]);
}

#[test]
fn test_11() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), None, None, None, None, None, None]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)]);
}

#[test]
fn test_12() {
    let result = Solution::insert_into_bst(build_tree(&[Some(2), Some(2), None, Some(2), None, Some(2)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(2), Some(2), Some(3), Some(2), None, None, None, Some(2)]);
}

#[test]
fn test_13() {
    let result = Solution::insert_into_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(35), Some(60), Some(90), Some(110), Some(140), Some(160), None, Some(5), None, None, Some(30), Some(40), None, Some(65), Some(85), Some(105), Some(135), Some(155), Some(165)]), 115);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(35), Some(60), Some(90), Some(110), Some(140), Some(160), None, Some(5), None, None, Some(30), Some(40), None, Some(65), Some(85), Some(105), Some(135), Some(155), Some(165), None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(115)]);
}

#[test]
fn test_14() {
    let result = Solution::insert_into_bst(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15)]), 16);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(16)]);
}

#[test]
fn test_15() {
    let result = Solution::insert_into_bst(build_tree(&[Some(60), Some(40), Some(80), Some(20), Some(50), Some(70), Some(90), Some(10), Some(30), Some(45), Some(55), Some(65), Some(75), Some(85), Some(95)]), 66);
    assert_eq!(tree_to_vec(&result), vec![Some(60), Some(40), Some(80), Some(20), Some(50), Some(70), Some(90), Some(10), Some(30), Some(45), Some(55), Some(65), Some(75), Some(85), Some(95), None, None, None, None, None, None, None, None, None, Some(66)]);
}

#[test]
fn test_16() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]), 6);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), None, None, Some(6)]);
}

#[test]
fn test_17() {
    let result = Solution::insert_into_bst(build_tree(&[Some(45), Some(30), Some(60), Some(10), Some(35), Some(50), Some(70), Some(5), Some(20), None, Some(40), None, None, Some(33), None]), 42);
    assert_eq!(tree_to_vec(&result), vec![Some(45), Some(30), Some(60), Some(10), Some(35), Some(50), Some(70), Some(5), Some(20), None, Some(40), None, None, Some(33), None, None, None, None, None, None, Some(42)]);
}

// test_18 deleted: original used Some(2.5)/Some(3.5)/Some(2.75) which cannot be
// represented as i32 while preserving BST semantics (values strictly between
// adjacent integers).

#[test]
fn test_19() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), None, Some(15), Some(12), Some(20), None, None, Some(13), Some(14), None, None, None, None]), 11);
    assert_eq!(tree_to_vec(&result), vec![Some(10), None, Some(15), Some(12), Some(20), Some(11), None, Some(13), Some(14)]);
}

#[test]
fn test_20() {
    let result = Solution::insert_into_bst(build_tree(&[Some(30), Some(20), Some(40), Some(10), Some(25), Some(35), Some(50), Some(5), None, Some(15), Some(28), None, None, Some(32), None, None, None, None, Some(33)]), 26);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(20), Some(40), Some(10), Some(25), Some(35), Some(50), Some(5), None, Some(15), Some(28), None, None, Some(32), None, None, None, None, Some(33), Some(26)]);
}

#[test]
fn test_21() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(20), None, Some(6), None, None, None, Some(30)]), 17);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(20), None, Some(6), None, None, Some(17), Some(30)]);
}

#[test]
fn test_22() {
    let result = Solution::insert_into_bst(build_tree(&[Some(50), Some(25), Some(75), Some(10), Some(30), Some(60), Some(80), None, Some(20), Some(28), Some(35), None, Some(55), Some(70), Some(85), Some(5), None, Some(22), None, None, Some(33), None, None, Some(52), None, Some(65), None, Some(73), None, Some(82), None, Some(88)]), 45);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(25), Some(75), Some(10), Some(30), Some(60), Some(80), None, Some(20), Some(28), Some(35), None, Some(55), Some(70), Some(85), Some(5), None, Some(22), None, None, Some(33), None, None, Some(52), None, Some(65), None, Some(73), None, Some(82), None, Some(88), Some(45)]);
}

#[test]
fn test_23() {
    let result = Solution::insert_into_bst(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(15), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85)]), 42);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(15), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85), None, None, None, None, None, None, Some(42)]);
}

#[test]
fn test_24() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(18)]), 6);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(18), None, None, Some(6)]);
}

#[test]
fn test_25() {
    let result = Solution::insert_into_bst(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), None, Some(18), Some(35), Some(48)]), 14);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), None, Some(18), Some(35), Some(48), None, None, None, None, None, Some(14)]);
}

#[test]
fn test_26() {
    let result = Solution::insert_into_bst(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(90)]), 68);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(90), None, None, None, None, None, None, None, None, None, None, None, Some(68)]);
}

#[test]
fn test_27() {
    let result = Solution::insert_into_bst(build_tree(&[Some(1000), Some(500), Some(1500), Some(250), Some(750), Some(1250), Some(1750), Some(125), Some(375), Some(625), Some(875), Some(1125), Some(1375), Some(1625), Some(1875)]), 1600);
    assert_eq!(tree_to_vec(&result), vec![Some(1000), Some(500), Some(1500), Some(250), Some(750), Some(1250), Some(1750), Some(125), Some(375), Some(625), Some(875), Some(1125), Some(1375), Some(1625), Some(1875), None, None, None, None, None, None, None, None, None, None, None, None, Some(1600)]);
}

#[test]
fn test_28() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(13), Some(18), Some(12), None, Some(14)]), 11);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), None, None, Some(13), Some(18), Some(12), None, Some(14), None, Some(11)]);
}

#[test]
fn test_29() {
    let result = Solution::insert_into_bst(build_tree(&[Some(8), Some(3), Some(10), None, Some(1), Some(6), None, Some(4), Some(7), Some(13), None, None, None, None, None, None, None, None]), 9);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(3), Some(10), None, Some(1), Some(6), None, Some(4), Some(7), Some(13), Some(9)]);
}

#[test]
fn test_30() {
    let result = Solution::insert_into_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(130), Some(160), Some(180), Some(5), Some(15), Some(20), Some(28), Some(55), Some(65), Some(70), Some(85), Some(105), Some(115), Some(128), Some(135), Some(155), Some(165), Some(178), Some(190)]), 140);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(130), Some(160), Some(180), Some(5), Some(15), Some(20), Some(28), Some(55), Some(65), Some(70), Some(85), Some(105), Some(115), Some(128), Some(135), Some(155), Some(165), Some(178), Some(190), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(140)]);
}

#[test]
fn test_31() {
    let result = Solution::insert_into_bst(build_tree(&[Some(3), Some(1), Some(4), None, Some(2), None, None]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(1), Some(4), None, Some(2), None, Some(5)]);
}

#[test]
fn test_32() {
    let result = Solution::insert_into_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(130), Some(160), Some(180)]), 140);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(130), Some(160), Some(180), None, None, None, None, None, None, None, None, None, None, None, Some(140)]);
}

#[test]
fn test_33() {
    let result = Solution::insert_into_bst(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), None, None, None, None, None, None, None, None, None, None]), 11);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), None, None, None, None, Some(11)]);
}

#[test]
fn test_34() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), None, None, None, None, None, None]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), None, Some(10)]);
}

#[test]
fn test_35() {
    let result = Solution::insert_into_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(130), Some(160), Some(180)]), 115);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(130), Some(160), Some(180), None, None, None, None, None, None, None, None, None, Some(115)]);
}

#[test]
fn test_36() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(12), Some(20)]), 13);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), None, None, Some(12), Some(20), None, Some(13)]);
}

#[test]
fn test_37() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7)]), 8);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7), None, None, None, None, None, None, None, Some(8)]);
}

#[test]
fn test_38() {
    let result = Solution::insert_into_bst(build_tree(&[Some(5), Some(2), Some(8), None, None, Some(6), Some(10), None, None, None, Some(9)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(2), Some(8), None, None, Some(6), Some(10), None, Some(7), None, Some(9)]);
}

#[test]
fn test_39() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6)]), 8);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6), Some(8)]);
}

#[test]
fn test_40() {
    let result = Solution::insert_into_bst(build_tree(&[Some(1), Some(3), None, None, Some(2)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(3), Some(4), None, Some(2)]);
}

#[test]
fn test_41() {
    let result = Solution::insert_into_bst(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(17), Some(25), Some(35), Some(43), Some(48), Some(55)]), 47);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(17), Some(25), Some(35), Some(43), Some(48), Some(55), None, None, None, None, None, None, None, None, None, None, None, None, Some(47)]);
}

#[test]
fn test_42() {
    let result = Solution::insert_into_bst(build_tree(&[Some(20), Some(15), Some(30), Some(10), Some(17), None, Some(35), None, None, Some(16), Some(19)]), 22);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(15), Some(30), Some(10), Some(17), Some(22), Some(35), None, None, Some(16), Some(19)]);
}

// test_43 deleted: original used Some(2.5)/Some(2.2) which cannot be represented
// as i32 while preserving BST semantics (values strictly between adjacent integers).

#[test]
fn test_44() {
    let result = Solution::insert_into_bst(build_tree(&[Some(50), Some(25), Some(75), Some(10), Some(35), Some(60), Some(85), None, Some(27), Some(30), Some(55), Some(65), Some(80), Some(90)]), 70);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(25), Some(75), Some(10), Some(35), Some(60), Some(85), None, Some(27), Some(30), Some(55), Some(65), Some(80), Some(90), None, None, None, None, None, None, None, None, None, Some(70)]);
}

#[test]
fn test_45() {
    let result = Solution::insert_into_bst(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]), 1);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), Some(1)]);
}

#[test]
fn test_46() {
    let result = Solution::insert_into_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(85), Some(110), Some(140), Some(160), Some(180)]), 105);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(85), Some(110), Some(140), Some(160), Some(180), None, None, None, None, None, None, None, None, Some(105)]);
}

#[test]
fn test_47() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(12), Some(20), None, None, Some(13), Some(14), Some(18), Some(22)]), 16);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), None, None, Some(12), Some(20), None, None, Some(13), Some(14), Some(18), Some(22), None, None, None, None, Some(16)]);
}

#[test]
fn test_48() {
    let result = Solution::insert_into_bst(build_tree(&[Some(3), Some(1), Some(4), None, Some(2), None, None]), 0);
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(1), Some(4), Some(0), Some(2)]);
}

#[test]
fn test_49() {
    let result = Solution::insert_into_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(90), Some(110), Some(140), Some(160), Some(180), Some(5), None, None, Some(28), Some(40), None, Some(85), Some(100), Some(115), Some(130), None, None, Some(145), Some(155), Some(165), Some(175), None, Some(185)]), 95);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(90), Some(110), Some(140), Some(160), Some(180), Some(5), None, None, Some(28), Some(40), None, Some(85), Some(100), Some(115), Some(130), None, None, Some(145), Some(155), Some(165), Some(175), None, Some(185), None, None, None, None, None, None, Some(95)]);
}

// test_50 deleted: original used Some(2.5)/Some(2.3) which cannot be represented
// as i32 while preserving BST semantics (values strictly between adjacent integers).

#[test]
fn test_51() {
    let result = Solution::insert_into_bst(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7), None, Some(1), None, None, None, None, None, None]), 8);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7), None, Some(1), None, None, None, Some(8)]);
}

#[test]
fn test_52() {
    let result = Solution::insert_into_bst(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, None, None, Some(65), Some(75), Some(85)]), 72);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, None, None, Some(65), Some(75), Some(85), None, None, None, None, Some(72)]);
}

#[test]
fn test_53() {
    let result = Solution::insert_into_bst(build_tree(&[Some(8), Some(5), Some(10), Some(3), Some(6), None, Some(12), None, None, None, Some(7), Some(11), Some(14)]), 9);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(10), Some(3), Some(6), Some(9), Some(12), None, None, None, Some(7), None, None, Some(11), Some(14)]);
}

#[test]
fn test_54() {
    let result = Solution::insert_into_bst(build_tree(&[Some(7), Some(5), Some(9), Some(4), Some(6), Some(8), Some(10)]), 11);
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(5), Some(9), Some(4), Some(6), Some(8), Some(10), None, None, None, None, None, None, None, Some(11)]);
}

#[test]
fn test_55() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), None, None, None, None, None, None]), 9);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), None, Some(9)]);
}

#[test]
fn test_56() {
    let result = Solution::insert_into_bst(build_tree(&[Some(8), Some(5), Some(15), Some(3), Some(6), Some(10), Some(18), Some(2), Some(4), Some(7), Some(9), Some(12), None, Some(16), None, None, None, None, None, None, Some(13), Some(14)]), 17);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(15), Some(3), Some(6), Some(10), Some(18), Some(2), Some(4), Some(7), Some(9), Some(12), None, Some(16), None, None, None, None, None, None, Some(13), Some(14), None, None, None, None, Some(17)]);
}

#[test]
fn test_57() {
    let result = Solution::insert_into_bst(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]), 9);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8), None, None, None, None, None, None, None, Some(9)]);
}

#[test]
fn test_58() {
    let result = Solution::insert_into_bst(build_tree(&[Some(12), Some(7), Some(19), Some(5), Some(9), Some(13), Some(20), Some(3), Some(6), Some(8), Some(10), Some(11), Some(14), Some(18), None, None, None, None, None, None, None, None, Some(16), None, None, None, Some(17)]), 12);
    assert_eq!(tree_to_vec(&result), vec![Some(12), Some(7), Some(19), Some(5), Some(9), Some(13), Some(20), Some(3), Some(6), Some(8), Some(10), Some(11), Some(14), Some(18), None, None, None, None, None, None, None, None, Some(16), None, Some(12), None, Some(17)]);
}

#[test]
fn test_59() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), None, None, Some(0), None, None, None, None, None]), -1);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), None, None, Some(0), None, None, None, Some(-1)]);
}

#[test]
fn test_60() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), None, None, None, None, None, None]), 0);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), None, None, Some(0)]);
}

#[test]
fn test_61() {
    let result = Solution::insert_into_bst(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85)]), 42);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85), None, None, None, None, None, None, Some(42)]);
}

#[test]
fn test_62() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(12), Some(20), None, None, Some(11), Some(13), None, None, None, None]), 14);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), None, None, Some(12), Some(20), None, Some(14), Some(11), Some(13)]);
}

#[test]
fn test_63() {
    let result = Solution::insert_into_bst(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(45), None, Some(65), None, None, Some(55)]), 52);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(45), Some(52), Some(65), None, None, Some(55)]);
}

#[test]
fn test_64() {
    let result = Solution::insert_into_bst(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), None, None, Some(3), Some(7), Some(12), Some(17), Some(25), Some(35), Some(11), Some(13), Some(23), Some(27), Some(32), Some(37)]), 26);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(30), Some(5), Some(15), Some(26), None, Some(3), Some(7), Some(12), Some(17), None, None, Some(25), Some(35), Some(11), Some(13), Some(23), Some(27), Some(32), Some(37)]);
}

#[test]
fn test_65() {
    let result = Solution::insert_into_bst(build_tree(&[Some(1), None, Some(3), None, Some(5)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(3), None, Some(5), Some(4)]);
}

#[test]
fn test_66() {
    let result = Solution::insert_into_bst(build_tree(&[Some(2), Some(1), Some(3), Some(4)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(2), Some(1), Some(3), Some(4), None, None, Some(5)]);
}

#[test]
fn test_67() {
    let result = Solution::insert_into_bst(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9), None, Some(10)]);
}

#[test]
fn test_68() {
    let result = Solution::insert_into_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180)]), 130);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180), None, None, None, None, None, None, None, None, None, None, Some(130)]);
}

#[test]
fn test_69() {
    let result = Solution::insert_into_bst(build_tree(&[Some(5), Some(3), Some(8), Some(2), Some(4), None, None, Some(1)]), 0);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(8), Some(2), Some(4), None, None, Some(1), None, None, None, Some(0)]);
}

#[test]
fn test_70() {
    let result = Solution::insert_into_bst(build_tree(&[Some(2147483647)]), -2147483648);
    assert_eq!(tree_to_vec(&result), vec![Some(2147483647), Some(-2147483648)]);
}

// test_71 deleted: original used Some(1.5)/Some(1.7) which cannot be represented
// as i32 while preserving BST semantics (values strictly between adjacent integers).

#[test]
fn test_72() {
    let result = Solution::insert_into_bst(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(3), Some(7), Some(13), Some(17), Some(23), Some(27), Some(33), Some(37)]), 18);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(3), Some(7), Some(13), Some(17), Some(23), Some(27), Some(33), Some(37), None, None, None, None, None, None, None, Some(18)]);
}

#[test]
fn test_73() {
    let result = Solution::insert_into_bst(build_tree(&[Some(50), Some(25), Some(75), Some(10), Some(30), Some(60), Some(80), Some(5), Some(15), Some(27), Some(35), Some(55), Some(65), Some(77), Some(90)]), 40);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(25), Some(75), Some(10), Some(30), Some(60), Some(80), Some(5), Some(15), Some(27), Some(35), Some(55), Some(65), Some(77), Some(90), None, None, None, None, None, None, None, Some(40)]);
}

#[test]
fn test_74() {
    let result = Solution::insert_into_bst(build_tree(&[Some(30), Some(15), Some(70), Some(10), Some(20), Some(60), Some(80), Some(5), Some(12), Some(18), Some(25), Some(50), Some(65), Some(75), Some(85)]), 35);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(15), Some(70), Some(10), Some(20), Some(60), Some(80), Some(5), Some(12), Some(18), Some(25), Some(50), Some(65), Some(75), Some(85), None, None, None, None, None, None, None, None, Some(35)]);
}

#[test]
fn test_75() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3)]), 8);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), None, Some(8)]);
}

#[test]
fn test_76() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), None, None, Some(0)]), -1);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), None, None, Some(0), None, None, None, Some(-1)]);
}

#[test]
fn test_77() {
    let result = Solution::insert_into_bst(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), None, None, None, None, None, None, None, None, None]), 11);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), None, None, None, None, Some(11)]);
}

#[test]
fn test_78() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), Some(2), None, Some(12), Some(18), None, None, Some(11), Some(14), Some(17), Some(19)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(2), None, Some(12), Some(18), None, None, Some(11), Some(14), Some(17), Some(19), None, None, None, None, None, None, None, Some(20)]);
}

#[test]
fn test_79() {
    let result = Solution::insert_into_bst(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), None, None, None, None, None, None]), 6);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6)]);
}

#[test]
fn test_80() {
    let result = Solution::insert_into_bst(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(12), Some(17), Some(22), Some(27), Some(32), Some(40)]), 11);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(12), Some(17), Some(22), Some(27), Some(32), Some(40), None, None, None, None, Some(11)]);
}

#[test]
fn test_81() {
    let result = Solution::insert_into_bst(build_tree(&[Some(10), Some(5), Some(15), None, None, Some(12), Some(20), None, None, Some(11), Some(13), None, Some(18), Some(17), Some(22)]), 9);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), None, Some(9), Some(12), Some(20), None, None, None, None, Some(11), Some(13), None, Some(18), Some(17), Some(22)]);
}

#[test]
fn test_82() {
    let result = Solution::insert_into_bst(build_tree(&[Some(8), Some(3), Some(10), None, Some(5), Some(9), Some(12), None, None, Some(4), Some(6)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(3), Some(10), None, Some(5), Some(9), Some(12), None, Some(7), Some(4), Some(6)]);
}

#[test]
fn test_83() {
    let result = Solution::insert_into_bst(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(12), Some(17), Some(22), Some(27), Some(32), Some(37)]), 21);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(12), Some(17), Some(22), Some(27), Some(32), Some(37), None, None, None, None, None, None, None, None, Some(21)]);
}

#[test]
fn test_84() {
    let result = Solution::insert_into_bst(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), None, None, Some(18), None, Some(28), None, None, None, None]), 19);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), None, Some(19), Some(18), None, Some(28)]);
}

#[test]
fn test_85() {
    let result = Solution::insert_into_bst(build_tree(&[Some(30), Some(20), Some(40), Some(10), Some(25), Some(35), Some(50), Some(5), None, None, Some(27), Some(32), Some(45), Some(55)]), 28);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(20), Some(40), Some(10), Some(25), Some(35), Some(50), Some(5), None, None, Some(27), Some(32), Some(45), Some(55), None, None, None, None, Some(28)]);
}

#[test]
fn test_86() {
    let result = Solution::insert_into_bst(build_tree(&[Some(2), Some(1), Some(3), None, None, None, Some(4), None, None, None, Some(5)]), 6);
    assert_eq!(tree_to_vec(&result), vec![Some(2), Some(1), Some(3), None, None, None, Some(4), None, Some(6)]);
}

#[test]
fn test_87() {
    let result = Solution::insert_into_bst(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), None, None, Some(12), None, Some(22), Some(28), None, None, None, None, None, None, None, None]), 27);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), None, None, Some(12), None, Some(22), Some(28), None, None, None, None, None, None, Some(27)]);
}

#[test]
fn test_88() {
    let result = Solution::insert_into_bst(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), None, None, Some(12), Some(17), Some(22), Some(27), None, None, None, None, None, None, None, None, None, None]), 19);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), None, None, Some(12), Some(17), Some(22), Some(27), None, None, None, None, None, Some(19)]);
}

#[test]
fn test_89() {
    let result = Solution::insert_into_bst(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None, None, None, None, None]), 6);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6)]);
}

#[test]
fn test_90() {
    let result = Solution::insert_into_bst(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(1), Some(7), None, Some(17), Some(23), None, Some(21), Some(27), None, None, None, None, None, Some(22), None, None, None, Some(26)]), 18);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(1), Some(7), None, Some(17), Some(23), None, Some(21), Some(27), None, None, None, None, None, Some(22), None, None, None, Some(26), None, None, Some(18)]);
}

#[test]
fn test_91() {
    let result = Solution::insert_into_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(90), Some(110), Some(140), Some(160), Some(180)]), 130);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(90), Some(110), Some(140), Some(160), Some(180), None, None, None, None, None, None, None, None, None, None, Some(130)]);
}

#[test]
fn test_92() {
    let result = Solution::insert_into_bst(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180)]), 90);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180), None, None, None, None, None, None, None, Some(90)]);
}

#[test]
fn test_93() {
    let result = Solution::insert_into_bst(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, Some(35), None, Some(55), None, None, None, None, None]), 25);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), None, Some(55)]);
}

#[test]
fn test_94() {
    let result = Solution::insert_into_bst(build_tree(&[Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70), Some(5), None, Some(25), None, Some(45), Some(55), None, None, None, None, None, Some(23), Some(35), None, Some(42), Some(48), None, None, None, None, None, Some(65), Some(75), Some(62), Some(68), None, None, Some(73), Some(77), None, None, None, None, None, None, None, None]), 74);
    assert_eq!(tree_to_vec(&result), vec![Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70), Some(5), None, Some(25), None, Some(45), Some(55), None, Some(74), None, None, None, Some(23), Some(35), None, Some(42), Some(48), None, None, None, None, None, None, None, Some(65), Some(75), Some(62), Some(68), None, None, Some(73), Some(77)]);
}

#[test]
fn test_95() {
    let result = Solution::insert_into_bst(build_tree(&[Some(45), Some(25), Some(65), Some(15), Some(35), Some(55), Some(75), Some(10), None, Some(30), Some(40), Some(50), Some(60), Some(70), Some(80)]), 27);
    assert_eq!(tree_to_vec(&result), vec![Some(45), Some(25), Some(65), Some(15), Some(35), Some(55), Some(75), Some(10), None, Some(30), Some(40), Some(50), Some(60), Some(70), Some(80), None, None, Some(27)]);
}

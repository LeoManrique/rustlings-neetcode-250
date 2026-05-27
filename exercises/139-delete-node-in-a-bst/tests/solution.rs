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
    let result = Solution::delete_node(build_tree(&[Some(1), None, Some(2)]), 1);
    assert_eq!(tree_to_vec(&result), vec![Some(2)]);
}

#[test]
fn test_2() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(6), Some(2), Some(4)]);
}

#[test]
fn test_3() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(4), Some(6), Some(2), None, None, Some(7)]);
}

#[test]
fn test_4() {
    let result = Solution::delete_node(build_tree(&[]), 0);
    assert_eq!(tree_to_vec(&result), vec![]);
}

#[test]
fn test_5() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]), 0);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]);
}

#[test]
fn test_6() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(6), None, Some(4), None, Some(7)]);
}

#[test]
fn test_7() {
    let result = Solution::delete_node(build_tree(&[Some(2), Some(1), Some(3)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(1)]);
}

#[test]
fn test_8() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), None, Some(40), Some(60), Some(80)]), 50);
    assert_eq!(tree_to_vec(&result), vec![Some(70), Some(60), Some(80), Some(30), None, None, None, None, Some(40)]);
}

#[test]
fn test_9() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(6), Some(2), None, None, Some(7)]);
}

#[test]
fn test_10() {
    let result = Solution::delete_node(build_tree(&[Some(1)]), 1);
    assert_eq!(tree_to_vec(&result), vec![]);
}

#[test]
fn test_11() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), None]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2)]);
}

#[test]
fn test_12() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(18), Some(22), Some(30), Some(8), Some(12), Some(16), Some(19), Some(21), Some(24), Some(27), Some(32)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(22), Some(30), Some(21), Some(24), Some(27), Some(32), Some(15), None, None, None, None, None, None, None, Some(10), Some(18), Some(8), Some(12), Some(16), Some(19)]);
}

#[test]
fn test_13() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 9);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_14() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), None, Some(7), Some(12), Some(18)]);
}

#[test]
fn test_15() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 55);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]);
}

#[test]
fn test_16() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(7), Some(4), Some(5), Some(14), Some(15), Some(8), Some(9), Some(10), Some(11), Some(6), None, None, None, None, None, None, None, None, None, None, None, Some(12), Some(13)]);
}

#[test]
fn test_17() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85), Some(5), Some(15), None, None, None, None, Some(32), Some(42), Some(47), None, Some(52), Some(62), Some(67), Some(72), Some(78), Some(82), Some(87), Some(90), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]), 40);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(45), Some(60), Some(80), Some(10), Some(25), Some(32), Some(42), Some(55), Some(65), Some(75), Some(85), Some(5), Some(15), None, None, Some(35), None, None, None, Some(47), None, Some(52), Some(62), Some(67), Some(72), Some(78), Some(82), Some(87), Some(90)]);
}

#[test]
fn test_18() {
    let result = Solution::delete_node(build_tree(&[Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70), Some(5), Some(15), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75)]), 50);
    assert_eq!(tree_to_vec(&result), vec![Some(40), Some(20), Some(60), Some(10), Some(30), Some(55), Some(70), Some(5), Some(15), Some(25), Some(35), Some(45), None, Some(65), Some(75)]);
}

#[test]
fn test_19() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 11);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_20() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(5), Some(8), Some(2), None, None, None, Some(1), Some(3), Some(9), Some(10)]);
}

#[test]
fn test_21() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_22() {
    let result = Solution::delete_node(build_tree(&[Some(25), Some(15), Some(35), Some(10), Some(20), Some(30), Some(40), Some(5), Some(12), None, None, Some(27), None, None, None, None, None, Some(8)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(20), Some(35), Some(10), None, Some(30), Some(40), Some(5), Some(12), Some(27), None, None, None, None, None, Some(8)]);
}

#[test]
fn test_23() {
    let result = Solution::delete_node(build_tree(&[Some(8), Some(4), Some(12), Some(2), Some(6), Some(10), Some(14), Some(1), Some(3), Some(5), Some(7), Some(9), Some(11), Some(13), Some(15), Some(17), Some(19)]), 12);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(4), Some(14), Some(2), Some(6), Some(13), Some(15), Some(1), Some(3), Some(5), Some(7), Some(10), None, None, None, Some(17), Some(19), None, None, None, None, None, None, Some(9), Some(11)]);
}

#[test]
fn test_24() {
    let result = Solution::delete_node(build_tree(&[Some(70), Some(30), Some(100), Some(20), Some(50), Some(80), Some(110), Some(10), Some(25), Some(40), Some(60), Some(75), Some(85), Some(105), Some(120), Some(5), Some(15), Some(22), Some(27), Some(35), Some(45), Some(55), Some(72), Some(78), Some(82), Some(87), Some(102), Some(108), Some(115), Some(125)]), 70);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(80), Some(110), Some(75), Some(85), Some(105), Some(120), Some(78), Some(82), Some(87), Some(102), Some(108), Some(115), Some(125), None, Some(30), None, None, None, None, None, None, None, None, None, None, None, None, None, Some(20), Some(50), Some(10), Some(25), Some(40), Some(60), Some(5), Some(15), Some(22), Some(27), Some(35), Some(45), Some(55), Some(72)]);
}

#[test]
fn test_25() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), None, Some(6), Some(8), Some(11), Some(13), Some(16), Some(19)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(15), Some(12), Some(18), Some(11), Some(13), Some(16), Some(19), Some(5), None, None, None, None, None, None, None, Some(3), Some(7), Some(1), None, Some(6), Some(8)]);
}

#[test]
fn test_26() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(8), Some(1), Some(3), Some(5), None, Some(9), Some(10)]);
}

#[test]
fn test_27() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]), 1);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(10), Some(3), Some(5), Some(8), Some(9)]);
}

#[test]
fn test_28() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(18), Some(3), Some(7)]);
}

#[test]
fn test_29() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(20), None, None, Some(6), Some(8), None, None, None, None, Some(16), Some(25)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(2), Some(8), None, Some(20), None, None, Some(16), Some(25), None, None, Some(6)]);
}

#[test]
fn test_30() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(17), Some(22), Some(30), Some(5), Some(12), Some(16), Some(18), Some(21), Some(23), Some(27), Some(35), Some(3), Some(8), None, None, None, None, None, Some(19), None, None, None, Some(24), Some(26), Some(32), Some(34), None, None, None, None, Some(9), Some(11), None, None, Some(14), None, None, None, None, Some(28), Some(31), Some(33), Some(36)]), 25);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(15), Some(30), Some(10), Some(17), Some(27), Some(35), Some(5), Some(12), Some(16), Some(18), Some(26), Some(32), Some(34), None, Some(3), Some(8), None, None, None, None, None, Some(19), Some(22), None, None, None, Some(28), Some(31), None, None, None, Some(9), Some(11), None, Some(21), Some(23), None, None, None, None, Some(33), Some(36), None, None, None, None, None, Some(24), None, None, None, None, None, Some(14)]);
}

#[test]
fn test_31() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18), Some(1), None, Some(6), Some(8)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(7), Some(15), Some(6), Some(8), None, Some(18), Some(3), None, None, None, None, None, Some(1)]);
}

#[test]
fn test_32() {
    let result = Solution::delete_node(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(3), Some(20), None, None, Some(9)]);
}

#[test]
fn test_33() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(0), Some(2), Some(0), Some(1)]), 0);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(1), Some(2), Some(0)]);
}

#[test]
fn test_34() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]), 9);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]);
}

#[test]
fn test_35() {
    let result = Solution::delete_node(build_tree(&[Some(8), Some(5), Some(12), Some(3), Some(7), Some(10), Some(15), Some(2), Some(4), Some(6), Some(9), Some(11), Some(13), Some(14), Some(17), Some(1), None, None, None, None, None, None, None, None, None, None, Some(16), Some(18)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(12), Some(3), Some(7), Some(13), Some(15), Some(2), Some(4), Some(6), Some(9), Some(11), Some(16), Some(14), Some(17), Some(1), None, None, None, None, None, None, None, None, None, None, None, Some(18)]);
}

#[test]
fn test_36() {
    let result = Solution::delete_node(build_tree(&[Some(3), Some(2), Some(4), Some(1), None, None, Some(5)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(2), Some(5), Some(1)]);
}

#[test]
fn test_37() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14)]);
}

#[test]
fn test_38() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(7), Some(2), None, Some(6), Some(8)]);
}

#[test]
fn test_39() {
    let result = Solution::delete_node(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(18), Some(25), Some(35), Some(48), None, None, None, None, None, None, None, None, None, None, None, Some(33), Some(42)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(20), Some(45), Some(18), Some(25), Some(40), Some(50), Some(10), None, None, None, Some(35), Some(48), None, None, Some(5), Some(12), None, Some(33), Some(42)]);
}

#[test]
fn test_40() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)]), 12);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)]);
}

#[test]
fn test_41() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 85);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, None, None, None, None, Some(90)]);
}

#[test]
fn test_42() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), None, Some(5), Some(8), Some(9), Some(10)]);
}

#[test]
fn test_43() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 8);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_44() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(17), Some(22), Some(30), Some(5), Some(12), Some(16), Some(18), Some(21), Some(23), Some(27), Some(35), Some(3), Some(8), None, None, None, None, None, Some(19), None, None, None, Some(24), Some(26), Some(32), Some(34), None, None, None, None, Some(9), Some(11), None, None, Some(14), None, None, None, None, Some(28), Some(31), Some(33), Some(36)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(15), Some(25), Some(10), Some(17), Some(22), Some(30), Some(8), Some(12), Some(16), Some(18), Some(21), Some(23), Some(27), Some(35), Some(3), Some(9), None, None, None, None, None, Some(19), None, None, None, Some(24), Some(26), Some(32), Some(34), None, None, None, Some(33), Some(36), Some(11), None, None, Some(14), None, None, None, None, Some(28), Some(31)]);
}

#[test]
fn test_45() {
    let result = Solution::delete_node(build_tree(&[Some(30), Some(10), Some(40), Some(5), Some(15), Some(35), Some(50), Some(1), Some(7), None, Some(18), Some(32), Some(45), Some(55), Some(60), None, None, None, None, Some(8), None, None, Some(23), Some(27), None, None, None, Some(42), Some(48), None, None, None, Some(53), None, Some(57), None, Some(62)]), 30);
    assert_eq!(tree_to_vec(&result), vec![Some(40), Some(35), Some(50), Some(32), Some(45), Some(55), Some(60), Some(10), Some(23), Some(27), None, None, None, Some(42), Some(48), Some(5), Some(15), None, Some(53), None, Some(57), None, Some(62), None, None, Some(1), Some(7), None, Some(18), None, None, None, None, None, None, None, None, None, None, Some(8)]);
}

#[test]
fn test_46() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(3), Some(7), Some(12), Some(17), Some(22), Some(27), Some(32), Some(37), Some(1), Some(4), Some(6), Some(8), Some(11), Some(13), Some(16), Some(18), Some(21), Some(23), Some(26), Some(28), Some(31), Some(33), Some(36), Some(38), Some(39)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(25), Some(35), Some(22), Some(27), Some(32), Some(37), Some(21), Some(23), Some(26), Some(28), Some(31), Some(33), Some(36), Some(38), Some(10), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(5), Some(15), Some(3), Some(7), Some(12), Some(17), Some(1), Some(4), Some(6), Some(8), Some(11), Some(13), Some(16), Some(18), Some(39)]);
}

#[test]
fn test_47() {
    let result = Solution::delete_node(build_tree(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(1), None, Some(2), None, Some(4), None, Some(5)]);
}

#[test]
fn test_48() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 65);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]);
}

#[test]
fn test_49() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 50);
    assert_eq!(tree_to_vec(&result), vec![Some(70), Some(60), Some(80), Some(65), None, None, Some(85), Some(30), None, None, None, Some(20), Some(40), Some(10), None, None, Some(55), None, None, None, Some(90)]);
}

#[test]
fn test_50() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]), 8);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), None, Some(9), Some(10)]);
}

#[test]
fn test_51() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 13);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_52() {
    let result = Solution::delete_node(build_tree(&[Some(8), Some(3), Some(10), None, Some(5), Some(9), Some(12), None, Some(7), None, None, None, Some(11), Some(13)]), 8);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(9), Some(12), Some(3), None, None, Some(11), None, Some(5), None, None, None, Some(7), Some(13)]);
}

#[test]
fn test_53() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(7), Some(15), Some(3), None, None, Some(18)]);
}

#[test]
fn test_54() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(6), Some(8), Some(3), None, None, None, Some(2), Some(4)]);
}

#[test]
fn test_55() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(3), Some(7), Some(1), None, Some(5), Some(8), Some(9), Some(10)]);
}

#[test]
fn test_56() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(7), None, Some(4), Some(6), Some(8)]);
}

#[test]
fn test_57() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(7), Some(15), Some(6), None, Some(13), Some(18), Some(3), None, None, None, None, None, Some(1)]);
}

#[test]
fn test_58() {
    let result = Solution::delete_node(build_tree(&[Some(100), Some(50), Some(150), None, Some(75), None, Some(200), None, Some(125), Some(175), Some(250)]), 150);
    assert_eq!(tree_to_vec(&result), vec![Some(100), Some(50), Some(200), None, Some(75), Some(175), Some(250), None, Some(125)]);
}

#[test]
fn test_59() {
    let result = Solution::delete_node(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), None, Some(18), Some(25), Some(35), Some(48), None, None, None, None, Some(19)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(15), Some(45), Some(10), Some(25), Some(40), Some(50), Some(5), None, Some(18), None, Some(35), Some(48), None, None, None, None, Some(19)]);
}

#[test]
fn test_60() {
    let result = Solution::delete_node(build_tree(&[Some(30), Some(20), Some(40), Some(15), Some(25), Some(35), Some(45), Some(10), None, None, Some(22), Some(32), Some(37), Some(42), Some(47), Some(5), None, None, None, None, Some(18), None, Some(24), Some(27), None, None, None, None, None, None, None, None, None, None, None]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(25), Some(40), Some(15), Some(22), Some(35), Some(45), Some(10), None, None, None, Some(32), Some(37), Some(42), Some(47), Some(5), None, None, Some(18), None, Some(24), Some(27)]);
}

#[test]
fn test_61() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85)]), 30);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(40), Some(70), Some(35), Some(45), Some(60), Some(80), Some(20), None, None, None, Some(55), Some(65), Some(75), Some(85), Some(10), Some(25)]);
}

#[test]
fn test_62() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), None, Some(6), Some(8), Some(11), Some(13), None, Some(14), None, None, Some(2)]), 100);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), None, Some(6), Some(8), Some(11), Some(13), None, Some(14), None, None, Some(2)]);
}

// test_63 deleted: original used Some(3.5)/Some(4.5) which cannot be represented
// as i32 while preserving BST semantics (values strictly between adjacent integers).

#[test]
fn test_64() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_65() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10)]);
}

#[test]
fn test_66() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)]);
}

#[test]
fn test_67() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), None, Some(8), Some(9), Some(10)]);
}

#[test]
fn test_68() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(17), Some(22), Some(30), Some(5), Some(12), Some(16), Some(18), Some(21), Some(23), Some(27), Some(35), Some(3), Some(8), None, None, None, None, None, Some(19), None, None, None, Some(24), Some(26), Some(32), Some(34), None, None, None, None, Some(9), Some(11), None, None, Some(14), None, None, None, None, Some(28), Some(31), Some(33), Some(36)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(22), Some(30), Some(21), Some(23), Some(27), Some(35), Some(15), None, None, Some(24), Some(26), Some(32), Some(34), None, Some(10), Some(17), None, Some(14), None, None, None, None, Some(28), Some(31), Some(5), Some(12), Some(16), Some(18), None, None, None, None, None, None, Some(3), Some(8), None, None, None, None, None, Some(19), None, None, None, Some(9), Some(11), None, Some(33), Some(36)]);
}

#[test]
fn test_69() {
    let result = Solution::delete_node(build_tree(&[Some(15), Some(10), Some(20), Some(8), Some(12), Some(17), Some(25), Some(5), Some(9), Some(11), Some(13), Some(16), Some(18), Some(22), Some(27), Some(3), Some(7), None, None, None, None, Some(14), None, Some(19), Some(21), Some(23), Some(26), Some(28), None, None, None, None, None, None, None, None]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(17), Some(25), Some(16), Some(18), Some(22), Some(27), Some(19), Some(21), Some(23), Some(26), Some(28), None, None, None, Some(10), None, None, None, None, None, None, None, None, None, Some(8), Some(12), Some(5), Some(9), Some(11), Some(13), Some(3), Some(7), None, None, None, None, Some(14)]);
}

#[test]
fn test_70() {
    let result = Solution::delete_node(build_tree(&[Some(15), Some(10), Some(20), Some(8), Some(12), Some(17), Some(25), Some(6), Some(9), Some(11), Some(13), Some(16), Some(18), Some(22), Some(28)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(17), Some(25), Some(16), Some(18), Some(22), Some(28), Some(10), None, None, None, None, None, None, None, Some(8), Some(12), Some(6), Some(9), Some(11), Some(13)]);
}

#[test]
fn test_71() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), Some(8), Some(9), Some(10)]);
}

#[test]
fn test_72() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(18), Some(22), Some(30), Some(5), Some(12), None, Some(17), None, None, None, None, Some(1), None]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(18), Some(25), Some(10), Some(17), Some(22), Some(30), Some(5), Some(12), None, None, None, None, None, None, Some(1)]);
}

#[test]
fn test_73() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(8), Some(2), Some(4), Some(7), Some(9), None, Some(10), None, None, None, None, Some(6)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(4), Some(8), Some(2), None, Some(7), Some(9), None, Some(10), None, None, Some(6)]);
}

#[test]
fn test_74() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(17), Some(22), Some(30), Some(5), Some(12), Some(16), Some(18), Some(21), Some(23), Some(27), Some(35), Some(3), Some(8), None, None, None, None, None, Some(19), None, None, None, Some(24), Some(26), Some(32), Some(34), None, None, None, None, Some(9), Some(11), None, None, Some(14), None, None, None, None, Some(28), Some(31), Some(33), Some(36)]), 30);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(15), Some(25), Some(10), Some(17), Some(22), Some(35), Some(5), Some(12), Some(16), Some(18), Some(21), Some(23), Some(34), None, Some(3), Some(8), None, None, None, None, None, Some(19), None, None, None, Some(24), Some(28), Some(31), None, None, None, Some(9), Some(11), None, None, Some(14), Some(27), None, None, None, Some(33), Some(36), None, None, None, None, Some(26), Some(32)]);
}

#[test]
fn test_75() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(3), None, Some(12), Some(18)]);
}

#[test]
fn test_76() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), None, Some(6), Some(8), Some(11), Some(13), Some(17), Some(19)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(15), Some(12), Some(18), Some(11), Some(13), Some(17), Some(19), Some(5), None, None, None, None, None, None, None, Some(3), Some(7), Some(1), None, Some(6), Some(8)]);
}

#[test]
fn test_77() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]), 6);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(7), Some(2), Some(4), None, Some(8)]);
}

#[test]
fn test_78() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18)]), 12);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]);
}

#[test]
fn test_79() {
    let result = Solution::delete_node(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(130), Some(160), Some(180)]), 100);
    assert_eq!(tree_to_vec(&result), vec![Some(150), Some(125), Some(175), Some(110), Some(130), Some(160), Some(180), Some(50), None, None, None, None, None, None, None, Some(25), Some(75), Some(10), Some(30), Some(60), Some(80)]);
}

#[test]
fn test_80() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), Some(4), Some(6), Some(8), Some(11), Some(13), Some(16), Some(19)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(7), Some(15), Some(6), Some(8), Some(12), Some(18), Some(3), None, None, None, Some(11), Some(13), Some(16), Some(19), Some(1), Some(4)]);
}

#[test]
fn test_81() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), None, Some(6), Some(8), Some(11), Some(13), Some(17), Some(19)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(3), Some(8), Some(12), Some(18), Some(1), None, Some(6), None, Some(11), Some(13), Some(17), Some(19)]);
}

#[test]
fn test_82() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7)]), 4);
    assert_eq!(tree_to_vec(&result), vec![Some(6), Some(5), Some(7), Some(2), None, None, None, Some(1), Some(3)]);
}

#[test]
fn test_83() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(4), None, Some(3), None, Some(2), None, Some(1)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(3), None, Some(2), None, Some(1)]);
}

#[test]
fn test_84() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]), 1);
    assert_eq!(tree_to_vec(&result), vec![Some(2), Some(3), None, Some(4), None, Some(5), None, Some(6), None, Some(7), None, Some(8), None, Some(9)]);
}

#[test]
fn test_85() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 1);
    assert_eq!(tree_to_vec(&result), vec![Some(3), Some(6), Some(7), Some(12), Some(13), Some(14), Some(15), Some(2), None, None, None, None, None, None, None, Some(4), Some(5), Some(8), Some(9), Some(10), Some(11)]);
}

#[test]
fn test_86() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(2), Some(7), None, Some(18), None, None, Some(6), Some(8)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(7), Some(15), Some(6), Some(8), None, Some(18), Some(2)]);
}

#[test]
fn test_87() {
    let result = Solution::delete_node(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(35), Some(60), Some(85), Some(115), Some(140), Some(160), Some(185), None, None, None, None, Some(28), None, Some(55), Some(90), None, Some(105), Some(110), Some(130), Some(135), None, None, None, None, None, None, None, None, Some(190), None, Some(195)]), 100);
    assert_eq!(tree_to_vec(&result), vec![Some(150), Some(125), Some(175), Some(115), Some(140), Some(160), Some(185), Some(50), Some(105), Some(110), Some(130), Some(135), None, None, None, Some(25), Some(75), None, Some(195), None, None, None, None, None, None, Some(10), Some(35), Some(60), Some(85), None, None, None, None, None, None, Some(28), None, Some(55), Some(90), None, None, None, None, None, Some(190)]);
}

#[test]
fn test_88() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), None, Some(6), Some(8), Some(11), Some(13), Some(17), Some(19)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(18), Some(3), Some(7), Some(17), Some(19), Some(1), None, Some(6), Some(8), Some(12), None, None, None, None, None, None, None, None, None, Some(11), Some(13)]);
}

#[test]
fn test_89() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(18), Some(3), Some(7), Some(12)]);
}

#[test]
fn test_90() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(15), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85)]), 60);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(65), Some(80), Some(15), Some(25), Some(35), Some(45), Some(55), None, Some(75), Some(85)]);
}

#[test]
fn test_91() {
    let result = Solution::delete_node(build_tree(&[Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(7), Some(13), None, None]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(3), Some(14), Some(1), Some(6), Some(13), None, None, None, Some(4), Some(7)]);
}

#[test]
fn test_92() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(17), Some(22), Some(30), Some(5), Some(12), Some(16), Some(18), Some(21), Some(23), Some(27), Some(35), Some(3), Some(8), None, None, None, None, None, Some(19), None, None, None, Some(24), Some(26), Some(32), Some(34), None, None, None, None, Some(9), Some(11), None, None, Some(14), None, None, None, None, Some(28), Some(31), Some(33), Some(36)]), 11);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(15), Some(25), Some(10), Some(17), Some(22), Some(30), Some(5), Some(12), Some(16), Some(18), Some(21), Some(23), Some(27), Some(35), Some(3), Some(8), None, None, None, None, None, Some(19), None, None, None, Some(24), Some(26), Some(32), Some(34), None, None, None, None, Some(9), Some(11), None, None, Some(14), None, None, None, None, Some(28), Some(31), Some(33), Some(36)]);
}

#[test]
fn test_93() {
    let result = Solution::delete_node(build_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9), None, None, None, None, None, Some(8)]), 6);
    assert_eq!(tree_to_vec(&result), vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(8), Some(9)]);
}

#[test]
fn test_94() {
    let result = Solution::delete_node(build_tree(&[Some(30), Some(15), Some(45), Some(10), Some(20), Some(40), Some(50), Some(5), Some(12), Some(18), Some(25), Some(35), Some(48), Some(3), Some(7), None, Some(13), Some(17), None, Some(23), None, Some(33), Some(43), Some(47), None, Some(52)]), 30);
    assert_eq!(tree_to_vec(&result), vec![Some(45), Some(40), Some(50), Some(35), Some(48), Some(3), Some(7), Some(47), None, Some(52), None, None, None, None, None, Some(15), None, None, None, Some(10), Some(20), Some(5), Some(12), Some(18), Some(25), None, Some(13), Some(17), None, Some(23), None, Some(33), Some(43)]);
}

#[test]
fn test_95() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(18), Some(22), Some(30), Some(5), Some(12), Some(17), Some(19), Some(21), Some(24), Some(28), Some(35)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(22), Some(30), Some(21), Some(24), Some(28), Some(35), Some(15), None, None, None, None, None, None, None, Some(10), Some(18), Some(5), Some(12), Some(17), Some(19)]);
}

#[test]
fn test_96() {
    let result = Solution::delete_node(build_tree(&[Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70), Some(5), Some(15), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(40), Some(30), Some(60), Some(25), Some(35), Some(50), Some(70), Some(10), None, None, None, Some(45), Some(55), Some(65), Some(75), Some(5), Some(15)]);
}

#[test]
fn test_97() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), None, None, None, Some(55), Some(65), None, None, Some(85), None, Some(90)]);
}

#[test]
fn test_98() {
    let result = Solution::delete_node(build_tree(&[Some(25), Some(10), Some(30), Some(5), Some(15), Some(28), Some(35), Some(3), Some(7), Some(12), Some(18), None, Some(29), Some(32), Some(34), Some(36)]), 30);
    assert_eq!(tree_to_vec(&result), vec![Some(25), Some(10), Some(35), Some(5), Some(15), Some(32), Some(34), Some(3), Some(7), Some(12), Some(18), Some(28), None, None, None, Some(36), None, None, None, None, None, None, None, None, Some(29)]);
}

#[test]
fn test_99() {
    let result = Solution::delete_node(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20), None, None, Some(8), Some(13), Some(18), Some(23), Some(17), Some(22), Some(21), Some(24), None, None, None, Some(16), None, None, None, None, None, None, None, Some(19)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(3), Some(20), None, None, Some(8), Some(13), Some(18), Some(23), Some(17), Some(22), Some(21), Some(24), None, None, None, Some(16), None, None, Some(9), None, None, None, None, Some(19)]);
}

#[test]
fn test_100() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 80);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(85), Some(10), None, None, Some(55), Some(65), None, None, None, None, None, None, Some(90)]);
}

#[test]
fn test_101() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18)]), 18);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(12)]);
}

#[test]
fn test_102() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 70);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(80), Some(20), Some(40), Some(60), Some(85), Some(10), None, None, Some(55), Some(65), None, None, None, None, None, None, Some(90)]);
}

#[test]
fn test_103() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(15), Some(12), Some(18), Some(5), None, None, None, Some(3), Some(7)]);
}

#[test]
fn test_104() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), None, Some(7), Some(12), Some(20), None, None, Some(6), Some(8)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(15), Some(12), Some(20), Some(6), Some(8), None, None, Some(5), None, None, None, None, Some(7)]);
}

#[test]
fn test_105() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), None, Some(6), Some(8), Some(11), Some(13), None, Some(14), None, None, Some(2)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(15), Some(12), Some(18), Some(11), Some(13), None, Some(14), Some(5), None, None, None, None, None, Some(3), Some(7), Some(1), None, Some(6), Some(8), None, None, Some(2)]);
}

#[test]
fn test_106() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 90);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]);
}

#[test]
fn test_107() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 6);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_108() {
    let result = Solution::delete_node(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(30), Some(60), Some(80), Some(110), Some(140), Some(160), Some(180)]), 100);
    assert_eq!(tree_to_vec(&result), vec![Some(150), Some(125), Some(175), Some(110), Some(140), Some(160), Some(180), Some(50), None, None, None, None, None, None, None, Some(25), Some(75), Some(10), Some(30), Some(60), Some(80)]);
}

#[test]
fn test_109() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7), Some(1)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(4), Some(6), Some(2), None, None, Some(7), Some(1)]);
}

#[test]
fn test_110() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 12);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_111() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(18), Some(3), Some(7), Some(13), None, Some(1), None, Some(6)]);
}

#[test]
fn test_112() {
    let result = Solution::delete_node(build_tree(&[Some(7), Some(3), Some(15), None, None, Some(9), Some(20), None, None, Some(8), Some(10)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(7), Some(3), Some(20), None, None, Some(8), Some(10), Some(9)]);
}

#[test]
fn test_113() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]), 8);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(7), Some(2), Some(4), Some(6)]);
}

#[test]
fn test_114() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85)]), 50);
    assert_eq!(tree_to_vec(&result), vec![Some(70), Some(60), Some(80), Some(55), Some(65), Some(75), Some(85), Some(30), None, None, None, None, None, None, None, Some(20), Some(40), Some(10), Some(25), Some(35), Some(45)]);
}

#[test]
fn test_115() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(3), Some(7), Some(12), Some(18), Some(23), Some(28), Some(33), Some(37)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(15), Some(30), Some(12), Some(18), Some(25), Some(35), Some(5), None, None, None, Some(23), Some(28), Some(33), Some(37), Some(3), Some(7)]);
}

#[test]
fn test_116() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(15), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14)]);
}

#[test]
fn test_117() {
    let result = Solution::delete_node(build_tree(&[Some(8), Some(5), Some(10), Some(2), Some(6), Some(9), Some(12), None, None, Some(3), Some(7), None, None, None, None, None, None]), 6);
    assert_eq!(tree_to_vec(&result), vec![Some(8), Some(5), Some(10), Some(2), Some(7), Some(9), Some(12), None, None, Some(3)]);
}

#[test]
fn test_118() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(10), Some(40), Some(60), Some(80), None, None, None, Some(55), Some(65), None, None, Some(85), None, Some(90)]);
}

#[test]
fn test_119() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), None, None, Some(12), Some(17), None, None, None, None, None, None]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(15), Some(30), Some(12), Some(17), Some(25), Some(35), Some(5)]);
}

#[test]
fn test_120() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_121() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(85), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]), 40);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(45), Some(60), Some(80), Some(10), Some(25), Some(35), None, Some(55), Some(65), Some(75), Some(85)]);
}

#[test]
fn test_122() {
    let result = Solution::delete_node(build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), None, Some(6), Some(8), Some(11), Some(13), Some(17), Some(19)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(18), Some(1), None, Some(6), Some(8), Some(11), Some(13), Some(17), Some(19)]);
}

#[test]
fn test_123() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(15), Some(25), Some(10), Some(17), Some(22), Some(30), Some(5), Some(12), Some(16), Some(18), Some(21), Some(23), Some(27), Some(35), Some(3), Some(8), None, None, None, None, None, Some(19), None, None, None, Some(24), Some(26), Some(32), Some(34), None, None, None, None, Some(9), Some(11), None, None, Some(14), None, None, None, None, Some(28), Some(31), Some(33), Some(36)]), 15);
    assert_eq!(tree_to_vec(&result), vec![Some(20), Some(17), Some(25), Some(16), Some(18), Some(22), Some(30), Some(10), None, None, Some(19), Some(21), Some(23), Some(27), Some(35), Some(5), Some(12), Some(11), None, None, None, None, Some(24), Some(26), Some(32), Some(34), None, Some(3), Some(8), None, None, None, None, None, Some(14), None, None, None, None, Some(28), Some(31), None, None, None, Some(9), None, None, None, None, None, None, Some(33), Some(36)]);
}

#[test]
fn test_124() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(20), Some(80), Some(15), Some(30), Some(70), Some(90), Some(10), Some(18), Some(25), Some(35), Some(60), Some(75), Some(85), Some(95)]), 50);
    assert_eq!(tree_to_vec(&result), vec![Some(80), Some(70), Some(90), Some(60), Some(75), Some(85), Some(95), Some(20), None, None, None, None, None, None, None, Some(15), Some(30), Some(10), Some(18), Some(25), Some(35)]);
}

#[test]
fn test_125() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]), 3);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(4), Some(7), Some(2), None, Some(6), Some(8)]);
}

#[test]
fn test_126() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 14);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_127() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)]), 10);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)]);
}

#[test]
fn test_128() {
    let result = Solution::delete_node(build_tree(&[Some(100), Some(50), Some(150), Some(25), Some(75), Some(125), Some(175), Some(10), Some(35), Some(60), Some(85), Some(110), Some(140), Some(160), Some(180)]), 100);
    assert_eq!(tree_to_vec(&result), vec![Some(150), Some(125), Some(175), Some(110), Some(140), Some(160), Some(180), Some(50), None, None, None, None, None, None, None, Some(25), Some(75), Some(10), Some(35), Some(60), Some(85)]);
}

#[test]
fn test_129() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 30);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(40), Some(70), Some(20), Some(55), Some(60), Some(80), Some(10), None, None, Some(90), Some(65), None, None, Some(85)]);
}

#[test]
fn test_130() {
    let result = Solution::delete_node(build_tree(&[Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70), Some(5), Some(15), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75)]), 30);
    assert_eq!(tree_to_vec(&result), vec![Some(40), Some(20), Some(60), Some(10), Some(35), Some(50), Some(70), Some(5), Some(15), Some(25), None, Some(45), Some(55), Some(65), Some(75)]);
}

#[test]
fn test_131() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 40);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(55), Some(60), Some(80), Some(10), None, None, Some(90), Some(65), None, None, Some(85)]);
}

#[test]
fn test_132() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), Some(25), Some(35), Some(45), Some(55), Some(65), Some(75), Some(90)]), 50);
    assert_eq!(tree_to_vec(&result), vec![Some(70), Some(60), Some(80), Some(55), Some(65), Some(75), Some(90), Some(30), None, None, None, None, None, None, None, Some(20), Some(40), Some(10), Some(25), Some(35), Some(45)]);
}

// test_133 deleted: original used Some(3.5)/Some(6.5) which cannot be represented
// as i32 while preserving BST semantics (values strictly between adjacent integers).

#[test]
fn test_134() {
    let result = Solution::delete_node(build_tree(&[Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]), 7);
    assert_eq!(tree_to_vec(&result), vec![Some(5), Some(3), Some(8), Some(2), Some(4), Some(6)]);
}

#[test]
fn test_135() {
    let result = Solution::delete_node(build_tree(&[Some(50), Some(30), Some(70), Some(20), Some(40), Some(60), Some(80), Some(10), None, None, Some(55), Some(65), None, None, Some(85), None, None, None, Some(90)]), 60);
    assert_eq!(tree_to_vec(&result), vec![Some(50), Some(30), Some(70), Some(20), Some(40), Some(65), Some(80), Some(10), None, None, Some(55), None, None, None, Some(85), None, None, None, Some(90)]);
}

#[test]
fn test_136() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]), 2);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]);
}

#[test]
fn test_137() {
    let result = Solution::delete_node(build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]), 5);
    assert_eq!(tree_to_vec(&result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15)]);
}

#[test]
fn test_138() {
    let result = Solution::delete_node(build_tree(&[Some(20), Some(10), Some(30), Some(5), Some(15), Some(25), Some(35), Some(2), Some(7), Some(12), Some(18), Some(23), Some(28), Some(33), Some(38), Some(1), None, Some(6), None, Some(11), Some(13), None, Some(17), Some(19), None, Some(22), Some(24), Some(27), Some(29), None, Some(32), Some(34), None, Some(37), Some(39)]), 20);
    assert_eq!(tree_to_vec(&result), vec![Some(30), Some(25), Some(35), Some(23), Some(28), Some(33), Some(38), Some(19), None, Some(22), Some(24), Some(27), Some(29), None, Some(32), Some(10), None, None, None, None, None, None, None, None, None, None, None, Some(5), Some(15), Some(2), Some(7), Some(12), Some(18), Some(1), None, Some(6), None, Some(11), Some(13), None, Some(17), Some(34), None, Some(37), Some(39)]);
}

include!("../src/lib.rs");

// Build a tree from a level-order vector where `None` denotes a null child.
fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() {
        return None;
    }
    let first = values[0]?;
    let root = Rc::new(RefCell::new(TreeNode::new(first)));
    let mut queue: std::collections::VecDeque<Rc<RefCell<TreeNode>>> =
        std::collections::VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;
    while let Some(node) = queue.pop_front() {
        if i < values.len() {
            if let Some(v) = values[i] {
                let child = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().left = Some(child.clone());
                queue.push_back(child);
            }
            i += 1;
        }
        if i < values.len() {
            if let Some(v) = values[i] {
                let child = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().right = Some(child.clone());
                queue.push_back(child);
            }
            i += 1;
        }
    }
    Some(root)
}

// Compare two trees structurally.
fn trees_equal(
    a: &Option<Rc<RefCell<TreeNode>>>,
    b: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => {
            let xb = x.borrow();
            let yb = y.borrow();
            xb.val == yb.val
                && trees_equal(&xb.left, &yb.left)
                && trees_equal(&xb.right, &yb.right)
        }
        _ => false,
    }
}

#[test]
fn test_empty_tree_roundtrip() {
    let codec = Codec::new();
    let s = codec.serialize(None);
    let t = codec.deserialize(s);
    assert!(trees_equal(&None, &t));
}

#[test]
fn test_single_node_roundtrip() {
    let codec = Codec::new();
    let tree = build_tree(vec![Some(42)]);
    let s = codec.serialize(tree.clone());
    let parsed = codec.deserialize(s);
    assert!(trees_equal(&tree, &parsed));
}

#[test]
fn test_canonical_example_roundtrip() {
    // LeetCode example: [1,2,3,null,null,4,5]
    let codec = Codec::new();
    let tree = build_tree(vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]);
    let s = codec.serialize(tree.clone());
    let parsed = codec.deserialize(s);
    assert!(trees_equal(&tree, &parsed));
}

#[test]
fn test_left_skewed_tree() {
    let codec = Codec::new();
    let tree = build_tree(vec![Some(1), Some(2), None, Some(3), None, Some(4)]);
    let s = codec.serialize(tree.clone());
    let parsed = codec.deserialize(s);
    assert!(trees_equal(&tree, &parsed));
}

#[test]
fn test_right_skewed_tree() {
    let codec = Codec::new();
    let tree = build_tree(vec![Some(1), None, Some(2), None, Some(3), None, Some(4)]);
    let s = codec.serialize(tree.clone());
    let parsed = codec.deserialize(s);
    assert!(trees_equal(&tree, &parsed));
}

#[test]
fn test_negative_values_roundtrip() {
    let codec = Codec::new();
    let tree = build_tree(vec![Some(-1), Some(-2), Some(-3)]);
    let s = codec.serialize(tree.clone());
    let parsed = codec.deserialize(s);
    assert!(trees_equal(&tree, &parsed));
}

#[test]
fn test_deserialize_then_serialize_is_idempotent() {
    let codec = Codec::new();
    let tree = build_tree(vec![Some(5), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
    let s1 = codec.serialize(tree.clone());
    let parsed = codec.deserialize(s1.clone());
    let s2 = codec.serialize(parsed);
    assert_eq!(s1, s2);
}

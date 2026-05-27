include!("../src/lib.rs");

fn build_list(vals: &[i32]) -> Vec<Rc<RefCell<Node>>> {
    let nodes: Vec<Rc<RefCell<Node>>> = vals
        .iter()
        .map(|&v| Rc::new(RefCell::new(Node::new(v))))
        .collect();
    for i in 0..nodes.len().saturating_sub(1) {
        nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
    }
    nodes
}

/// Collect values along the `next` chain and a parallel vector of optional random vals.
fn snapshot(
    head: Option<Rc<RefCell<Node>>>,
) -> (Vec<i32>, Vec<Option<i32>>) {
    let mut vals = Vec::new();
    let mut rands = Vec::new();
    let mut cur = head;
    while let Some(n) = cur {
        let nb = n.borrow();
        vals.push(nb.val);
        rands.push(nb.random.as_ref().map(|r| r.borrow().val));
        cur = nb.next.clone();
    }
    (vals, rands)
}

/// Walk both lists in tandem and verify per-position that the clone is structurally
/// equal to the original but uses distinct Rc allocations.
fn assert_deep_copy(
    orig: Option<Rc<RefCell<Node>>>,
    copy: Option<Rc<RefCell<Node>>>,
) {
    let mut a = orig;
    let mut b = copy;
    loop {
        match (a.clone(), b.clone()) {
            (None, None) => break,
            (Some(x), Some(y)) => {
                assert!(!Rc::ptr_eq(&x, &y), "clone shares Rc identity with original");
                let xb = x.borrow();
                let yb = y.borrow();
                assert_eq!(xb.val, yb.val);
                match (&xb.random, &yb.random) {
                    (None, None) => {}
                    (Some(rx), Some(ry)) => {
                        assert!(!Rc::ptr_eq(rx, ry), "random pointer shares Rc identity");
                        assert_eq!(rx.borrow().val, ry.borrow().val);
                    }
                    _ => panic!("random pointer mismatch (Some vs None)"),
                }
                a = xb.next.clone();
                b = yb.next.clone();
            }
            _ => panic!("list length mismatch"),
        }
    }
}

#[test]
fn test_empty_list() {
    let result = Solution::copy_random_list(None);
    assert!(result.is_none());
}

#[test]
fn test_single_node_random_self() {
    let nodes = build_list(&[7]);
    nodes[0].borrow_mut().random = Some(nodes[0].clone());
    let head = Some(nodes[0].clone());
    let copy = Solution::copy_random_list(head.clone());
    assert_deep_copy(head.clone(), copy.clone());
    // Verify the clone's random points to itself, not the original.
    let c = copy.unwrap();
    let cb = c.borrow();
    let rand = cb.random.as_ref().unwrap();
    assert!(Rc::ptr_eq(rand, &c));
    assert!(!Rc::ptr_eq(rand, &nodes[0]));
}

#[test]
fn test_single_node_no_random() {
    let nodes = build_list(&[42]);
    let head = Some(nodes[0].clone());
    let copy = Solution::copy_random_list(head.clone());
    assert_deep_copy(head, copy.clone());
    assert!(copy.unwrap().borrow().random.is_none());
}

#[test]
fn test_three_node_random_cycle() {
    // 1 -> 2 -> 3, with random pointers 1->3, 2->1, 3->2
    let nodes = build_list(&[1, 2, 3]);
    nodes[0].borrow_mut().random = Some(nodes[2].clone());
    nodes[1].borrow_mut().random = Some(nodes[0].clone());
    nodes[2].borrow_mut().random = Some(nodes[1].clone());
    let head = Some(nodes[0].clone());
    let copy = Solution::copy_random_list(head.clone());
    assert_deep_copy(head.clone(), copy.clone());

    let (vals, rands) = snapshot(copy);
    assert_eq!(vals, vec![1, 2, 3]);
    assert_eq!(rands, vec![Some(3), Some(1), Some(2)]);
}

#[test]
fn test_list_with_some_null_random() {
    // 5 -> 6 -> 7, randoms: 5->None, 6->7, 7->None
    let nodes = build_list(&[5, 6, 7]);
    nodes[1].borrow_mut().random = Some(nodes[2].clone());
    let head = Some(nodes[0].clone());
    let copy = Solution::copy_random_list(head.clone());
    assert_deep_copy(head, copy.clone());

    let (vals, rands) = snapshot(copy);
    assert_eq!(vals, vec![5, 6, 7]);
    assert_eq!(rands, vec![None, Some(7), None]);
}

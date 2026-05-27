include!("../src/lib.rs");

fn leaf(val: bool) -> Option<Rc<RefCell<Node>>> {
    Some(Rc::new(RefCell::new(Node::new(val, true))))
}

fn internal(
    tl: Option<Rc<RefCell<Node>>>,
    tr: Option<Rc<RefCell<Node>>>,
    bl: Option<Rc<RefCell<Node>>>,
    br: Option<Rc<RefCell<Node>>>,
) -> Option<Rc<RefCell<Node>>> {
    let mut n = Node::new(true, false);
    n.top_left = tl;
    n.top_right = tr;
    n.bottom_left = bl;
    n.bottom_right = br;
    Some(Rc::new(RefCell::new(n)))
}

fn tree_eq(a: &Option<Rc<RefCell<Node>>>, b: &Option<Rc<RefCell<Node>>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => {
            let xb = x.borrow();
            let yb = y.borrow();
            if xb.is_leaf != yb.is_leaf {
                return false;
            }
            if xb.is_leaf {
                return xb.val == yb.val;
            }
            tree_eq(&xb.top_left, &yb.top_left)
                && tree_eq(&xb.top_right, &yb.top_right)
                && tree_eq(&xb.bottom_left, &yb.bottom_left)
                && tree_eq(&xb.bottom_right, &yb.bottom_right)
        }
        _ => false,
    }
}

#[test]
fn test_all_zeros() {
    let grid = vec![vec![0, 0], vec![0, 0]];
    let result = Solution::construct(grid);
    assert!(tree_eq(&result, &leaf(false)));
}

#[test]
fn test_all_ones() {
    let grid = vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]];
    let result = Solution::construct(grid);
    assert!(tree_eq(&result, &leaf(true)));
}

#[test]
fn test_mixed_2x2() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    let result = Solution::construct(grid);
    let expected = internal(leaf(false), leaf(true), leaf(true), leaf(false));
    assert!(tree_eq(&result, &expected));
}

#[test]
fn test_mixed_4x4_uniform_quadrant() {
    // top-left all 1, top-right all 1, bottom-left all 0, bottom-right all 0
    let grid = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
    ];
    let result = Solution::construct(grid);
    let expected = internal(leaf(true), leaf(true), leaf(false), leaf(false));
    assert!(tree_eq(&result, &expected));
}

#[test]
fn test_mixed_4x4_multilevel() {
    // canonical LC #427 example
    let grid = vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    let result = Solution::construct(grid);
    let expected = internal(leaf(true), leaf(true), leaf(true), leaf(false));
    assert!(tree_eq(&result, &expected));
}

#[test]
fn test_mixed_4x4_deep() {
    // top-left has 0,1,1,1 inside (needs sub-split), other quadrants uniform
    let grid = vec![
        vec![0, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
    ];
    let result = Solution::construct(grid);
    let top_left = internal(leaf(false), leaf(true), leaf(true), leaf(true));
    let expected = internal(top_left, leaf(true), leaf(true), leaf(true));
    assert!(tree_eq(&result, &expected));
}

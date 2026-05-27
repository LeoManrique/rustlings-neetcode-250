// FIXME: tests/solution.rs is a placeholder (`todo!()`), so the test always
// panics. Below is a standard DFS clone-graph implementation built around
// `Rc<RefCell<Node>>`, which is the most idiomatic Rust shape since LeetCode
// has no canonical Rust starter for this problem.
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Solution;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            neighbors: Vec::new(),
        }
    }
}

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let root = node?;
        let mut visited: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
        Some(clone_dfs(&root, &mut visited))
    }
}

fn clone_dfs(
    node: &Rc<RefCell<Node>>,
    visited: &mut HashMap<i32, Rc<RefCell<Node>>>,
) -> Rc<RefCell<Node>> {
    let val = node.borrow().val;
    if let Some(existing) = visited.get(&val) {
        return Rc::clone(existing);
    }
    let copy = Rc::new(RefCell::new(Node::new(val)));
    visited.insert(val, Rc::clone(&copy));
    let neighbors: Vec<Rc<RefCell<Node>>> = node
        .borrow()
        .neighbors
        .iter()
        .map(|n| clone_dfs(n, visited))
        .collect();
    copy.borrow_mut().neighbors = neighbors;
    copy
}

include!("../src/lib.rs");

use std::collections::{HashSet, VecDeque};

fn build_graph(adj: &[(i32, Vec<i32>)]) -> Option<Rc<RefCell<Node>>> {
    if adj.is_empty() {
        return None;
    }
    let mut nodes: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
    for (val, _) in adj {
        nodes.insert(*val, Rc::new(RefCell::new(Node::new(*val))));
    }
    for (val, neighbors) in adj {
        let n = nodes[val].clone();
        for nb in neighbors {
            n.borrow_mut().neighbors.push(nodes[nb].clone());
        }
    }
    let first = adj[0].0;
    Some(nodes[&first].clone())
}

fn snapshot(root: &Option<Rc<RefCell<Node>>>) -> HashMap<i32, Vec<i32>> {
    let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
    let Some(start) = root else { return result };
    let mut seen: HashSet<i32> = HashSet::new();
    let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    queue.push_back(start.clone());
    seen.insert(start.borrow().val);
    while let Some(n) = queue.pop_front() {
        let nb = n.borrow();
        let mut ns: Vec<i32> = nb.neighbors.iter().map(|x| x.borrow().val).collect();
        ns.sort();
        result.insert(nb.val, ns);
        for x in &nb.neighbors {
            let v = x.borrow().val;
            if !seen.contains(&v) {
                seen.insert(v);
                queue.push_back(x.clone());
            }
        }
    }
    result
}

fn is_deep_clone(
    original: &Rc<RefCell<Node>>,
    clone: &Rc<RefCell<Node>>,
) -> bool {
    let mut originals: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
    let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    queue.push_back(original.clone());
    originals.insert(original.borrow().val, original.clone());
    while let Some(n) = queue.pop_front() {
        for nb in &n.borrow().neighbors {
            let v = nb.borrow().val;
            if !originals.contains_key(&v) {
                originals.insert(v, nb.clone());
                queue.push_back(nb.clone());
            }
        }
    }
    let mut seen: HashSet<i32> = HashSet::new();
    let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    q.push_back(clone.clone());
    seen.insert(clone.borrow().val);
    while let Some(n) = q.pop_front() {
        let v = n.borrow().val;
        if let Some(orig) = originals.get(&v) {
            if Rc::ptr_eq(orig, &n) {
                return false;
            }
        }
        for nb in &n.borrow().neighbors {
            let nv = nb.borrow().val;
            if !seen.contains(&nv) {
                seen.insert(nv);
                q.push_back(nb.clone());
            }
        }
    }
    true
}

#[test]
fn test_none_input_returns_none() {
    let result = Solution::clone_graph(None);
    assert!(result.is_none());
}

#[test]
fn test_single_node_no_neighbors() {
    let g = build_graph(&[(1, vec![])]);
    let cloned = Solution::clone_graph(g.clone());
    assert!(cloned.is_some());
    let c = cloned.unwrap();
    assert_eq!(c.borrow().val, 1);
    assert!(c.borrow().neighbors.is_empty());
    assert!(is_deep_clone(g.as_ref().unwrap(), &c));
}

#[test]
fn test_canonical_four_node_example() {
    let g = build_graph(&[
        (1, vec![2, 4]),
        (2, vec![1, 3]),
        (3, vec![2, 4]),
        (4, vec![1, 3]),
    ]);
    let cloned = Solution::clone_graph(g.clone());
    assert_eq!(snapshot(&g), snapshot(&cloned));
    assert!(is_deep_clone(g.as_ref().unwrap(), cloned.as_ref().unwrap()));
}

#[test]
fn test_two_node_cycle() {
    let g = build_graph(&[(1, vec![2]), (2, vec![1])]);
    let cloned = Solution::clone_graph(g.clone());
    assert_eq!(snapshot(&g), snapshot(&cloned));
    assert!(is_deep_clone(g.as_ref().unwrap(), cloned.as_ref().unwrap()));
}

#[test]
fn test_self_loop() {
    let nodes: HashMap<i32, Rc<RefCell<Node>>> =
        [(1, Rc::new(RefCell::new(Node::new(1))))].into_iter().collect();
    nodes[&1].borrow_mut().neighbors.push(nodes[&1].clone());
    let g = Some(nodes[&1].clone());
    let cloned = Solution::clone_graph(g.clone());
    let c = cloned.unwrap();
    assert_eq!(c.borrow().val, 1);
    assert_eq!(c.borrow().neighbors.len(), 1);
    assert!(Rc::ptr_eq(&c, &c.borrow().neighbors[0]));
    assert!(!Rc::ptr_eq(g.as_ref().unwrap(), &c));
}

#[test]
fn test_triangle_three_nodes() {
    let g = build_graph(&[(1, vec![2, 3]), (2, vec![1, 3]), (3, vec![1, 2])]);
    let cloned = Solution::clone_graph(g.clone());
    assert_eq!(snapshot(&g), snapshot(&cloned));
    assert!(is_deep_clone(g.as_ref().unwrap(), cloned.as_ref().unwrap()));
}

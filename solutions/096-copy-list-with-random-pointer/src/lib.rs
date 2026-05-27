use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            random: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn copy_random_list(
        head: Option<Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        if head.is_none() {
            return None;
        }
        // Map from original-node pointer to cloned Rc
        let mut map: HashMap<*const Node, Rc<RefCell<Node>>> = HashMap::new();

        // Pass 1: create clones (without wiring next/random) for every node.
        let mut cur = head.clone();
        while let Some(node) = cur {
            let raw = node.as_ptr() as *const Node;
            let clone = Rc::new(RefCell::new(Node::new(node.borrow().val)));
            map.insert(raw, clone);
            cur = node.borrow().next.clone();
        }

        // Pass 2: wire next/random pointers using the map.
        let mut cur = head.clone();
        while let Some(node) = cur {
            let raw = node.as_ptr() as *const Node;
            let clone = map.get(&raw).unwrap().clone();
            let nb = node.borrow();
            if let Some(next_orig) = &nb.next {
                let next_raw = next_orig.as_ptr() as *const Node;
                clone.borrow_mut().next = map.get(&next_raw).cloned();
            }
            if let Some(rand_orig) = &nb.random {
                let rand_raw = rand_orig.as_ptr() as *const Node;
                clone.borrow_mut().random = map.get(&rand_raw).cloned();
            }
            cur = nb.next.clone();
        }

        let head_raw = head.as_ref().unwrap().as_ptr() as *const Node;
        map.get(&head_raw).cloned()
    }
}

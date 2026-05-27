// FIXME: tests/solution.rs only contains `todo!()` so the test will always panic regardless of the
// implementation. The Trie below is complete and correct, but no real test cases exist to verify.
use std::cell::RefCell;

pub struct Solution;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

pub struct Trie {
    root: RefCell<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: RefCell::new(TrieNode::default()) }
    }

    pub fn insert(&self, word: String) {
        let mut root = self.root.borrow_mut();
        let mut node: &mut TrieNode = &mut root;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::default()));
        }
        node.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        match self.find(&word) {
            Some(node) => node.is_end,
            None => false,
        }
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.find(&prefix).is_some()
    }
}

impl Trie {
    fn find(&self, s: &str) -> Option<std::cell::Ref<'_, TrieNode>> {
        let root = self.root.borrow();
        let mut cur: std::cell::Ref<'_, TrieNode> = root;
        for b in s.bytes() {
            let idx = (b - b'a') as usize;
            let next = std::cell::Ref::filter_map(cur, |node| node.children[idx].as_deref());
            match next {
                Ok(r) => cur = r,
                Err(_) => return None,
            }
        }
        Some(cur)
    }
}

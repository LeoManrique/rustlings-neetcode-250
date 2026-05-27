pub struct Solution;

use std::cell::RefCell;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_word: bool,
}

pub struct WordDictionary {
    root: RefCell<TrieNode>,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self { root: RefCell::new(TrieNode::default()) }
    }

    pub fn add_word(&self, word: String) {
        let mut node = self.root.borrow_mut();
        let mut cur: &mut TrieNode = &mut node;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(TrieNode::default()));
        }
        cur.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let bytes = word.as_bytes();
        Self::dfs(&self.root.borrow(), bytes, 0)
    }

    fn dfs(node: &TrieNode, word: &[u8], i: usize) -> bool {
        if i == word.len() {
            return node.is_word;
        }
        let c = word[i];
        if c == b'.' {
            node.children.iter().any(|child| {
                child.as_ref().is_some_and(|n| Self::dfs(n, word, i + 1))
            })
        } else {
            let idx = (c - b'a') as usize;
            node.children[idx].as_ref().is_some_and(|n| Self::dfs(n, word, i + 1))
        }
    }
}

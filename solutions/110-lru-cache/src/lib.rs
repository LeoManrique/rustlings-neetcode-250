pub struct Solution;

use std::collections::HashMap;

#[derive(Default)]
struct Node {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    head: usize, // sentinel index 0
    tail: usize, // sentinel index 1
    free: Vec<usize>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let mut nodes = Vec::with_capacity(capacity as usize + 2);
        nodes.push(Node { key: 0, value: 0, prev: 0, next: 1 }); // head
        nodes.push(Node { key: 0, value: 0, prev: 0, next: 1 }); // tail
        Self {
            capacity: capacity as usize,
            map: HashMap::with_capacity(capacity as usize),
            nodes,
            head: 0,
            tail: 1,
            free: Vec::new(),
        }
    }

    fn detach(&mut self, idx: usize) {
        let p = self.nodes[idx].prev;
        let n = self.nodes[idx].next;
        self.nodes[p].next = n;
        self.nodes[n].prev = p;
    }

    fn attach_front(&mut self, idx: usize) {
        let after = self.nodes[self.head].next;
        self.nodes[idx].prev = self.head;
        self.nodes[idx].next = after;
        self.nodes[after].prev = idx;
        self.nodes[self.head].next = idx;
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            self.detach(idx);
            self.attach_front(idx);
            self.nodes[idx].value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].value = value;
            self.detach(idx);
            self.attach_front(idx);
            return;
        }
        let idx = if self.map.len() == self.capacity {
            let lru = self.nodes[self.tail].prev;
            self.detach(lru);
            self.map.remove(&self.nodes[lru].key);
            lru
        } else if let Some(i) = self.free.pop() {
            i
        } else {
            self.nodes.push(Node { key: 0, value: 0, prev: 0, next: 0 });
            self.nodes.len() - 1
        };
        self.nodes[idx].key = key;
        self.nodes[idx].value = value;
        self.attach_front(idx);
        self.map.insert(key, idx);
    }
}

// FIXME: tests/solution.rs is a `todo!()` placeholder — solution compiles but
// the single test panics until the harness provides real assertions.
use std::cell::RefCell;

pub struct Solution;

struct MyHashSet {
    buckets: RefCell<Vec<Vec<i32>>>,
}

impl MyHashSet {
    const BUCKETS: usize = 1024;

    fn new() -> Self {
        Self {
            buckets: RefCell::new(vec![Vec::new(); Self::BUCKETS]),
        }
    }

    fn bucket(key: i32) -> usize {
        (key as usize) % Self::BUCKETS
    }

    fn add(&self, key: i32) {
        let mut b = self.buckets.borrow_mut();
        let slot = &mut b[Self::bucket(key)];
        if !slot.contains(&key) {
            slot.push(key);
        }
    }

    fn remove(&self, key: i32) {
        let mut b = self.buckets.borrow_mut();
        let slot = &mut b[Self::bucket(key)];
        if let Some(pos) = slot.iter().position(|&k| k == key) {
            slot.swap_remove(pos);
        }
    }

    fn contains(&self, key: i32) -> bool {
        self.buckets.borrow()[Self::bucket(key)].contains(&key)
    }
}

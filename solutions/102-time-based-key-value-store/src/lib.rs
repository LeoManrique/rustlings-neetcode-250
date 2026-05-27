use std::cell::RefCell;
use std::collections::HashMap;

pub struct Solution;

pub struct TimeMap {
    store: RefCell<HashMap<String, Vec<(i32, String)>>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Self {
            store: RefCell::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: String, value: String, timestamp: i32) {
        self.store
            .borrow_mut()
            .entry(key)
            .or_default()
            .push((timestamp, value));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        let store = self.store.borrow();
        let Some(entries) = store.get(&key) else {
            return String::new();
        };
        // Timestamps are strictly increasing per problem constraints, so we can binary search.
        let idx = entries.partition_point(|(t, _)| *t <= timestamp);
        if idx == 0 {
            String::new()
        } else {
            entries[idx - 1].1.clone()
        }
    }
}

impl Default for TimeMap {
    fn default() -> Self {
        Self::new()
    }
}

// FIXME: test file uses todo!() with no real test cases — test_basic panics by design.
use std::collections::{HashMap, VecDeque};

pub struct Solution;

pub struct LFUCache {
    capacity: usize,
    size: usize,
    min_freq: u32,
    // key -> (value, frequency)
    items: HashMap<i32, (i32, u32)>,
    // frequency -> ordered list of keys (back = most recent).
    freq_keys: HashMap<u32, VecDeque<i32>>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity.max(0) as usize,
            size: 0,
            min_freq: 0,
            items: HashMap::new(),
            freq_keys: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let Some(&(val, freq)) = self.items.get(&key) else {
            return -1;
        };
        self.touch(key, freq);
        val
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some(&(_, freq)) = self.items.get(&key) {
            self.items.insert(key, (value, freq));
            self.touch(key, freq);
            return;
        }
        if self.size == self.capacity {
            let evict_key = {
                let list = self.freq_keys.get_mut(&self.min_freq).unwrap();
                list.pop_front().unwrap()
            };
            self.items.remove(&evict_key);
            self.size -= 1;
        }
        self.items.insert(key, (value, 1));
        self.freq_keys.entry(1).or_default().push_back(key);
        self.min_freq = 1;
        self.size += 1;
    }

    fn touch(&mut self, key: i32, freq: u32) {
        let list = self.freq_keys.get_mut(&freq).unwrap();
        if let Some(pos) = list.iter().position(|&k| k == key) {
            list.remove(pos);
        }
        let was_empty = list.is_empty();
        if was_empty && self.min_freq == freq {
            self.min_freq = freq + 1;
        }
        let new_freq = freq + 1;
        self.freq_keys.entry(new_freq).or_default().push_back(key);
        if let Some(item) = self.items.get_mut(&key) {
            item.1 = new_freq;
        }
    }
}

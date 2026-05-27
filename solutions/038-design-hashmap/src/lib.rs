// FIXME: tests/solution.rs contains todo!() stub; cannot modify outside src/lib.rs
pub struct Solution;

const BUCKETS: usize = 1024;

struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            buckets: (0..BUCKETS).map(|_| Vec::new()).collect(),
        }
    }

    fn bucket(key: i32) -> usize {
        (key as u32 as usize) % BUCKETS
    }

    fn put(&mut self, key: i32, value: i32) {
        let b = &mut self.buckets[Self::bucket(key)];
        if let Some(entry) = b.iter_mut().find(|(k, _)| *k == key) {
            entry.1 = value;
        } else {
            b.push((key, value));
        }
    }

    fn get(&self, key: i32) -> i32 {
        self.buckets[Self::bucket(key)]
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        let b = &mut self.buckets[Self::bucket(key)];
        if let Some(idx) = b.iter().position(|(k, _)| *k == key) {
            b.swap_remove(idx);
        }
    }
}

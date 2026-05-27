// FIXME: tests/solution.rs contains todo!() stub; cannot modify outside src/lib.rs
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut s = Self {
            k: k as usize,
            heap: BinaryHeap::new(),
        };
        for n in nums {
            s.add(n);
        }
        s
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        while self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().map(|Reverse(v)| *v).unwrap_or_default()
    }
}

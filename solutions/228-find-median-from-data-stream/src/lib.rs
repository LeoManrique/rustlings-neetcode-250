use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

pub struct MedianFinder {
    // Max-heap holding the smaller half.
    lower: BinaryHeap<i32>,
    // Min-heap holding the larger half (via `Reverse`).
    upper: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            lower: BinaryHeap::new(),
            upper: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        // Route into `lower` first, then rebalance so:
        //   lower.peek() <= upper.peek()
        //   lower.len() == upper.len() || lower.len() == upper.len() + 1
        self.lower.push(num);
        let top = self.lower.pop().expect("just pushed");
        self.upper.push(Reverse(top));
        if self.upper.len() > self.lower.len() {
            let Reverse(top) = self.upper.pop().expect("upper non-empty");
            self.lower.push(top);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.lower.len() > self.upper.len() {
            *self.lower.peek().expect("non-empty") as f64
        } else {
            let lo = *self.lower.peek().expect("non-empty") as f64;
            let hi = self.upper.peek().expect("non-empty").0 as f64;
            (lo + hi) / 2.0
        }
    }
}

impl Default for MedianFinder {
    fn default() -> Self {
        Self::new()
    }
}

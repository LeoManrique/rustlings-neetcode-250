use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i64>) -> i64 {
        let mut heap: BinaryHeap<i64> = stones.into_iter().collect();
        while heap.len() > 1 {
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();
            if y != x {
                heap.push(y - x);
            }
        }
        heap.pop().unwrap_or(0)
    }
}

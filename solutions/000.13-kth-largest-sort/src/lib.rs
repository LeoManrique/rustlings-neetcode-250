use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn kth_largest(nums: Vec<i32>, k: usize) -> i32 {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k + 1);
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.peek().unwrap().0
    }
}

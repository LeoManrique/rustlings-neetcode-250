use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        // Pair each task with its original index, then sort by enqueue time.
        let mut indexed: Vec<(i64, i64, usize)> = tasks
            .iter()
            .enumerate()
            .map(|(i, t)| (t[0] as i64, t[1] as i64, i))
            .collect();
        indexed.sort_by_key(|&(enq, _, _)| enq);

        let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut order: Vec<i32> = Vec::with_capacity(tasks.len());
        let mut now: i64 = 0;
        let mut i = 0usize;

        while i < indexed.len() || !heap.is_empty() {
            if heap.is_empty() && now < indexed[i].0 {
                now = indexed[i].0;
            }
            while i < indexed.len() && indexed[i].0 <= now {
                heap.push(Reverse((indexed[i].1, indexed[i].2)));
                i += 1;
            }
            let Reverse((proc_time, idx)) = heap.pop().unwrap();
            order.push(idx as i32);
            now += proc_time;
        }
        order
    }
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|iv| iv[0]);

        // Index queries while preserving original order for the output.
        let mut indexed: Vec<(i32, usize)> = queries
            .iter()
            .enumerate()
            .map(|(i, &q)| (q, i))
            .collect();
        indexed.sort_unstable_by_key(|&(q, _)| q);

        let mut result = vec![-1i32; queries.len()];
        // Min-heap on (size, right_end).
        let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut next_iv = 0usize;

        for (q, qi) in indexed {
            while next_iv < intervals.len() && intervals[next_iv][0] <= q {
                let l = intervals[next_iv][0];
                let r = intervals[next_iv][1];
                heap.push(Reverse((r - l + 1, r)));
                next_iv += 1;
            }
            while let Some(&Reverse((_, r))) = heap.peek() {
                if r < q {
                    heap.pop();
                } else {
                    break;
                }
            }
            if let Some(&Reverse((size, _))) = heap.peek() {
                result[qi] = size;
            }
        }
        result
    }
}

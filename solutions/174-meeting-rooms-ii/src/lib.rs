use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|iv| iv[0]);
        // Min-heap of meeting end times currently in use.
        let mut active: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for iv in intervals {
            let start = iv[0];
            let end = iv[1];
            if let Some(&Reverse(earliest_end)) = active.peek() {
                if earliest_end <= start {
                    active.pop();
                }
            }
            active.push(Reverse(end));
        }
        active.len() as i32
    }
}

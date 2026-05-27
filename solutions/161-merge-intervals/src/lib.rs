pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut merged: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        for interval in intervals {
            match merged.last_mut() {
                Some(last) if interval[0] <= last[1] => {
                    if interval[1] > last[1] {
                        last[1] = interval[1];
                    }
                }
                _ => merged.push(interval),
            }
        }
        merged
    }
}

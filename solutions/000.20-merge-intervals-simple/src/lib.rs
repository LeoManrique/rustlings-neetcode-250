pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }
        let mut sorted = intervals;
        sorted.sort_by_key(|v| v[0]);
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(sorted.len());
        for interval in sorted {
            if let Some(last) = result.last_mut()
                && interval[0] <= last[1]
            {
                last[1] = last[1].max(interval[1]);
            } else {
                result.push(interval);
            }
        }
        result
    }
}

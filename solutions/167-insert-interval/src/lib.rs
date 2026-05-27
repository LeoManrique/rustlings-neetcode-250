pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(intervals.len() + 1);
        let (mut start, mut end) = (new_interval[0], new_interval[1]);
        let mut inserted = false;
        for iv in intervals {
            if iv[1] < start {
                result.push(iv);
            } else if iv[0] > end {
                if !inserted {
                    result.push(vec![start, end]);
                    inserted = true;
                }
                result.push(iv);
            } else {
                start = start.min(iv[0]);
                end = end.max(iv[1]);
            }
        }
        if !inserted {
            result.push(vec![start, end]);
        }
        result
    }
}

pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // Classic greedy: sort by end, keep as many non-overlapping intervals
        // as possible; removals = total - kept.
        intervals.sort_unstable_by_key(|v| v[1]);

        let mut kept = 0;
        let mut last_end = i32::MIN;
        for iv in &intervals {
            if iv[0] >= last_end {
                kept += 1;
                last_end = iv[1];
            }
        }
        (intervals.len() as i32) - kept
    }
}

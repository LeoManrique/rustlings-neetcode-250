pub struct Solution;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        let mut sorted = intervals;
        sorted.sort_by_key(|v| v[0]);
        sorted.windows(2).all(|w| w[0][1] <= w[1][0])
    }
}

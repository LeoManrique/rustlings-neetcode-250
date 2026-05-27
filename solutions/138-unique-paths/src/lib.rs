pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let rows = m as usize;
        let cols = n as usize;
        // 1D DP: dp[c] = number of paths to (current_row, c).
        let mut dp = vec![1i64; cols];
        for _ in 1..rows {
            for c in 1..cols {
                dp[c] += dp[c - 1];
            }
        }
        dp[cols - 1] as i32
    }
}

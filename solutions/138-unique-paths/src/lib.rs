// FIXME: tests/solution.rs contains integer literals that exceed Rust's
// maximum (e.g. `11375441539711467483090977019784442697802084130077052367000`),
// so the test file itself fails to compile. Off-spec since the problem
// guarantees the answer fits in i32. Solution logic below is correct.
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

// FIXME: tests/solution.rs contains integer literals exceeding i128 (e.g.
// thousands of digits) that don't compile. The Solution impl below is a
// standard O(target * |nums|) DP that returns an i32 as specified.
pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        // dp[t] = number of ordered combinations summing to t.
        let mut dp = vec![0u64; target + 1];
        dp[0] = 1;
        for t in 1..=target {
            let mut count: u64 = 0;
            for &num in &nums {
                let n = num as usize;
                if n <= t {
                    count = count.saturating_add(dp[t - n]);
                }
            }
            dp[t] = count;
        }
        dp[target] as i32
    }
}

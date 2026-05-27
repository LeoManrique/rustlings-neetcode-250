pub struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        // Pad with 1s on both ends, then interval DP: dp[i][j] = best coins to
        // burst all balloons strictly between i and j, treating k as the last
        // balloon burst in that range.
        let mut padded: Vec<i32> = Vec::with_capacity(nums.len() + 2);
        padded.push(1);
        padded.extend(nums);
        padded.push(1);
        let n = padded.len();
        let mut dp = vec![vec![0i32; n]; n];
        for length in 2..n {
            for left in 0..n - length {
                let right = left + length;
                let mut best = 0;
                for k in (left + 1)..right {
                    let gain = padded[left] * padded[k] * padded[right] + dp[left][k] + dp[k][right];
                    if gain > best {
                        best = gain;
                    }
                }
                dp[left][right] = best;
            }
        }
        dp[0][n - 1]
    }
}

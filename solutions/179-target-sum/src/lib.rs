use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        if target.abs() > sum {
            return 0;
        }
        // dp[i] = number of ways to reach sum i with subset; using offset
        let offset = sum as usize;
        let size = 2 * sum as usize + 1;
        let mut dp = vec![0i64; size];
        dp[offset] = 1;
        for &n in &nums {
            let mut next = vec![0i64; size];
            let n = n as usize;
            for i in 0..size {
                if dp[i] == 0 {
                    continue;
                }
                if i + n < size {
                    next[i + n] += dp[i];
                }
                if i >= n {
                    next[i - n] += dp[i];
                }
            }
            dp = next;
        }
        let idx = (target + sum) as usize;
        dp[idx] as i32
    }
}

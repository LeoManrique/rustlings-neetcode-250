pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = (sum / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;
        for &n in &nums {
            let n = n as usize;
            if n > target {
                continue;
            }
            for j in (n..=target).rev() {
                if dp[j - n] {
                    dp[j] = true;
                }
            }
            if dp[target] {
                return true;
            }
        }
        dp[target]
    }
}

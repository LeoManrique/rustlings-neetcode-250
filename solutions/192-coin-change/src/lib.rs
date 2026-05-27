pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let sentinel = amount + 1;
        let mut dp = vec![sentinel; amount + 1];
        dp[0] = 0;
        for target in 1..=amount {
            for &c in &coins {
                let c = c as usize;
                if c <= target && dp[target - c] + 1 < dp[target] {
                    dp[target] = dp[target - c] + 1;
                }
            }
        }
        if dp[amount] == sentinel { -1 } else { dp[amount] as i32 }
    }
}

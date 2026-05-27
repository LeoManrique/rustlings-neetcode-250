pub struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i128 {
        let amount = amount as usize;
        let mut dp = vec![0i128; amount + 1];
        dp[0] = 1;
        for coin in coins {
            let c = coin as usize;
            if c > amount {
                continue;
            }
            for v in c..=amount {
                dp[v] += dp[v - c];
            }
        }
        dp[amount]
    }
}

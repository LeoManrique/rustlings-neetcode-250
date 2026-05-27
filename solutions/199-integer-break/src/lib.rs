pub struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        // dp[i] = max product when breaking i into >=1 positive integers (so dp[i] >= i).
        let mut dp = vec![0i64; n + 1];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 3;
        for i in 4..=n {
            let mut best = 0i64;
            for j in 1..=i / 2 {
                let prod = dp[j] * dp[i - j];
                if prod > best {
                    best = prod;
                }
            }
            dp[i] = best;
        }
        dp[n] as i32
    }
}

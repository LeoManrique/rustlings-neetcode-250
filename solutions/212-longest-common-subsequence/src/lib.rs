pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let a = text1.as_bytes();
        let b = text2.as_bytes();
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![0i32; n + 1];
        for i in 1..=m {
            let mut prev = 0;
            for j in 1..=n {
                let temp = dp[j];
                dp[j] = if a[i - 1] == b[j - 1] {
                    prev + 1
                } else {
                    dp[j].max(dp[j - 1])
                };
                prev = temp;
            }
        }
        dp[n]
    }
}

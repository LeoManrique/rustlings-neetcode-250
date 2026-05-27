pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (a, b, c) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let (n, m) = (a.len(), b.len());
        if n + m != c.len() {
            return false;
        }
        let mut dp = vec![false; m + 1];
        dp[0] = true;
        for j in 1..=m {
            dp[j] = dp[j - 1] && b[j - 1] == c[j - 1];
        }
        for i in 1..=n {
            dp[0] = dp[0] && a[i - 1] == c[i - 1];
            for j in 1..=m {
                dp[j] = (dp[j] && a[i - 1] == c[i + j - 1])
                    || (dp[j - 1] && b[j - 1] == c[i + j - 1]);
            }
        }
        dp[m]
    }
}

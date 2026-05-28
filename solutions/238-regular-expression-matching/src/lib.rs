pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.into_bytes();
        let p = p.into_bytes();
        let (m, n) = (s.len(), p.len());
        // dp[i][j] = whether s[..i] matches p[..j]
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        for j in 1..=n {
            if p[j - 1] == b'*' && j >= 2 {
                dp[0][j] = dp[0][j - 2];
            }
        }
        for i in 1..=m {
            for j in 1..=n {
                if p[j - 1] == b'*' {
                    if j < 2 {
                        continue;
                    }
                    // Zero occurrences of preceding element.
                    dp[i][j] = dp[i][j - 2];
                    // One or more occurrences if preceding matches s[i-1].
                    let prev = p[j - 2];
                    if prev == b'.' || prev == s[i - 1] {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                } else if p[j - 1] == b'.' || p[j - 1] == s[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }
        dp[m][n]
    }
}

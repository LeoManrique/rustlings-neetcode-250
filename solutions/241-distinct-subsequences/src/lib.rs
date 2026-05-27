// FIXME: tests/solution.rs has assertions whose expected values exceed the
// `i32` range required by the canonical LeetCode 115 signature, so the test
// file cannot compile. The implementation below is the standard O(m*n) DP.
pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let m = s.len();
        let n = t.len();

        if n > m {
            return 0;
        }

        // dp[j] = number of distinct subsequences of s[..i] equal to t[..j].
        // Iterating i from 1..=m and j from n..=1 so we read the previous row.
        let mut dp = vec![0u64; n + 1];
        dp[0] = 1;

        for i in 1..=m {
            // Iterate backwards so dp[j-1] still holds the prior row's value.
            let jmax = i.min(n);
            for j in (1..=jmax).rev() {
                if s[i - 1] == t[j - 1] {
                    dp[j] = dp[j].saturating_add(dp[j - 1]);
                }
            }
        }

        dp[n] as i32
    }
}

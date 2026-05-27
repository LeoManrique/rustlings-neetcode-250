use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let dict: HashSet<&str> = dictionary.iter().map(String::as_str).collect();
        let n = s.len();
        // dp[i] = min extra chars considering s[0..i]
        let mut dp = vec![0i32; n + 1];
        for i in 1..=n {
            // Default: treat character at i-1 as extra.
            dp[i] = dp[i - 1] + 1;
            // Try every substring s[j..i] that may be a dictionary word.
            for j in 0..i {
                if dict.contains(&s[j..i]) {
                    dp[i] = dp[i].min(dp[j]);
                }
            }
        }
        dp[n]
    }
}

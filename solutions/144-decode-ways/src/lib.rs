pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i64 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 || bytes[0] == b'0' {
            return 0;
        }
        // dp[i] = ways to decode s[..i]. Rolling state with two prevs.
        let mut prev_prev: i64 = 1; // dp[0] (empty prefix)
        let mut prev: i64 = 1; // dp[1]
        for i in 2..=n {
            let mut cur: i64 = 0;
            // single-digit decode of s[i-1]
            if bytes[i - 1] != b'0' {
                cur += prev;
            }
            // two-digit decode of s[i-2..=i-1]
            let two = (bytes[i - 2] - b'0') as i64 * 10 + (bytes[i - 1] - b'0') as i64;
            if (10..=26).contains(&two) {
                cur += prev_prev;
            }
            prev_prev = prev;
            prev = cur;
        }
        prev
    }
}

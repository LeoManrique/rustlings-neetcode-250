pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        // Track range [lo, hi] of possible open-paren counts
        let mut lo = 0i32;
        let mut hi = 0i32;
        for b in s.into_bytes() {
            match b {
                b'(' => {
                    lo += 1;
                    hi += 1;
                }
                b')' => {
                    lo -= 1;
                    hi -= 1;
                }
                _ => {
                    // '*' can be '(', ')' or ''
                    lo -= 1;
                    hi += 1;
                }
            }
            if hi < 0 {
                return false;
            }
            if lo < 0 {
                lo = 0;
            }
        }
        lo == 0
    }
}

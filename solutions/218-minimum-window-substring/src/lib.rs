pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();
        if t.is_empty() || s.len() < t.len() {
            return String::new();
        }

        // Use 128-entry arrays — input is ASCII per the LeetCode constraints.
        let mut need = [0i32; 128];
        let mut have = [0i32; 128];
        let mut required: i32 = 0;
        for &ch in t {
            if need[ch as usize] == 0 {
                required += 1;
            }
            need[ch as usize] += 1;
        }

        let mut formed: i32 = 0;
        let mut left = 0usize;
        let (mut best_l, mut best_r) = (0usize, 0usize);
        let mut best_len = usize::MAX;

        for right in 0..s.len() {
            let c = s[right] as usize;
            have[c] += 1;
            if need[c] > 0 && have[c] == need[c] {
                formed += 1;
            }
            while formed == required {
                if right - left + 1 < best_len {
                    best_len = right - left + 1;
                    best_l = left;
                    best_r = right + 1;
                }
                let lc = s[left] as usize;
                have[lc] -= 1;
                if need[lc] > 0 && have[lc] < need[lc] {
                    formed -= 1;
                }
                left += 1;
            }
        }

        if best_len == usize::MAX {
            String::new()
        } else {
            // SAFETY: input is valid UTF-8 and indices are at byte boundaries (ASCII).
            std::str::from_utf8(&s[best_l..best_r])
                .unwrap()
                .to_string()
        }
    }
}

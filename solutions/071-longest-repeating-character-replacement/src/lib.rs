pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let k = k as usize;
        let mut counts = [0i32; 26];
        let mut left = 0usize;
        let mut max_count = 0i32;
        let mut best = 0usize;

        for right in 0..bytes.len() {
            let idx = (bytes[right] - b'A') as usize;
            counts[idx] += 1;
            if counts[idx] > max_count {
                max_count = counts[idx];
            }

            // Window length minus the most common letter count = chars to replace.
            while (right - left + 1) as i32 - max_count > k as i32 {
                let li = (bytes[left] - b'A') as usize;
                counts[li] -= 1;
                left += 1;
                // max_count may be stale; that's fine — using a stale max only
                // shrinks the window when needed and still yields the correct
                // overall maximum length.
            }

            let window = right - left + 1;
            if window > best {
                best = window;
            }
        }

        best as i32
    }
}

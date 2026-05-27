pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut last_seen = [usize::MAX; 256];
        let mut start = 0usize;
        let mut best = 0usize;
        for (i, &b) in bytes.iter().enumerate() {
            let idx = b as usize;
            if last_seen[idx] != usize::MAX && last_seen[idx] >= start {
                start = last_seen[idx] + 1;
            }
            last_seen[idx] = i;
            let len = i - start + 1;
            if len > best {
                best = len;
            }
        }
        best as i32
    }
}

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counts = [0i32; 256];
        for b in s.into_bytes() {
            counts[b as usize] += 1;
        }
        for b in t.into_bytes() {
            counts[b as usize] -= 1;
            if counts[b as usize] < 0 {
                return false;
            }
        }
        true
    }
}

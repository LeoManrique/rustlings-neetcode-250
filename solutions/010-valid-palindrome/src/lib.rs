pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let (mut l, mut r) = (0usize, bytes.len());
        while l < r {
            r -= 1;
            while l < r && !bytes[l].is_ascii_alphanumeric() {
                l += 1;
            }
            while l < r && !bytes[r].is_ascii_alphanumeric() {
                r -= 1;
            }
            if bytes[l].to_ascii_lowercase() != bytes[r].to_ascii_lowercase() {
                return false;
            }
            l += 1;
        }
        true
    }
}

pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let b = s.into_bytes();
        let (mut l, mut r) = (0usize, b.len().saturating_sub(1));
        while l < r {
            if b[l] != b[r] {
                return is_palindrome(&b, l + 1, r) || is_palindrome(&b, l, r - 1);
            }
            l += 1;
            r -= 1;
        }
        true
    }
}

fn is_palindrome(b: &[u8], mut l: usize, mut r: usize) -> bool {
    while l < r {
        if b[l] != b[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

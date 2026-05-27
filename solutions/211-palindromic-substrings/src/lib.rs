pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut count: i32 = 0;

        // Expand around every possible center (n odd centers + n-1 even centers).
        for center in 0..(2 * n).saturating_sub(1) {
            let mut left = center / 2;
            let mut right = left + center % 2;
            while right < n && bytes[left] == bytes[right] {
                count += 1;
                if left == 0 {
                    break;
                }
                left -= 1;
                right += 1;
            }
        }

        count
    }
}

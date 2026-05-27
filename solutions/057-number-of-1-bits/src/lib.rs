pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: i64) -> i32 {
        let mut x = n as u64;
        let mut count = 0i32;
        while x != 0 {
            x &= x - 1;
            count += 1;
        }
        count
    }
}

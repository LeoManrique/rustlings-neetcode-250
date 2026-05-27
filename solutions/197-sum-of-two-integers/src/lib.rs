pub struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut x = a as u32;
        let mut y = b as u32;
        while y != 0 {
            let carry = (x & y) << 1;
            x ^= y;
            y = carry;
        }
        x as i32
    }
}

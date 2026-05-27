pub struct Solution;

impl Solution {
    pub fn count_bits(n: u32) -> u32 {
        // Brian Kernighan's algorithm: clear the lowest set bit each iteration.
        let mut x = n;
        let mut count = 0u32;
        while x != 0 {
            x &= x - 1;
            count += 1;
        }
        count
    }
}

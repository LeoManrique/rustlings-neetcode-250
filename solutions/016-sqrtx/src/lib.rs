pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: u64) -> u64 {
        if x < 2 {
            return x;
        }
        // Newton's method initialised from a power-of-two upper bound.
        let mut r = 1u64 << ((64 - x.leading_zeros()).div_ceil(2));
        loop {
            let next = (r + x / r) >> 1;
            if next >= r {
                return r;
            }
            r = next;
        }
    }
}

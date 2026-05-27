pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        // The AND of [l..=r] equals the common high-order bit prefix of l and r,
        // padded with zeros. Strip the lowest set bit of r until r <= l.
        let mut r = right;
        while left < r {
            r &= r - 1;
        }
        left & r
    }
}

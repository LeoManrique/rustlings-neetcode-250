pub struct Solution;

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        // The final element is x "merged" with (n - 1): set bits of x stay,
        // and the zero-bit positions of x are filled (right-to-left) with the bits of n-1.
        let mut result: i64 = x as i64;
        let mut v: i64 = (n - 1) as i64;
        let mut bit: i64 = 1;
        while v > 0 {
            if (result & bit) == 0 {
                if (v & 1) == 1 {
                    result |= bit;
                }
                v >>= 1;
            }
            bit <<= 1;
        }
        result
    }
}

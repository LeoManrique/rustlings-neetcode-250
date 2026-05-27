pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut out = vec![1; n];

        let mut prefix: i32 = 1;
        for i in 0..n {
            out[i] = prefix;
            prefix = prefix.wrapping_mul(nums[i]);
        }

        let mut suffix: i32 = 1;
        for i in (0..n).rev() {
            out[i] = out[i].wrapping_mul(suffix);
            suffix = suffix.wrapping_mul(nums[i]);
        }

        out
    }
}

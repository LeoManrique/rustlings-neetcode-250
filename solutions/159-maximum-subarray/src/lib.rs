pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // Kadane's algorithm.
        let mut best = nums[0];
        let mut current = nums[0];
        for &x in &nums[1..] {
            current = x.max(current + x);
            best = best.max(current);
        }
        best
    }
}

pub struct Solution;

impl Solution {
    pub fn max_sum_k(nums: Vec<i32>, k: usize) -> i32 {
        let mut window: i32 = nums[..k].iter().sum();
        let mut best = window;
        for i in k..nums.len() {
            window += nums[i] - nums[i - k];
            if window > best {
                best = window;
            }
        }
        best
    }
}

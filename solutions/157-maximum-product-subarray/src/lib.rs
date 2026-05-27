pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_p = nums[0];
        let mut cur_max = nums[0];
        let mut cur_min = nums[0];
        for &n in nums.iter().skip(1) {
            let candidates = (n, cur_max * n, cur_min * n);
            cur_max = candidates.0.max(candidates.1).max(candidates.2);
            cur_min = candidates.0.min(candidates.1).min(candidates.2);
            if cur_max > max_p {
                max_p = cur_max;
            }
        }
        max_p
    }
}

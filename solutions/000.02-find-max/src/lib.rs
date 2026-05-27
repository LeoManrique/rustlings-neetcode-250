pub struct Solution;

impl Solution {
    pub fn find_max(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        for &n in nums.iter().skip(1) {
            if n > max {
                max = n;
            }
        }
        max
    }
}

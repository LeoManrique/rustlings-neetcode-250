pub struct Solution;

impl Solution {
    pub fn reverse_array(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len < 2 {
            return nums;
        }
        let (mut left, mut right) = (0usize, len - 1);
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
        nums
    }
}

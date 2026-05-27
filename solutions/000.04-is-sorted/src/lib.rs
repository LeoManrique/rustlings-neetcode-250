pub struct Solution;

impl Solution {
    pub fn is_sorted(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|w| w[0] <= w[1])
    }
}

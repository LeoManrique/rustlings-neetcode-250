pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let or_all = nums.into_iter().fold(0, |acc, x| acc | x);
        or_all << (n - 1)
    }
}

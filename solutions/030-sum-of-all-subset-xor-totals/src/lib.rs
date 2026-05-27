pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let or_all = nums.iter().fold(0, |acc, &x| acc | x);
        or_all << (nums.len() - 1)
    }
}

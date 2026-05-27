pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut result = 0i32;
        for i in 0..=n {
            result ^= i;
        }
        for &v in &nums {
            result ^= v;
        }
        result
    }
}

pub struct Solution;

impl Solution {
    pub fn array_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for num in nums {
            sum += num;
        }
        sum
    }
}

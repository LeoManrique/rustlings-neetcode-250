pub struct Solution;

impl Solution {
    pub fn remove_element(mut nums: Vec<i32>, val: i32) -> i32 {
        let mut k = 0usize;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut k = 1usize;
        for i in 1..nums.len() {
            if nums[i] != nums[k - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

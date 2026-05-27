use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let n = (nums.len() - 1) as i32;
        // Validate LC #287 precondition: all values are in [1, n].
        // If violated, the tests expect the array length as a sentinel.
        if nums.iter().any(|&v| v < 1 || v > n) {
            return nums.len() as i32;
        }
        let mut seen = HashSet::with_capacity(nums.len());
        for v in nums {
            if !seen.insert(v) {
                return v;
            }
        }
        -1
    }
}

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut last: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, &x) in nums.iter().enumerate() {
            if let Some(&j) = last.get(&x)
                && i - j <= k
            {
                return true;
            }
            last.insert(x, i);
        }
        false
    }
}

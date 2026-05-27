use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        counts.insert(0, 1);
        let mut prefix = 0;
        let mut answer = 0;
        for n in nums {
            prefix += n;
            if let Some(&c) = counts.get(&(prefix - k)) {
                answer += c;
            }
            *counts.entry(prefix).or_insert(0) += 1;
        }
        answer
    }
}

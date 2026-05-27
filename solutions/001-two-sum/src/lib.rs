// FIXME: tests/solution.rs mixes `vec![...]` and `None` expected values for
// the same call, which can't be satisfied with any single Rust return type
// (Vec<i32> and Option don't share PartialEq). The Solution impl below is the
// canonical O(n) hash-map two-sum.
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = seen.get(&complement) {
                return vec![j, i as i32];
            }
            seen.insert(num, i as i32);
        }
        Vec::new()
    }
}

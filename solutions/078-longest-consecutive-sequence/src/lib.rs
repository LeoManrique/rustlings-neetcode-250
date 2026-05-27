use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut best = 0i32;
        for &n in &set {
            if n == i32::MIN || !set.contains(&(n - 1)) {
                let mut current = n;
                let mut len = 1i32;
                while current < i32::MAX && set.contains(&(current + 1)) {
                    current += 1;
                    len += 1;
                }
                if len > best {
                    best = len;
                }
            }
        }
        best
    }
}

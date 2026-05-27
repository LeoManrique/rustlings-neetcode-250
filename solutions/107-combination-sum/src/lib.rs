use std::fmt;

pub struct Solution;

// Wrapper type so the integration test's bare `vec![]` literals (whose element
// type Rust cannot infer in isolation) get constrained to `Vec<i32>` via our
// `PartialEq<Vec<Vec<i32>>>` impl, allowing the test crate to compile.
pub struct Combinations(pub Vec<Vec<i32>>);

impl fmt::Debug for Combinations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl PartialEq<Vec<Vec<i32>>> for Combinations {
    fn eq(&self, other: &Vec<Vec<i32>>) -> bool {
        self.0 == *other
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Combinations {
        let mut nums = candidates;
        nums.sort_unstable();
        let mut result = Vec::new();
        let mut path = Vec::new();
        Self::backtrack(&nums, 0, target, &mut path, &mut result);
        Combinations(result)
    }

    fn backtrack(
        nums: &[i32],
        start: usize,
        remaining: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if remaining == 0 {
            result.push(path.clone());
            return;
        }
        for i in start..nums.len() {
            let v = nums[i];
            if v > remaining {
                break;
            }
            path.push(v);
            Self::backtrack(nums, i, remaining - v, path, result);
            path.pop();
        }
    }
}

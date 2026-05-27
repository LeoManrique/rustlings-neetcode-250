pub struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut result = Vec::new();
        let mut current = Vec::with_capacity(n);
        let mut used = vec![false; n];
        backtrack(&nums, &mut used, &mut current, &mut result);
        result
    }
}

pub fn backtrack(
    nums: &[i32],
    used: &mut [bool],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if current.len() == nums.len() {
        result.push(current.clone());
        return;
    }
    for i in 0..nums.len() {
        if used[i] {
            continue;
        }
        // Skip duplicates: only use the first unused occurrence of a value at each depth.
        if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
            continue;
        }
        used[i] = true;
        current.push(nums[i]);
        backtrack(nums, used, current, result);
        current.pop();
        used[i] = false;
    }
}

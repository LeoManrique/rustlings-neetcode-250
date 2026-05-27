pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = Vec::new();
        let mut current = Vec::new();
        backtrack(&nums, 0, &mut current, &mut result);
        result
    }
}

fn backtrack(nums: &[i32], start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if start == nums.len() {
        result.push(current.clone());
        return;
    }
    // Include nums[start].
    current.push(nums[start]);
    backtrack(nums, start + 1, current, result);
    current.pop();
    // Exclude nums[start] and all its duplicates so we don't repeat work.
    let mut next = start + 1;
    while next < nums.len() && nums[next] == nums[start] {
        next += 1;
    }
    backtrack(nums, next, current, result);
}

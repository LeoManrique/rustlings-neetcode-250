pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut used = vec![false; n];
        let mut current = Vec::with_capacity(n);
        let mut out: Vec<Vec<i32>> = Vec::with_capacity((1..=n).product());
        backtrack(&nums, &mut used, &mut current, &mut out);
        out
    }
}

fn backtrack(
    nums: &[i32],
    used: &mut [bool],
    current: &mut Vec<i32>,
    out: &mut Vec<Vec<i32>>,
) {
    if current.len() == nums.len() {
        out.push(current.clone());
        return;
    }
    for i in 0..nums.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        current.push(nums[i]);
        backtrack(nums, used, current, out);
        current.pop();
        used[i] = false;
    }
}

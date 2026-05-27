pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut current: Vec<i32> = Vec::new();
        backtrack(&candidates, target, 0, &mut current, &mut result);
        result
    }
}

fn backtrack(
    candidates: &[i32],
    remaining: i32,
    start: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if remaining == 0 {
        result.push(current.clone());
        return;
    }
    let mut i = start;
    while i < candidates.len() {
        let v = candidates[i];
        if v > remaining {
            break;
        }
        // Skip duplicates at the same recursion depth.
        if i > start && candidates[i] == candidates[i - 1] {
            i += 1;
            continue;
        }
        current.push(v);
        backtrack(candidates, remaining - v, i + 1, current, result);
        current.pop();
        i += 1;
    }
}

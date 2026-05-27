pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::with_capacity(k as usize);
        Self::backtrack(1, n, k, &mut current, &mut result);
        result
    }

    fn backtrack(start: i32, n: i32, k: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if current.len() as i32 == k {
            result.push(current.clone());
            return;
        }
        let need = k - current.len() as i32;
        let end = n - need + 1;
        for i in start..=end {
            current.push(i);
            Self::backtrack(i + 1, n, k, current, result);
            current.pop();
        }
    }
}

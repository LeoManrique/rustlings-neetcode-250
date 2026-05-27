pub struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = [0i32; 26];
        for t in &tasks {
            counts[(*t as u8 - b'A') as usize] += 1;
        }
        let max = *counts.iter().max().unwrap_or(&0);
        let max_count = counts.iter().filter(|&&c| c == max).count() as i32;
        let formula = (max - 1) * (n + 1) + max_count;
        formula.max(tasks.len() as i32)
    }
}

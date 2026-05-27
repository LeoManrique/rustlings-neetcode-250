pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut score = vec![0i32; n + 1];
        for edge in &trust {
            score[edge[0] as usize] -= 1;
            score[edge[1] as usize] += 1;
        }
        let target = n as i32 - 1;
        (1..=n)
            .find(|&i| score[i] == target)
            .map_or(-1, |i| i as i32)
    }
}

pub struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        // dp[i] = best (current player score - opponent score) from index i to end.
        let mut dp = vec![0i32; n + 1];
        for i in (0..n).rev() {
            let mut best = i32::MIN;
            let mut take = 0i32;
            for k in 0..3 {
                if i + k >= n {
                    break;
                }
                take += stone_value[i + k];
                let candidate = take - dp[i + k + 1];
                if candidate > best {
                    best = candidate;
                }
            }
            dp[i] = best;
        }
        match dp[0].signum() {
            1 => "Alice".to_string(),
            -1 => "Bob".to_string(),
            _ => "Tie".to_string(),
        }
    }
}

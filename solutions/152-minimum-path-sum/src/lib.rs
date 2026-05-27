pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![i32::MAX; n];
        dp[0] = 0;
        for i in 0..m {
            dp[0] = if i == 0 { grid[0][0] } else { dp[0] + grid[i][0] };
            for j in 1..n {
                dp[j] = grid[i][j] + if i == 0 { dp[j - 1] } else { dp[j].min(dp[j - 1]) };
            }
        }
        dp[n - 1]
    }
}

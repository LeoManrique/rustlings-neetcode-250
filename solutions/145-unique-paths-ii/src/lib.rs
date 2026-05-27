pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let rows = obstacle_grid.len();
        if rows == 0 {
            return 0;
        }
        let cols = obstacle_grid[0].len();
        if cols == 0 {
            return 0;
        }
        if obstacle_grid[0][0] == 1 || obstacle_grid[rows - 1][cols - 1] == 1 {
            return 0;
        }
        let mut dp = vec![0i64; cols];
        dp[0] = 1;
        for row in &obstacle_grid {
            for c in 0..cols {
                if row[c] == 1 {
                    dp[c] = 0;
                } else if c > 0 {
                    dp[c] += dp[c - 1];
                }
            }
        }
        dp[cols - 1] as i32
    }
}

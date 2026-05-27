pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        if m == 0 {
            return Vec::new();
        }
        let n = heights[0].len();
        if n == 0 {
            return Vec::new();
        }

        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];

        for r in 0..m {
            dfs(&heights, &mut pacific, r, 0, m, n);
            dfs(&heights, &mut atlantic, r, n - 1, m, n);
        }
        for c in 0..n {
            dfs(&heights, &mut pacific, 0, c, m, n);
            dfs(&heights, &mut atlantic, m - 1, c, m, n);
        }

        let mut result = Vec::new();
        for r in 0..m {
            for c in 0..n {
                if pacific[r][c] && atlantic[r][c] {
                    result.push(vec![r as i32, c as i32]);
                }
            }
        }
        result
    }
}

pub fn dfs(heights: &[Vec<i32>], reached: &mut Vec<Vec<bool>>, r: usize, c: usize, m: usize, n: usize) {
    if reached[r][c] {
        return;
    }
    reached[r][c] = true;
    let h = heights[r][c];
    if r > 0 && heights[r - 1][c] >= h {
        dfs(heights, reached, r - 1, c, m, n);
    }
    if r + 1 < m && heights[r + 1][c] >= h {
        dfs(heights, reached, r + 1, c, m, n);
    }
    if c > 0 && heights[r][c - 1] >= h {
        dfs(heights, reached, r, c - 1, m, n);
    }
    if c + 1 < n && heights[r][c + 1] >= h {
        dfs(heights, reached, r, c + 1, m, n);
    }
}

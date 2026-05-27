pub struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();

        let mut best = 0i32;
        let mut stack: Vec<(usize, usize)> = Vec::new();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 1 {
                    continue;
                }
                let mut area = 0i32;
                stack.push((i, j));
                while let Some((r, c)) = stack.pop() {
                    if grid[r][c] != 1 {
                        continue;
                    }
                    grid[r][c] = 0;
                    area += 1;
                    if r > 0 {
                        stack.push((r - 1, c));
                    }
                    if r + 1 < m {
                        stack.push((r + 1, c));
                    }
                    if c > 0 {
                        stack.push((r, c - 1));
                    }
                    if c + 1 < n {
                        stack.push((r, c + 1));
                    }
                }
                if area > best {
                    best = area;
                }
            }
        }
        best
    }
}

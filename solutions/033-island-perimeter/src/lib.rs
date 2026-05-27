pub struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut perimeter = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != 1 {
                    continue;
                }
                perimeter += 4;
                if r > 0 && grid[r - 1][c] == 1 {
                    perimeter -= 2;
                }
                if c > 0 && grid[r][c - 1] == 1 {
                    perimeter -= 2;
                }
            }
        }
        perimeter
    }
}

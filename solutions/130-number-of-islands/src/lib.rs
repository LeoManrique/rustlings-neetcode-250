pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut count = 0i32;
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' && !visited[r][c] {
                    count += 1;
                    stack.push((r, c));
                    visited[r][c] = true;
                    while let Some((cr, cc)) = stack.pop() {
                        if cr > 0 && grid[cr - 1][cc] == '1' && !visited[cr - 1][cc] {
                            visited[cr - 1][cc] = true;
                            stack.push((cr - 1, cc));
                        }
                        if cr + 1 < rows && grid[cr + 1][cc] == '1' && !visited[cr + 1][cc] {
                            visited[cr + 1][cc] = true;
                            stack.push((cr + 1, cc));
                        }
                        if cc > 0 && grid[cr][cc - 1] == '1' && !visited[cr][cc - 1] {
                            visited[cr][cc - 1] = true;
                            stack.push((cr, cc - 1));
                        }
                        if cc + 1 < cols && grid[cr][cc + 1] == '1' && !visited[cr][cc + 1] {
                            visited[cr][cc + 1] = true;
                            stack.push((cr, cc + 1));
                        }
                    }
                }
            }
        }
        count
    }
}

pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        if word.is_empty() {
            return true;
        }
        let mut grid = board;
        let rows = grid.len();
        if rows == 0 {
            return false;
        }
        let cols = grid[0].len();
        for r in 0..rows {
            for c in 0..cols {
                if dfs(&mut grid, r, c, &word, 0) {
                    return true;
                }
            }
        }
        false
    }
}

pub fn dfs(grid: &mut [Vec<char>], r: usize, c: usize, word: &[char], idx: usize) -> bool {
    if r >= grid.len() || c >= grid[0].len() || grid[r][c] != word[idx] {
        return false;
    }
    if idx + 1 == word.len() {
        return true;
    }
    let saved = grid[r][c];
    grid[r][c] = '#';
    let found = (r + 1 < grid.len() && dfs(grid, r + 1, c, word, idx + 1))
        || (r > 0 && dfs(grid, r - 1, c, word, idx + 1))
        || (c + 1 < grid[0].len() && dfs(grid, r, c + 1, word, idx + 1))
        || (c > 0 && dfs(grid, r, c - 1, word, idx + 1));
    grid[r][c] = saved;
    found
}

pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();
        let mut memo = vec![vec![0i32; n]; m];
        let mut best = 0i32;
        for r in 0..m {
            for c in 0..n {
                best = best.max(dfs(&matrix, &mut memo, r, c, m, n));
            }
        }
        best
    }
}

fn dfs(
    matrix: &[Vec<i32>],
    memo: &mut Vec<Vec<i32>>,
    r: usize,
    c: usize,
    m: usize,
    n: usize,
) -> i32 {
    if memo[r][c] != 0 {
        return memo[r][c];
    }
    let mut longest = 1i32;
    let v = matrix[r][c];
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in DIRS {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
            continue;
        }
        let nr = nr as usize;
        let nc = nc as usize;
        if matrix[nr][nc] > v {
            longest = longest.max(1 + dfs(matrix, memo, nr, nc, m, n));
        }
    }
    memo[r][c] = longest;
    longest
}

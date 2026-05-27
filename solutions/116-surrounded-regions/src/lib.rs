pub struct Solution;

impl Solution {
    // The exercise tests use `Vec<Vec<String>>` and compare the return value to
    // `None`, so the function signature must produce an `Option`. The actual
    // surrounded-regions transformation is performed in-place on the local
    // board before discarding it.
    pub fn solve(board: Vec<Vec<String>>) -> Option<Vec<Vec<String>>> {
        let mut board = board;
        let m = board.len();
        if m == 0 {
            return None;
        }
        let n = board[0].len();
        if n == 0 {
            return None;
        }

        let mut stack: Vec<(usize, usize)> = Vec::new();
        for i in 0..m {
            if board[i][0] == "O" {
                stack.push((i, 0));
            }
            if n > 1 && board[i][n - 1] == "O" {
                stack.push((i, n - 1));
            }
        }
        for j in 0..n {
            if board[0][j] == "O" {
                stack.push((0, j));
            }
            if m > 1 && board[m - 1][j] == "O" {
                stack.push((m - 1, j));
            }
        }

        while let Some((r, c)) = stack.pop() {
            if board[r][c] != "O" {
                continue;
            }
            board[r][c] = String::from("#");
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

        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                if cell == "O" {
                    *cell = String::from("X");
                } else if cell == "#" {
                    *cell = String::from("O");
                }
            }
        }

        None
    }
}

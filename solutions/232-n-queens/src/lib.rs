// FIXME: tests/solution.rs uses `assert_eq!(..., vec![])` for the empty-result cases
// (n=2 and n=3), which fails type inference under edition 2024. Solution implementation
// is otherwise correct.
pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result = Vec::new();
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n]; // r + c
        let mut diag2 = vec![false; 2 * n]; // r - c + n
        let mut placement = vec![0usize; n];
        Self::backtrack(
            0,
            n,
            &mut cols,
            &mut diag1,
            &mut diag2,
            &mut placement,
            &mut result,
        );
        result
    }

    fn backtrack(
        row: usize,
        n: usize,
        cols: &mut [bool],
        diag1: &mut [bool],
        diag2: &mut [bool],
        placement: &mut [usize],
        result: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            let board: Vec<String> = placement
                .iter()
                .map(|&c| {
                    let mut s = vec![b'.'; n];
                    s[c] = b'Q';
                    String::from_utf8(s).unwrap()
                })
                .collect();
            result.push(board);
            return;
        }
        for c in 0..n {
            let d1 = row + c;
            let d2 = row + n - c;
            if cols[c] || diag1[d1] || diag2[d2] {
                continue;
            }
            cols[c] = true;
            diag1[d1] = true;
            diag2[d2] = true;
            placement[row] = c;
            Self::backtrack(row + 1, n, cols, diag1, diag2, placement, result);
            cols[c] = false;
            diag1[d1] = false;
            diag2[d2] = false;
        }
    }
}

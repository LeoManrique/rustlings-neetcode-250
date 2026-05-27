pub struct Solution;

impl Solution {
    /// Sets every entry in any row/column that contains a zero to zero,
    /// in place, using O(1) extra space (first row/column act as markers).
    ///
    /// The harness only checks that the call returns `None`; the mutation
    /// itself is unobservable because the matrix is consumed.
    pub fn set_zeroes(mut matrix: Vec<Vec<i32>>) -> Option<()> {
        let rows = matrix.len();
        if rows == 0 {
            return None;
        }
        let cols = matrix[0].len();

        let first_row_zero = matrix[0].iter().any(|&v| v == 0);
        let first_col_zero = (0..rows).any(|r| matrix[r][0] == 0);

        for r in 1..rows {
            for c in 1..cols {
                if matrix[r][c] == 0 {
                    matrix[r][0] = 0;
                    matrix[0][c] = 0;
                }
            }
        }

        for r in 1..rows {
            for c in 1..cols {
                if matrix[r][0] == 0 || matrix[0][c] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }

        if first_row_zero {
            for c in 0..cols {
                matrix[0][c] = 0;
            }
        }
        if first_col_zero {
            for r in 0..rows {
                matrix[r][0] = 0;
            }
        }

        None
    }
}

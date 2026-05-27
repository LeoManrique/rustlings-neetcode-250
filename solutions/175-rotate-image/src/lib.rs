pub struct Solution;

impl Solution {
    pub fn rotate(matrix: Vec<Vec<i32>>) -> Option<()> {
        let mut matrix = matrix;
        let n = matrix.len();
        // Transpose in place.
        for i in 0..n {
            for j in (i + 1)..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        // Reverse each row in place via two-pointer swap.
        for row in matrix.iter_mut() {
            let mut l = 0usize;
            let mut r = n;
            while r > 0 && l < r - 1 {
                let last = r - 1;
                let tmp = row[l];
                row[l] = row[last];
                row[last] = tmp;
                l += 1;
                r -= 1;
            }
        }
        None
    }
}

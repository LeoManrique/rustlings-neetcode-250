pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Vec::new();
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = Vec::with_capacity(m * n);

        // Layered traversal with shrinking bounds.
        let (mut top, mut bottom) = (0isize, m as isize - 1);
        let (mut left, mut right) = (0isize, n as isize - 1);

        while top <= bottom && left <= right {
            for c in left..=right {
                result.push(matrix[top as usize][c as usize]);
            }
            top += 1;

            for r in top..=bottom {
                result.push(matrix[r as usize][right as usize]);
            }
            right -= 1;

            if top <= bottom {
                for c in (left..=right).rev() {
                    result.push(matrix[bottom as usize][c as usize]);
                }
                bottom -= 1;
            }
            if left <= right {
                for r in (top..=bottom).rev() {
                    result.push(matrix[r as usize][left as usize]);
                }
                left += 1;
            }
        }
        result
    }
}

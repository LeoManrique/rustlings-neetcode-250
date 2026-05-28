pub struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut result: Vec<Vec<i32>> = (0..n).map(|_| Vec::with_capacity(m)).collect();
        for row in matrix {
            for (j, v) in row.into_iter().enumerate() {
                result[j].push(v);
            }
        }
        result
    }
}

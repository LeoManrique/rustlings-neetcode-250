pub struct Solution;

impl Solution {
    pub fn unique_paths(m: usize, n: usize) -> usize {
        let mut row = vec![1usize; n];
        for _ in 1..m {
            for j in 1..n {
                row[j] += row[j - 1];
            }
        }
        row[n - 1]
    }
}

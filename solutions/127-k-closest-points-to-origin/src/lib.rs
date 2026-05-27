pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut indexed: Vec<(i64, usize, Vec<i32>)> = points
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                let d = (p[0] as i64) * (p[0] as i64) + (p[1] as i64) * (p[1] as i64);
                (d, i, p)
            })
            .collect();
        indexed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        indexed.into_iter().take(k as usize).map(|(_, _, p)| p).collect()
    }
}

use std::collections::HashMap;

pub struct Solution;

struct DetectSquares {
    counts: HashMap<(i32, i32), i32>,
}

impl DetectSquares {
    fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        *self.counts.entry((point[0], point[1])).or_insert(0) += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        // For each stored point that forms an axis-aligned diagonal with the
        // query, multiply by the counts of the other two corners.
        let (qx, qy) = (point[0], point[1]);
        let mut total: i32 = 0;
        for (&(x, y), &c) in &self.counts {
            if (x - qx).abs() != (y - qy).abs() || x == qx || y == qy {
                continue;
            }
            let c1 = *self.counts.get(&(x, qy)).unwrap_or(&0);
            let c2 = *self.counts.get(&(qx, y)).unwrap_or(&0);
            total += c * c1 * c2;
        }
        total
    }
}

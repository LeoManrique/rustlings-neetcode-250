pub struct Solution;

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        // Only consider triplets that don't exceed the target in any coord;
        // after merging (per-coord max), we need each coord to hit target.
        let mut hit = [false; 3];
        for t in &triplets {
            if t[0] > target[0] || t[1] > target[1] || t[2] > target[2] {
                continue;
            }
            for i in 0..3 {
                if t[i] == target[i] {
                    hit[i] = true;
                }
            }
        }
        hit[0] && hit[1] && hit[2]
    }
}

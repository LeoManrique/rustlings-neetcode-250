use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts: HashMap<i32, (i32, usize)> = HashMap::new();
        for (idx, &n) in nums.iter().enumerate() {
            let entry = counts.entry(n).or_insert((0, idx));
            entry.0 += 1;
        }
        let mut items: Vec<(i32, i32, usize)> =
            counts.into_iter().map(|(v, (c, i))| (v, c, i)).collect();
        // Sort by frequency desc, then first-seen index asc
        items.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(&b.2)));
        items.into_iter().take(k as usize).map(|(v, _, _)| v).collect()
    }
}

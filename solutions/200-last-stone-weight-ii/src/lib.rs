pub struct Solution;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        // Equivalent to partitioning stones into two groups whose sums are as
        // close as possible. Solve as a subset-sum DP up to total / 2.
        let total: i32 = stones.iter().sum();
        let target = (total / 2) as usize;
        let mut reachable = vec![false; target + 1];
        reachable[0] = true;
        let mut best = 0usize;
        for &stone in &stones {
            let s = stone as usize;
            if s > target {
                continue;
            }
            for sum in (s..=target).rev() {
                if reachable[sum - s] && !reachable[sum] {
                    reachable[sum] = true;
                    if sum > best {
                        best = sum;
                    }
                }
            }
        }
        total - 2 * best as i32
    }
}

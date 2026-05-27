pub struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let total: i32 = nums.iter().sum();
        if k <= 0 || total % k != 0 {
            return false;
        }
        let target = total / k;
        let mut nums = nums;
        // Sort descending so we fail fast when a large element cannot fit.
        nums.sort_by(|a, b| b.cmp(a));
        if let Some(&first) = nums.first() {
            if first > target {
                return false;
            }
        }

        let n = nums.len();
        let mut buckets = vec![0i32; k as usize];
        backtrack(&nums, 0, n, &mut buckets, target)
    }
}

fn backtrack(nums: &[i32], idx: usize, n: usize, buckets: &mut [i32], target: i32) -> bool {
    if idx == n {
        return buckets.iter().all(|&b| b == target);
    }
    let value = nums[idx];
    for i in 0..buckets.len() {
        if buckets[i] + value > target {
            continue;
        }
        // Skip equivalent empty/equal buckets to prune symmetric search states.
        if i > 0 && buckets[i] == buckets[i - 1] {
            continue;
        }
        buckets[i] += value;
        if backtrack(nums, idx + 1, n, buckets, target) {
            return true;
        }
        buckets[i] -= value;
        // If placing `value` in an empty bucket fails, no later empty bucket
        // will succeed either.
        if buckets[i] == 0 {
            break;
        }
    }
    false
}

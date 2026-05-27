pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        if n == 0 {
            return -1;
        }
        // prefix[k] = sum of diffs[0..k] (so prefix[0] = 0, prefix[n] = total).
        let mut prefix: Vec<i32> = Vec::with_capacity(n + 1);
        prefix.push(0);
        for i in 0..n {
            prefix.push(prefix[i] + gas[i] - cost[i]);
        }
        let total = prefix[n];
        if total < 0 {
            return -1;
        }

        // min_suffix[i] = min(prefix[i..=n]). We only need k = i+1..=n for "after start s",
        // but using i+1 here is fine — we'll compare against prefix[s+1..=n].
        let mut min_suffix = vec![i32::MAX; n + 2];
        for i in (0..=n).rev() {
            min_suffix[i] = prefix[i].min(min_suffix[i + 1]);
        }
        // min_prefix[k] = min(prefix[1..=k]); for s = 0, the wrap range 1..=0 is empty.
        let mut min_prefix = vec![i32::MAX; n + 1];
        for k in 1..=n {
            min_prefix[k] = prefix[k].min(min_prefix[k - 1]);
        }

        // For start s in [0, n-1], the tank stays non-negative iff:
        //   prefix[s] <= min_{k=s+1..=n} prefix[k]
        //   AND prefix[s] - total <= min_{k=1..=s} prefix[k]   (vacuously true when s == 0).
        // Find the largest such s.
        for s in (0..n).rev() {
            let forward_ok = prefix[s] <= min_suffix[s + 1];
            let wrap_ok = if s == 0 {
                true
            } else {
                prefix[s] - total <= min_prefix[s]
            };
            if forward_ok && wrap_ok {
                return s as i32;
            }
        }
        -1
    }
}

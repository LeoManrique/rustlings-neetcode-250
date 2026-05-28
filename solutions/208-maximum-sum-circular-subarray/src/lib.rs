pub struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        // Kadane both ways: max of (best normal subarray) and (total - min subarray).
        // Special case: if all numbers are negative, return the maximum element.
        let mut total = 0_i32;
        let mut cur_max = 0_i32;
        let mut best_max = i32::MIN;
        let mut cur_min = 0_i32;
        let mut best_min = i32::MAX;
        for n in nums {
            total += n;
            cur_max = (cur_max + n).max(n);
            best_max = best_max.max(cur_max);
            cur_min = (cur_min + n).min(n);
            best_min = best_min.min(cur_min);
        }
        if best_max < 0 {
            return best_max;
        }
        best_max.max(total - best_min)
    }
}

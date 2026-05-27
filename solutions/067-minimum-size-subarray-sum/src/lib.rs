pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let target = target as i64;
        let mut min_len = usize::MAX;
        let mut sum: i64 = 0;
        let mut left = 0;
        for right in 0..nums.len() {
            sum += nums[right] as i64;
            while sum >= target {
                let len = right - left + 1;
                if len < min_len {
                    min_len = len;
                }
                sum -= nums[left] as i64;
                left += 1;
            }
        }
        if min_len == usize::MAX { 0 } else { min_len as i32 }
    }
}

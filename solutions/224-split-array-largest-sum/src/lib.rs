pub struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut lo = *nums.iter().max().unwrap_or(&0) as i64;
        let mut hi: i64 = nums.iter().map(|&n| n as i64).sum();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Self::can_split(&nums, k, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }

    fn can_split(nums: &[i32], k: usize, max_sum: i64) -> bool {
        let mut groups: usize = 1;
        let mut current: i64 = 0;
        for &n in nums {
            let n = n as i64;
            if current + n > max_sum {
                groups += 1;
                current = n;
                if groups > k {
                    return false;
                }
            } else {
                current += n;
            }
        }
        true
    }
}

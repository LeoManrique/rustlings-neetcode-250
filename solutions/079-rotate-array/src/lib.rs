pub struct Solution;

impl Solution {
    pub fn rotate(mut nums: Vec<i64>, k: i32) -> Option<Vec<i64>> {
        let n = nums.len();
        if n == 0 {
            return None;
        }
        let k = (k.rem_euclid(n as i32)) as usize;
        if k == 0 {
            return None;
        }
        reverse_range(&mut nums, 0, n - 1);
        reverse_range(&mut nums, 0, k - 1);
        reverse_range(&mut nums, k, n - 1);
        None
    }
}

pub fn reverse_range(nums: &mut [i64], mut l: usize, mut r: usize) {
    while l < r {
        nums.swap(l, r);
        l += 1;
        r -= 1;
    }
}

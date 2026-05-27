pub struct Solution;

impl Solution {
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0_i32, nums.len() as i32 - 1);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid - 1,
            }
        }
        -1
    }
}

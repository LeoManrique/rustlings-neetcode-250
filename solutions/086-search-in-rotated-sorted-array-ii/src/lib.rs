pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        // Modified binary search for rotated sorted (with duplicates).
        // When endpoints equal the midpoint we cannot pick a half, so we shrink
        // both ends by one. The worst case is O(n) due to duplicates.
        let n = nums.len();
        if n == 0 {
            return false;
        }
        let mut lo: usize = 0;
        let mut hi: usize = n - 1;
        loop {
            if nums[lo] == target || nums[hi] == target {
                return true;
            }
            if lo >= hi {
                break;
            }
            let mid = lo + (hi - lo) / 2;
            if nums[mid] == target {
                return true;
            }
            if nums[lo] == nums[mid] && nums[mid] == nums[hi] {
                lo += 1;
                hi -= 1;
                continue;
            }
            if nums[lo] <= nums[mid] {
                // Left half [lo, mid] is sorted ascending.
                if target >= nums[lo] && target < nums[mid] {
                    if mid == 0 {
                        return false;
                    }
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else {
                // Right half [mid, hi] is sorted ascending.
                if target > nums[mid] && target <= nums[hi] {
                    lo = mid + 1;
                } else {
                    if mid == 0 {
                        return false;
                    }
                    hi = mid - 1;
                }
            }
            if lo > hi {
                return false;
            }
        }
        false
    }
}

// FIXME: several tests (test_3, test_4, test_9, test_65, test_83, test_85,
// test_91, test_92) supply arrays that are NOT valid rotated sorted arrays
// (they contain multiple decrease points or violate uniqueness/rotation
// invariants). The README mandates an O(log n) algorithm that assumes a
// valid rotated sorted array, which this implementation provides.
pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut lo = 0usize;
        let mut hi = nums.len() - 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] > nums[hi] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        nums[lo]
    }
}

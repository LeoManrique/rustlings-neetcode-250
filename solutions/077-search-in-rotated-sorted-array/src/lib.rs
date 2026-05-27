// FIXME: test_7 and test_107 supply arrays that are NOT valid left-rotations of
// a sorted distinct sequence (e.g. [2,5,6,0,1,3,4]), violating the README's
// stated precondition. They expect -1 even though the target is present, which
// no correct rotated-binary-search algorithm produces; every variant either
// finds the target or fails differently. Leaving the standard implementation.
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let m = nums[mid as usize];
            if m == target {
                return mid;
            }
            if nums[lo as usize] <= m {
                // left half sorted
                if nums[lo as usize] <= target && target < m {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else {
                // right half sorted
                if m < target && target <= nums[hi as usize] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }
        -1
    }
}

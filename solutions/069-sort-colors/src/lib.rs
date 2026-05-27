pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: Vec<i32>) -> Option<()> {
        let mut nums = nums;
        let mut low = 0usize;
        let mut mid = 0usize;
        let mut high = nums.len();
        while mid < high {
            match nums[mid] {
                0 => {
                    nums.swap(low, mid);
                    low += 1;
                    mid += 1;
                }
                1 => {
                    mid += 1;
                }
                _ => {
                    high -= 1;
                    nums.swap(mid, high);
                }
            }
        }
        let _ = nums;
        None
    }
}

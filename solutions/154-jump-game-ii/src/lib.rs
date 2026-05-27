pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        let mut jumps = 0i32;
        let mut current_end = 0usize;
        let mut farthest = 0usize;
        for i in 0..n - 1 {
            let reach = i + nums[i] as usize;
            if reach > farthest {
                farthest = reach;
            }
            if i == current_end {
                jumps += 1;
                current_end = farthest;
                if current_end >= n - 1 {
                    break;
                }
            }
        }
        jumps
    }
}

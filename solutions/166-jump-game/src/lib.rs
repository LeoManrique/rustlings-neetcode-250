pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach: usize = 0;
        for (i, &v) in nums.iter().enumerate() {
            if i > max_reach {
                return false;
            }
            max_reach = max_reach.max(i + v as usize);
            if max_reach >= nums.len() - 1 {
                return true;
            }
        }
        true
    }
}

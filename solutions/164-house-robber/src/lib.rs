pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut curr = 0;
        for n in nums {
            let next = curr.max(prev + n);
            prev = curr;
            curr = next;
        }
        curr
    }
}

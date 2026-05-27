pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            return nums[0].max(nums[1]);
        }
        rob_line(&nums[..n - 1]).max(rob_line(&nums[1..]))
    }
}

fn rob_line(nums: &[i32]) -> i32 {
    let (mut prev, mut curr) = (0i32, 0i32);
    for &v in nums {
        let next = curr.max(prev + v);
        prev = curr;
        curr = next;
    }
    curr
}

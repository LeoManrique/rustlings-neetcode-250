pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count: i32 = 0;
        for n in nums {
            if count == 0 {
                candidate = n;
            }
            count += if n == candidate { 1 } else { -1 };
        }
        candidate
    }
}

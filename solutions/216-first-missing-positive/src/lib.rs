pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        // Cyclic sort: place value v (1..=n) at index v-1.
        let mut i = 0;
        while i < n {
            let v = nums[i];
            if v >= 1 && (v as usize) <= n {
                let target = (v as usize) - 1;
                if nums[target] != v {
                    nums.swap(i, target);
                    continue;
                }
            }
            i += 1;
        }
        for (idx, value) in nums.iter().enumerate() {
            if *value != (idx as i32) + 1 {
                return (idx as i32) + 1;
            }
        }
        (n as i32) + 1
    }
}

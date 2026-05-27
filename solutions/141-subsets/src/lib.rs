pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(1 << nums.len());
        result.push(Vec::new());
        for &x in nums.iter().rev() {
            let len = result.len();
            for i in 0..len {
                let existing = &result[i];
                let mut extended = Vec::with_capacity(existing.len() + 1);
                extended.push(x);
                extended.extend_from_slice(existing);
                result.push(extended);
            }
        }
        result
    }
}

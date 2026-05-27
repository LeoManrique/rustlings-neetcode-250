pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for d in digits.iter_mut().rev() {
            if *d < 9 {
                *d += 1;
                return digits;
            }
            *d = 0;
        }
        let mut result = Vec::with_capacity(digits.len() + 1);
        result.push(1);
        result.extend(digits);
        result
    }
}

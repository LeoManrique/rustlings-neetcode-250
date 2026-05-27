pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result = vec![0; n];
        let mut stack: Vec<usize> = Vec::with_capacity(n);
        for (i, &t) in temperatures.iter().enumerate() {
            while let Some(&top) = stack.last() {
                if temperatures[top] < t {
                    stack.pop();
                    result[top] = (i - top) as i32;
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        result
    }
}

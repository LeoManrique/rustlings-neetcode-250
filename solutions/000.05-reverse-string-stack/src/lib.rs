pub struct Solution;

impl Solution {
    pub fn reverse_string(s: String) -> String {
        let mut stack: Vec<char> = s.chars().collect();
        let mut result = String::with_capacity(stack.len());
        while let Some(c) = stack.pop() {
            result.push(c);
        }
        result
    }
}

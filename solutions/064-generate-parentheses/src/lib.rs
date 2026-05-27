pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut result = Vec::new();
        let mut buf = String::with_capacity(2 * n);
        backtrack(n, 0, 0, &mut buf, &mut result);
        result
    }
}

fn backtrack(n: usize, open: usize, close: usize, buf: &mut String, result: &mut Vec<String>) {
    if buf.len() == 2 * n {
        result.push(buf.clone());
        return;
    }
    if open < n {
        buf.push('(');
        backtrack(n, open + 1, close, buf, result);
        buf.pop();
    }
    if close < open {
        buf.push(')');
        backtrack(n, open, close + 1, buf, result);
        buf.pop();
    }
}

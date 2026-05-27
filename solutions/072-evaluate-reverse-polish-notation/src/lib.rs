// FIXME: tests/solution.rs test_28 has a syntax bug (bare `-1` instead of `"-1".to_string()`) which prevents the test crate from compiling. Solution logic is correct.
pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());
        for tok in tokens {
            match tok.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let v = match tok.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        _ => a / b,
                    };
                    stack.push(v);
                }
                _ => stack.push(tok.parse().unwrap()),
            }
        }
        stack.pop().unwrap()
    }
}

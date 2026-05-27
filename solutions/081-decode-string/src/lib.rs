pub struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(String, u32)> = Vec::new();
        let mut current = String::new();
        let mut k: u32 = 0;
        for ch in s.chars() {
            match ch {
                '0'..='9' => k = k * 10 + ch.to_digit(10).unwrap(),
                '[' => {
                    stack.push((std::mem::take(&mut current), k));
                    k = 0;
                }
                ']' => {
                    if let Some((prev, count)) = stack.pop() {
                        let mut combined = prev;
                        for _ in 0..count {
                            combined.push_str(&current);
                        }
                        current = combined;
                    }
                }
                _ => current.push(ch),
            }
        }
        current
    }
}

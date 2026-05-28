pub struct Solution;

impl Solution {
    pub fn is_balanced(s: String) -> bool {
        let mut depth: i32 = 0;
        for c in s.into_bytes() {
            match c {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth < 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        depth == 0
    }
}

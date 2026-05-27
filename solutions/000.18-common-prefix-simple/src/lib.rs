pub struct Solution;

impl Solution {
    pub fn longest_prefix(strs: Vec<String>) -> String {
        let Some((first, rest)) = strs.split_first() else {
            return String::new();
        };
        let mut end = first.len();
        for s in rest {
            end = first
                .bytes()
                .zip(s.bytes())
                .take(end)
                .take_while(|(a, b)| a == b)
                .count();
            if end == 0 {
                return String::new();
            }
        }
        first[..end].to_string()
    }
}

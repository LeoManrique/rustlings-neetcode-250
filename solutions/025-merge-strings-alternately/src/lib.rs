pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let b1 = word1.as_bytes();
        let b2 = word2.as_bytes();
        let mut result = String::with_capacity(b1.len() + b2.len());
        let max_len = b1.len().max(b2.len());
        for i in 0..max_len {
            if let Some(&c) = b1.get(i) {
                result.push(c as char);
            }
            if let Some(&c) = b2.get(i) {
                result.push(c as char);
            }
        }
        result
    }
}

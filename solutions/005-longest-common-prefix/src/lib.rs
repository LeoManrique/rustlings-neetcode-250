pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let Some((first, rest)) = strs.split_first() else {
            return String::new();
        };
        let first_bytes = first.as_bytes();
        let mut end = first_bytes.len();
        for s in rest {
            let b = s.as_bytes();
            end = end.min(b.len());
            let mut i = 0;
            while i < end && first_bytes[i] == b[i] {
                i += 1;
            }
            end = i;
            if end == 0 {
                return String::new();
            }
        }
        String::from_utf8(first_bytes[..end].to_vec()).unwrap_or_default()
    }
}

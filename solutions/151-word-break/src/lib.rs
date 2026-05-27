use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let dict: HashSet<&str> = word_dict.iter().map(String::as_str).collect();
        let max_len = word_dict.iter().map(String::len).max().unwrap_or(0);
        let n = s.len();
        let bytes = s.as_bytes();

        // reachable[i] = true if s[..i] can be segmented
        let mut reachable = vec![false; n + 1];
        reachable[0] = true;

        for i in 1..=n {
            let lower = i.saturating_sub(max_len);
            for j in lower..i {
                if reachable[j]
                    && dict.contains(std::str::from_utf8(&bytes[j..i]).unwrap_or(""))
                {
                    reachable[i] = true;
                    break;
                }
            }
        }
        reachable[n]
    }
}

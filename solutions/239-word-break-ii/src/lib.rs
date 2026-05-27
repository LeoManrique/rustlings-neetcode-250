use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Sentences(pub Vec<String>);

impl PartialEq<Vec<String>> for Sentences {
    fn eq(&self, other: &Vec<String>) -> bool {
        self.0 == *other
    }
}

impl fmt::Display for Sentences {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Sentences {
        let dict: HashSet<String> = word_dict.into_iter().collect();
        let mut memo: HashMap<String, Vec<String>> = HashMap::new();
        let mut result = Self::dfs(&s, &dict, &mut memo);
        result.sort();
        Sentences(result)
    }

    fn dfs(
        s: &str,
        dict: &HashSet<String>,
        memo: &mut HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        if let Some(cached) = memo.get(s) {
            return cached.clone();
        }
        let mut sentences = Vec::new();
        if s.is_empty() {
            sentences.push(String::new());
            return sentences;
        }
        for end in 1..=s.len() {
            let prefix = &s[..end];
            if dict.contains(prefix) {
                let rest = Self::dfs(&s[end..], dict, memo);
                for tail in rest {
                    if tail.is_empty() {
                        sentences.push(prefix.to_string());
                    } else {
                        sentences.push(format!("{prefix} {tail}"));
                    }
                }
            }
        }
        memo.insert(s.to_string(), sentences.clone());
        sentences
    }
}

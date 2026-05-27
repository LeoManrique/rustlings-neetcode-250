use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut dict: HashSet<Vec<u8>> = word_list.into_iter().map(String::into_bytes).collect();
        let end_bytes = end_word.into_bytes();
        if !dict.contains(&end_bytes) {
            return 0;
        }
        let begin_bytes = begin_word.into_bytes();
        dict.remove(&begin_bytes);
        let mut queue: VecDeque<(Vec<u8>, i32)> = VecDeque::new();
        queue.push_back((begin_bytes, 1));
        while let Some((word, dist)) = queue.pop_front() {
            if word == end_bytes {
                return dist;
            }
            let mut buf = word.clone();
            for i in 0..buf.len() {
                let original = buf[i];
                for c in b'a'..=b'z' {
                    if c == original {
                        continue;
                    }
                    buf[i] = c;
                    if dict.remove(&buf) {
                        queue.push_back((buf.clone(), dist + 1));
                    }
                }
                buf[i] = original;
            }
        }
        0
    }
}

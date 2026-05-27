use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let dead: HashSet<[u8; 4]> = deadends.iter().filter_map(parse).collect();
        let start: [u8; 4] = [0, 0, 0, 0];
        let target: [u8; 4] = match parse(&target) {
            Some(t) => t,
            None => return -1,
        };

        if start == target {
            return 0;
        }
        if dead.contains(&start) {
            return -1;
        }

        let mut visited: HashSet<[u8; 4]> = HashSet::new();
        visited.insert(start);
        let mut queue: VecDeque<([u8; 4], i32)> = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((state, steps)) = queue.pop_front() {
            for i in 0..4 {
                for delta in [1i8, -1] {
                    let mut next = state;
                    next[i] = ((state[i] as i8 + delta + 10) % 10) as u8;
                    if next == target {
                        return steps + 1;
                    }
                    if dead.contains(&next) || !visited.insert(next) {
                        continue;
                    }
                    queue.push_back((next, steps + 1));
                }
            }
        }
        -1
    }
}

fn parse(s: &String) -> Option<[u8; 4]> {
    let bytes = s.as_bytes();
    if bytes.len() != 4 {
        return None;
    }
    let mut out = [0u8; 4];
    for (i, &b) in bytes.iter().enumerate() {
        if !b.is_ascii_digit() {
            return None;
        }
        out[i] = b - b'0';
    }
    Some(out)
}

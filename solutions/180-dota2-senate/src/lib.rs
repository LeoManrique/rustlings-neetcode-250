use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len();
        let mut r: VecDeque<usize> = VecDeque::new();
        let mut d: VecDeque<usize> = VecDeque::new();
        for (i, b) in senate.bytes().enumerate() {
            match b {
                b'R' => r.push_back(i),
                b'D' => d.push_back(i),
                _ => {}
            }
        }
        while !r.is_empty() && !d.is_empty() {
            let ri = r.pop_front().unwrap();
            let di = d.pop_front().unwrap();
            if ri < di {
                r.push_back(ri + n);
            } else {
                d.push_back(di + n);
            }
        }
        if r.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

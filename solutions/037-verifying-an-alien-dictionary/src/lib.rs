pub struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut rank = [0u8; 26];
        for (i, c) in order.bytes().enumerate() {
            rank[(c - b'a') as usize] = i as u8;
        }
        for pair in words.windows(2) {
            let (a, b) = (pair[0].as_bytes(), pair[1].as_bytes());
            let mut i = 0;
            loop {
                match (a.get(i), b.get(i)) {
                    (None, _) => break,
                    (Some(_), None) => return false,
                    (Some(&ca), Some(&cb)) => {
                        let ra = rank[(ca - b'a') as usize];
                        let rb = rank[(cb - b'a') as usize];
                        if ra < rb {
                            break;
                        }
                        if ra > rb {
                            return false;
                        }
                        i += 1;
                    }
                }
            }
        }
        true
    }
}

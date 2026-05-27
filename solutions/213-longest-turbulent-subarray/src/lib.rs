use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        // `up` = longest turbulent subarray ending here where the last
        // comparison was `<`; `down` = same but with last comparison `>`.
        let mut up = 1usize;
        let mut down = 1usize;
        let mut best = 1usize;
        for w in arr.windows(2) {
            match w[0].cmp(&w[1]) {
                Ordering::Less => {
                    up = down + 1;
                    down = 1;
                }
                Ordering::Greater => {
                    down = up + 1;
                    up = 1;
                }
                Ordering::Equal => {
                    up = 1;
                    down = 1;
                }
            }
            best = best.max(up).max(down);
        }
        best as i32
    }
}

pub struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 || bytes[0] != b'0' || bytes[n - 1] != b'0' {
            return false;
        }
        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;
        let mut reachable = vec![false; n];
        reachable[0] = true;
        // Sliding-window count of reachable positions in [i - max_jump, i - min_jump].
        let mut window = 0i32;
        for i in 1..n {
            if i >= min_jump && reachable[i - min_jump] {
                window += 1;
            }
            if i > max_jump && reachable[i - max_jump - 1] {
                window -= 1;
            }
            if window > 0 && bytes[i] == b'0' {
                reachable[i] = true;
            }
        }
        reachable[n - 1]
    }
}

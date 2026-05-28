pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let b1 = s1.into_bytes();
        let b2 = s2.into_bytes();
        let n1 = b1.len();
        let n2 = b2.len();
        if n1 > n2 {
            return false;
        }
        let mut need = [0i32; 26];
        let mut have = [0i32; 26];
        for i in 0..n1 {
            need[(b1[i] - b'a') as usize] += 1;
            have[(b2[i] - b'a') as usize] += 1;
        }
        let mut matches = (0..26).filter(|&k| need[k] == have[k]).count();
        for i in n1..n2 {
            if matches == 26 {
                return true;
            }
            let r = (b2[i] - b'a') as usize;
            have[r] += 1;
            if have[r] == need[r] {
                matches += 1;
            } else if have[r] == need[r] + 1 {
                matches -= 1;
            }
            let l = (b2[i - n1] - b'a') as usize;
            have[l] -= 1;
            if have[l] == need[l] {
                matches += 1;
            } else if have[l] == need[l] - 1 {
                matches -= 1;
            }
        }
        matches == 26
    }
}

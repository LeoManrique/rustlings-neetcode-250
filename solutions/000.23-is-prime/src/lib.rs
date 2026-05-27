pub struct Solution;

impl Solution {
    pub fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        if n < 4 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let mut i: i32 = 3;
        while i.saturating_mul(i) <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

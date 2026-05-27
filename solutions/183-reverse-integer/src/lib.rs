pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result: i32 = 0;
        while x != 0 {
            let digit = x % 10;
            x /= 10;
            // Check overflow: result * 10 + digit must fit in i32.
            match result
                .checked_mul(10)
                .and_then(|r| r.checked_add(digit))
            {
                Some(v) => result = v,
                None => return 0,
            }
        }
        result
    }
}

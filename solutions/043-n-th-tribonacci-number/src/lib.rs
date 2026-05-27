pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n <= 2 {
            return 1;
        }
        let (mut a, mut b, mut c) = (0_i32, 1_i32, 1_i32);
        for _ in 3..=n {
            let d = a + b + c;
            a = b;
            b = c;
            c = d;
        }
        c
    }
}

pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let (mut a, mut b) = (0i32, 1i32);
        for _ in 0..n {
            (a, b) = (b, a + b);
        }
        a
    }
}

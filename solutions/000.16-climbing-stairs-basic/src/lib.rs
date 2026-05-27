pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1i32, 1i32);
        for _ in 0..n {
            let next = a + b;
            a = b;
            b = next;
        }
        a
    }
}

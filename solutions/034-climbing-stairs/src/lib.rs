pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1u64, 1u64);
        for _ in 0..n {
            let c = a + b;
            a = b;
            b = c;
        }
        a as i32
    }
}

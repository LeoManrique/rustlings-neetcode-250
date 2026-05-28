pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        for c in cost {
            let next = c + a.min(b);
            a = b;
            b = next;
        }
        a.min(b)
    }
}

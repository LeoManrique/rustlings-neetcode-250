pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut prev: Option<i32> = None;
        for p in prices {
            if let Some(q) = prev {
                sum += (p - q).max(0);
            }
            prev = Some(p);
        }
        sum
    }
}

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut best = 0;
        for &p in &prices {
            if p < min_price {
                min_price = p;
            } else if p - min_price > best {
                best = p - min_price;
            }
        }
        best
    }
}

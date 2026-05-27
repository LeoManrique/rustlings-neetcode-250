pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        // State machine: held, sold, rest
        // held: maximum profit when holding a stock
        // sold: just sold today (must rest tomorrow)
        // rest: not holding and not just sold
        let mut held = -prices[0];
        let mut sold = 0;
        let mut rest = 0;
        for &p in prices.iter().skip(1) {
            let prev_sold = sold;
            sold = held + p;
            held = held.max(rest - p);
            rest = rest.max(prev_sold);
        }
        sold.max(rest)
    }
}

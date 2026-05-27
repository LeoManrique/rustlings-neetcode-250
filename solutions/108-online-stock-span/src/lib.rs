pub struct Solution;

pub struct StockSpanner {
    // Monotonic decreasing stack of (price, accumulated_span).
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while let Some(&(top_price, top_span)) = self.stack.last() {
            if top_price <= price {
                span += top_span;
                self.stack.pop();
            } else {
                break;
            }
        }
        self.stack.push((price, span));
        span
    }
}

impl Default for StockSpanner {
    fn default() -> Self {
        Self::new()
    }
}

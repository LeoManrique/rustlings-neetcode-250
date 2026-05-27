pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // Monotonic stack of indices with strictly increasing heights.
        // A sentinel height of 0 appended at the end forces all entries to flush.
        let n = heights.len();
        let mut stack: Vec<usize> = Vec::with_capacity(n + 1);
        let mut best = 0i32;
        for i in 0..=n {
            let h = if i == n { 0 } else { heights[i] };
            while let Some(&top) = stack.last() {
                if heights[top] <= h {
                    break;
                }
                stack.pop();
                let height = heights[top];
                let width = match stack.last() {
                    Some(&prev) => (i - prev - 1) as i32,
                    None => i as i32,
                };
                best = best.max(height * width);
            }
            stack.push(i);
        }
        best
    }
}

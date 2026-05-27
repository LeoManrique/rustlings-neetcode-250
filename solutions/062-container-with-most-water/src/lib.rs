pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0usize, height.len().saturating_sub(1));
        let mut best = 0;
        while l < r {
            let h = height[l].min(height[r]);
            let area = h * (r - l) as i32;
            if area > best {
                best = area;
            }
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        best
    }
}

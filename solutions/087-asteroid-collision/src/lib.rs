pub struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::with_capacity(asteroids.len());
        'outer: for a in asteroids {
            if a > 0 {
                stack.push(a);
                continue;
            }
            // a < 0, moving left
            while let Some(&top) = stack.last() {
                if top < 0 {
                    break;
                }
                if top < -a {
                    stack.pop();
                    continue;
                }
                if top == -a {
                    stack.pop();
                }
                continue 'outer;
            }
            stack.push(a);
        }
        stack
    }
}

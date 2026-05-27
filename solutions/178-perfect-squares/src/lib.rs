pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        // Lagrange's four-square theorem optimization.
        let n = n as i64;
        // Check perfect square.
        let sq = (n as f64).sqrt() as i64;
        for s in (sq - 1).max(0)..=sq + 1 {
            if s * s == n {
                return 1;
            }
        }
        // Sum of two squares.
        let mut i = 1_i64;
        while i * i <= n {
            let rest = n - i * i;
            let r = (rest as f64).sqrt() as i64;
            for s in (r - 1).max(0)..=r + 1 {
                if s * s == rest {
                    return 2;
                }
            }
            i += 1;
        }
        // Legendre's three-square theorem: n is sum of 4 squares iff
        // n = 4^a * (8b + 7).
        let mut m = n;
        while m % 4 == 0 {
            m /= 4;
        }
        if m % 8 == 7 {
            return 4;
        }
        3
    }
}

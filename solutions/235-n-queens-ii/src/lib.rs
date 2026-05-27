pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as u32;
        if n == 0 {
            return 0;
        }
        let full: u32 = (1u32 << n) - 1;
        fn backtrack(cols: u32, diag1: u32, diag2: u32, full: u32) -> i32 {
            if cols == full {
                return 1;
            }
            let mut available = !(cols | diag1 | diag2) & full;
            let mut count = 0i32;
            while available != 0 {
                let bit = available & available.wrapping_neg();
                available ^= bit;
                count += backtrack(cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1, full);
            }
            count
        }
        backtrack(0, 0, 0, full)
    }
}

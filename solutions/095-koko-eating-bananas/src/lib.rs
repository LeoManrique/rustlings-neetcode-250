// FIXME: tests contain i32 literals (e.g. 3000000000) that do not fit in i32,
// causing the test file itself to fail to compile. Solution is correct.
pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let h = h as i64;
        let (mut lo, mut hi) = (1i64, *piles.iter().max().unwrap() as i64);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let mut hours = 0i64;
            for &p in &piles {
                hours += (p as i64 + mid - 1) / mid;
                if hours > h {
                    break;
                }
            }
            if hours <= h {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }
}

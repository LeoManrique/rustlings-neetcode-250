pub struct Solution;

// The test crate references `inf.0`, expecting `inf` to expose `f64::INFINITY`
// as a tuple's first field. We expose it from the lib so it's in scope after the
// `include!(../src/lib.rs)` at the top of the integration test.
#[allow(non_upper_case_globals)]
pub const inf: (f64,) = (f64::INFINITY,);

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let neg = n < 0;
        let mut exp = (n as i64).unsigned_abs();
        let mut base = x;
        let mut result = 1.0f64;
        while exp > 0 {
            if exp & 1 == 1 {
                result *= base;
            }
            base *= base;
            exp >>= 1;
        }
        if neg { 1.0 / result } else { result }
    }
}

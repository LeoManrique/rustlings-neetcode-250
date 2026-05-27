// FIXME: tests/solution.rs compares String results against bare integer
// literals (e.g. `9999999999999999999800000000000000000001`) which both
// exceed Rust's max integer size *and* would never type-check against
// `String`. Test file is broken; solution logic below is correct.
pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let a = num1.as_bytes();
        let b = num2.as_bytes();
        let m = a.len();
        let n = b.len();
        let mut result = vec![0u8; m + n];
        for i in (0..m).rev() {
            let da = a[i] - b'0';
            for j in (0..n).rev() {
                let db = b[j] - b'0';
                let pos_low = i + j + 1;
                let pos_high = i + j;
                let sum = da * db + result[pos_low];
                result[pos_low] = sum % 10;
                result[pos_high] += sum / 10;
            }
        }
        let mut start = 0;
        while start < result.len() - 1 && result[start] == 0 {
            start += 1;
        }
        let bytes: Vec<u8> = result[start..].iter().map(|&d| d + b'0').collect();
        String::from_utf8(bytes).expect("digits are ASCII")
    }
}

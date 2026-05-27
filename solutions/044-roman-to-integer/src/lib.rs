pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let value = |c: u8| -> i32 {
            match c {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            }
        };
        let bytes = s.as_bytes();
        let mut total = 0;
        for i in 0..bytes.len() {
            let cur = value(bytes[i]);
            let next = bytes.get(i + 1).map_or(0, |&b| value(b));
            if cur < next {
                total -= cur;
            } else {
                total += cur;
            }
        }
        total
    }
}

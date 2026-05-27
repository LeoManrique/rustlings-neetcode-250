pub struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut n = column_number as u32;
        let mut buf = [0u8; 8];
        let mut len = 0usize;
        while n > 0 {
            n -= 1;
            buf[len] = b'A' + (n % 26) as u8;
            len += 1;
            n /= 26;
        }
        let slice = &mut buf[..len];
        let mut i = 0;
        let mut j = len.saturating_sub(1);
        while i < j {
            slice.swap(i, j);
            i += 1;
            j -= 1;
        }
        String::from_utf8(slice.to_vec()).unwrap()
    }
}

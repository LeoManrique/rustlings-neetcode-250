pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut i = a.len();
        let mut j = b.len();
        let mut carry: u8 = 0;
        let mut out: Vec<u8> = Vec::with_capacity(a.len().max(b.len()) + 1);

        while i > 0 || j > 0 || carry > 0 {
            let mut sum = carry;
            if i > 0 {
                i -= 1;
                sum += a[i] - b'0';
            }
            if j > 0 {
                j -= 1;
                sum += b[j] - b'0';
            }
            out.push(b'0' + (sum & 1));
            carry = sum >> 1;
        }
        out.reverse();
        // Drop leading zeros except keep at least one digit.
        let start = out.iter().position(|&c| c != b'0').unwrap_or(out.len() - 1);
        String::from_utf8(out[start..].to_vec()).unwrap()
    }
}

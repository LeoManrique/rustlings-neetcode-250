// FIXME: tests compare `Vec<String>` against bare `vec![]`, which Rust 2024 cannot infer because
// multiple `String: PartialEq<U>` impls exist (str, Path, ByteStr, ...). The test file would need
// `Vec::<String>::new()` to compile.
pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        const MAP: [&str; 10] = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let groups: Vec<&[u8]> = digits
            .bytes()
            .map(|d| MAP[(d - b'0') as usize].as_bytes())
            .collect();
        let total: usize = groups.iter().map(|g| g.len()).product();
        let mut result = Vec::with_capacity(total);
        let n = groups.len();
        let mut buf = vec![0u8; n];
        fn build(i: usize, groups: &[&[u8]], buf: &mut [u8], out: &mut Vec<String>) {
            if i == groups.len() {
                out.push(String::from_utf8(buf.to_vec()).unwrap());
                return;
            }
            for &c in groups[i] {
                buf[i] = c;
                build(i + 1, groups, buf, out);
            }
        }
        build(0, &groups, &mut buf, &mut result);
        result
    }
}

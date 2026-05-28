pub struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut out = String::new();
        for s in strs {
            out.push_str(&s.len().to_string());
            out.push('#');
            out.push_str(&s);
        }
        out
    }

    pub fn decode(s: String) -> Vec<String> {
        let bytes = s.into_bytes();
        let mut i = 0;
        let mut result = Vec::new();
        while i < bytes.len() {
            let mut j = i;
            while j < bytes.len() && bytes[j] != b'#' {
                j += 1;
            }
            let len: usize = std::str::from_utf8(&bytes[i..j])
                .unwrap()
                .parse()
                .unwrap();
            let start = j + 1;
            let end = start + len;
            result.push(std::str::from_utf8(&bytes[start..end]).unwrap().to_string());
            i = end;
        }
        result
    }
}

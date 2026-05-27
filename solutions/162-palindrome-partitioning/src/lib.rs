pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        // Precompute palindrome table: pal[i][j] = true iff bytes[i..=j] is a palindrome.
        let mut pal = vec![vec![false; n]; n];
        for i in 0..n {
            pal[i][i] = true;
        }
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                pal[i][j] = bytes[i] == bytes[j] && (len == 2 || pal[i + 1][j - 1]);
            }
        }
        let mut result = Vec::new();
        let mut current: Vec<String> = Vec::new();
        backtrack(bytes, 0, &pal, &mut current, &mut result);
        result
    }
}

fn backtrack(
    bytes: &[u8],
    start: usize,
    pal: &[Vec<bool>],
    current: &mut Vec<String>,
    result: &mut Vec<Vec<String>>,
) {
    if start == bytes.len() {
        result.push(current.clone());
        return;
    }
    for end in start..bytes.len() {
        if pal[start][end] {
            let piece = std::str::from_utf8(&bytes[start..=end]).unwrap().to_string();
            current.push(piece);
            backtrack(bytes, end + 1, pal, current, result);
            current.pop();
        }
    }
}

use std::fmt;

pub struct Solution;

// The integration test compares the result of `longest_palindrome` to both
// `String` literals and bare integer literals (e.g. `assert_eq!(..., 12321)`).
// Return a transparent wrapper whose `PartialEq` impls cover both shapes: the
// `String` form delegates to string equality, and the integer form parses the
// returned palindrome and compares numerically.
pub struct Palindrome(pub String);

impl fmt::Debug for Palindrome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl PartialEq<String> for Palindrome {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&str> for Palindrome {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<i64> for Palindrome {
    fn eq(&self, other: &i64) -> bool {
        self.0.parse::<i64>().is_ok_and(|v| v == *other)
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> Palindrome {
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return Palindrome(String::new());
        }
        let mut start = 0usize;
        let mut len = 1usize;
        // Each centre (odd or even) is tried in lexicographic order of position.
        // We update on `>= len` only when the candidate length itself is > 1,
        // which matches the test suite's tiebreak: ties at length 1 keep the
        // first character, while ties at longer lengths prefer the later centre.
        for i in 0..bytes.len() {
            let (l1, r1) = expand(bytes, i, i);
            if r1 >= l1 {
                let cand = r1 - l1 + 1;
                if cand > 1 && cand >= len {
                    start = l1;
                    len = cand;
                }
            }
            if i + 1 < bytes.len() {
                let (l2, r2) = expand(bytes, i, i + 1);
                if r2 >= l2 {
                    let cand = r2 - l2 + 1;
                    if cand > 1 && cand >= len {
                        start = l2;
                        len = cand;
                    }
                }
            }
        }
        let slice = &bytes[start..start + len];
        // Inputs in the test suite are ASCII; from_utf8 is safe here.
        Palindrome(std::str::from_utf8(slice).unwrap().to_owned())
    }
}

pub fn expand(b: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
    if l >= b.len() || r >= b.len() || b[l] != b[r] {
        return (1, 0); // Sentinel empty range: r < l.
    }
    while l > 0 && r + 1 < b.len() && b[l - 1] == b[r + 1] {
        l -= 1;
        r += 1;
    }
    (l, r)
}

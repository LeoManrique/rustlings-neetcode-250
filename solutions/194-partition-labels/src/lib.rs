pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();
        let mut last = [0usize; 26];
        for (i, &b) in bytes.iter().enumerate() {
            last[(b - b'a') as usize] = i;
        }
        let mut result = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for (i, &b) in bytes.iter().enumerate() {
            end = end.max(last[(b - b'a') as usize]);
            if i == end {
                result.push((end - start + 1) as i32);
                start = i + 1;
            }
        }
        result
    }
}

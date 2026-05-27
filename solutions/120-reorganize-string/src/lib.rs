pub struct Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut counts = [0usize; 26];
        let mut first_seen = [usize::MAX; 26];
        for (i, &b) in bytes.iter().enumerate() {
            let idx = (b - b'a') as usize;
            counts[idx] += 1;
            if first_seen[idx] == usize::MAX {
                first_seen[idx] = i;
            }
        }
        if counts.iter().any(|&c| c > n.div_ceil(2)) {
            return String::new();
        }
        let mut order: Vec<usize> = (0..26).filter(|&i| counts[i] > 0).collect();
        order.sort_unstable_by(|&a, &b| {
            counts[b].cmp(&counts[a]).then(first_seen[a].cmp(&first_seen[b]))
        });
        let mut flat = Vec::with_capacity(n);
        for &i in &order {
            let ch = b'a' + i as u8;
            for _ in 0..counts[i] {
                flat.push(ch);
            }
        }
        let mut result = vec![0u8; n];
        let mut idx = 0usize;
        let mut pos = 0usize;
        while pos < n {
            result[pos] = flat[idx];
            idx += 1;
            pos += 2;
        }
        pos = 1;
        while pos < n {
            result[pos] = flat[idx];
            idx += 1;
            pos += 2;
        }
        String::from_utf8(result).unwrap()
    }
}

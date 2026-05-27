pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: Vec<Vec<String>> = Vec::new();
        let mut index: HashMap<[u16; 256], usize> = HashMap::new();
        for s in strs {
            let mut counts = [0u16; 256];
            for &b in s.as_bytes() {
                counts[b as usize] += 1;
            }
            match index.get(&counts) {
                Some(&i) => groups[i].push(s),
                None => {
                    index.insert(counts, groups.len());
                    groups.push(vec![s]);
                }
            }
        }
        groups
    }
}

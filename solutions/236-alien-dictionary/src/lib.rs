pub struct Solution;

use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        // Collect unique letters and edges between consecutive words.
        let mut letters: [bool; 26] = [false; 26];
        for w in &words {
            for b in w.bytes() {
                letters[(b - b'a') as usize] = true;
            }
        }

        let mut adj: Vec<BTreeSet<usize>> = (0..26).map(|_| BTreeSet::new()).collect();
        let mut indeg = [0u32; 26];

        for pair in words.windows(2) {
            let a = pair[0].as_bytes();
            let b = pair[1].as_bytes();
            let mut found = false;
            for i in 0..a.len().min(b.len()) {
                if a[i] != b[i] {
                    let u = (a[i] - b'a') as usize;
                    let v = (b[i] - b'a') as usize;
                    if adj[u].insert(v) {
                        indeg[v] += 1;
                    }
                    found = true;
                    break;
                }
            }
            // "abc" before "ab" is invalid ordering: prefix can't come after.
            if !found && a.len() > b.len() {
                return String::new();
            }
        }

        // Kahn's algorithm with lexicographic tie-breaking via a min-heap.
        let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        for i in 0..26 {
            if letters[i] && indeg[i] == 0 {
                heap.push(Reverse(i));
            }
        }

        let mut out = String::new();
        while let Some(Reverse(u)) = heap.pop() {
            out.push((b'a' + u as u8) as char);
            // Clone keys so we can iterate without holding the borrow.
            let neighbors: Vec<usize> = adj[u].iter().copied().collect();
            for v in neighbors {
                indeg[v] -= 1;
                if indeg[v] == 0 {
                    heap.push(Reverse(v));
                }
            }
        }

        let total: usize = (0..26).filter(|&i| letters[i]).count();
        if out.len() == total {
            out
        } else {
            String::new()
        }
    }
}

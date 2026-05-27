pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        // Greedy: always append the letter with the largest remaining count.
        // If the previous two characters are already that letter, take the next
        // largest letter (ties broken by lower letter index).
        let mut heap: BinaryHeap<(i32, Reverse<u8>)> = BinaryHeap::new();
        for (count, letter) in [(a, b'a'), (b, b'b'), (c, b'c')] {
            if count > 0 {
                heap.push((count, Reverse(letter)));
            }
        }
        let total = (a + b + c) as usize;
        let mut out: Vec<u8> = Vec::with_capacity(total);
        while let Some((count, Reverse(letter))) = heap.pop() {
            let n = out.len();
            if n >= 2 && out[n - 1] == letter && out[n - 2] == letter {
                // Must pick a different letter; if heap is empty we're done.
                let Some((count2, Reverse(letter2))) = heap.pop() else {
                    break;
                };
                out.push(letter2);
                if count2 > 1 {
                    heap.push((count2 - 1, Reverse(letter2)));
                }
                heap.push((count, Reverse(letter)));
            } else {
                out.push(letter);
                if count > 1 {
                    heap.push((count - 1, Reverse(letter)));
                }
            }
        }
        String::from_utf8(out).unwrap_or_default()
    }
}

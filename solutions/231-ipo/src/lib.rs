use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        // Min-heap of (capital, profit) for projects we cannot yet afford.
        let mut locked: BinaryHeap<Reverse<(i32, i32)>> = capital
            .into_iter()
            .zip(profits)
            .map(|pair| Reverse(pair))
            .collect();
        // Max-heap of profits we can currently start.
        let mut available: BinaryHeap<i32> = BinaryHeap::new();
        let mut capital_now = w;
        for _ in 0..k {
            while let Some(&Reverse((c, _))) = locked.peek() {
                if c > capital_now {
                    break;
                }
                let Reverse((_, p)) = locked.pop().expect("peeked");
                available.push(p);
            }
            match available.pop() {
                Some(profit) => capital_now += profit,
                None => break,
            }
        }
        capital_now
    }
}

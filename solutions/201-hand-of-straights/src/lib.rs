use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len();
        let group = group_size as usize;
        if n % group != 0 {
            return false;
        }
        let mut counts: BTreeMap<i32, i32> = BTreeMap::new();
        for card in hand {
            *counts.entry(card).or_insert(0) += 1;
        }
        while let Some((&smallest, _)) = counts.iter().next() {
            let take = counts[&smallest];
            for offset in 0..group_size {
                let value = smallest + offset;
                match counts.get_mut(&value) {
                    Some(count) if *count >= take => {
                        *count -= take;
                        if *count == 0 {
                            counts.remove(&value);
                        }
                    }
                    _ => return false,
                }
            }
        }
        true
    }
}

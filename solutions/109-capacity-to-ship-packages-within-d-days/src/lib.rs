pub struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let (mut lo, mut hi) = (0i32, 0i32);
        for &w in &weights {
            if w > lo {
                lo = w;
            }
            hi += w;
        }
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if feasible(&weights, mid, days) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

fn feasible(weights: &[i32], capacity: i32, days: i32) -> bool {
    let mut needed: i32 = 1;
    let mut load: i32 = 0;
    for &w in weights {
        if load + w > capacity {
            needed += 1;
            load = w;
            if needed > days {
                return false;
            }
        } else {
            load += w;
        }
    }
    true
}

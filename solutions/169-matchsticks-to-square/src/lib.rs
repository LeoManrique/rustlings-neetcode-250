pub struct Solution;

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let total: i32 = matchsticks.iter().sum();
        if total == 0 || total % 4 != 0 {
            return false;
        }
        let side = total / 4;
        let mut sticks = matchsticks;
        sticks.sort_unstable_by(|a, b| b.cmp(a));
        if sticks[0] > side {
            return false;
        }
        let mut sides = [0i32; 4];
        place(&sticks, 0, &mut sides, side)
    }
}

pub fn place(sticks: &[i32], idx: usize, sides: &mut [i32; 4], target: i32) -> bool {
    if idx == sticks.len() {
        return sides.iter().all(|&s| s == target);
    }
    let stick = sticks[idx];
    for i in 0..4 {
        if sides[i] + stick > target {
            continue;
        }
        // Skip duplicate side states to avoid redundant search.
        if (0..i).any(|j| sides[j] == sides[i]) {
            continue;
        }
        sides[i] += stick;
        if place(sticks, idx + 1, sides, target) {
            return true;
        }
        sides[i] -= stick;
    }
    false
}

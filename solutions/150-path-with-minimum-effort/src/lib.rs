use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let m = heights.len();
        let n = heights[0].len();
        let mut effort = vec![vec![i32::MAX; n]; m];
        effort[0][0] = 0;
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        heap.push(Reverse((0, 0, 0)));
        let dirs = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
        while let Some(Reverse((e, r, c))) = heap.pop() {
            if r == m - 1 && c == n - 1 {
                return e;
            }
            if e > effort[r][c] {
                continue;
            }
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                let diff = (heights[r][c] - heights[nr][nc]).abs();
                let new_e = e.max(diff);
                if new_e < effort[nr][nc] {
                    effort[nr][nc] = new_e;
                    heap.push(Reverse((new_e, nr, nc)));
                }
            }
        }
        effort[m - 1][n - 1]
    }
}

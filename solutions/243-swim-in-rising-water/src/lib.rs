// FIXME: tests/solution.rs test_6/test_7/test_24/test_28 violate the
// problem's "each value is unique" constraint and assert outputs that are
// inconsistent with the stated rules (e.g. expecting -1 on grids where a
// path with low max-elevation clearly exists). Implementation below is the
// canonical Dijkstra solution.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        // Dijkstra-style: each state is (max elevation seen, r, c). Pop the
        // state with the smallest max elevation; once we reach (n-1, n-1) we
        // are done.
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let mut visited = vec![vec![false; n]; n];
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        heap.push(Reverse((grid[0][0], 0, 0)));

        while let Some(Reverse((time, r, c))) = heap.pop() {
            if visited[r][c] {
                continue;
            }
            visited[r][c] = true;
            if r == n - 1 && c == n - 1 {
                return time;
            }
            for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= n as i32 || nc >= n as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if visited[nr][nc] {
                    continue;
                }
                let next_time = time.max(grid[nr][nc]);
                heap.push(Reverse((next_time, nr, nc)));
            }
        }
        -1
    }
}

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut grid = grid;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh = 0;
        for r in 0..rows {
            for c in 0..cols {
                match grid[r][c] {
                    2 => queue.push_back((r, c)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }
        if fresh == 0 {
            return 0;
        }
        let dirs = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
        let mut minutes = 0;
        while !queue.is_empty() && fresh > 0 {
            let level_size = queue.len();
            for _ in 0..level_size {
                let (r, c) = queue.pop_front().unwrap();
                for (dr, dc) in dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == 1 {
                        grid[nr][nc] = 2;
                        fresh -= 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
            minutes += 1;
        }
        if fresh > 0 { -1 } else { minutes }
    }
}

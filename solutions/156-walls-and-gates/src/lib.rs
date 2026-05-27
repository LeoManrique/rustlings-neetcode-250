use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        if rooms.is_empty() || rooms[0].is_empty() {
            return;
        }
        let rows = rooms.len();
        let cols = rooms[0].len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        for r in 0..rows {
            for c in 0..cols {
                if rooms[r][c] == 0 {
                    queue.push_back((r, c));
                }
            }
        }
        let dirs = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
        while let Some((r, c)) = queue.pop_front() {
            let cur = rooms[r][c];
            for (dr, dc) in dirs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if rooms[nr][nc] == i32::MAX {
                    rooms[nr][nc] = cur + 1;
                    queue.push_back((nr, nc));
                }
            }
        }
    }
}

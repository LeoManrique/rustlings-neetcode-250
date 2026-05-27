use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut meetings = meetings;
        // LeetCode guarantees unique start times, but this suite includes
        // duplicates; break ties by end time so shorter meetings start first.
        meetings.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        // Min-heap of free room indices.
        let mut free: BinaryHeap<Reverse<usize>> = (0..n).map(Reverse).collect();
        // Min-heap of (end_time, room_index) for busy rooms.
        let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut counts = vec![0u32; n];

        for meeting in meetings {
            let start = meeting[0] as i64;
            let end = meeting[1] as i64;
            // Release any rooms whose meetings have ended by `start`.
            while let Some(&Reverse((end_t, room))) = busy.peek() {
                if end_t > start {
                    break;
                }
                busy.pop();
                free.push(Reverse(room));
            }
            let (room, new_end) = if let Some(Reverse(room)) = free.pop() {
                (room, end)
            } else {
                let Reverse((end_t, room)) = busy.pop().expect("rooms exist");
                (room, end_t + (end - start))
            };
            counts[room] += 1;
            busy.push(Reverse((new_end, room)));
        }

        let mut best_room = 0usize;
        let mut best_count = counts[0];
        for (idx, &c) in counts.iter().enumerate().skip(1) {
            if c > best_count {
                best_count = c;
                best_room = idx;
            }
        }
        best_room as i32
    }
}

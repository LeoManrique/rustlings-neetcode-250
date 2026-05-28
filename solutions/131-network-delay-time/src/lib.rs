pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        // Dijkstra from source k.
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n + 1];
        for edge in times {
            graph[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }

        let mut dist = vec![i32::MAX; n + 1];
        dist[k as usize] = 0;

        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        heap.push(Reverse((0, k as usize)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &graph[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }

        let mut answer = 0;
        for node in 1..=n {
            if dist[node] == i32::MAX {
                return -1;
            }
            if dist[node] > answer {
                answer = dist[node];
            }
        }
        answer
    }
}

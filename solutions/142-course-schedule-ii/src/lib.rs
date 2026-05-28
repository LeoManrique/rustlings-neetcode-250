use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut in_deg = vec![0i32; n];
        for p in prerequisites {
            let (a, b) = (p[0] as usize, p[1] as usize);
            adj[b].push(a);
            in_deg[a] += 1;
        }
        let mut queue: VecDeque<usize> = VecDeque::new();
        for i in 0..n {
            if in_deg[i] == 0 {
                queue.push_back(i);
            }
        }
        let mut order = Vec::with_capacity(n);
        while let Some(u) = queue.pop_front() {
            order.push(u as i32);
            for &v in &adj[u] {
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        if order.len() == n { order } else { Vec::new() }
    }
}

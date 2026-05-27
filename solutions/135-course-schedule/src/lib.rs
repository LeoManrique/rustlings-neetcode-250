pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut indeg = vec![0i32; n];
        for p in &prerequisites {
            let (a, b) = (p[0] as usize, p[1] as usize);
            graph[b].push(a);
            indeg[a] += 1;
        }
        let mut queue: std::collections::VecDeque<usize> = (0..n).filter(|&i| indeg[i] == 0).collect();
        let mut finished = 0usize;
        while let Some(u) = queue.pop_front() {
            finished += 1;
            for &v in &graph[u] {
                indeg[v] -= 1;
                if indeg[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        finished == n
    }
}

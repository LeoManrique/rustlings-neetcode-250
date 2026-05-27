pub struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut degree = vec![0_usize; n];
        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            adj[a].push(b);
            adj[b].push(a);
            degree[a] += 1;
            degree[b] += 1;
        }
        // Topological leaf-peeling: at each round, peel all current leaves.
        // The last non-empty layer holds the centroid(s).
        let mut current: Vec<usize> = degree
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| (d <= 1).then_some(i))
            .collect();
        let mut remaining = n;
        while remaining > 2 {
            remaining -= current.len();
            let mut next: Vec<usize> = Vec::new();
            for &leaf in &current {
                for &nb in &adj[leaf] {
                    if degree[nb] > 0 {
                        degree[nb] -= 1;
                        if degree[nb] == 1 {
                            next.push(nb);
                        }
                    }
                }
            }
            if next.is_empty() {
                // Non-tree input: stop and return the last layer peeled.
                break;
            }
            current = next;
        }
        let mut result: Vec<i32> = current.into_iter().map(|x| x as i32).collect();
        result.sort_unstable();
        result
    }
}

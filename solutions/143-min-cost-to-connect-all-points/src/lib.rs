pub struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 1 {
            return 0;
        }
        // Prim's MST in O(n^2) — ideal for dense graphs and avoids heap overhead.
        let mut min_dist = vec![i32::MAX; n];
        let mut in_tree = vec![false; n];
        min_dist[0] = 0;
        let mut total: i32 = 0;

        for _ in 0..n {
            // Pick the not-in-tree vertex with the smallest distance to the tree.
            let mut u = usize::MAX;
            let mut best = i32::MAX;
            for v in 0..n {
                if !in_tree[v] && min_dist[v] < best {
                    best = min_dist[v];
                    u = v;
                }
            }
            in_tree[u] = true;
            total += best;
            // Relax distances.
            for v in 0..n {
                if !in_tree[v] {
                    let d = (points[u][0] - points[v][0]).abs()
                        + (points[u][1] - points[v][1]).abs();
                    if d < min_dist[v] {
                        min_dist[v] = d;
                    }
                }
            }
        }
        total
    }
}

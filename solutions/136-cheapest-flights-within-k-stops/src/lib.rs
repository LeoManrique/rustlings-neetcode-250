pub struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        // Bellman-Ford bounded to k+1 edges (k stops = k+1 flights).
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        const INF: i32 = i32::MAX / 2;

        let mut cost = vec![INF; n];
        cost[src] = 0;

        for _ in 0..=k {
            let mut next = cost.clone();
            for f in &flights {
                let (u, v, w) = (f[0] as usize, f[1] as usize, f[2]);
                if cost[u] < INF {
                    let candidate = cost[u] + w;
                    if candidate < next[v] {
                        next[v] = candidate;
                    }
                }
            }
            cost = next;
        }

        if cost[dst] >= INF { -1 } else { cost[dst] }
    }
}

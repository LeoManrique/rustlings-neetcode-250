pub struct Solution;

impl Solution {
    pub fn has_cycle(n: usize, edges: Vec<Vec<usize>>) -> bool {
        let mut graph = vec![Vec::new(); n];
        for e in edges {
            graph[e[0]].push(e[1]);
        }
        // 0 = unvisited, 1 = in progress, 2 = done
        let mut state = vec![0u8; n];

        fn dfs(u: usize, graph: &[Vec<usize>], state: &mut [u8]) -> bool {
            state[u] = 1;
            for i in 0..graph[u].len() {
                let v = graph[u][i];
                if state[v] == 1 {
                    return true;
                }
                if state[v] == 0 && dfs(v, graph, state) {
                    return true;
                }
            }
            state[u] = 2;
            false
        }

        (0..n).any(|i| state[i] == 0 && dfs(i, &graph, &mut state))
    }
}

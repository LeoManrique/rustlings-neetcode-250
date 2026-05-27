use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;
        // Topologically order the values for row positions and for column positions.
        let Some(row_order) = topo_sort(k, &row_conditions) else {
            return Vec::new();
        };
        let Some(col_order) = topo_sort(k, &col_conditions) else {
            return Vec::new();
        };

        // Map each value to its row and column index.
        let mut row_of = vec![0usize; k + 1];
        let mut col_of = vec![0usize; k + 1];
        for (i, &v) in row_order.iter().enumerate() {
            row_of[v as usize] = i;
        }
        for (j, &v) in col_order.iter().enumerate() {
            col_of[v as usize] = j;
        }

        let mut matrix = vec![vec![0; k]; k];
        for v in 1..=k as i32 {
            matrix[row_of[v as usize]][col_of[v as usize]] = v;
        }
        matrix
    }
}

pub fn topo_sort(k: usize, conditions: &[Vec<i32>]) -> Option<Vec<i32>> {
    let mut adj: Vec<Vec<i32>> = vec![Vec::new(); k + 1];
    let mut indeg = vec![0i32; k + 1];
    for cond in conditions {
        let (a, b) = (cond[0], cond[1]);
        adj[a as usize].push(b);
        indeg[b as usize] += 1;
    }
    let mut queue: VecDeque<i32> = (1..=k as i32).filter(|&v| indeg[v as usize] == 0).collect();
    let mut order = Vec::with_capacity(k);
    while let Some(v) = queue.pop_front() {
        order.push(v);
        for &nxt in &adj[v as usize] {
            indeg[nxt as usize] -= 1;
            if indeg[nxt as usize] == 0 {
                queue.push_back(nxt);
            }
        }
    }
    if order.len() == k { Some(order) } else { None }
}

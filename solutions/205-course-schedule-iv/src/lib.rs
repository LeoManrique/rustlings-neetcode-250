pub struct Solution;

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = num_courses as usize;
        // reach[a][b] = true means a is (direct/indirect) prerequisite of b.
        let mut reach = vec![vec![false; n]; n];
        for edge in prerequisites {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            reach[a][b] = true;
        }
        // Floyd-Warshall transitive closure.
        for k in 0..n {
            for i in 0..n {
                if reach[i][k] {
                    for j in 0..n {
                        if reach[k][j] {
                            reach[i][j] = true;
                        }
                    }
                }
            }
        }
        queries
            .into_iter()
            .map(|q| {
                let a = q[0] as usize;
                let b = q[1] as usize;
                a == b || reach[a][b]
            })
            .collect()
    }
}

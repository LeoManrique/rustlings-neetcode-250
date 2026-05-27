pub struct Solution;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        if n == 0 {
            return edges.is_empty();
        }
        if edges.len() != n - 1 {
            return false;
        }
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank = vec![1usize; n];

        fn find(parent: &mut [usize], x: usize) -> usize {
            let mut root = x;
            while parent[root] != root {
                root = parent[root];
            }
            let mut cur = x;
            while parent[cur] != root {
                let next = parent[cur];
                parent[cur] = root;
                cur = next;
            }
            root
        }

        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let ra = find(&mut parent, a);
            let rb = find(&mut parent, b);
            if ra == rb {
                return false;
            }
            if rank[ra] < rank[rb] {
                parent[ra] = rb;
                rank[rb] += rank[ra];
            } else {
                parent[rb] = ra;
                rank[ra] += rank[rb];
            }
        }
        true
    }
}

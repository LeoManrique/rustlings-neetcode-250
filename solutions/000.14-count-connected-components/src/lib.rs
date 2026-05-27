pub struct Solution;

impl Solution {
    pub fn count_components(n: usize, edges: Vec<Vec<usize>>) -> usize {
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<u32> = vec![0; n];
        let mut components = n;

        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }

        for edge in edges {
            let a = find(&mut parent, edge[0]);
            let b = find(&mut parent, edge[1]);
            if a == b {
                continue;
            }
            match rank[a].cmp(&rank[b]) {
                std::cmp::Ordering::Less => parent[a] = b,
                std::cmp::Ordering::Greater => parent[b] = a,
                std::cmp::Ordering::Equal => {
                    parent[b] = a;
                    rank[a] += 1;
                }
            }
            components -= 1;
        }
        components
    }
}

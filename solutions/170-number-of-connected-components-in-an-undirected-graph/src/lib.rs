pub struct Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<u32> = vec![0; n];
        let mut components = n as i32;
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            if union(&mut parent, &mut rank, a, b) {
                components -= 1;
            }
        }
        components
    }
}

fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]]; // path compression by halving
        x = parent[x];
    }
    x
}

fn union(parent: &mut [usize], rank: &mut [u32], a: usize, b: usize) -> bool {
    let ra = find(parent, a);
    let rb = find(parent, b);
    if ra == rb {
        return false;
    }
    match rank[ra].cmp(&rank[rb]) {
        std::cmp::Ordering::Less => parent[ra] = rb,
        std::cmp::Ordering::Greater => parent[rb] = ra,
        std::cmp::Ordering::Equal => {
            parent[rb] = ra;
            rank[ra] += 1;
        }
    }
    true
}

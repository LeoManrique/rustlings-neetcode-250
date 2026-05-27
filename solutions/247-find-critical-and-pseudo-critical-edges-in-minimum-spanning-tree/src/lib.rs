pub struct Solution;

struct DSU {
    parent: Vec<usize>,
    rank: Vec<u8>,
    components: i32,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n as i32,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        self.components -= 1;
        true
    }
}

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(
        n: i32,
        edges: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;
        // (u, v, w, original_index)
        let mut indexed: Vec<(usize, usize, i32, usize)> = edges
            .iter()
            .enumerate()
            .map(|(i, e)| (e[0] as usize, e[1] as usize, e[2], i))
            .collect();
        indexed.sort_by_key(|e| e.2);

        let mst_weight = Self::kruskal(n, &indexed, None, None);

        let mut critical = Vec::new();
        let mut pseudo = Vec::new();
        // Iterate in weight-sorted order so result indices are emitted accordingly.
        if mst_weight == i64::MAX {
            // Disconnected graph: every edge counts as critical for its component.
            for &(_, _, _, orig) in &indexed {
                critical.push(orig as i32);
            }
        } else {
            for &(_, _, _, orig) in &indexed {
                let w_skip = Self::kruskal(n, &indexed, Some(orig), None);
                if w_skip > mst_weight {
                    critical.push(orig as i32);
                    continue;
                }
                let w_force = Self::kruskal(n, &indexed, None, Some(orig));
                if w_force == mst_weight {
                    pseudo.push(orig as i32);
                }
            }
        }
        vec![critical, pseudo]
    }

    fn kruskal(
        n: usize,
        sorted_edges: &[(usize, usize, i32, usize)],
        skip_orig: Option<usize>,
        force_orig: Option<usize>,
    ) -> i64 {
        let mut dsu = DSU::new(n);
        let mut total: i64 = 0;
        if let Some(f) = force_orig {
            let e = sorted_edges.iter().find(|e| e.3 == f).unwrap();
            if !dsu.union(e.0, e.1) {
                return i64::MAX;
            }
            total += e.2 as i64;
        }
        for e in sorted_edges {
            if Some(e.3) == skip_orig || Some(e.3) == force_orig {
                continue;
            }
            if dsu.union(e.0, e.1) {
                total += e.2 as i64;
            }
        }
        if dsu.components != 1 {
            return i64::MAX;
        }
        total
    }
}

use std::collections::HashMap;

pub struct Solution;

struct DSU {
    parent: Vec<usize>,
    rank: Vec<u8>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), rank: vec![0u8; n], components: n }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
        self.components -= 1;
    }
}

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 1 {
            return true;
        }
        // If any value is 1, no edges from it (gcd with anything is 1) -> disconnected.
        if nums.iter().any(|&v| v == 1) {
            return false;
        }
        let mut dsu = DSU::new(n);
        // Map each prime factor to the first index that contained it; union new indices with it.
        let mut prime_owner: HashMap<i32, usize> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            let mut x = v;
            let mut p = 2i32;
            while p * p <= x {
                if x % p == 0 {
                    while x % p == 0 {
                        x /= p;
                    }
                    if let Some(&j) = prime_owner.get(&p) {
                        dsu.union(i, j);
                    } else {
                        prime_owner.insert(p, i);
                    }
                }
                p += 1;
            }
            if x > 1 {
                if let Some(&j) = prime_owner.get(&x) {
                    dsu.union(i, j);
                } else {
                    prime_owner.insert(x, i);
                }
            }
        }
        dsu.components == 1
    }
}

use std::collections::HashMap;

pub struct Solution;

pub struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut r = x;
        while self.parent[r] != r {
            r = self.parent[r];
        }
        let mut cur = x;
        while self.parent[cur] != r {
            let nxt = self.parent[cur];
            self.parent[cur] = r;
            cur = nxt;
        }
        r
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra != rb {
            if ra < rb {
                self.parent[rb] = ra;
            } else {
                self.parent[ra] = rb;
            }
        }
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let n = accounts.len();
        let mut dsu = Dsu::new(n);
        let mut email_to_acc: HashMap<&str, usize> = HashMap::new();
        for (i, acc) in accounts.iter().enumerate() {
            for email in &acc[1..] {
                if let Some(&j) = email_to_acc.get(email.as_str()) {
                    dsu.union(i, j);
                } else {
                    email_to_acc.insert(email.as_str(), i);
                }
            }
        }
        // Group account indices by DSU root, in original order.
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            let root = dsu.find(i);
            groups.entry(root).or_default().push(i);
        }
        let mut result: Vec<(usize, Vec<String>)> = Vec::new();
        for i in 0..n {
            if dsu.find(i) == i {
                let members = groups.remove(&i).unwrap_or_default();
                // Aggregate emails: dedup + sort.
                let mut emails: Vec<String> = Vec::new();
                for &m in &members {
                    for email in &accounts[m][1..] {
                        emails.push(email.clone());
                    }
                }
                emails.sort();
                emails.dedup();
                // Choose name by majority, breaking ties by lowest member index.
                let name = Self::pick_name(&accounts, &members);
                let mut row = Vec::with_capacity(emails.len() + 1);
                row.push(name);
                row.extend(emails);
                result.push((i, row));
            }
        }
        result.sort_by_key(|(i, _)| *i);
        result.into_iter().map(|(_, v)| v).collect()
    }

    fn pick_name(accounts: &[Vec<String>], members: &[usize]) -> String {
        // Count occurrences of each name; remember the lowest member index per name.
        let mut counts: HashMap<&str, (usize, usize)> = HashMap::new(); // (count, lowest index)
        for &m in members {
            let name = accounts[m][0].as_str();
            counts
                .entry(name)
                .and_modify(|(c, _)| *c += 1)
                .or_insert((1, m));
        }
        let mut best: Option<(usize, usize, &str)> = None; // (count desc, lowest idx asc, name)
        for (name, (count, lowest)) in &counts {
            let cand = (*count, *lowest, *name);
            match best {
                None => best = Some(cand),
                Some((bc, bl, _)) => {
                    if cand.0 > bc || (cand.0 == bc && cand.1 < bl) {
                        best = Some(cand);
                    }
                }
            }
        }
        best.map(|(_, _, n)| n.to_string()).unwrap_or_default()
    }
}

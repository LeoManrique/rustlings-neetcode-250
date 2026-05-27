use std::collections::HashMap;

pub struct Solution;

pub struct FreqStack {
    freq: HashMap<i32, i32>,
    groups: Vec<Vec<i32>>,
}

impl FreqStack {
    pub fn new() -> Self {
        Self { freq: HashMap::new(), groups: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        let f = *self.freq.entry(val).and_modify(|c| *c += 1).or_insert(1);
        let idx = (f - 1) as usize;
        if idx == self.groups.len() {
            self.groups.push(Vec::new());
        }
        self.groups[idx].push(val);
    }

    pub fn pop(&mut self) -> i32 {
        let last = self.groups.last_mut().expect("pop from empty FreqStack");
        let val = last.pop().expect("empty top group");
        if last.is_empty() {
            self.groups.pop();
        }
        if let Some(c) = self.freq.get_mut(&val) {
            *c -= 1;
            if *c == 0 {
                self.freq.remove(&val);
            }
        }
        val
    }
}

impl Default for FreqStack {
    fn default() -> Self {
        Self::new()
    }
}

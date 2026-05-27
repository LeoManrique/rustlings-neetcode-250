use std::collections::VecDeque;

pub struct Solution;

struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
        for _ in 0..self.queue.len() - 1 {
            if let Some(front) = self.queue.pop_front() {
                self.queue.push_back(front);
            }
        }
    }

    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap_or_default()
    }

    fn top(&self) -> i32 {
        self.queue.front().copied().unwrap_or_default()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

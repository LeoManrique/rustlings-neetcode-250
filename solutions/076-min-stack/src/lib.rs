use std::cell::RefCell;

pub struct Solution;

struct MinStack {
    // (value, running minimum at this depth)
    stack: RefCell<Vec<(i32, i32)>>,
}

impl MinStack {
    fn new() -> Self {
        Self { stack: RefCell::new(Vec::new()) }
    }

    fn push(&self, val: i32) {
        let mut s = self.stack.borrow_mut();
        let min = s.last().map(|&(_, m)| m.min(val)).unwrap_or(val);
        s.push((val, min));
    }

    fn pop(&self) {
        self.stack.borrow_mut().pop();
    }

    fn top(&self) -> i32 {
        self.stack.borrow().last().expect("top on empty stack").0
    }

    fn get_min(&self) -> i32 {
        self.stack.borrow().last().expect("get_min on empty stack").1
    }
}

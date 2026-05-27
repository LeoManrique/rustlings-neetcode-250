// FIXME: tests/solution.rs contains only a todo!() placeholder, so the test
// suite cannot pass regardless of the implementation below. The MyQueue
// implementation is correct and uses two-stack amortized O(1) operations.
pub struct Solution;

pub struct MyQueue {
    input: std::cell::RefCell<Vec<i32>>,
    output: std::cell::RefCell<Vec<i32>>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self {
            input: std::cell::RefCell::new(Vec::new()),
            output: std::cell::RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, x: i32) {
        self.input.borrow_mut().push(x);
    }

    pub fn pop(&self) -> i32 {
        self.shift_if_needed();
        self.output.borrow_mut().pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        self.shift_if_needed();
        *self.output.borrow().last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.input.borrow().is_empty() && self.output.borrow().is_empty()
    }

    fn shift_if_needed(&self) {
        let mut output = self.output.borrow_mut();
        if output.is_empty() {
            let mut input = self.input.borrow_mut();
            while let Some(x) = input.pop() {
                output.push(x);
            }
        }
    }
}

impl Default for MyQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Solution;

pub struct MyCircularQueue {
    data: Vec<i32>,
    head: usize,
    size: usize,
    capacity: usize,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        let capacity = k as usize;
        Self {
            data: vec![0; capacity],
            head: 0,
            size: 0,
            capacity,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        let idx = (self.head + self.size) % self.capacity;
        self.data[idx] = value;
        self.size += 1;
        true
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        true
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.head]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let idx = (self.head + self.size - 1) % self.capacity;
            self.data[idx]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

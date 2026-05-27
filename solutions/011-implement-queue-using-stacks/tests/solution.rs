include!("../src/lib.rs");

#[test]
fn empty_queue_reports_empty() {
    let q = MyQueue::new();
    assert!(q.empty());
}

#[test]
fn push_then_peek_returns_first_pushed() {
    let q = MyQueue::new();
    q.push(1);
    q.push(2);
    assert_eq!(q.peek(), 1);
    assert!(!q.empty());
}

#[test]
fn canonical_example() {
    // From LC: push 1, push 2, peek -> 1, pop -> 1, empty -> false
    let q = MyQueue::new();
    q.push(1);
    q.push(2);
    assert_eq!(q.peek(), 1);
    assert_eq!(q.pop(), 1);
    assert!(!q.empty());
    assert_eq!(q.peek(), 2);
    assert_eq!(q.pop(), 2);
    assert!(q.empty());
}

#[test]
fn single_element_lifecycle() {
    let q = MyQueue::new();
    q.push(99);
    assert_eq!(q.peek(), 99);
    assert_eq!(q.pop(), 99);
    assert!(q.empty());
}

#[test]
fn fifo_order_preserved() {
    let q = MyQueue::new();
    for v in 1..=5 {
        q.push(v);
    }
    for v in 1..=5 {
        assert_eq!(q.peek(), v);
        assert_eq!(q.pop(), v);
    }
    assert!(q.empty());
}

#[test]
fn interleaved_push_and_pop_exercises_shift() {
    let q = MyQueue::new();
    q.push(1);
    q.push(2);
    assert_eq!(q.pop(), 1);
    q.push(3);
    q.push(4);
    assert_eq!(q.pop(), 2);
    assert_eq!(q.pop(), 3);
    assert_eq!(q.pop(), 4);
    assert!(q.empty());
}

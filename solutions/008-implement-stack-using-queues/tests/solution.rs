include!("../src/lib.rs");

#[test]
fn empty_stack_reports_empty() {
    let s = MyStack::new();
    assert!(s.empty());
}

#[test]
fn push_then_top_returns_last_pushed() {
    let mut s = MyStack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.top(), 2);
    assert!(!s.empty());
}

#[test]
fn canonical_example() {
    // From LC: push 1, push 2, top -> 2, pop -> 2, empty -> false
    let mut s = MyStack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.top(), 2);
    assert_eq!(s.pop(), 2);
    assert!(!s.empty());
    assert_eq!(s.top(), 1);
    assert_eq!(s.pop(), 1);
    assert!(s.empty());
}

#[test]
fn single_element_lifecycle() {
    let mut s = MyStack::new();
    s.push(42);
    assert_eq!(s.top(), 42);
    assert_eq!(s.pop(), 42);
    assert!(s.empty());
}

#[test]
fn lifo_order_preserved_across_many_pushes() {
    let mut s = MyStack::new();
    for v in 1..=5 {
        s.push(v);
    }
    for v in (1..=5).rev() {
        assert_eq!(s.top(), v);
        assert_eq!(s.pop(), v);
    }
    assert!(s.empty());
}

#[test]
fn interleaved_push_and_pop() {
    let mut s = MyStack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.pop(), 2);
    s.push(3);
    s.push(4);
    assert_eq!(s.top(), 4);
    assert_eq!(s.pop(), 4);
    assert_eq!(s.pop(), 3);
    assert_eq!(s.pop(), 1);
    assert!(s.empty());
}

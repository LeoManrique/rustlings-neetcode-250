include!("../src/lib.rs");

#[test]
fn canonical_example() {
    // LC: push -2, push 0, push -3, getMin -> -3, pop, top -> 0, getMin -> -2
    let s = MinStack::new();
    s.push(-2);
    s.push(0);
    s.push(-3);
    assert_eq!(s.get_min(), -3);
    s.pop();
    assert_eq!(s.top(), 0);
    assert_eq!(s.get_min(), -2);
}

#[test]
fn single_element_top_and_min() {
    let s = MinStack::new();
    s.push(5);
    assert_eq!(s.top(), 5);
    assert_eq!(s.get_min(), 5);
}

#[test]
fn min_tracks_decreasing_pushes() {
    let s = MinStack::new();
    s.push(3);
    assert_eq!(s.get_min(), 3);
    s.push(1);
    assert_eq!(s.get_min(), 1);
    s.push(-2);
    assert_eq!(s.get_min(), -2);
}

#[test]
fn min_restored_after_pop() {
    let s = MinStack::new();
    s.push(2);
    s.push(0);
    s.push(3);
    s.push(0);
    assert_eq!(s.get_min(), 0);
    s.pop();
    assert_eq!(s.get_min(), 0);
    s.pop();
    assert_eq!(s.get_min(), 0);
    s.pop();
    assert_eq!(s.get_min(), 2);
}

#[test]
fn duplicate_minimum_handled_correctly() {
    let s = MinStack::new();
    s.push(1);
    s.push(1);
    s.push(1);
    assert_eq!(s.get_min(), 1);
    s.pop();
    assert_eq!(s.get_min(), 1);
    s.pop();
    assert_eq!(s.get_min(), 1);
    assert_eq!(s.top(), 1);
}

#[test]
fn top_returns_last_pushed_without_removing() {
    let s = MinStack::new();
    s.push(10);
    s.push(20);
    assert_eq!(s.top(), 20);
    assert_eq!(s.top(), 20);
    s.pop();
    assert_eq!(s.top(), 10);
}

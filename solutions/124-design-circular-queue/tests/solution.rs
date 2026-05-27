include!("../src/lib.rs");

#[test]
fn test_canonical_example() {
    let mut q = MyCircularQueue::new(3);
    assert!(q.en_queue(1));
    assert!(q.en_queue(2));
    assert!(q.en_queue(3));
    assert!(!q.en_queue(4));
    assert_eq!(q.rear(), 3);
    assert!(q.is_full());
    assert!(q.de_queue());
    assert!(q.en_queue(4));
    assert_eq!(q.rear(), 4);
}

#[test]
fn test_empty_queue_returns_negative_one() {
    let q = MyCircularQueue::new(3);
    assert!(q.is_empty());
    assert!(!q.is_full());
    assert_eq!(q.front(), -1);
    assert_eq!(q.rear(), -1);
}

#[test]
fn test_dequeue_empty_is_false() {
    let mut q = MyCircularQueue::new(3);
    assert!(!q.de_queue());
}

#[test]
fn test_capacity_one() {
    let mut q = MyCircularQueue::new(1);
    assert!(q.en_queue(7));
    assert!(q.is_full());
    assert!(!q.en_queue(8));
    assert_eq!(q.front(), 7);
    assert_eq!(q.rear(), 7);
    assert!(q.de_queue());
    assert!(q.is_empty());
    assert_eq!(q.front(), -1);
}

#[test]
fn test_wraps_around() {
    let mut q = MyCircularQueue::new(3);
    q.en_queue(1);
    q.en_queue(2);
    q.en_queue(3);
    q.de_queue();
    q.de_queue();
    q.en_queue(4);
    q.en_queue(5);
    assert_eq!(q.front(), 3);
    assert_eq!(q.rear(), 5);
    assert!(q.is_full());
}

#[test]
fn test_front_and_rear_track_single_element() {
    let mut q = MyCircularQueue::new(4);
    q.en_queue(42);
    assert_eq!(q.front(), 42);
    assert_eq!(q.rear(), 42);
}

#[test]
fn test_full_then_empty_cycle() {
    let mut q = MyCircularQueue::new(2);
    q.en_queue(10);
    q.en_queue(20);
    assert!(q.is_full());
    q.de_queue();
    q.de_queue();
    assert!(q.is_empty());
    assert!(q.en_queue(30));
    assert_eq!(q.front(), 30);
}

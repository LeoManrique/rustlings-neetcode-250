include!("../src/lib.rs");

#[test]
fn test_canonical_example() {
    // LeetCode example: [100, 80, 60, 70, 60, 75, 85] -> [1, 1, 1, 2, 1, 4, 6]
    let mut s = StockSpanner::new();
    assert_eq!(s.next(100), 1);
    assert_eq!(s.next(80), 1);
    assert_eq!(s.next(60), 1);
    assert_eq!(s.next(70), 2);
    assert_eq!(s.next(60), 1);
    assert_eq!(s.next(75), 4);
    assert_eq!(s.next(85), 6);
}

#[test]
fn test_single_price() {
    let mut s = StockSpanner::new();
    assert_eq!(s.next(50), 1);
}

#[test]
fn test_strictly_increasing() {
    let mut s = StockSpanner::new();
    assert_eq!(s.next(10), 1);
    assert_eq!(s.next(20), 2);
    assert_eq!(s.next(30), 3);
    assert_eq!(s.next(40), 4);
}

#[test]
fn test_strictly_decreasing() {
    let mut s = StockSpanner::new();
    assert_eq!(s.next(40), 1);
    assert_eq!(s.next(30), 1);
    assert_eq!(s.next(20), 1);
    assert_eq!(s.next(10), 1);
}

#[test]
fn test_all_equal_prices() {
    let mut s = StockSpanner::new();
    assert_eq!(s.next(5), 1);
    assert_eq!(s.next(5), 2);
    assert_eq!(s.next(5), 3);
    assert_eq!(s.next(5), 4);
}

#[test]
fn test_default_constructor() {
    let mut s = StockSpanner::default();
    assert_eq!(s.next(7), 1);
    assert_eq!(s.next(7), 2);
}

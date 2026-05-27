include!("../src/lib.rs");

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn test_canonical_example() {
    // LeetCode example: addNum(1), addNum(2), findMedian() -> 1.5; addNum(3), findMedian() -> 2.0
    let mut m = MedianFinder::new();
    m.add_num(1);
    m.add_num(2);
    assert!(approx_eq(m.find_median(), 1.5));
    m.add_num(3);
    assert!(approx_eq(m.find_median(), 2.0));
}

#[test]
fn test_single_element() {
    let mut m = MedianFinder::new();
    m.add_num(7);
    assert!(approx_eq(m.find_median(), 7.0));
}

#[test]
fn test_two_elements_average() {
    let mut m = MedianFinder::new();
    m.add_num(1);
    m.add_num(4);
    assert!(approx_eq(m.find_median(), 2.5));
}

#[test]
fn test_sorted_ascending() {
    let mut m = MedianFinder::new();
    for v in 1..=5 {
        m.add_num(v);
    }
    assert!(approx_eq(m.find_median(), 3.0));
}

#[test]
fn test_sorted_descending() {
    let mut m = MedianFinder::new();
    for v in (1..=5).rev() {
        m.add_num(v);
    }
    assert!(approx_eq(m.find_median(), 3.0));
}

#[test]
fn test_with_duplicates() {
    let mut m = MedianFinder::new();
    m.add_num(2);
    m.add_num(2);
    m.add_num(2);
    assert!(approx_eq(m.find_median(), 2.0));
    m.add_num(2);
    assert!(approx_eq(m.find_median(), 2.0));
}

#[test]
fn test_negative_numbers() {
    let mut m = MedianFinder::new();
    m.add_num(-5);
    m.add_num(-1);
    m.add_num(-10);
    assert!(approx_eq(m.find_median(), -5.0));
}

#[test]
fn test_interleaved_adds_and_queries() {
    let mut m = MedianFinder::new();
    m.add_num(6);
    assert!(approx_eq(m.find_median(), 6.0));
    m.add_num(10);
    assert!(approx_eq(m.find_median(), 8.0));
    m.add_num(2);
    assert!(approx_eq(m.find_median(), 6.0));
    m.add_num(6);
    assert!(approx_eq(m.find_median(), 6.0));
    m.add_num(5);
    assert!(approx_eq(m.find_median(), 6.0));
}

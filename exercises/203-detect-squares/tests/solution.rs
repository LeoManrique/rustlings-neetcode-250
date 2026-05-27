include!("../src/lib.rs");

#[test]
fn test_canonical_example() {
    // LeetCode example:
    // add([3,10]), add([11,2]), add([3,2])
    // count([11,10]) -> 1
    // count([14,8])  -> 0
    // add([11,2]) (duplicate)
    // count([11,10]) -> 2
    let mut d = DetectSquares::new();
    d.add(vec![3, 10]);
    d.add(vec![11, 2]);
    d.add(vec![3, 2]);
    assert_eq!(d.count(vec![11, 10]), 1);
    assert_eq!(d.count(vec![14, 8]), 0);
    d.add(vec![11, 2]);
    assert_eq!(d.count(vec![11, 10]), 2);
}

#[test]
fn test_empty_returns_zero() {
    let d = DetectSquares::new();
    assert_eq!(d.count(vec![0, 0]), 0);
}

#[test]
fn test_no_square_possible() {
    let mut d = DetectSquares::new();
    d.add(vec![0, 0]);
    d.add(vec![1, 0]);
    d.add(vec![0, 1]);
    // Query point (1,1) plus the three added points form a unit square.
    assert_eq!(d.count(vec![1, 1]), 1);
    // Query point that shares an axis is excluded.
    assert_eq!(d.count(vec![0, 0]), 0);
}

#[test]
fn test_multiple_duplicate_points_multiplies_count() {
    let mut d = DetectSquares::new();
    d.add(vec![0, 0]);
    d.add(vec![0, 0]); // duplicate
    d.add(vec![2, 0]);
    d.add(vec![0, 2]);
    // Query (2,2) -> there are 2 instances at (0,0), 1 at (2,0), 1 at (0,2).
    // So total squares contributed: 2 * 1 * 1 = 2.
    assert_eq!(d.count(vec![2, 2]), 2);
}

#[test]
fn test_query_on_axis_with_existing_point_excluded() {
    let mut d = DetectSquares::new();
    d.add(vec![5, 5]);
    d.add(vec![5, 10]);
    d.add(vec![10, 5]);
    d.add(vec![10, 10]);
    // From (5,5): diagonal corner is (10,10), forms a square with (5,10) and (10,5).
    assert_eq!(d.count(vec![5, 5]), 1);
}

#[test]
fn test_collinear_points_do_not_form_square() {
    let mut d = DetectSquares::new();
    d.add(vec![1, 1]);
    d.add(vec![1, 2]);
    d.add(vec![1, 3]);
    assert_eq!(d.count(vec![2, 2]), 0);
}

#[test]
fn test_negative_coordinates() {
    let mut d = DetectSquares::new();
    d.add(vec![-1, -1]);
    d.add(vec![-1, 1]);
    d.add(vec![1, -1]);
    assert_eq!(d.count(vec![1, 1]), 1);
}

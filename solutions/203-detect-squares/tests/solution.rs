include!("../src/lib.rs");

#[test]
fn test_canonical_example() {
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
    assert_eq!(d.count(vec![1, 1]), 1);
    assert_eq!(d.count(vec![0, 0]), 0);
}

#[test]
fn test_multiple_duplicate_points_multiplies_count() {
    let mut d = DetectSquares::new();
    d.add(vec![0, 0]);
    d.add(vec![0, 0]);
    d.add(vec![2, 0]);
    d.add(vec![0, 2]);
    assert_eq!(d.count(vec![2, 2]), 2);
}

#[test]
fn test_query_on_axis_with_existing_point_excluded() {
    let mut d = DetectSquares::new();
    d.add(vec![5, 5]);
    d.add(vec![5, 10]);
    d.add(vec![10, 5]);
    d.add(vec![10, 10]);
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

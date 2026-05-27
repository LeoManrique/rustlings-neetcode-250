include!("../src/lib.rs");

fn canonical_matrix() -> Vec<Vec<i32>> {
    vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]
}

#[test]
fn canonical_example_sum_region_2_1_4_3() {
    // From LC #304 example: rectangle (2,1)-(4,3) sums to 8.
    let nm = NumMatrix::new(canonical_matrix());
    assert_eq!(nm.sum_region(2, 1, 4, 3), 8);
}

#[test]
fn canonical_example_sum_region_1_1_2_2() {
    // Rectangle (1,1)-(2,2) sums to 11.
    let nm = NumMatrix::new(canonical_matrix());
    assert_eq!(nm.sum_region(1, 1, 2, 2), 11);
}

#[test]
fn canonical_example_sum_region_1_2_2_4() {
    // Rectangle (1,2)-(2,4) sums to 12.
    let nm = NumMatrix::new(canonical_matrix());
    assert_eq!(nm.sum_region(1, 2, 2, 4), 12);
}

#[test]
fn single_cell_query() {
    let nm = NumMatrix::new(canonical_matrix());
    assert_eq!(nm.sum_region(0, 0, 0, 0), 3);
    assert_eq!(nm.sum_region(4, 4, 4, 4), 5);
    assert_eq!(nm.sum_region(2, 2, 2, 2), 0);
}

#[test]
fn full_matrix_sum() {
    let nm = NumMatrix::new(canonical_matrix());
    let total: i32 = canonical_matrix().iter().flatten().sum();
    assert_eq!(nm.sum_region(0, 0, 4, 4), total);
}

#[test]
fn single_row_matrix() {
    let nm = NumMatrix::new(vec![vec![1, 2, 3, 4]]);
    assert_eq!(nm.sum_region(0, 0, 0, 3), 10);
    assert_eq!(nm.sum_region(0, 1, 0, 2), 5);
    assert_eq!(nm.sum_region(0, 2, 0, 2), 3);
}

#[test]
fn single_column_matrix() {
    let nm = NumMatrix::new(vec![vec![1], vec![2], vec![3], vec![4]]);
    assert_eq!(nm.sum_region(0, 0, 3, 0), 10);
    assert_eq!(nm.sum_region(1, 0, 2, 0), 5);
}

#[test]
fn handles_negative_values() {
    let nm = NumMatrix::new(vec![vec![-1, -2], vec![-3, -4]]);
    assert_eq!(nm.sum_region(0, 0, 1, 1), -10);
    assert_eq!(nm.sum_region(0, 0, 0, 1), -3);
    assert_eq!(nm.sum_region(1, 1, 1, 1), -4);
}

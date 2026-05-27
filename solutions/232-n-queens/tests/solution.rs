include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::solve_n_queens(3), vec![]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::solve_n_queens(2), vec![]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q".to_string()]]);
}

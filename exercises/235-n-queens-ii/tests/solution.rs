include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::total_n_queens(8), 92);
}

#[test]
fn test_2() {
    assert_eq!(Solution::total_n_queens(3), 0);
}

#[test]
fn test_3() {
    assert_eq!(Solution::total_n_queens(4), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::total_n_queens(9), 352);
}

#[test]
fn test_5() {
    assert_eq!(Solution::total_n_queens(6), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::total_n_queens(2), 0);
}

#[test]
fn test_7() {
    assert_eq!(Solution::total_n_queens(1), 1);
}

#[test]
fn test_8() {
    assert_eq!(Solution::total_n_queens(7), 40);
}

#[test]
fn test_9() {
    assert_eq!(Solution::total_n_queens(5), 10);
}

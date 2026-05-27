include!("../src/lib.rs");

#[test]
fn test_2() {
    assert_eq!(Solution::unique_paths(1, 100), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
}

#[test]
fn test_4() {
    assert_eq!(Solution::unique_paths(5, 5), 70);
}

#[test]
fn test_5() {
    assert_eq!(Solution::unique_paths(10, 10), 48620);
}

#[test]
fn test_6() {
    assert_eq!(Solution::unique_paths(5, 3), 15);
}

#[test]
fn test_8() {
    assert_eq!(Solution::unique_paths(1, 1), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::unique_paths(3, 2), 3);
}

#[test]
fn test_10() {
    assert_eq!(Solution::unique_paths(100, 1), 1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::unique_paths(5, 95), 3612280);
}

#[test]
fn test_15() {
    assert_eq!(Solution::unique_paths(15, 15), 40116600);
}

#[test]
fn test_16() {
    assert_eq!(Solution::unique_paths(1, 50), 1);
}

#[test]
fn test_25() {
    assert_eq!(Solution::unique_paths(100, 5), 4421275);
}

#[test]
fn test_27() {
    assert_eq!(Solution::unique_paths(40, 10), 1677106640);
}

#[test]
fn test_29() {
    assert_eq!(Solution::unique_paths(5, 8), 330);
}

#[test]
fn test_30() {
    assert_eq!(Solution::unique_paths(99, 1), 1);
}

#[test]
fn test_31() {
    assert_eq!(Solution::unique_paths(5, 100), 4421275);
}

#[test]
fn test_32() {
    assert_eq!(Solution::unique_paths(99, 2), 99);
}

#[test]
fn test_35() {
    assert_eq!(Solution::unique_paths(2, 99), 99);
}

#[test]
fn test_36() {
    assert_eq!(Solution::unique_paths(1, 99), 1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::unique_paths(50, 2), 50);
}

#[test]
fn test_43() {
    assert_eq!(Solution::unique_paths(15, 5), 3060);
}

#[test]
fn test_46() {
    assert_eq!(Solution::unique_paths(15, 20), 818809200);
}

#[test]
fn test_49() {
    assert_eq!(Solution::unique_paths(2, 50), 50);
}

#[test]
fn test_51() {
    assert_eq!(Solution::unique_paths(50, 1), 1);
}


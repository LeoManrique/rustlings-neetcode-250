include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::tribonacci(0), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::tribonacci(3), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::tribonacci(4), 4);
}

#[test]
fn test_4() {
    assert_eq!(Solution::tribonacci(37), 2082876103);
}

#[test]
fn test_5() {
    assert_eq!(Solution::tribonacci(2), 1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::tribonacci(1), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::tribonacci(25), 1389537);
}

#[test]
fn test_8() {
    assert_eq!(Solution::tribonacci(30), 29249425);
}

#[test]
fn test_9() {
    assert_eq!(Solution::tribonacci(15), 3136);
}

#[test]
fn test_10() {
    assert_eq!(Solution::tribonacci(22), 223317);
}

#[test]
fn test_11() {
    assert_eq!(Solution::tribonacci(12), 504);
}

#[test]
fn test_12() {
    assert_eq!(Solution::tribonacci(35), 615693474);
}

#[test]
fn test_13() {
    assert_eq!(Solution::tribonacci(26), 2555757);
}

#[test]
fn test_14() {
    assert_eq!(Solution::tribonacci(27), 4700770);
}

#[test]
fn test_15() {
    assert_eq!(Solution::tribonacci(18), 19513);
}

#[test]
fn test_16() {
    assert_eq!(Solution::tribonacci(20), 66012);
}

#[test]
fn test_17() {
    assert_eq!(Solution::tribonacci(10), 149);
}

#[test]
fn test_18() {
    assert_eq!(Solution::tribonacci(5), 7);
}

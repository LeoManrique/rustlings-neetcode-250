include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::array_sum(vec![1, 2, 3]), 6); }
#[test]
fn test_2() { assert_eq!(Solution::array_sum(vec![-1, 0, 1]), 0); }
#[test]
fn test_3() { assert_eq!(Solution::array_sum(vec![]), 0); }
#[test]
fn test_4() { assert_eq!(Solution::array_sum(vec![42]), 42); }
#[test]
fn test_5() { assert_eq!(Solution::array_sum(vec![10, -10, 20, -20]), 0); }

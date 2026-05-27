include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::find_max(vec![1, 3, 2]), 3); }
#[test]
fn test_2() { assert_eq!(Solution::find_max(vec![-5, -1, -3]), -1); }
#[test]
fn test_3() { assert_eq!(Solution::find_max(vec![42]), 42); }
#[test]
fn test_4() { assert_eq!(Solution::find_max(vec![0, 0, 0]), 0); }
#[test]
fn test_5() { assert_eq!(Solution::find_max(vec![-100, 100, 0]), 100); }

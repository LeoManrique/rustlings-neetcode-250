include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::unique_paths(3, 2), 3); }
#[test]
fn test_2() { assert_eq!(Solution::unique_paths(1, 1), 1); }
#[test]
fn test_3() { assert_eq!(Solution::unique_paths(3, 7), 28); }
#[test]
fn test_4() { assert_eq!(Solution::unique_paths(2, 2), 2); }

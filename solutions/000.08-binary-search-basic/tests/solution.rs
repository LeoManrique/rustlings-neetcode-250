include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::binary_search(vec![1,3,5,7,9], 5), 2); }
#[test]
fn test_2() { assert_eq!(Solution::binary_search(vec![1,3,5,7,9], 4), -1); }
#[test]
fn test_3() { assert_eq!(Solution::binary_search(vec![1], 1), 0); }
#[test]
fn test_4() { assert_eq!(Solution::binary_search(vec![1,2,3], 1), 0); }
#[test]
fn test_5() { assert_eq!(Solution::binary_search(vec![1,2,3], 3), 2); }
#[test]
fn test_6() { assert_eq!(Solution::binary_search(vec![], 1), -1); }

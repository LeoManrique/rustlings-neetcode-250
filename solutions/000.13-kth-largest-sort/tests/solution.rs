include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::kth_largest(vec![3,1,5,2,4], 2), 4); }
#[test]
fn test_2() { assert_eq!(Solution::kth_largest(vec![1,1,1], 1), 1); }
#[test]
fn test_3() { assert_eq!(Solution::kth_largest(vec![7], 1), 7); }
#[test]
fn test_4() { assert_eq!(Solution::kth_largest(vec![3,2,1,5,6,4], 2), 5); }
#[test]
fn test_5() { assert_eq!(Solution::kth_largest(vec![3,2,3,1,2,4,5,5,6], 4), 4); }

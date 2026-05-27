include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::max_sum_k(vec![1,3,2,6,1], 2), 8); }
#[test]
fn test_2() { assert_eq!(Solution::max_sum_k(vec![1,1,1,1], 2), 2); }
#[test]
fn test_3() { assert_eq!(Solution::max_sum_k(vec![5], 1), 5); }
#[test]
fn test_4() { assert_eq!(Solution::max_sum_k(vec![1,2,3,4,5], 3), 12); }
#[test]
fn test_5() { assert_eq!(Solution::max_sum_k(vec![-1,2,-1,3,1], 2), 4); }

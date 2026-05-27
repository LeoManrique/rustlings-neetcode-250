include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5); }
#[test]
fn test_2() { assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0); }
#[test]
fn test_3() { assert_eq!(Solution::max_profit(vec![1,2]), 1); }
#[test]
fn test_4() { assert_eq!(Solution::max_profit(vec![2,1]), 0); }
#[test]
fn test_5() { assert_eq!(Solution::max_profit(vec![1]), 0); }

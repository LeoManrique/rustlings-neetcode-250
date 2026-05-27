include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::is_sorted(vec![1,2,3,4]), true); }
#[test]
fn test_2() { assert_eq!(Solution::is_sorted(vec![3,1,2]), false); }
#[test]
fn test_3() { assert_eq!(Solution::is_sorted(vec![1]), true); }
#[test]
fn test_4() { assert_eq!(Solution::is_sorted(vec![]), true); }
#[test]
fn test_5() { assert_eq!(Solution::is_sorted(vec![1,1,1]), true); }
#[test]
fn test_6() { assert_eq!(Solution::is_sorted(vec![1,2,1]), false); }

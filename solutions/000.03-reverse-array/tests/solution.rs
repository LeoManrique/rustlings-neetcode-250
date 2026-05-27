include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::reverse_array(vec![1,2,3,4]), vec![4,3,2,1]); }
#[test]
fn test_2() { assert_eq!(Solution::reverse_array(vec![1]), vec![1]); }
#[test]
fn test_3() { assert_eq!(Solution::reverse_array(vec![]), vec![]); }
#[test]
fn test_4() { assert_eq!(Solution::reverse_array(vec![1,2]), vec![2,1]); }
#[test]
fn test_5() { assert_eq!(Solution::reverse_array(vec![5,5,5]), vec![5,5,5]); }

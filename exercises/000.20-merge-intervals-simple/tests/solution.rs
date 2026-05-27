include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::merge(vec![vec![1,3],vec![2,6],vec![8,10]]), vec![vec![1,6],vec![8,10]]); }
#[test]
fn test_2() { assert_eq!(Solution::merge(vec![vec![1,4],vec![4,5]]), vec![vec![1,5]]); }
#[test]
fn test_3() { assert_eq!(Solution::merge(vec![vec![1,2]]), vec![vec![1,2]]); }
#[test]
fn test_4() { assert_eq!(Solution::merge(vec![vec![1,10],vec![2,3],vec![4,5]]), vec![vec![1,10]]); }

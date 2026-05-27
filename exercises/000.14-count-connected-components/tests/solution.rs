include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::count_components(5, vec![vec![0,1],vec![1,2],vec![3,4]]), 2); }
#[test]
fn test_2() { assert_eq!(Solution::count_components(3, vec![]), 3); }
#[test]
fn test_3() { assert_eq!(Solution::count_components(1, vec![]), 1); }
#[test]
fn test_4() { assert_eq!(Solution::count_components(4, vec![vec![0,1],vec![1,2],vec![2,3]]), 1); }

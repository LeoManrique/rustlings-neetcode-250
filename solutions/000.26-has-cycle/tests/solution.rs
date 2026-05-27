include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::has_cycle(3, vec![vec![0,1],vec![1,2],vec![2,0]]), true); }
#[test]
fn test_2() { assert_eq!(Solution::has_cycle(3, vec![vec![0,1],vec![1,2]]), false); }
#[test]
fn test_3() { assert_eq!(Solution::has_cycle(1, vec![]), false); }
#[test]
fn test_4() { assert_eq!(Solution::has_cycle(2, vec![vec![0,1],vec![1,0]]), true); }

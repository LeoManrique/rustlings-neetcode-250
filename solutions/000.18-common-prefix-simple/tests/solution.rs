include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::longest_prefix(vec!["flower","flow","flight"].into_iter().map(String::from).collect()), "fl"); }
#[test]
fn test_2() { assert_eq!(Solution::longest_prefix(vec!["dog","car"].into_iter().map(String::from).collect()), ""); }
#[test]
fn test_3() { assert_eq!(Solution::longest_prefix(vec!["a"].into_iter().map(String::from).collect()), "a"); }
#[test]
fn test_4() { assert_eq!(Solution::longest_prefix(vec!["abc","abc","abc"].into_iter().map(String::from).collect()), "abc"); }

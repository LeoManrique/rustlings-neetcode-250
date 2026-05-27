include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::reverse_string("hello".to_string()), "olleh"); }
#[test]
fn test_2() { assert_eq!(Solution::reverse_string("ab".to_string()), "ba"); }
#[test]
fn test_3() { assert_eq!(Solution::reverse_string("a".to_string()), "a"); }
#[test]
fn test_4() { assert_eq!(Solution::reverse_string("".to_string()), ""); }
#[test]
fn test_5() { assert_eq!(Solution::reverse_string("racecar".to_string()), "racecar"); }

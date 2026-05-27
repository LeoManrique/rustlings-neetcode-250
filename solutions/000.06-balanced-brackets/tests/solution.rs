include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::is_balanced("(())".to_string()), true); }
#[test]
fn test_2() { assert_eq!(Solution::is_balanced(")(".to_string()), false); }
#[test]
fn test_3() { assert_eq!(Solution::is_balanced("((())())".to_string()), true); }
#[test]
fn test_4() { assert_eq!(Solution::is_balanced("".to_string()), true); }
#[test]
fn test_5() { assert_eq!(Solution::is_balanced("((".to_string()), false); }
#[test]
fn test_6() { assert_eq!(Solution::is_balanced("()()()".to_string()), true); }

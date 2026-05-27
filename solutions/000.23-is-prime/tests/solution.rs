include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::is_prime(7), true); }
#[test]
fn test_2() { assert_eq!(Solution::is_prime(4), false); }
#[test]
fn test_3() { assert_eq!(Solution::is_prime(1), false); }
#[test]
fn test_4() { assert_eq!(Solution::is_prime(2), true); }
#[test]
fn test_5() { assert_eq!(Solution::is_prime(97), true); }
#[test]
fn test_6() { assert_eq!(Solution::is_prime(100), false); }

include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::fib(0), 0); }
#[test]
fn test_2() { assert_eq!(Solution::fib(1), 1); }
#[test]
fn test_3() { assert_eq!(Solution::fib(5), 5); }
#[test]
fn test_4() { assert_eq!(Solution::fib(10), 55); }
#[test]
fn test_5() { assert_eq!(Solution::fib(2), 1); }

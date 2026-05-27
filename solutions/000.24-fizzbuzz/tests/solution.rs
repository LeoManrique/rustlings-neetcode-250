include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::fizzbuzz(5), vec!["1","2","Fizz","4","Buzz"]); }
#[test]
fn test_2() { assert_eq!(Solution::fizzbuzz(1), vec!["1"]); }
#[test]
fn test_3() { assert_eq!(Solution::fizzbuzz(15)[14], "FizzBuzz"); }
#[test]
fn test_4() { assert_eq!(Solution::fizzbuzz(3), vec!["1","2","Fizz"]); }

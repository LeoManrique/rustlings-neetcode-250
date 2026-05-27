include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::is_power_of_two(8), true); }
#[test]
fn test_2() { assert_eq!(Solution::is_power_of_two(6), false); }
#[test]
fn test_3() { assert_eq!(Solution::is_power_of_two(1), true); }
#[test]
fn test_4() { assert_eq!(Solution::is_power_of_two(0), false); }
#[test]
fn test_5() { assert_eq!(Solution::is_power_of_two(16), true); }
#[test]
fn test_6() { assert_eq!(Solution::is_power_of_two(-4), false); }

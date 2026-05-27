include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::count_bits(11), 3); }
#[test]
fn test_2() { assert_eq!(Solution::count_bits(0), 0); }
#[test]
fn test_3() { assert_eq!(Solution::count_bits(1), 1); }
#[test]
fn test_4() { assert_eq!(Solution::count_bits(255), 8); }
#[test]
fn test_5() { assert_eq!(Solution::count_bits(128), 1); }

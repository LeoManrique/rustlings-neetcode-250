include!("../src/lib.rs");

#[test]
fn test_1() { assert_eq!(Solution::climb_stairs(1), 1); }
#[test]
fn test_2() { assert_eq!(Solution::climb_stairs(2), 2); }
#[test]
fn test_3() { assert_eq!(Solution::climb_stairs(3), 3); }
#[test]
fn test_4() { assert_eq!(Solution::climb_stairs(5), 8); }
#[test]
fn test_5() { assert_eq!(Solution::climb_stairs(10), 89); }

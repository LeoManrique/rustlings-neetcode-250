include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::climb_stairs(3), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::climb_stairs(45), 1836311903);
}

#[test]
fn test_3() {
    assert_eq!(Solution::climb_stairs(4), 5);
}

#[test]
fn test_4() {
    assert_eq!(Solution::climb_stairs(20), 10946);
}

#[test]
fn test_5() {
    assert_eq!(Solution::climb_stairs(2), 2);
}

#[test]
fn test_6() {
    assert_eq!(Solution::climb_stairs(1), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::climb_stairs(10), 89);
}

#[test]
fn test_8() {
    assert_eq!(Solution::climb_stairs(5), 8);
}

#[test]
fn test_9() {
    assert_eq!(Solution::climb_stairs(30), 1346269);
}

#[test]
fn test_10() {
    assert_eq!(Solution::climb_stairs(15), 987);
}

#[test]
fn test_11() {
    assert_eq!(Solution::climb_stairs(40), 165580141);
}

#[test]
fn test_12() {
    assert_eq!(Solution::climb_stairs(12), 233);
}

#[test]
fn test_13() {
    assert_eq!(Solution::climb_stairs(35), 14930352);
}

#[test]
fn test_14() {
    assert_eq!(Solution::climb_stairs(18), 4181);
}

#[test]
fn test_15() {
    assert_eq!(Solution::climb_stairs(7), 21);
}

#[test]
fn test_16() {
    assert_eq!(Solution::climb_stairs(25), 121393);
}

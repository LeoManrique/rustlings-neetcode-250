include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::integer_break(11), 54);
}

#[test]
fn test_2() {
    assert_eq!(Solution::integer_break(30), 59049);
}

#[test]
fn test_3() {
    assert_eq!(Solution::integer_break(20), 1458);
}

#[test]
fn test_4() {
    assert_eq!(Solution::integer_break(2), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::integer_break(10), 36);
}

#[test]
fn test_6() {
    assert_eq!(Solution::integer_break(58), 1549681956);
}

#[test]
fn test_7() {
    assert_eq!(Solution::integer_break(29), 39366);
}

#[test]
fn test_8() {
    assert_eq!(Solution::integer_break(45), 14348907);
}

#[test]
fn test_9() {
    assert_eq!(Solution::integer_break(49), 57395628);
}

#[test]
fn test_10() {
    assert_eq!(Solution::integer_break(12), 81);
}

#[test]
fn test_11() {
    assert_eq!(Solution::integer_break(47), 28697814);
}

#[test]
fn test_12() {
    assert_eq!(Solution::integer_break(53), 258280326);
}

#[test]
fn test_13() {
    assert_eq!(Solution::integer_break(57), 1162261467);
}

#[test]
fn test_14() {
    assert_eq!(Solution::integer_break(50), 86093442);
}

#[test]
fn test_15() {
    assert_eq!(Solution::integer_break(28), 26244);
}

#[test]
fn test_16() {
    assert_eq!(Solution::integer_break(56), 774840978);
}

#[test]
fn test_17() {
    assert_eq!(Solution::integer_break(40), 2125764);
}

#[test]
fn test_18() {
    assert_eq!(Solution::integer_break(37), 708588);
}

#[test]
fn test_19() {
    assert_eq!(Solution::integer_break(42), 4782969);
}

#[test]
fn test_20() {
    assert_eq!(Solution::integer_break(35), 354294);
}

#[test]
fn test_21() {
    assert_eq!(Solution::integer_break(18), 729);
}

#[test]
fn test_22() {
    assert_eq!(Solution::integer_break(32), 118098);
}

#[test]
fn test_23() {
    assert_eq!(Solution::integer_break(36), 531441);
}

#[test]
fn test_24() {
    assert_eq!(Solution::integer_break(19), 972);
}

#[test]
fn test_25() {
    assert_eq!(Solution::integer_break(48), 43046721);
}

#[test]
fn test_26() {
    assert_eq!(Solution::integer_break(15), 243);
}

#[test]
fn test_27() {
    assert_eq!(Solution::integer_break(6), 9);
}

#[test]
fn test_28() {
    assert_eq!(Solution::integer_break(55), 516560652);
}

#[test]
fn test_29() {
    assert_eq!(Solution::integer_break(13), 108);
}

#[test]
fn test_30() {
    assert_eq!(Solution::integer_break(25), 8748);
}

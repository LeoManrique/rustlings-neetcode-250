include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::can_reach("0100100010001000".to_string(), 2, 5), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::can_reach("00001000".to_string(), 2, 4), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::can_reach("0010010010010010010010010010010010010010".to_string(), 2, 4), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::can_reach("011010".to_string(), 2, 3), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::can_reach("0100000".to_string(), 1, 2), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::can_reach("00001000".to_string(), 3, 5), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::can_reach("01010101".to_string(), 2, 4), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::can_reach("0101010101010101010101010101010101010101".to_string(), 5, 10), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::can_reach("0111111111111111111111111111111111111111".to_string(), 3, 5), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::can_reach("000000".to_string(), 1, 2), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::can_reach("001000".to_string(), 2, 3), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::can_reach("010101".to_string(), 1, 2), false);
}

#[test]
fn test_13() {
    assert_eq!(Solution::can_reach("00000000".to_string(), 1, 2), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::can_reach("00100".to_string(), 3, 4), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::can_reach("01101110".to_string(), 2, 3), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::can_reach("010000".to_string(), 1, 3), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::can_reach("00000000001000000000".to_string(), 4, 7), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::can_reach("0000011111000000000000000000".to_string(), 1, 5), false);
}

#[test]
fn test_19() {
    assert_eq!(Solution::can_reach("00000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(), 5, 10), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::can_reach("0000000000000000000000000000".to_string(), 2, 2), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::can_reach("0111111111111111111111111110".to_string(), 10, 20), false);
}

#[test]
fn test_22() {
    assert_eq!(Solution::can_reach("0100000000000000000000000000".to_string(), 2, 3), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::can_reach("00000000000000000000000000000000000000000000000000".to_string(), 10, 20), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::can_reach("01010101010101010101010101010101010101010101010101".to_string(), 5, 15), false);
}

#[test]
fn test_25() {
    assert_eq!(Solution::can_reach("0010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101".to_string(), 3, 7), false);
}

#[test]
fn test_26() {
    assert_eq!(Solution::can_reach("0000000000000000000000000001".to_string(), 5000, 10000), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::can_reach("0000000000000000000000000001".to_string(), 1, 1), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::can_reach("000001000001000001000001000001000001000001000001".to_string(), 6, 12), false);
}

#[test]
fn test_29() {
    assert_eq!(Solution::can_reach("000111000111000111000111000111000111000111000111".to_string(), 3, 6), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::can_reach("01001001001001001001001000".to_string(), 5, 10), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::can_reach("001001001000".to_string(), 2, 4), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::can_reach("0100000000000000000000000000".to_string(), 50000, 50000), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::can_reach("0000100100010001000100010001000100010001000100010000".to_string(), 5, 15), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::can_reach("0000000000000000000000000000000000000000000000000001000000000000000000000000".to_string(), 10, 20), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::can_reach("01001001001001001001001001001001001001001001001001".to_string(), 4, 8), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::can_reach("0000000000000000000000000000".to_string(), 5, 5), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::can_reach("010001000100010001000100010001000100010000".to_string(), 4, 12), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::can_reach("0000000000000000000000000000".to_string(), 1, 10), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::can_reach("0101010101010101010101010101".to_string(), 3, 5), false);
}

#[test]
fn test_40() {
    assert_eq!(Solution::can_reach("0100000000000000000000000000".to_string(), 10, 10), false);
}

#[test]
fn test_41() {
    assert_eq!(Solution::can_reach("0001010101010101010101010101010100".to_string(), 3, 8), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::can_reach("0000000000000000000000000000".to_string(), 50, 100), false);
}

#[test]
fn test_43() {
    assert_eq!(Solution::can_reach("0000000000".to_string(), 4, 6), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::can_reach("0000000000000000000000000000".to_string(), 1, 99999), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::can_reach("0100000000000000000000000000".to_string(), 10000, 50000), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::can_reach("0000000000000000000000000000".to_string(), 10000, 10000), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::can_reach("0000000000000000000000000000".to_string(), 100, 100), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::can_reach("0000000000000000000000000000".to_string(), 1, 50000), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::can_reach("0001000100010001000100010000".to_string(), 3, 5), true);
}

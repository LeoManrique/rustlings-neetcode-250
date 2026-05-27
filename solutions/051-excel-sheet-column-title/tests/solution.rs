include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::convert_to_title(1), "A".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::convert_to_title(28), "AB".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::convert_to_title(1045), "ANE".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::convert_to_title(456976), "YYYZ".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::convert_to_title(52), "AZ".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::convert_to_title(1048576), "BGQCV".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::convert_to_title(26), "Z".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::convert_to_title(134217728), "KGRJXH".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::convert_to_title(702), "ZZ".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::convert_to_title(1047), "ANG".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::convert_to_title(1046527), "BGNCA".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::convert_to_title(351), "MM".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::convert_to_title(27), "AA".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::convert_to_title(4194304), "IDPOJ".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::convert_to_title(466527), "ZNCI".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::convert_to_title(1048575), "BGQCU".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::convert_to_title(703), "AAA".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::convert_to_title(1234567), "BRFGI".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::convert_to_title(2702), "CYX".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::convert_to_title(16384), "XFD".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::convert_to_title(1000), "ALL".to_string());
}

#[test]
fn test_24() {
    assert_eq!(Solution::convert_to_title(1379), "BAA".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::convert_to_title(123456), "FZPH".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::convert_to_title(4095), "FAM".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::convert_to_title(676), "YZ".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::convert_to_title(10000000), "UVXWJ".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::convert_to_title(14776336), "AFHRLP".to_string());
}

#[test]
fn test_30() {
    assert_eq!(Solution::convert_to_title(1378), "AZZ".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::convert_to_title(140625), "GYZQ".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::convert_to_title(728), "AAZ".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::convert_to_title(2704), "CYZ".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::convert_to_title(18278), "ZZZ".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::convert_to_title(234567890), "SSGWWX".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::convert_to_title(255), "IU".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::convert_to_title(99999999), "HJUNYU".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::convert_to_title(11829215), "YVZUU".to_string());
}

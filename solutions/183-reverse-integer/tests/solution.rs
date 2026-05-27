include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::reverse(-2147483412), -2143847412);
}

#[test]
fn test_2() {
    assert_eq!(Solution::reverse(2147483647), 0);
}

#[test]
fn test_3() {
    assert_eq!(Solution::reverse(120), 21);
}

#[test]
fn test_4() {
    assert_eq!(Solution::reverse(-123), -321);
}

#[test]
fn test_5() {
    assert_eq!(Solution::reverse(1534236469), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::reverse(0), 0);
}

#[test]
fn test_7() {
    assert_eq!(Solution::reverse(-2147483648), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::reverse(-1534236469), 0);
}

#[test]
fn test_9() {
    assert_eq!(Solution::reverse(123), 321);
}

#[test]
fn test_10() {
    assert_eq!(Solution::reverse(-10), -1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::reverse(-100000), -1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::reverse(10), 1);
}

#[test]
fn test_13() {
    assert_eq!(Solution::reverse(-999999999), -999999999);
}

#[test]
fn test_14() {
    assert_eq!(Solution::reverse(1), 1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::reverse(2147483646), 0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::reverse(-123000), -321);
}

#[test]
fn test_17() {
    assert_eq!(Solution::reverse(-900000), -9);
}

#[test]
fn test_18() {
    assert_eq!(Solution::reverse(-100100100), -1001001);
}

#[test]
fn test_19() {
    assert_eq!(Solution::reverse(-2147483647), 0);
}

#[test]
fn test_20() {
    assert_eq!(Solution::reverse(-1010101010), -101010101);
}

#[test]
fn test_21() {
    assert_eq!(Solution::reverse(1000000001), 1000000001);
}

#[test]
fn test_22() {
    assert_eq!(Solution::reverse(-1), -1);
}

#[test]
fn test_23() {
    assert_eq!(Solution::reverse(123000), 321);
}

#[test]
fn test_24() {
    assert_eq!(Solution::reverse(-2000000002), -2000000002);
}

#[test]
fn test_25() {
    assert_eq!(Solution::reverse(101010101), 101010101);
}

#[test]
fn test_26() {
    assert_eq!(Solution::reverse(1111111111), 1111111111);
}

#[test]
fn test_27() {
    assert_eq!(Solution::reverse(2147447412), 2147447412);
}

#[test]
fn test_28() {
    assert_eq!(Solution::reverse(-101010101), -101010101);
}

#[test]
fn test_29() {
    assert_eq!(Solution::reverse(900000), 9);
}

#[test]
fn test_30() {
    assert_eq!(Solution::reverse(987654321), 123456789);
}

#[test]
fn test_31() {
    assert_eq!(Solution::reverse(999999999), 999999999);
}

#[test]
fn test_32() {
    assert_eq!(Solution::reverse(-1000000000), -1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::reverse(1001001001), 1001001001);
}

#[test]
fn test_34() {
    assert_eq!(Solution::reverse(-987654321), -123456789);
}

#[test]
fn test_35() {
    assert_eq!(Solution::reverse(-1000000001), -1000000001);
}

#[test]
fn test_36() {
    assert_eq!(Solution::reverse(10000000000), 1);
}

#[test]
fn test_37() {
    assert_eq!(Solution::reverse(11000000001), 0);
}

#[test]
fn test_38() {
    assert_eq!(Solution::reverse(876543210), 12345678);
}

#[test]
fn test_39() {
    assert_eq!(Solution::reverse(2147483640), 463847412);
}

#[test]
fn test_40() {
    assert_eq!(Solution::reverse(100100100), 1001001);
}

#[test]
fn test_41() {
    assert_eq!(Solution::reverse(100000), 1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::reverse(2000000002), 2000000002);
}

#[test]
fn test_43() {
    assert_eq!(Solution::reverse(-9646324351), -1534236469);
}

#[test]
fn test_44() {
    assert_eq!(Solution::reverse(-123456789), -987654321);
}

#[test]
fn test_45() {
    assert_eq!(Solution::reverse(7463847412), 2147483647);
}

#[test]
fn test_46() {
    assert_eq!(Solution::reverse(-1000000003), 0);
}

#[test]
fn test_47() {
    assert_eq!(Solution::reverse(-3000000001), -1000000003);
}

#[test]
fn test_48() {
    assert_eq!(Solution::reverse(-876543210), -12345678);
}

#[test]
fn test_49() {
    assert_eq!(Solution::reverse(-1111111111), -1111111111);
}

#[test]
fn test_50() {
    assert_eq!(Solution::reverse(-1001001001), -1001001001);
}

#[test]
fn test_51() {
    assert_eq!(Solution::reverse(9646324351), 1534236469);
}

#[test]
fn test_52() {
    assert_eq!(Solution::reverse(1010101010), 101010101);
}

#[test]
fn test_53() {
    assert_eq!(Solution::reverse(123456789), 987654321);
}

#[test]
fn test_54() {
    assert_eq!(Solution::reverse(-10000000000), -1);
}

#[test]
fn test_55() {
    assert_eq!(Solution::reverse(-2147483640), -463847412);
}

#[test]
fn test_56() {
    assert_eq!(Solution::reverse(-7463847412), -2147483647);
}

#[test]
fn test_57() {
    assert_eq!(Solution::reverse(3000000001), 1000000003);
}

#[test]
fn test_58() {
    assert_eq!(Solution::reverse(1000000000), 1);
}

#[test]
fn test_59() {
    assert_eq!(Solution::reverse(1000000003), 0);
}

#[test]
fn test_60() {
    assert_eq!(Solution::reverse(-214748364), -463847412);
}

#[test]
fn test_61() {
    assert_eq!(Solution::reverse(9000000000), 9);
}

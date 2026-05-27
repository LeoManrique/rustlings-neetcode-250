include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::my_sqrt(2147483647), 46340);
}

#[test]
fn test_2() {
    assert_eq!(Solution::my_sqrt(26), 5);
}

#[test]
fn test_3() {
    assert_eq!(Solution::my_sqrt(4), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::my_sqrt(1), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::my_sqrt(25), 5);
}

#[test]
fn test_6() {
    assert_eq!(Solution::my_sqrt(0), 0);
}

#[test]
fn test_7() {
    assert_eq!(Solution::my_sqrt(101), 10);
}

#[test]
fn test_8() {
    assert_eq!(Solution::my_sqrt(100), 10);
}

#[test]
fn test_9() {
    assert_eq!(Solution::my_sqrt(8), 2);
}

#[test]
fn test_10() {
    assert_eq!(Solution::my_sqrt(1025), 32);
}

#[test]
fn test_11() {
    assert_eq!(Solution::my_sqrt(10), 3);
}

#[test]
fn test_12() {
    assert_eq!(Solution::my_sqrt(1234567890123456789), 1111111106);
}

#[test]
fn test_13() {
    assert_eq!(Solution::my_sqrt(40000000000), 200000);
}

#[test]
fn test_14() {
    assert_eq!(Solution::my_sqrt(18014398509481984), 134217728);
}

#[test]
fn test_15() {
    assert_eq!(Solution::my_sqrt(16384), 128);
}

#[test]
fn test_16() {
    assert_eq!(Solution::my_sqrt(1000), 31);
}

#[test]
fn test_17() {
    assert_eq!(Solution::my_sqrt(1524157875), 39040);
}

#[test]
fn test_18() {
    assert_eq!(Solution::my_sqrt(2147483646), 46340);
}

#[test]
fn test_19() {
    assert_eq!(Solution::my_sqrt(99), 9);
}

#[test]
fn test_20() {
    assert_eq!(Solution::my_sqrt(4294967296), 65536);
}

#[test]
fn test_21() {
    assert_eq!(Solution::my_sqrt(225), 15);
}

#[test]
fn test_22() {
    assert_eq!(Solution::my_sqrt(1522756), 1234);
}

#[test]
fn test_23() {
    assert_eq!(Solution::my_sqrt(17), 4);
}

#[test]
fn test_24() {
    assert_eq!(Solution::my_sqrt(2147395600), 46340);
}

#[test]
fn test_25() {
    assert_eq!(Solution::my_sqrt(15), 3);
}

#[test]
fn test_26() {
    assert_eq!(Solution::my_sqrt(18446744073709551615), 4294967295);
}

#[test]
fn test_27() {
    assert_eq!(Solution::my_sqrt(16777215), 4095);
}

#[test]
fn test_28() {
    assert_eq!(Solution::my_sqrt(1000000), 1000);
}

#[test]
fn test_29() {
    assert_eq!(Solution::my_sqrt(18014398509481983), 134217727);
}

#[test]
fn test_30() {
    assert_eq!(Solution::my_sqrt(1000000001), 31622);
}

#[test]
fn test_31() {
    assert_eq!(Solution::my_sqrt(169), 13);
}

#[test]
fn test_32() {
    assert_eq!(Solution::my_sqrt(150), 12);
}

#[test]
fn test_33() {
    assert_eq!(Solution::my_sqrt(2), 1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::my_sqrt(141421356237), 376060);
}

#[test]
fn test_35() {
    assert_eq!(Solution::my_sqrt(10000), 100);
}

#[test]
fn test_36() {
    assert_eq!(Solution::my_sqrt(4294967295), 65535);
}

#[test]
fn test_37() {
    assert_eq!(Solution::my_sqrt(361), 19);
}

#[test]
fn test_38() {
    assert_eq!(Solution::my_sqrt(30), 5);
}

#[test]
fn test_39() {
    assert_eq!(Solution::my_sqrt(987654321), 31426);
}

#[test]
fn test_40() {
    assert_eq!(Solution::my_sqrt(999999999), 31622);
}

#[test]
fn test_41() {
    assert_eq!(Solution::my_sqrt(499999999), 22360);
}

#[test]
fn test_42() {
    assert_eq!(Solution::my_sqrt(2048), 45);
}

#[test]
fn test_43() {
    assert_eq!(Solution::my_sqrt(1024), 32);
}

#[test]
fn test_44() {
    assert_eq!(Solution::my_sqrt(1048575), 1023);
}

#[test]
fn test_45() {
    assert_eq!(Solution::my_sqrt(16777216), 4096);
}

#[test]
fn test_46() {
    assert_eq!(Solution::my_sqrt(256), 16);
}

#[test]
fn test_47() {
    assert_eq!(Solution::my_sqrt(131072), 362);
}

#[test]
fn test_48() {
    assert_eq!(Solution::my_sqrt(2097152), 1448);
}

#[test]
fn test_49() {
    assert_eq!(Solution::my_sqrt(441), 21);
}

#[test]
fn test_50() {
    assert_eq!(Solution::my_sqrt(1048576), 1024);
}

#[test]
fn test_51() {
    assert_eq!(Solution::my_sqrt(65536), 256);
}

#[test]
fn test_52() {
    assert_eq!(Solution::my_sqrt(10000000), 3162);
}

#[test]
fn test_53() {
    assert_eq!(Solution::my_sqrt(49), 7);
}

#[test]
fn test_54() {
    assert_eq!(Solution::my_sqrt(144), 12);
}

#[test]
fn test_55() {
    assert_eq!(Solution::my_sqrt(121), 11);
}

#[test]
fn test_56() {
    assert_eq!(Solution::my_sqrt(1234567890123456788), 1111111106);
}

#[test]
fn test_57() {
    assert_eq!(Solution::my_sqrt(4096), 64);
}

#[test]
fn test_58() {
    assert_eq!(Solution::my_sqrt(196), 14);
}

#[test]
fn test_59() {
    assert_eq!(Solution::my_sqrt(9223372036854775807), 3037000499);
}

#[test]
fn test_60() {
    assert_eq!(Solution::my_sqrt(123456789), 11111);
}

#[test]
fn test_61() {
    assert_eq!(Solution::my_sqrt(289), 17);
}

#[test]
fn test_62() {
    assert_eq!(Solution::my_sqrt(324), 18);
}

#[test]
fn test_63() {
    assert_eq!(Solution::my_sqrt(24), 4);
}

#[test]
fn test_64() {
    assert_eq!(Solution::my_sqrt(16), 4);
}

#[test]
fn test_65() {
    assert_eq!(Solution::my_sqrt(1000000000), 31622);
}

#[test]
fn test_66() {
    assert_eq!(Solution::my_sqrt(3), 1);
}

#[test]
fn test_67() {
    assert_eq!(Solution::my_sqrt(36028797018963968), 189812531);
}

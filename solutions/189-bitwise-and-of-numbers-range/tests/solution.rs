include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
}

#[test]
fn test_2() {
    assert_eq!(Solution::range_bitwise_and(100, 105), 96);
}

#[test]
fn test_3() {
    assert_eq!(Solution::range_bitwise_and(16, 31), 16);
}

#[test]
fn test_4() {
    assert_eq!(Solution::range_bitwise_and(123456, 654321), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::range_bitwise_and(8, 12), 8);
}

#[test]
fn test_6() {
    assert_eq!(Solution::range_bitwise_and(0, 0), 0);
}

#[test]
fn test_7() {
    assert_eq!(Solution::range_bitwise_and(33, 35), 32);
}

#[test]
fn test_8() {
    assert_eq!(Solution::range_bitwise_and(1, 1), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::range_bitwise_and(8, 8), 8);
}

#[test]
fn test_11() {
    assert_eq!(Solution::range_bitwise_and(10, 15), 8);
}

#[test]
fn test_12() {
    assert_eq!(Solution::range_bitwise_and(100, 200), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::range_bitwise_and(4194304, 4194305), 4194304);
}

#[test]
fn test_14() {
    assert_eq!(Solution::range_bitwise_and(1000000, 1000010), 1000000);
}

#[test]
fn test_15() {
    assert_eq!(Solution::range_bitwise_and(4095, 8191), 0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::range_bitwise_and(8388608, 8388608), 8388608);
}

#[test]
fn test_17() {
    assert_eq!(Solution::range_bitwise_and(134217728, 134217729), 134217728);
}

#[test]
fn test_18() {
    assert_eq!(Solution::range_bitwise_and(536870912, 536870913), 536870912);
}

#[test]
fn test_19() {
    assert_eq!(Solution::range_bitwise_and(32768, 32769), 32768);
}

#[test]
fn test_20() {
    assert_eq!(Solution::range_bitwise_and(32, 33), 32);
}

#[test]
fn test_21() {
    assert_eq!(Solution::range_bitwise_and(1024, 2047), 1024);
}

#[test]
fn test_22() {
    assert_eq!(Solution::range_bitwise_and(1, 1073741824), 0);
}

#[test]
fn test_23() {
    assert_eq!(Solution::range_bitwise_and(16777215, 16777216), 0);
}

#[test]
fn test_24() {
    assert_eq!(Solution::range_bitwise_and(1000000000, 1000000100), 1000000000);
}

#[test]
fn test_25() {
    assert_eq!(Solution::range_bitwise_and(512, 1023), 512);
}

#[test]
fn test_26() {
    assert_eq!(Solution::range_bitwise_and(100000000, 200000000), 0);
}

#[test]
fn test_27() {
    assert_eq!(Solution::range_bitwise_and(65536, 131071), 65536);
}

#[test]
fn test_28() {
    assert_eq!(Solution::range_bitwise_and(123456, 123458), 123456);
}

#[test]
fn test_29() {
    assert_eq!(Solution::range_bitwise_and(1, 3), 0);
}

#[test]
fn test_30() {
    assert_eq!(Solution::range_bitwise_and(1024, 1025), 1024);
}

#[test]
fn test_31() {
    assert_eq!(Solution::range_bitwise_and(131072, 131073), 131072);
}

#[test]
fn test_32() {
    assert_eq!(Solution::range_bitwise_and(1073741824, 1073741825), 1073741824);
}

#[test]
fn test_33() {
    assert_eq!(Solution::range_bitwise_and(536870912, 1073741823), 536870912);
}

#[test]
fn test_34() {
    assert_eq!(Solution::range_bitwise_and(1, 10), 0);
}

#[test]
fn test_35() {
    assert_eq!(Solution::range_bitwise_and(16384, 32767), 16384);
}

#[test]
fn test_36() {
    assert_eq!(Solution::range_bitwise_and(512, 513), 512);
}

#[test]
fn test_37() {
    assert_eq!(Solution::range_bitwise_and(32768, 65535), 32768);
}

#[test]
fn test_38() {
    assert_eq!(Solution::range_bitwise_and(16384, 16385), 16384);
}

#[test]
fn test_39() {
    assert_eq!(Solution::range_bitwise_and(67108864, 67108865), 67108864);
}

#[test]
fn test_40() {
    assert_eq!(Solution::range_bitwise_and(256, 257), 256);
}

#[test]
fn test_41() {
    assert_eq!(Solution::range_bitwise_and(64, 65), 64);
}

#[test]
fn test_42() {
    assert_eq!(Solution::range_bitwise_and(262144, 262145), 262144);
}

#[test]
fn test_43() {
    assert_eq!(Solution::range_bitwise_and(2097152, 2097153), 2097152);
}

#[test]
fn test_44() {
    assert_eq!(Solution::range_bitwise_and(16, 17), 16);
}

#[test]
fn test_45() {
    assert_eq!(Solution::range_bitwise_and(536870912, 536870919), 536870912);
}

#[test]
fn test_46() {
    assert_eq!(Solution::range_bitwise_and(16777216, 33554431), 16777216);
}

#[test]
fn test_47() {
    assert_eq!(Solution::range_bitwise_and(2048, 2049), 2048);
}

#[test]
fn test_48() {
    assert_eq!(Solution::range_bitwise_and(512, 768), 512);
}

#[test]
fn test_49() {
    assert_eq!(Solution::range_bitwise_and(2147483644, 2147483647), 2147483644);
}

#[test]
fn test_50() {
    assert_eq!(Solution::range_bitwise_and(1000000, 1000100), 999936);
}

#[test]
fn test_51() {
    assert_eq!(Solution::range_bitwise_and(500000000, 500000100), 500000000);
}

#[test]
fn test_52() {
    assert_eq!(Solution::range_bitwise_and(4096, 4096), 4096);
}

#[test]
fn test_53() {
    assert_eq!(Solution::range_bitwise_and(1048576, 1048577), 1048576);
}

#[test]
fn test_54() {
    assert_eq!(Solution::range_bitwise_and(33554432, 33554433), 33554432);
}

#[test]
fn test_55() {
    assert_eq!(Solution::range_bitwise_and(1073741824, 2147483647), 1073741824);
}

#[test]
fn test_56() {
    assert_eq!(Solution::range_bitwise_and(4, 7), 4);
}

#[test]
fn test_57() {
    assert_eq!(Solution::range_bitwise_and(2147483646, 2147483647), 2147483646);
}

#[test]
fn test_58() {
    assert_eq!(Solution::range_bitwise_and(1, 1000), 0);
}

#[test]
fn test_59() {
    assert_eq!(Solution::range_bitwise_and(16777216, 16777217), 16777216);
}

#[test]
fn test_60() {
    assert_eq!(Solution::range_bitwise_and(268435456, 268435457), 268435456);
}

#[test]
fn test_61() {
    assert_eq!(Solution::range_bitwise_and(32, 63), 32);
}

#[test]
fn test_62() {
    assert_eq!(Solution::range_bitwise_and(524288, 524289), 524288);
}

#[test]
fn test_63() {
    assert_eq!(Solution::range_bitwise_and(256, 511), 256);
}

#[test]
fn test_64() {
    assert_eq!(Solution::range_bitwise_and(1023, 1024), 0);
}

#[test]
fn test_65() {
    assert_eq!(Solution::range_bitwise_and(8192, 8193), 8192);
}

#[test]
fn test_66() {
    assert_eq!(Solution::range_bitwise_and(2147483645, 2147483647), 2147483644);
}

#[test]
fn test_67() {
    assert_eq!(Solution::range_bitwise_and(512, 1024), 0);
}

#[test]
fn test_68() {
    assert_eq!(Solution::range_bitwise_and(268435456, 268435460), 268435456);
}

#[test]
fn test_69() {
    assert_eq!(Solution::range_bitwise_and(1073741824, 1073741827), 1073741824);
}

#[test]
fn test_70() {
    assert_eq!(Solution::range_bitwise_and(130, 135), 128);
}

#[test]
fn test_71() {
    assert_eq!(Solution::range_bitwise_and(8388608, 8388609), 8388608);
}

#[test]
fn test_72() {
    assert_eq!(Solution::range_bitwise_and(134217728, 268435455), 134217728);
}

#[test]
fn test_73() {
    assert_eq!(Solution::range_bitwise_and(1024, 2048), 0);
}

#[test]
fn test_74() {
    assert_eq!(Solution::range_bitwise_and(50, 100), 0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::range_bitwise_and(33554432, 67108863), 33554432);
}

#[test]
fn test_76() {
    assert_eq!(Solution::range_bitwise_and(100000, 100099), 99840);
}

#[test]
fn test_77() {
    assert_eq!(Solution::range_bitwise_and(1048576, 2097151), 1048576);
}

#[test]
fn test_78() {
    assert_eq!(Solution::range_bitwise_and(268435456, 536870911), 268435456);
}

#[test]
fn test_79() {
    assert_eq!(Solution::range_bitwise_and(4096, 4097), 4096);
}

#[test]
fn test_80() {
    assert_eq!(Solution::range_bitwise_and(16777215, 16777219), 0);
}

#[test]
fn test_81() {
    assert_eq!(Solution::range_bitwise_and(16777215, 16777215), 16777215);
}

#[test]
fn test_82() {
    assert_eq!(Solution::range_bitwise_and(123456, 123479), 123456);
}

#[test]
fn test_83() {
    assert_eq!(Solution::range_bitwise_and(33554431, 33554432), 0);
}

#[test]
fn test_84() {
    assert_eq!(Solution::range_bitwise_and(65536, 65537), 65536);
}

#[test]
fn test_85() {
    assert_eq!(Solution::range_bitwise_and(8, 15), 8);
}

#[test]
fn test_86() {
    assert_eq!(Solution::range_bitwise_and(8191, 8192), 0);
}

#[test]
fn test_87() {
    assert_eq!(Solution::range_bitwise_and(1048576, 1049087), 1048576);
}

#[test]
fn test_88() {
    assert_eq!(Solution::range_bitwise_and(1048576, 1048580), 1048576);
}

#[test]
fn test_89() {
    assert_eq!(Solution::range_bitwise_and(500000000, 1000000000), 0);
}

#[test]
fn test_90() {
    assert_eq!(Solution::range_bitwise_and(2, 3), 2);
}

#[test]
fn test_91() {
    assert_eq!(Solution::range_bitwise_and(1023, 2047), 0);
}

#[test]
fn test_92() {
    assert_eq!(Solution::range_bitwise_and(128, 128), 128);
}

#[test]
fn test_93() {
    assert_eq!(Solution::range_bitwise_and(123456, 123500), 123456);
}

#[test]
fn test_94() {
    assert_eq!(Solution::range_bitwise_and(128, 255), 128);
}

#[test]
fn test_95() {
    assert_eq!(Solution::range_bitwise_and(1, 1000000000), 0);
}

#[test]
fn test_96() {
    assert_eq!(Solution::range_bitwise_and(128, 129), 128);
}

#[test]
fn test_97() {
    assert_eq!(Solution::range_bitwise_and(16777216, 16777232), 16777216);
}

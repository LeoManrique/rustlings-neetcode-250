include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_end(4, 8), 11);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_end(5, 1), 9);
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_end(10, 1), 19);
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_end(5, 3), 19);
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_end(2, 7), 15);
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_end(1, 10), 10);
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_end(10, 16), 25);
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_end(1, 5), 5);
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_end(1, 32), 32);
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_end(3, 4), 6);
}

#[test]
fn test_11() {
    assert_eq!(Solution::min_end(500000, 256), 1000223);
}

#[test]
fn test_12() {
    assert_eq!(Solution::min_end(30, 2048), 2077);
}

#[test]
fn test_13() {
    assert_eq!(Solution::min_end(500000, 2047), 1023999999);
}

#[test]
fn test_14() {
    assert_eq!(Solution::min_end(50, 512), 561);
}

#[test]
fn test_15() {
    assert_eq!(Solution::min_end(100, 128), 227);
}

#[test]
fn test_16() {
    assert_eq!(Solution::min_end(8, 4096), 4103);
}

#[test]
fn test_17() {
    assert_eq!(Solution::min_end(20, 8), 43);
}

#[test]
fn test_18() {
    assert_eq!(Solution::min_end(100000, 65535), 6553599999);
}

#[test]
fn test_19() {
    assert_eq!(Solution::min_end(512, 32768), 33279);
}

#[test]
fn test_20() {
    assert_eq!(Solution::min_end(7, 12), 30);
}

#[test]
fn test_21() {
    assert_eq!(Solution::min_end(15, 8), 30);
}

#[test]
fn test_22() {
    assert_eq!(Solution::min_end(4, 3), 15);
}

#[test]
fn test_23() {
    assert_eq!(Solution::min_end(20, 16), 51);
}

#[test]
fn test_24() {
    assert_eq!(Solution::min_end(8, 3), 31);
}

#[test]
fn test_25() {
    assert_eq!(Solution::min_end(12, 3), 47);
}

#[test]
fn test_26() {
    assert_eq!(Solution::min_end(1, 1073741824), 1073741824);
}

#[test]
fn test_27() {
    assert_eq!(Solution::min_end(1000, 512), 2023);
}

#[test]
fn test_28() {
    assert_eq!(Solution::min_end(8, 1023), 8191);
}

#[test]
fn test_29() {
    assert_eq!(Solution::min_end(100000000, 1073741823), 107374182399999999);
}

#[test]
fn test_30() {
    assert_eq!(Solution::min_end(100, 16), 211);
}

#[test]
fn test_31() {
    assert_eq!(Solution::min_end(30, 17), 123);
}

#[test]
fn test_32() {
    assert_eq!(Solution::min_end(30, 64), 93);
}

#[test]
fn test_33() {
    assert_eq!(Solution::min_end(15, 31), 479);
}

#[test]
fn test_34() {
    assert_eq!(Solution::min_end(100, 255), 25599);
}

#[test]
fn test_35() {
    assert_eq!(Solution::min_end(4, 63), 255);
}

#[test]
fn test_36() {
    assert_eq!(Solution::min_end(5, 1023), 5119);
}

#[test]
fn test_37() {
    assert_eq!(Solution::min_end(10, 1024), 1033);
}

#[test]
fn test_38() {
    assert_eq!(Solution::min_end(8, 7), 63);
}

#[test]
fn test_39() {
    assert_eq!(Solution::min_end(5, 8), 12);
}

#[test]
fn test_40() {
    assert_eq!(Solution::min_end(50000, 31), 1599999);
}

#[test]
fn test_41() {
    assert_eq!(Solution::min_end(9, 512), 520);
}

#[test]
fn test_42() {
    assert_eq!(Solution::min_end(4, 15), 63);
}

#[test]
fn test_43() {
    assert_eq!(Solution::min_end(6, 2048), 2053);
}

#[test]
fn test_44() {
    assert_eq!(Solution::min_end(7, 10), 30);
}

#[test]
fn test_45() {
    assert_eq!(Solution::min_end(15, 100), 126);
}

#[test]
fn test_46() {
    assert_eq!(Solution::min_end(6, 10), 27);
}

#[test]
fn test_47() {
    assert_eq!(Solution::min_end(3, 16), 18);
}

#[test]
fn test_48() {
    assert_eq!(Solution::min_end(50, 2048), 2097);
}

#[test]
fn test_49() {
    assert_eq!(Solution::min_end(1000, 4096), 5095);
}

#[test]
fn test_50() {
    assert_eq!(Solution::min_end(10, 1048575), 10485759);
}

#[test]
fn test_51() {
    assert_eq!(Solution::min_end(15, 1024), 1038);
}

#[test]
fn test_52() {
    assert_eq!(Solution::min_end(50, 128), 177);
}

#[test]
fn test_53() {
    assert_eq!(Solution::min_end(20, 2047), 40959);
}

#[test]
fn test_54() {
    assert_eq!(Solution::min_end(25, 2048), 2072);
}

#[test]
fn test_55() {
    assert_eq!(Solution::min_end(6, 24), 29);
}

#[test]
fn test_56() {
    assert_eq!(Solution::min_end(7, 128), 134);
}

#[test]
fn test_57() {
    assert_eq!(Solution::min_end(100, 1047552), 1047651);
}

#[test]
fn test_58() {
    assert_eq!(Solution::min_end(15, 512), 526);
}

#[test]
fn test_59() {
    assert_eq!(Solution::min_end(3, 12), 14);
}

#[test]
fn test_60() {
    assert_eq!(Solution::min_end(20, 512), 531);
}

#[test]
fn test_61() {
    assert_eq!(Solution::min_end(1, 1023), 1023);
}

#[test]
fn test_62() {
    assert_eq!(Solution::min_end(100000, 1023), 102399999);
}

#[test]
fn test_63() {
    assert_eq!(Solution::min_end(3, 31), 95);
}

#[test]
fn test_64() {
    assert_eq!(Solution::min_end(5, 31), 159);
}

#[test]
fn test_65() {
    assert_eq!(Solution::min_end(50, 31), 1599);
}

#[test]
fn test_66() {
    assert_eq!(Solution::min_end(500, 512), 1011);
}

#[test]
fn test_67() {
    assert_eq!(Solution::min_end(10000000, 4095), 40959999999);
}

#[test]
fn test_68() {
    assert_eq!(Solution::min_end(64, 2048), 2111);
}

#[test]
fn test_69() {
    assert_eq!(Solution::min_end(8, 32), 39);
}

#[test]
fn test_70() {
    assert_eq!(Solution::min_end(3, 13), 29);
}

#[test]
fn test_71() {
    assert_eq!(Solution::min_end(8, 31), 255);
}

#[test]
fn test_72() {
    assert_eq!(Solution::min_end(7, 64), 70);
}

#[test]
fn test_73() {
    assert_eq!(Solution::min_end(1, 1024), 1024);
}

#[test]
fn test_74() {
    assert_eq!(Solution::min_end(1024, 65535), 67108863);
}

#[test]
fn test_75() {
    assert_eq!(Solution::min_end(100, 4096), 4195);
}

#[test]
fn test_76() {
    assert_eq!(Solution::min_end(7, 1024), 1030);
}

#[test]
fn test_77() {
    assert_eq!(Solution::min_end(9, 256), 264);
}

#[test]
fn test_78() {
    assert_eq!(Solution::min_end(1000, 2048), 3047);
}

#[test]
fn test_79() {
    assert_eq!(Solution::min_end(10, 32), 41);
}

#[test]
fn test_80() {
    assert_eq!(Solution::min_end(50, 255), 12799);
}

#[test]
fn test_81() {
    assert_eq!(Solution::min_end(100000, 128), 200095);
}

#[test]
fn test_82() {
    assert_eq!(Solution::min_end(100000, 1), 199999);
}

#[test]
fn test_83() {
    assert_eq!(Solution::min_end(8, 255), 2047);
}

#[test]
fn test_84() {
    assert_eq!(Solution::min_end(5, 15), 79);
}

#[test]
fn test_85() {
    assert_eq!(Solution::min_end(100, 256), 355);
}

#[test]
fn test_86() {
    assert_eq!(Solution::min_end(25, 128), 152);
}

#[test]
fn test_87() {
    assert_eq!(Solution::min_end(6, 64), 69);
}

#[test]
fn test_88() {
    assert_eq!(Solution::min_end(7, 32), 38);
}

#[test]
fn test_89() {
    assert_eq!(Solution::min_end(5, 12), 28);
}

#[test]
fn test_90() {
    assert_eq!(Solution::min_end(1, 2048), 2048);
}

#[test]
fn test_91() {
    assert_eq!(Solution::min_end(2, 1073741823), 2147483647);
}

#[test]
fn test_92() {
    assert_eq!(Solution::min_end(14, 1023), 14335);
}

#[test]
fn test_93() {
    assert_eq!(Solution::min_end(50, 1024), 1073);
}

#[test]
fn test_94() {
    assert_eq!(Solution::min_end(200, 8192), 8391);
}

#[test]
fn test_95() {
    assert_eq!(Solution::min_end(128, 16383), 2097151);
}

#[test]
fn test_96() {
    assert_eq!(Solution::min_end(20, 3), 79);
}

#[test]
fn test_97() {
    assert_eq!(Solution::min_end(11, 512), 522);
}

#[test]
fn test_98() {
    assert_eq!(Solution::min_end(10, 256), 265);
}

#[test]
fn test_99() {
    assert_eq!(Solution::min_end(1, 8191), 8191);
}

#[test]
fn test_100() {
    assert_eq!(Solution::min_end(1, 1048575), 1048575);
}

#[test]
fn test_101() {
    assert_eq!(Solution::min_end(30, 8191), 245759);
}

#[test]
fn test_102() {
    assert_eq!(Solution::min_end(4, 17), 23);
}

#[test]
fn test_103() {
    assert_eq!(Solution::min_end(5, 10), 26);
}

#[test]
fn test_104() {
    assert_eq!(Solution::min_end(30, 512), 541);
}

#[test]
fn test_105() {
    assert_eq!(Solution::min_end(15, 7), 119);
}

#[test]
fn test_106() {
    assert_eq!(Solution::min_end(9, 255), 2303);
}

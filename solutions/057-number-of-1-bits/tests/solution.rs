include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::hamming_weight(0), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::hamming_weight(4095), 12);
}

#[test]
fn test_3() {
    assert_eq!(Solution::hamming_weight(11), 3);
}

#[test]
fn test_4() {
    assert_eq!(Solution::hamming_weight(15), 4);
}

#[test]
fn test_5() {
    assert_eq!(Solution::hamming_weight(2147483647), 31);
}

#[test]
fn test_6() {
    assert_eq!(Solution::hamming_weight(32), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::hamming_weight(1023), 10);
}

#[test]
fn test_8() {
    assert_eq!(Solution::hamming_weight(2147483645), 30);
}

#[test]
fn test_9() {
    assert_eq!(Solution::hamming_weight(1), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::hamming_weight(65535), 16);
}

#[test]
fn test_11() {
    assert_eq!(Solution::hamming_weight(128), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::hamming_weight(3), 2);
}

#[test]
fn test_13() {
    assert_eq!(Solution::hamming_weight(1099511627776), 1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::hamming_weight(2147483646), 30);
}

#[test]
fn test_15() {
    assert_eq!(Solution::hamming_weight(239), 7);
}

#[test]
fn test_16() {
    assert_eq!(Solution::hamming_weight(53), 4);
}

#[test]
fn test_17() {
    assert_eq!(Solution::hamming_weight(262143), 18);
}

#[test]
fn test_18() {
    assert_eq!(Solution::hamming_weight(43), 4);
}

#[test]
fn test_19() {
    assert_eq!(Solution::hamming_weight(268435456), 1);
}

#[test]
fn test_20() {
    assert_eq!(Solution::hamming_weight(2047), 11);
}

#[test]
fn test_21() {
    assert_eq!(Solution::hamming_weight(5), 2);
}

#[test]
fn test_22() {
    assert_eq!(Solution::hamming_weight(549755813888), 1);
}

#[test]
fn test_23() {
    assert_eq!(Solution::hamming_weight(103), 5);
}

#[test]
fn test_24() {
    assert_eq!(Solution::hamming_weight(37), 3);
}

#[test]
fn test_25() {
    assert_eq!(Solution::hamming_weight(223), 7);
}

#[test]
fn test_26() {
    assert_eq!(Solution::hamming_weight(211), 5);
}

#[test]
fn test_27() {
    assert_eq!(Solution::hamming_weight(73), 3);
}

#[test]
fn test_28() {
    assert_eq!(Solution::hamming_weight(1022), 9);
}

#[test]
fn test_29() {
    assert_eq!(Solution::hamming_weight(33554432), 1);
}

#[test]
fn test_30() {
    assert_eq!(Solution::hamming_weight(251), 7);
}

#[test]
fn test_31() {
    assert_eq!(Solution::hamming_weight(181), 5);
}

#[test]
fn test_32() {
    assert_eq!(Solution::hamming_weight(8), 1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::hamming_weight(193), 3);
}

#[test]
fn test_34() {
    assert_eq!(Solution::hamming_weight(131), 3);
}

#[test]
fn test_35() {
    assert_eq!(Solution::hamming_weight(2048), 1);
}

#[test]
fn test_36() {
    assert_eq!(Solution::hamming_weight(241), 5);
}

#[test]
fn test_37() {
    assert_eq!(Solution::hamming_weight(263), 4);
}

#[test]
fn test_38() {
    assert_eq!(Solution::hamming_weight(8191), 13);
}

#[test]
fn test_39() {
    assert_eq!(Solution::hamming_weight(1000000000), 13);
}

#[test]
fn test_40() {
    assert_eq!(Solution::hamming_weight(32768), 1);
}

#[test]
fn test_41() {
    assert_eq!(Solution::hamming_weight(1048575), 20);
}

#[test]
fn test_42() {
    assert_eq!(Solution::hamming_weight(500000000), 13);
}

#[test]
fn test_43() {
    assert_eq!(Solution::hamming_weight(313), 5);
}

#[test]
fn test_44() {
    assert_eq!(Solution::hamming_weight(167), 5);
}

#[test]
fn test_45() {
    assert_eq!(Solution::hamming_weight(97), 3);
}

#[test]
fn test_46() {
    assert_eq!(Solution::hamming_weight(317), 6);
}

#[test]
fn test_47() {
    assert_eq!(Solution::hamming_weight(107374182), 14);
}

#[test]
fn test_48() {
    assert_eq!(Solution::hamming_weight(271), 5);
}

#[test]
fn test_49() {
    assert_eq!(Solution::hamming_weight(8589934591), 33);
}

#[test]
fn test_50() {
    assert_eq!(Solution::hamming_weight(21), 3);
}

#[test]
fn test_51() {
    assert_eq!(Solution::hamming_weight(107), 5);
}

#[test]
fn test_52() {
    assert_eq!(Solution::hamming_weight(163), 4);
}

#[test]
fn test_53() {
    assert_eq!(Solution::hamming_weight(524288), 1);
}

#[test]
fn test_54() {
    assert_eq!(Solution::hamming_weight(524287), 19);
}

#[test]
fn test_55() {
    assert_eq!(Solution::hamming_weight(28), 3);
}

#[test]
fn test_56() {
    assert_eq!(Solution::hamming_weight(268435455), 28);
}

#[test]
fn test_57() {
    assert_eq!(Solution::hamming_weight(137), 3);
}

#[test]
fn test_58() {
    assert_eq!(Solution::hamming_weight(4), 1);
}

#[test]
fn test_59() {
    assert_eq!(Solution::hamming_weight(134217728), 1);
}

#[test]
fn test_60() {
    assert_eq!(Solution::hamming_weight(257), 2);
}

#[test]
fn test_61() {
    assert_eq!(Solution::hamming_weight(1073741823), 30);
}

#[test]
fn test_62() {
    assert_eq!(Solution::hamming_weight(1610612735), 30);
}

#[test]
fn test_63() {
    assert_eq!(Solution::hamming_weight(16777215), 24);
}

#[test]
fn test_64() {
    assert_eq!(Solution::hamming_weight(101), 4);
}

#[test]
fn test_65() {
    assert_eq!(Solution::hamming_weight(68719476735), 36);
}

#[test]
fn test_66() {
    assert_eq!(Solution::hamming_weight(2199023255551), 41);
}

#[test]
fn test_67() {
    assert_eq!(Solution::hamming_weight(89), 4);
}

#[test]
fn test_68() {
    assert_eq!(Solution::hamming_weight(256), 1);
}

#[test]
fn test_69() {
    assert_eq!(Solution::hamming_weight(1099511627775), 40);
}

#[test]
fn test_70() {
    assert_eq!(Solution::hamming_weight(71), 4);
}

#[test]
fn test_71() {
    assert_eq!(Solution::hamming_weight(151), 5);
}

#[test]
fn test_72() {
    assert_eq!(Solution::hamming_weight(293), 4);
}

#[test]
fn test_73() {
    assert_eq!(Solution::hamming_weight(41), 3);
}

#[test]
fn test_74() {
    assert_eq!(Solution::hamming_weight(9), 2);
}

#[test]
fn test_75() {
    assert_eq!(Solution::hamming_weight(83), 4);
}

#[test]
fn test_76() {
    assert_eq!(Solution::hamming_weight(123456789), 16);
}

#[test]
fn test_77() {
    assert_eq!(Solution::hamming_weight(283), 5);
}

#[test]
fn test_78() {
    assert_eq!(Solution::hamming_weight(8388608), 1);
}

#[test]
fn test_79() {
    assert_eq!(Solution::hamming_weight(79), 5);
}

#[test]
fn test_80() {
    assert_eq!(Solution::hamming_weight(63), 6);
}

#[test]
fn test_81() {
    assert_eq!(Solution::hamming_weight(269), 4);
}

#[test]
fn test_82() {
    assert_eq!(Solution::hamming_weight(61), 5);
}

#[test]
fn test_83() {
    assert_eq!(Solution::hamming_weight(999999999), 21);
}

#[test]
fn test_84() {
    assert_eq!(Solution::hamming_weight(49), 3);
}

#[test]
fn test_85() {
    assert_eq!(Solution::hamming_weight(227), 5);
}

#[test]
fn test_86() {
    assert_eq!(Solution::hamming_weight(47), 5);
}

#[test]
fn test_87() {
    assert_eq!(Solution::hamming_weight(4096), 1);
}

#[test]
fn test_88() {
    assert_eq!(Solution::hamming_weight(113), 4);
}

#[test]
fn test_89() {
    assert_eq!(Solution::hamming_weight(233), 5);
}

#[test]
fn test_90() {
    assert_eq!(Solution::hamming_weight(32767), 15);
}

#[test]
fn test_91() {
    assert_eq!(Solution::hamming_weight(511), 9);
}

#[test]
fn test_92() {
    assert_eq!(Solution::hamming_weight(4294967295), 32);
}

#[test]
fn test_93() {
    assert_eq!(Solution::hamming_weight(1879048192), 3);
}

#[test]
fn test_94() {
    assert_eq!(Solution::hamming_weight(4194303), 22);
}

#[test]
fn test_95() {
    assert_eq!(Solution::hamming_weight(191), 7);
}

#[test]
fn test_96() {
    assert_eq!(Solution::hamming_weight(16), 1);
}

#[test]
fn test_97() {
    assert_eq!(Solution::hamming_weight(4026531840), 4);
}

#[test]
fn test_98() {
    assert_eq!(Solution::hamming_weight(17179869183), 34);
}

#[test]
fn test_99() {
    assert_eq!(Solution::hamming_weight(311), 6);
}

#[test]
fn test_100() {
    assert_eq!(Solution::hamming_weight(179), 5);
}

#[test]
fn test_101() {
    assert_eq!(Solution::hamming_weight(2), 1);
}

#[test]
fn test_102() {
    assert_eq!(Solution::hamming_weight(3221225471), 31);
}

#[test]
fn test_103() {
    assert_eq!(Solution::hamming_weight(2147483644), 29);
}

#[test]
fn test_104() {
    assert_eq!(Solution::hamming_weight(34359738367), 35);
}

#[test]
fn test_105() {
    assert_eq!(Solution::hamming_weight(255), 8);
}

#[test]
fn test_106() {
    assert_eq!(Solution::hamming_weight(127), 7);
}

#[test]
fn test_107() {
    assert_eq!(Solution::hamming_weight(987654321), 17);
}

#[test]
fn test_108() {
    assert_eq!(Solution::hamming_weight(307), 5);
}

#[test]
fn test_109() {
    assert_eq!(Solution::hamming_weight(14), 3);
}

#[test]
fn test_110() {
    assert_eq!(Solution::hamming_weight(26), 3);
}

#[test]
fn test_111() {
    assert_eq!(Solution::hamming_weight(281), 4);
}

#[test]
fn test_112() {
    assert_eq!(Solution::hamming_weight(536870911), 29);
}

#[test]
fn test_113() {
    assert_eq!(Solution::hamming_weight(33554431), 25);
}

#[test]
fn test_114() {
    assert_eq!(Solution::hamming_weight(13), 3);
}

#[test]
fn test_115() {
    assert_eq!(Solution::hamming_weight(277), 4);
}

#[test]
fn test_116() {
    assert_eq!(Solution::hamming_weight(197), 4);
}

#[test]
fn test_117() {
    assert_eq!(Solution::hamming_weight(157), 5);
}

#[test]
fn test_118() {
    assert_eq!(Solution::hamming_weight(16383), 14);
}

#[test]
fn test_119() {
    assert_eq!(Solution::hamming_weight(59), 5);
}

#[test]
fn test_120() {
    assert_eq!(Solution::hamming_weight(173), 5);
}

#[test]
fn test_121() {
    assert_eq!(Solution::hamming_weight(67), 3);
}

#[test]
fn test_122() {
    assert_eq!(Solution::hamming_weight(134217727), 27);
}

#[test]
fn test_123() {
    assert_eq!(Solution::hamming_weight(64), 1);
}

#[test]
fn test_124() {
    assert_eq!(Solution::hamming_weight(8388607), 23);
}

#[test]
fn test_125() {
    assert_eq!(Solution::hamming_weight(254), 7);
}

#[test]
fn test_126() {
    assert_eq!(Solution::hamming_weight(4094), 11);
}

#[test]
fn test_127() {
    assert_eq!(Solution::hamming_weight(1024), 1);
}

#[test]
fn test_128() {
    assert_eq!(Solution::hamming_weight(1073741824), 1);
}

#[test]
fn test_129() {
    assert_eq!(Solution::hamming_weight(22), 3);
}

#[test]
fn test_130() {
    assert_eq!(Solution::hamming_weight(149), 4);
}

#[test]
fn test_131() {
    assert_eq!(Solution::hamming_weight(19), 3);
}

#[test]
fn test_132() {
    assert_eq!(Solution::hamming_weight(1599999999), 23);
}

#[test]
fn test_133() {
    assert_eq!(Solution::hamming_weight(139), 4);
}

#[test]
fn test_134() {
    assert_eq!(Solution::hamming_weight(109), 5);
}

#[test]
fn test_135() {
    assert_eq!(Solution::hamming_weight(31), 5);
}

#[test]
fn test_136() {
    assert_eq!(Solution::hamming_weight(229), 5);
}

#[test]
fn test_137() {
    assert_eq!(Solution::hamming_weight(199), 5);
}

#[test]
fn test_138() {
    assert_eq!(Solution::hamming_weight(7), 3);
}

#[test]
fn test_139() {
    assert_eq!(Solution::hamming_weight(25), 3);
}

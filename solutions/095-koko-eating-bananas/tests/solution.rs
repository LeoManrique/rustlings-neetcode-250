include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3], 5), 2);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_eating_speed(vec![805306400, 805306400, 805306400], 3000000000), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000], 1000000000), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_eating_speed(vec![805306457, 805306457, 805306457], 1000000000), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), 2);
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_eating_speed(vec![805306457, 933693859, 908256970, 820324087, 610103336], 5), 933693859);
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1], 4), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_eating_speed(vec![10, 10, 10, 10], 10), 5);
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_eating_speed(vec![3, 3, 3, 3, 3], 5), 3);
}

#[test]
fn test_11() {
    assert_eq!(Solution::min_eating_speed(vec![3, 3, 3, 3, 3], 10), 2);
}

#[test]
fn test_12() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10), 1);
}

#[test]
fn test_13() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 10), 3);
}

#[test]
fn test_14() {
    assert_eq!(Solution::min_eating_speed(vec![1], 1), 1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
}

#[test]
fn test_16() {
    assert_eq!(Solution::min_eating_speed(vec![8, 9, 7, 4, 2], 3), 10);
}

#[test]
fn test_17() {
    assert_eq!(Solution::min_eating_speed(vec![1, 999999999], 2), 999999999);
}

#[test]
fn test_18() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3], 3), 3);
}

#[test]
fn test_19() {
    assert_eq!(Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000), 3);
}

#[test]
fn test_20() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000, 1000000000, 1000000000], 3), 1000000000);
}

#[test]
fn test_21() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
}

#[test]
fn test_22() {
    assert_eq!(Solution::min_eating_speed(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 10), 3);
}

#[test]
fn test_23() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1], 5), 1);
}

#[test]
fn test_24() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20, 35], 7), 30);
}

#[test]
fn test_25() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), 10);
}

#[test]
fn test_26() {
    assert_eq!(Solution::min_eating_speed(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 20), 35);
}

#[test]
fn test_27() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000], 5), 1000000000);
}

#[test]
fn test_28() {
    assert_eq!(Solution::min_eating_speed(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 10), 19);
}

#[test]
fn test_29() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), 10);
}

#[test]
fn test_30() {
    assert_eq!(Solution::min_eating_speed(vec![5, 8, 6, 29, 33, 12], 9), 15);
}

#[test]
fn test_31() {
    assert_eq!(Solution::min_eating_speed(vec![5, 8, 6, 12, 20], 5), 20);
}

#[test]
fn test_32() {
    assert_eq!(Solution::min_eating_speed(vec![5, 8, 10, 12, 15, 17, 20, 23, 25, 28, 30, 33, 35, 38, 40], 15), 40);
}

#[test]
fn test_33() {
    assert_eq!(Solution::min_eating_speed(vec![332484035, 524908576, 855865114, 632739624, 265801521], 8), 427932557);
}

#[test]
fn test_34() {
    assert_eq!(Solution::min_eating_speed(vec![312884470], 312884469), 2);
}

#[test]
fn test_35() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11, 30, 20], 15), 6);
}

#[test]
fn test_36() {
    assert_eq!(Solution::min_eating_speed(vec![5, 8, 6, 3], 15), 2);
}

#[test]
fn test_37() {
    assert_eq!(Solution::min_eating_speed(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 5), 11);
}

#[test]
fn test_38() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000, 999999999, 888888888, 777777777], 10), 444444444);
}

#[test]
fn test_39() {
    assert_eq!(Solution::min_eating_speed(vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000], 9), 100000000);
}

#[test]
fn test_40() {
    assert_eq!(Solution::min_eating_speed(vec![1, 10, 100, 1000, 10000, 100000], 6), 100000);
}

#[test]
fn test_41() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 100), 1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::min_eating_speed(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 10), 10);
}

#[test]
fn test_43() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11, 20, 25, 30], 15), 9);
}

#[test]
fn test_44() {
    assert_eq!(Solution::min_eating_speed(vec![50, 25, 75, 25, 100, 50, 25, 75, 25, 100, 50, 25, 75, 25, 100, 50, 25, 75, 25, 100], 50), 25);
}

#[test]
fn test_45() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 15), 15);
}

#[test]
fn test_46() {
    assert_eq!(Solution::min_eating_speed(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 9), 9);
}

#[test]
fn test_47() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 20), 20);
}

#[test]
fn test_48() {
    assert_eq!(Solution::min_eating_speed(vec![1, 10, 100, 1000, 10000, 100000, 1000000], 14), 125000);
}

#[test]
fn test_49() {
    assert_eq!(Solution::min_eating_speed(vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000], 25), 66666667);
}

#[test]
fn test_50() {
    assert_eq!(Solution::min_eating_speed(vec![10, 20, 30, 40, 50], 100), 2);
}

#[test]
fn test_51() {
    assert_eq!(Solution::min_eating_speed(vec![5, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30], 15), 26);
}

#[test]
fn test_52() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000, 1, 1000000000, 1, 1000000000], 1000000000), 4);
}

#[test]
fn test_53() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11, 11, 11, 11, 11, 11, 11], 10), 11);
}

#[test]
fn test_54() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11, 20, 30, 40, 50, 60, 70, 80, 90, 100], 15), 80);
}

#[test]
fn test_55() {
    assert_eq!(Solution::min_eating_speed(vec![5, 8, 6, 7, 1, 2, 3, 4, 5], 9), 8);
}

#[test]
fn test_56() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 100), 3);
}

#[test]
fn test_57() {
    assert_eq!(Solution::min_eating_speed(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 5), 10);
}

#[test]
fn test_58() {
    assert_eq!(Solution::min_eating_speed(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 20), 10);
}

#[test]
fn test_59() {
    assert_eq!(Solution::min_eating_speed(vec![100, 200, 300, 400, 500], 50), 32);
}

#[test]
fn test_60() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000, 1000000000, 1000000000], 3), 1000000000);
}

#[test]
fn test_61() {
    assert_eq!(Solution::min_eating_speed(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 10), 10);
}

#[test]
fn test_62() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10), 2);
}

#[test]
fn test_63() {
    assert_eq!(Solution::min_eating_speed(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000], 100), 234);
}

#[test]
fn test_64() {
    assert_eq!(Solution::min_eating_speed(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 10), 6);
}

#[test]
fn test_65() {
    assert_eq!(Solution::min_eating_speed(vec![300000000, 100000000, 200000000, 400000000, 500000000], 1000000000), 2);
}

#[test]
fn test_66() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 100), 1);
}

#[test]
fn test_67() {
    assert_eq!(Solution::min_eating_speed(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 5), 11);
}

#[test]
fn test_68() {
    assert_eq!(Solution::min_eating_speed(vec![5, 4, 3, 2, 1], 15), 1);
}

#[test]
fn test_69() {
    assert_eq!(Solution::min_eating_speed(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 10), 100);
}

#[test]
fn test_70() {
    assert_eq!(Solution::min_eating_speed(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 9), 20);
}

#[test]
fn test_71() {
    assert_eq!(Solution::min_eating_speed(vec![1, 10, 100, 1000, 10000], 10), 1667);
}

#[test]
fn test_72() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1), 2);
}

#[test]
fn test_73() {
    assert_eq!(Solution::min_eating_speed(vec![100, 150, 200, 250, 300, 350, 400], 15), 150);
}

#[test]
fn test_74() {
    assert_eq!(Solution::min_eating_speed(vec![100000000, 200000000, 300000000, 400000000, 500000000], 5), 500000000);
}

#[test]
fn test_75() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11, 100, 101, 200], 20), 26);
}

#[test]
fn test_76() {
    assert_eq!(Solution::min_eating_speed(vec![8, 5, 6, 10, 12], 7), 8);
}

#[test]
fn test_77() {
    assert_eq!(Solution::min_eating_speed(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100], 10), 100);
}

#[test]
fn test_78() {
    assert_eq!(Solution::min_eating_speed(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 50), 120);
}

#[test]
fn test_79() {
    assert_eq!(Solution::min_eating_speed(vec![100, 100, 100, 100, 100], 10), 50);
}

#[test]
fn test_80() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000, 1000000000, 1000000000], 1000000000), 4);
}

#[test]
fn test_81() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 19), 2);
}

#[test]
fn test_82() {
    assert_eq!(Solution::min_eating_speed(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50], 100), 5);
}

#[test]
fn test_83() {
    assert_eq!(Solution::min_eating_speed(vec![332484035, 524900671, 855865114, 632088198, 232463062], 8), 427932557);
}

#[test]
fn test_84() {
    assert_eq!(Solution::min_eating_speed(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 10), 5);
}

#[test]
fn test_85() {
    assert_eq!(Solution::min_eating_speed(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 15), 10);
}

#[test]
fn test_86() {
    assert_eq!(Solution::min_eating_speed(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 9), 11);
}

#[test]
fn test_87() {
    assert_eq!(Solution::min_eating_speed(vec![10, 15, 20, 25, 30, 35, 40], 20), 10);
}

#[test]
fn test_88() {
    assert_eq!(Solution::min_eating_speed(vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000], 10000), 1);
}

#[test]
fn test_89() {
    assert_eq!(Solution::min_eating_speed(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 5), 101);
}

#[test]
fn test_90() {
    assert_eq!(Solution::min_eating_speed(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 15), 50);
}

#[test]
fn test_91() {
    assert_eq!(Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000), 3);
}

#[test]
fn test_92() {
    assert_eq!(Solution::min_eating_speed(vec![900000000, 800000000, 700000000, 600000000, 500000000], 5), 900000000);
}

#[test]
fn test_93() {
    assert_eq!(Solution::min_eating_speed(vec![8, 5, 4, 10, 7, 9, 6], 15), 4);
}

#[test]
fn test_94() {
    assert_eq!(Solution::min_eating_speed(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50], 15), 25);
}

#[test]
fn test_95() {
    assert_eq!(Solution::min_eating_speed(vec![10, 15, 7, 30], 6), 15);
}

#[test]
fn test_96() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11, 33, 45, 55], 25), 7);
}

#[test]
fn test_97() {
    assert_eq!(Solution::min_eating_speed(vec![8, 12, 15, 18, 22], 12), 8);
}

#[test]
fn test_98() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11, 11, 11, 11, 11, 11, 11], 20), 6);
}

#[test]
fn test_99() {
    assert_eq!(Solution::min_eating_speed(vec![10, 20, 30, 40, 50], 20), 9);
}

#[test]
fn test_100() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9), 11);
}

#[test]
fn test_101() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), 2);
}

#[test]
fn test_102() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000, 1000000000, 1000000000], 3000000000), 1);
}

#[test]
fn test_103() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 20), 1);
}

#[test]
fn test_104() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11, 20, 25, 30], 25), 5);
}

#[test]
fn test_105() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000], 32), 250000000);
}

#[test]
fn test_106() {
    assert_eq!(Solution::min_eating_speed(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 5), 20);
}

#[test]
fn test_107() {
    assert_eq!(Solution::min_eating_speed(vec![999999999, 999999998, 999999997, 999999996, 999999995], 10), 500000000);
}

#[test]
fn test_108() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 15), 21);
}

#[test]
fn test_109() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 11);
}

#[test]
fn test_110() {
    assert_eq!(Solution::min_eating_speed(vec![33, 41, 17, 29, 38, 36, 40, 9, 66, 27], 13), 38);
}

#[test]
fn test_111() {
    assert_eq!(Solution::min_eating_speed(vec![1000000000, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10), 1000000000);
}

#[test]
fn test_112() {
    assert_eq!(Solution::min_eating_speed(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 9), 20);
}

#[test]
fn test_113() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 7), 20);
}

#[test]
fn test_114() {
    assert_eq!(Solution::min_eating_speed(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8], 5), 9);
}

#[test]
fn test_115() {
    assert_eq!(Solution::min_eating_speed(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30], 20), 20);
}

#[test]
fn test_116() {
    assert_eq!(Solution::min_eating_speed(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 5), 11);
}

#[test]
fn test_117() {
    assert_eq!(Solution::min_eating_speed(vec![987654321, 123456789, 987654321, 123456789, 987654321], 5), 987654321);
}

#[test]
fn test_118() {
    assert_eq!(Solution::min_eating_speed(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 1), 11);
}

#[test]
fn test_119() {
    assert_eq!(Solution::min_eating_speed(vec![2, 4, 9, 16, 25, 36, 49, 64, 81, 100], 15), 41);
}

#[test]
fn test_120() {
    assert_eq!(Solution::min_eating_speed(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 25), 27);
}

#[test]
fn test_121() {
    assert_eq!(Solution::min_eating_speed(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10), 1);
}

#[test]
fn test_122() {
    assert_eq!(Solution::min_eating_speed(vec![1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100], 20), 100);
}

#[test]
fn test_123() {
    assert_eq!(Solution::min_eating_speed(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 1), 11);
}

#[test]
fn test_124() {
    assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 20), 4);
}

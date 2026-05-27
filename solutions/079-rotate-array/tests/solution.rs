include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1), None);
}

#[test]
fn test_2() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), None);
}

#[test]
fn test_3() {
    assert_eq!(Solution::rotate(vec![0, 1, 0, 1, 0, 1, 0], 2), None);
}

#[test]
fn test_4() {
    assert_eq!(Solution::rotate(vec![1, 2], 1), None);
}

#[test]
fn test_5() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), None);
}

#[test]
fn test_6() {
    assert_eq!(Solution::rotate(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 100000), None);
}

#[test]
fn test_7() {
    assert_eq!(Solution::rotate(vec![1, 2, 3], 0), None);
}

#[test]
fn test_8() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3), None);
}

#[test]
fn test_9() {
    assert_eq!(Solution::rotate(vec![1], 1), None);
}

#[test]
fn test_10() {
    assert_eq!(Solution::rotate(vec![1], 0), None);
}

#[test]
fn test_11() {
    assert_eq!(Solution::rotate(vec![-1, -100, 3, 99], 2), None);
}

#[test]
fn test_12() {
    assert_eq!(Solution::rotate(vec![1], 100), None);
}

#[test]
fn test_13() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7], 3), None);
}

#[test]
fn test_14() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 2), None);
}

#[test]
fn test_15() {
    assert_eq!(Solution::rotate(vec![5, 3, 7, 8, 1], 5), None);
}

#[test]
fn test_16() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 15), None);
}

#[test]
fn test_17() {
    assert_eq!(Solution::rotate(vec![1, 2, 3], 4), None);
}

#[test]
fn test_18() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7], 7), None);
}

#[test]
fn test_19() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7], 100000), None);
}

#[test]
fn test_20() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7], 14), None);
}

#[test]
fn test_21() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7], 0), None);
}

#[test]
fn test_22() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], 100000), None);
}

#[test]
fn test_23() {
    assert_eq!(Solution::rotate(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 8), None);
}

#[test]
fn test_24() {
    assert_eq!(Solution::rotate(vec![1, 2, 3], 99999), None);
}

#[test]
fn test_25() {
    assert_eq!(Solution::rotate(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10], 4), None);
}

#[test]
fn test_26() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 19), None);
}

#[test]
fn test_27() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 17), None);
}

#[test]
fn test_28() {
    assert_eq!(Solution::rotate(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 100), None);
}

#[test]
fn test_29() {
    assert_eq!(Solution::rotate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130], 5), None);
}

#[test]
fn test_30() {
    assert_eq!(Solution::rotate(vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10], 7), None);
}

#[test]
fn test_31() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 0), None);
}

#[test]
fn test_32() {
    assert_eq!(Solution::rotate(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 7), None);
}

#[test]
fn test_33() {
    assert_eq!(Solution::rotate(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], 10), None);
}

#[test]
fn test_34() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 30), None);
}

#[test]
fn test_35() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 0), None);
}

#[test]
fn test_36() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5], 5), None);
}

#[test]
fn test_37() {
    assert_eq!(Solution::rotate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250], 17), None);
}

#[test]
fn test_38() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 20), None);
}

#[test]
fn test_39() {
    assert_eq!(Solution::rotate(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1000), None);
}

#[test]
fn test_40() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 100000), None);
}

#[test]
fn test_41() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40], 100), None);
}

#[test]
fn test_42() {
    assert_eq!(Solution::rotate(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 3), None);
}

#[test]
fn test_43() {
    assert_eq!(Solution::rotate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 100000), None);
}

#[test]
fn test_44() {
    assert_eq!(Solution::rotate(vec![-2147483648, 2147483647, 0, -1, 1], 1), None);
}

#[test]
fn test_45() {
    assert_eq!(Solution::rotate(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 50), None);
}

#[test]
fn test_46() {
    assert_eq!(Solution::rotate(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10), None);
}

#[test]
fn test_47() {
    assert_eq!(Solution::rotate(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21], 11), None);
}

#[test]
fn test_48() {
    assert_eq!(Solution::rotate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 5), None);
}

#[test]
fn test_49() {
    assert_eq!(Solution::rotate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], 17), None);
}

#[test]
fn test_50() {
    assert_eq!(Solution::rotate(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20], 20), None);
}

#[test]
fn test_51() {
    assert_eq!(Solution::rotate(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 35), None);
}

#[test]
fn test_52() {
    assert_eq!(Solution::rotate(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], 20), None);
}

#[test]
fn test_53() {
    assert_eq!(Solution::rotate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), None);
}

#[test]
fn test_54() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 21), None);
}

#[test]
fn test_55() {
    assert_eq!(Solution::rotate(vec![-2147483648, 2147483647, -1, 0, 1], 100000), None);
}

#[test]
fn test_56() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50], 45), None);
}

#[test]
fn test_57() {
    assert_eq!(Solution::rotate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 55), None);
}

#[test]
fn test_58() {
    assert_eq!(Solution::rotate(vec![100000, 200000, 300000, 400000, 500000], 100000), None);
}

#[test]
fn test_59() {
    assert_eq!(Solution::rotate(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 100000), None);
}

#[test]
fn test_60() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 20), None);
}

#[test]
fn test_61() {
    assert_eq!(Solution::rotate(vec![3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93, 95, 97, 99], 45), None);
}

#[test]
fn test_62() {
    assert_eq!(Solution::rotate(vec![5, 4, 3, 2, 1], 10), None);
}

#[test]
fn test_63() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5], 25), None);
}

#[test]
fn test_64() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 7), None);
}

#[test]
fn test_65() {
    assert_eq!(Solution::rotate(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 10), None);
}

#[test]
fn test_66() {
    assert_eq!(Solution::rotate(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], 25), None);
}

#[test]
fn test_67() {
    assert_eq!(Solution::rotate(vec![2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40], 41), None);
}

#[test]
fn test_68() {
    assert_eq!(Solution::rotate(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 30), None);
}

#[test]
fn test_69() {
    assert_eq!(Solution::rotate(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000], 1000), None);
}

#[test]
fn test_70() {
    assert_eq!(Solution::rotate(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5], 3), None);
}

#[test]
fn test_71() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 1), None);
}

#[test]
fn test_72() {
    assert_eq!(Solution::rotate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10000), None);
}

#[test]
fn test_73() {
    assert_eq!(Solution::rotate(vec![5, 4, 3, 2, 1], 3), None);
}

#[test]
fn test_74() {
    assert_eq!(Solution::rotate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 5), None);
}

#[test]
fn test_75() {
    assert_eq!(Solution::rotate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200], 19), None);
}

#[test]
fn test_76() {
    assert_eq!(Solution::rotate(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15], 12), None);
}

#[test]
fn test_77() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], 0), None);
}

#[test]
fn test_78() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 35), None);
}

#[test]
fn test_79() {
    assert_eq!(Solution::rotate(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31], 18), None);
}

#[test]
fn test_80() {
    assert_eq!(Solution::rotate(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100], 7), None);
}

#[test]
fn test_81() {
    assert_eq!(Solution::rotate(vec![5, 3, 1, 2, 4, 6, 7, 8, 9, 10], 10), None);
}

#[test]
fn test_82() {
    assert_eq!(Solution::rotate(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 3), None);
}

#[test]
fn test_83() {
    assert_eq!(Solution::rotate(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30], 15), None);
}

#[test]
fn test_84() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 19), None);
}

#[test]
fn test_85() {
    assert_eq!(Solution::rotate(vec![1], 100000), None);
}

#[test]
fn test_86() {
    assert_eq!(Solution::rotate(vec![5, 3, 8, 6, 2, 7, 4, 1], 3), None);
}

#[test]
fn test_87() {
    assert_eq!(Solution::rotate(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 0), None);
}

#[test]
fn test_88() {
    assert_eq!(Solution::rotate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 29), None);
}

#[test]
fn test_89() {
    assert_eq!(Solution::rotate(vec![1000000000, 2000000000, 3000000000, 4000000000, 5000000000], 3), None);
}

#[test]
fn test_90() {
    assert_eq!(Solution::rotate(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 5), None);
}

#[test]
fn test_91() {
    assert_eq!(Solution::rotate(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31], 13), None);
}

#[test]
fn test_92() {
    assert_eq!(Solution::rotate(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 19), None);
}

#[test]
fn test_93() {
    assert_eq!(Solution::rotate(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 4), None);
}

#[test]
fn test_94() {
    assert_eq!(Solution::rotate(vec![-10, -20, -30, -40, -50, -60, -70, -80, -90, -100], 3), None);
}

#[test]
fn test_95() {
    assert_eq!(Solution::rotate(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 200000), None);
}

#[test]
fn test_96() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 21), None);
}

#[test]
fn test_97() {
    assert_eq!(Solution::rotate(vec![29, 59, 89, 119, 149, 179, 209, 239, 269, 299, 329, 359, 389, 419, 449, 479, 509, 539, 569, 599], 150), None);
}

#[test]
fn test_98() {
    assert_eq!(Solution::rotate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), None);
}

#[test]
fn test_99() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 10), None);
}

#[test]
fn test_100() {
    assert_eq!(Solution::rotate(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 3), None);
}

#[test]
fn test_101() {
    assert_eq!(Solution::rotate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 34), None);
}

#[test]
fn test_102() {
    assert_eq!(Solution::rotate(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3), None);
}

#[test]
fn test_103() {
    assert_eq!(Solution::rotate(vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000], 100000), None);
}

#[test]
fn test_104() {
    assert_eq!(Solution::rotate(vec![-1, 0, 1, -2, 2, -3, 3, -4, 4, -5, 5, -6, 6, -7, 7], 8), None);
}

#[test]
fn test_105() {
    assert_eq!(Solution::rotate(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 3), None);
}

#[test]
fn test_106() {
    assert_eq!(Solution::rotate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 100000), None);
}

#[test]
fn test_107() {
    assert_eq!(Solution::rotate(vec![1000000000, 1000000001, 1000000002, 1000000003, 1000000004], 2), None);
}

#[test]
fn test_108() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 1), None);
}

#[test]
fn test_109() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 20), None);
}

#[test]
fn test_110() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35], 30), None);
}

#[test]
fn test_111() {
    assert_eq!(Solution::rotate(vec![2, 3, 4, 5, 6, 7, 8, 9, 1, 0, -1, -2, -3, -4, -5], 7), None);
}

#[test]
fn test_112() {
    assert_eq!(Solution::rotate(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000], 19), None);
}

#[test]
fn test_113() {
    assert_eq!(Solution::rotate(vec![1, 2], 100000), None);
}

#[test]
fn test_114() {
    assert_eq!(Solution::rotate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 25), None);
}

#[test]
fn test_115() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5], 0), None);
}

#[test]
fn test_116() {
    assert_eq!(Solution::rotate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 30), None);
}

#[test]
fn test_117() {
    assert_eq!(Solution::rotate(vec![5, 1, 9, 3, 7, 2, 8, 6, 4, 0, 11, 12, 13, 14, 15], 7), None);
}

#[test]
fn test_118() {
    assert_eq!(Solution::rotate(vec![-2147483648, 2147483647, 0, 100, -100, 50, -50, 1, -1, 2, -2], 4), None);
}

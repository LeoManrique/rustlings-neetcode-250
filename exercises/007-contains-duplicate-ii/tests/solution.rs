include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, -1000000000, 1000000000, -1000000000], 2), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 8), false);
}

#[test]
fn test_3() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0], 10), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9], 1), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 2, 1], 2), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5], 4), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5], 5), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 9), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 18), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![9, 9], 1), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![9, 1, 2, 3, 9], 4), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1], 0), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![999999999, 999999999, 1, 1], 2), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![999999999, -999999999, 999999999], 2), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 2], 2), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 8), false);
}

#[test]
fn test_20() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1], 0), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1], 20), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
}

#[test]
fn test_23() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0), false);
}

#[test]
fn test_24() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 1, 2, 1, 2, 1, 2], 1), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 7), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 2, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], 1), false);
}

#[test]
fn test_29() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![999999999, 999999998, 999999997, 999999996, 999999995, 999999994, 999999993, 999999992, 999999991, 999999999], 9), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1], 1), false);
}

#[test]
fn test_31() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 10], 10), true);
}

#[test]
fn test_33() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 50], 49), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1], 2), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 1, 1], 20), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 1, 2, 3, 4, 5], 49), false);
}

#[test]
fn test_38() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 10), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 20), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1], 10), false);
}

#[test]
fn test_41() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 9), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1], 20), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1], 10), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 1, 1], 1), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1], 10), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 30], 29), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, 1000000000, 1, 2, 3], 1), true);
}

#[test]
fn test_50() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, -1000000000, 1000000000, -1000000000, 1000000000], 2), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 20), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1], 14), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9], 9), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4], 1), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9], 8), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9], 17), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1], 0), false);
}

#[test]
fn test_58() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, 999999999, 1000000000, 999999999, 1000000000, 999999999], 2), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, 1000000000], 1), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), false);
}

#[test]
fn test_61() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5], 4), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1], 29), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 20), false);
}

#[test]
fn test_65() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 10], 9), true);
}

#[test]
fn test_66() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 9), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 1], 49), false);
}

#[test]
fn test_68() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1], 10), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![-1000000000, -2000000000, -1000000000, -3000000000, -2000000000], 2), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1], 100000), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 18), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 8), false);
}

#[test]
fn test_74() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, 1000000001, 1000000002, 1000000003, 1000000004, 1000000005, 1000000006, 1000000007, 1000000008, 1000000009, 1000000000], 10), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![10, 1, 2, 10, 3, 4, 5, 10, 6, 7, 8, 9, 10], 8), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 1), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 1], 24), false);
}

#[test]
fn test_78() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1, 0, 1, 0, 1], 2), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1], 19), false);
}

#[test]
fn test_80() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 20], 20), true);
}

#[test]
fn test_81() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1], 1), true);
}

#[test]
fn test_82() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 5), true);
}

#[test]
fn test_83() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 5), false);
}

#[test]
fn test_84() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, 2000000000, 1000000000, 3000000000, 2000000000], 2), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1], 20), true);
}

#[test]
fn test_86() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, 2000000000, 1000000000], 2), true);
}

#[test]
fn test_87() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1], 10), true);
}

#[test]
fn test_88() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1, 0, 1, 0, 1, 0, 1], 2), true);
}

#[test]
fn test_89() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 19), false);
}

#[test]
fn test_90() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9], 10), true);
}

#[test]
fn test_91() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 10), true);
}

#[test]
fn test_92() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1], 30), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), false);
}

#[test]
fn test_94() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5], 10), false);
}

#[test]
fn test_95() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 1], 39), false);
}

#[test]
fn test_96() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1), true);
}

#[test]
fn test_97() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 1], 2), true);
}

#[test]
fn test_98() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, -1000000000, 1000000000, -1000000000], 2), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1, 0, 1], 2), true);
}

#[test]
fn test_100() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![5, 4, 3, 2, 1, 0, 5, 4, 3, 2, 1, 0], 5), false);
}

#[test]
fn test_101() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1000000000, 1000000001, 1000000000, 1000000001, 1000000000], 2), true);
}

#[test]
fn test_102() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 1), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![10, 20, 30, 40, 50, 10, 20, 30, 40, 50], 4), false);
}

#[test]
fn test_104() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 1], 20), true);
}

#[test]
fn test_105() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![999999999, 999999998, 999999997, 999999996, 999999995, 999999994, 999999993, 999999992, 999999991, 999999990, 999999999], 9), false);
}

#[test]
fn test_106() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5], 10), true);
}

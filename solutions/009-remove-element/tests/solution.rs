include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::remove_element(vec![], 1), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::remove_element(vec![0, 0, 0, 0, 0], 0), 0);
}

#[test]
fn test_3() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 9);
}

#[test]
fn test_4() {
    assert_eq!(Solution::remove_element(vec![5, 5, 5, 5, 5, 5], 5), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::remove_element(vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);
}

#[test]
fn test_6() {
    assert_eq!(Solution::remove_element(vec![50, 49, 48, 47, 46], 46), 4);
}

#[test]
fn test_7() {
    assert_eq!(Solution::remove_element(vec![1, 1, 1, 1, 1], 1), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::remove_element(vec![3, 2, 2, 3], 3), 2);
}

#[test]
fn test_9() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5], 6), 5);
}

#[test]
fn test_10() {
    assert_eq!(Solution::remove_element(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 5), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::remove_element(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::remove_element(vec![49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 25), 49);
}

#[test]
fn test_13() {
    assert_eq!(Solution::remove_element(vec![42, 27, 15, 42, 27, 15, 42, 27, 15, 42], 42), 6);
}

#[test]
fn test_14() {
    assert_eq!(Solution::remove_element(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40], 2), 19);
}

#[test]
fn test_15() {
    assert_eq!(Solution::remove_element(vec![42], 100), 1);
}

#[test]
fn test_16() {
    assert_eq!(Solution::remove_element(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1), 0);
}

#[test]
fn test_17() {
    assert_eq!(Solution::remove_element(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6], 5), 10);
}

#[test]
fn test_18() {
    assert_eq!(Solution::remove_element(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19], 0), 19);
}

#[test]
fn test_19() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], 12), 24);
}

#[test]
fn test_20() {
    assert_eq!(Solution::remove_element(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 2), 0);
}

#[test]
fn test_21() {
    assert_eq!(Solution::remove_element(vec![10, 20, 30, 40, 50, 10, 20, 30, 40, 50, 10, 20, 30, 40, 50], 50), 12);
}

#[test]
fn test_22() {
    assert_eq!(Solution::remove_element(vec![3, 3, 3, 3, 3, 2, 2, 1, 1, 0, 0], 3), 6);
}

#[test]
fn test_23() {
    assert_eq!(Solution::remove_element(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75], 25), 14);
}

#[test]
fn test_24() {
    assert_eq!(Solution::remove_element(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 1), 18);
}

#[test]
fn test_25() {
    assert_eq!(Solution::remove_element(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 10);
}

#[test]
fn test_26() {
    assert_eq!(Solution::remove_element(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 0), 10);
}

#[test]
fn test_27() {
    assert_eq!(Solution::remove_element(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31], 46), 19);
}

#[test]
fn test_28() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10), 19);
}

#[test]
fn test_29() {
    assert_eq!(Solution::remove_element(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41], 21), 20);
}

#[test]
fn test_30() {
    assert_eq!(Solution::remove_element(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5, 0, 2, 8, 8, 4, 1, 9, 7, 1, 6, 9, 3, 9, 9, 9, 0, 0], 9), 39);
}

#[test]
fn test_31() {
    assert_eq!(Solution::remove_element(vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42], 42), 0);
}

#[test]
fn test_32() {
    assert_eq!(Solution::remove_element(vec![10, 20, 10, 30, 10, 40, 50, 10], 10), 4);
}

#[test]
fn test_33() {
    assert_eq!(Solution::remove_element(vec![42], 42), 0);
}

#[test]
fn test_34() {
    assert_eq!(Solution::remove_element(vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 1), 8);
}

#[test]
fn test_35() {
    assert_eq!(Solution::remove_element(vec![30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44], 33), 14);
}

#[test]
fn test_36() {
    assert_eq!(Solution::remove_element(vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42], 42), 0);
}

#[test]
fn test_37() {
    assert_eq!(Solution::remove_element(vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 4, 5, 6], 2), 5);
}

#[test]
fn test_38() {
    assert_eq!(Solution::remove_element(vec![7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9], 8), 8);
}

#[test]
fn test_39() {
    assert_eq!(Solution::remove_element(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], 3), 19);
}

#[test]
fn test_40() {
    assert_eq!(Solution::remove_element(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 5), 18);
}

#[test]
fn test_41() {
    assert_eq!(Solution::remove_element(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41], 43), 9);
}

#[test]
fn test_42() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10), 19);
}

#[test]
fn test_43() {
    assert_eq!(Solution::remove_element(vec![1, 3, 3, 3, 2, 2, 2, 4, 4, 4, 5, 5, 5, 6, 6, 6], 3), 13);
}

#[test]
fn test_44() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 2, 2, 2, 3, 4, 5], 2), 7);
}

#[test]
fn test_45() {
    assert_eq!(Solution::remove_element(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2], 2), 10);
}

#[test]
fn test_46() {
    assert_eq!(Solution::remove_element(vec![3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, -1, -1, -1, -1], 1), 16);
}

#[test]
fn test_47() {
    assert_eq!(Solution::remove_element(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50], 50), 0);
}

#[test]
fn test_48() {
    assert_eq!(Solution::remove_element(vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42], 42), 0);
}

#[test]
fn test_49() {
    assert_eq!(Solution::remove_element(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 10), 0);
}

#[test]
fn test_50() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 11), 10);
}

#[test]
fn test_51() {
    assert_eq!(Solution::remove_element(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 10), 0);
}

#[test]
fn test_52() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], 15), 24);
}

#[test]
fn test_53() {
    assert_eq!(Solution::remove_element(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22], 2), 10);
}

#[test]
fn test_54() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 7), 14);
}

#[test]
fn test_55() {
    assert_eq!(Solution::remove_element(vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9], 0), 18);
}

#[test]
fn test_56() {
    assert_eq!(Solution::remove_element(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40], 20), 19);
}

#[test]
fn test_57() {
    assert_eq!(Solution::remove_element(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), 0);
}

#[test]
fn test_58() {
    assert_eq!(Solution::remove_element(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50], 25), 18);
}

#[test]
fn test_59() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 3, 3, 2, 1, 0, 3], 3), 7);
}

#[test]
fn test_60() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 3, 2, 1], 2), 6);
}

#[test]
fn test_61() {
    assert_eq!(Solution::remove_element(vec![30, 28, 26, 24, 22, 20, 18, 16, 14, 12, 10, 8, 6, 4, 2], 28), 14);
}

#[test]
fn test_62() {
    assert_eq!(Solution::remove_element(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 3), 18);
}

#[test]
fn test_63() {
    assert_eq!(Solution::remove_element(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], 40), 20);
}

#[test]
fn test_64() {
    assert_eq!(Solution::remove_element(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 8), 14);
}

#[test]
fn test_65() {
    assert_eq!(Solution::remove_element(vec![42, 24, 42, 24, 42, 24, 42, 24], 24), 4);
}

#[test]
fn test_66() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 15), 19);
}

#[test]
fn test_67() {
    assert_eq!(Solution::remove_element(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41], 41), 9);
}

#[test]
fn test_68() {
    assert_eq!(Solution::remove_element(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), 0);
}

#[test]
fn test_69() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 1), 29);
}

#[test]
fn test_70() {
    assert_eq!(Solution::remove_element(vec![7, 7, 7, 7, 7, 8, 8, 8, 9, 9, 10], 7), 6);
}

#[test]
fn test_71() {
    assert_eq!(Solution::remove_element(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20], 15), 38);
}

#[test]
fn test_72() {
    assert_eq!(Solution::remove_element(vec![10, 20, 10, 30, 10, 40, 10, 50], 10), 4);
}

#[test]
fn test_73() {
    assert_eq!(Solution::remove_element(vec![10, 20, 30, 40, 50, 10, 20, 30, 40, 50, 60], 10), 9);
}

#[test]
fn test_74() {
    assert_eq!(Solution::remove_element(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50], 50), 0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::remove_element(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31], 46), 19);
}

#[test]
fn test_76() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 15), 19);
}

#[test]
fn test_77() {
    assert_eq!(Solution::remove_element(vec![25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25], 25), 0);
}

#[test]
fn test_78() {
    assert_eq!(Solution::remove_element(vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42], 42), 0);
}

#[test]
fn test_79() {
    assert_eq!(Solution::remove_element(vec![10, 20, 30, 10, 20, 30, 10, 20, 30, 10], 10), 6);
}

#[test]
fn test_80() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1), 18);
}

#[test]
fn test_81() {
    assert_eq!(Solution::remove_element(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21], 11), 10);
}

#[test]
fn test_82() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50], 50), 49);
}

#[test]
fn test_83() {
    assert_eq!(Solution::remove_element(vec![3, 1, 3, 3, 2, 3, 3, 3, 3, 3], 3), 2);
}

#[test]
fn test_84() {
    assert_eq!(Solution::remove_element(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5], 5), 8);
}

#[test]
fn test_85() {
    assert_eq!(Solution::remove_element(vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10], 0), 10);
}

#[test]
fn test_86() {
    assert_eq!(Solution::remove_element(vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3], 2), 10);
}

#[test]
fn test_87() {
    assert_eq!(Solution::remove_element(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100], 50), 19);
}

#[test]
fn test_88() {
    assert_eq!(Solution::remove_element(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 42), 50);
}

#[test]
fn test_89() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 1), 19);
}

#[test]
fn test_90() {
    assert_eq!(Solution::remove_element(vec![10, 20, 30, 20, 10, 30, 40, 20], 20), 5);
}

#[test]
fn test_91() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5], 6), 5);
}

#[test]
fn test_92() {
    assert_eq!(Solution::remove_element(vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42], 42), 0);
}

#[test]
fn test_93() {
    assert_eq!(Solution::remove_element(vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42], 42), 0);
}

#[test]
fn test_94() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 18);
}

#[test]
fn test_95() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 20), 19);
}

#[test]
fn test_96() {
    assert_eq!(Solution::remove_element(vec![10, 20, 30, 40, 50, 10, 20, 30, 40, 50], 30), 8);
}

#[test]
fn test_97() {
    assert_eq!(Solution::remove_element(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31], 40), 19);
}

#[test]
fn test_98() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1], 1), 9);
}

#[test]
fn test_99() {
    assert_eq!(Solution::remove_element(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30], 22), 14);
}

#[test]
fn test_100() {
    assert_eq!(Solution::remove_element(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1), 0);
}

#[test]
fn test_101() {
    assert_eq!(Solution::remove_element(vec![10, 20, 30, 40, 50, 50, 40, 30, 20, 10], 10), 8);
}

#[test]
fn test_102() {
    assert_eq!(Solution::remove_element(vec![10, 10, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9], 10), 9);
}

#[test]
fn test_103() {
    assert_eq!(Solution::remove_element(vec![10, 10, 10, 9, 9, 8, 8, 8, 7, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 1], 10), 18);
}

#[test]
fn test_104() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 10), 14);
}

#[test]
fn test_105() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 11), 10);
}

#[test]
fn test_106() {
    assert_eq!(Solution::remove_element(vec![10, 1, 10, 2, 10, 3, 10, 4], 10), 4);
}

#[test]
fn test_107() {
    assert_eq!(Solution::remove_element(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], 13), 14);
}

#[test]
fn test_108() {
    assert_eq!(Solution::remove_element(vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42], 42), 0);
}

#[test]
fn test_109() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 15), 14);
}

#[test]
fn test_110() {
    assert_eq!(Solution::remove_element(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7], 7), 0);
}

#[test]
fn test_111() {
    assert_eq!(Solution::remove_element(vec![1, 1, 2, 3, 3, 4, 4, 5, 5, 5], 3), 8);
}

#[test]
fn test_112() {
    assert_eq!(Solution::remove_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 7), 14);
}

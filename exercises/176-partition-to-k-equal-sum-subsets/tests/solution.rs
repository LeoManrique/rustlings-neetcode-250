include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 3, 4, 5], 4), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::can_partition_k_subsets(vec![4, 4, 6, 2, 3, 8, 10, 2, 10, 7], 4), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 1, 1, 1, 1, 1, 1, 1], 2), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3), false);
}

#[test]
fn test_5() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 1, 1, 1, 1, 1, 1, 1], 8), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4, 5, 6, 7, 8], 4), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::can_partition_k_subsets(vec![10, 10, 10, 10, 10, 10, 10, 10], 4), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::can_partition_k_subsets(vec![10, 10, 10, 10, 10, 10, 10, 10], 8), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::can_partition_k_subsets(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 16), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::can_partition_k_subsets(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600], 8), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::can_partition_k_subsets(vec![9, 4, 4, 9, 2, 4, 5, 4, 5, 3, 2, 9, 10, 7, 4, 7], 8), false);
}

#[test]
fn test_13() {
    assert_eq!(Solution::can_partition_k_subsets(vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 16], 4), false);
}

#[test]
fn test_14() {
    assert_eq!(Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 3, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7], 7), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::can_partition_k_subsets(vec![12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27], 6), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::can_partition_k_subsets(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 16), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::can_partition_k_subsets(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 6), false);
}

#[test]
fn test_18() {
    assert_eq!(Solution::can_partition_k_subsets(vec![3, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), false);
}

#[test]
fn test_19() {
    assert_eq!(Solution::can_partition_k_subsets(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 5), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::can_partition_k_subsets(vec![33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33], 12), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::can_partition_k_subsets(vec![16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16], 8), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9], 8), false);
}

#[test]
fn test_23() {
    assert_eq!(Solution::can_partition_k_subsets(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8], 12), false);
}

#[test]
fn test_24() {
    assert_eq!(Solution::can_partition_k_subsets(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8], 16), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 16), false);
}

#[test]
fn test_26() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 2), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::can_partition_k_subsets(vec![15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15], 12), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::can_partition_k_subsets(vec![8, 16, 32, 64, 128, 256, 512, 1024, 1, 2, 3, 4, 5, 6, 7, 8], 8), false);
}

#[test]
fn test_29() {
    assert_eq!(Solution::can_partition_k_subsets(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7], 16), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::can_partition_k_subsets(vec![6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6], 12), false);
}

#[test]
fn test_31() {
    assert_eq!(Solution::can_partition_k_subsets(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1], 8), false);
}

#[test]
fn test_32() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 1), true);
}

#[test]
fn test_33() {
    assert_eq!(Solution::can_partition_k_subsets(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160], 8), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::can_partition_k_subsets(vec![10, 10, 20, 20, 30, 30, 40, 40, 50, 50, 60, 60, 70, 70, 80, 80], 8), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6], 6), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8], 8), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::can_partition_k_subsets(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3], 8), false);
}

#[test]
fn test_38() {
    assert_eq!(Solution::can_partition_k_subsets(vec![9, 18, 27, 36, 45, 54, 63, 72, 81, 90, 99, 108, 117, 126, 135, 144], 9), false);
}

#[test]
fn test_39() {
    assert_eq!(Solution::can_partition_k_subsets(vec![10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000], 8), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::can_partition_k_subsets(vec![3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5, 3, 5], 8), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::can_partition_k_subsets(vec![16, 16, 16, 16, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4], 8), false);
}

#[test]
fn test_42() {
    assert_eq!(Solution::can_partition_k_subsets(vec![7, 2, 1, 6, 3, 4, 3, 3, 1, 2, 1, 2, 2, 2, 3, 3], 6), false);
}

#[test]
fn test_43() {
    assert_eq!(Solution::can_partition_k_subsets(vec![9, 4, 5, 9, 8, 4, 5, 10, 3, 4, 6, 7, 1, 2, 8], 3), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::can_partition_k_subsets(vec![9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3], 8), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 15), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::can_partition_k_subsets(vec![16, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8], 16), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::can_partition_k_subsets(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100], 8), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 6, 9, 12, 18, 27, 54, 81, 162, 324, 648, 1296, 2592, 5184], 6), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::can_partition_k_subsets(vec![15, 15, 15, 15, 10, 10, 10, 10, 5, 5, 5, 5, 1, 1, 1, 1], 6), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 16), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::can_partition_k_subsets(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600], 4), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 8), true);
}

#[test]
fn test_53() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32], 16), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 16), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::can_partition_k_subsets(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 15), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::can_partition_k_subsets(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7], 7), false);
}

#[test]
fn test_57() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 2), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 4), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::can_partition_k_subsets(vec![4, 1, 5, 3, 2, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 16], 8), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::can_partition_k_subsets(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 4), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 3, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 5), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::can_partition_k_subsets(vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48], 16), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::can_partition_k_subsets(vec![15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 6), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42], 8), false);
}

#[test]
fn test_65() {
    assert_eq!(Solution::can_partition_k_subsets(vec![15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15], 15), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::can_partition_k_subsets(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7], 8), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::can_partition_k_subsets(vec![15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15], 16), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::can_partition_k_subsets(vec![7, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36], 16), false);
}

#[test]
fn test_69() {
    assert_eq!(Solution::can_partition_k_subsets(vec![10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000], 8), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::can_partition_k_subsets(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 9), false);
}

#[test]
fn test_71() {
    assert_eq!(Solution::can_partition_k_subsets(vec![16, 8, 4, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10), false);
}

#[test]
fn test_72() {
    assert_eq!(Solution::can_partition_k_subsets(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5], 8), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 8), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4], 4), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::can_partition_k_subsets(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8], 8), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::can_partition_k_subsets(vec![8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128], 16), false);
}

#[test]
fn test_77() {
    assert_eq!(Solution::can_partition_k_subsets(vec![5, 5, 10, 10, 15, 15, 20, 20, 25, 25, 30, 30, 35, 35, 40, 40], 8), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::can_partition_k_subsets(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160], 16), false);
}

#[test]
fn test_79() {
    assert_eq!(Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5], 16), false);
}

#[test]
fn test_80() {
    assert_eq!(Solution::can_partition_k_subsets(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160], 4), true);
}

#[test]
fn test_81() {
    assert_eq!(Solution::can_partition_k_subsets(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600], 16), false);
}

#[test]
fn test_82() {
    assert_eq!(Solution::can_partition_k_subsets(vec![7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6], 3), false);
}

#[test]
fn test_83() {
    assert_eq!(Solution::can_partition_k_subsets(vec![12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12], 6), false);
}

#[test]
fn test_84() {
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9], 6), false);
}

#[test]
fn test_85() {
    assert_eq!(Solution::can_partition_k_subsets(vec![3, 1, 2, 2, 3, 3, 2, 2, 1, 3, 3, 3, 2, 1, 2, 2], 6), false);
}

#[test]
fn test_86() {
    assert_eq!(Solution::can_partition_k_subsets(vec![3, 5, 1, 1, 3, 2, 2, 3, 1, 1, 1, 1, 1, 1, 1, 1], 5), false);
}

#[test]
fn test_87() {
    assert_eq!(Solution::can_partition_k_subsets(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 10), false);
}

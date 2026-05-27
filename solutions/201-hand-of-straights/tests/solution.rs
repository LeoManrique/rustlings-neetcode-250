include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6], 2), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3], 3), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 4), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6], 3), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::is_n_straight_hand(vec![8, 10, 12], 3), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9], 5), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40], 10), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13], 4), false);
}

#[test]
fn test_12() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12], 3), false);
}

#[test]
fn test_13() {
    assert_eq!(Solution::is_n_straight_hand(vec![100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120], 5), false);
}

#[test]
fn test_14() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40], 10), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::is_n_straight_hand(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40], 4), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::is_n_straight_hand(vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24], 5), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::is_n_straight_hand(vec![5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8], 5), false);
}

#[test]
fn test_18() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20], 4), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31], 2), false);
}

#[test]
fn test_20() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40], 8), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 5), false);
}

#[test]
fn test_22() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32], 8), false);
}

#[test]
fn test_23() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], 5), false);
}

#[test]
fn test_24() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 3), false);
}

#[test]
fn test_25() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 2), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 4), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 5), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 4), false);
}

#[test]
fn test_29() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22], 6), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 2, 4, 5, 6, 8, 7, 10, 9, 12, 11, 14, 13, 16, 15, 18, 17, 20, 19], 2), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::is_n_straight_hand(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 4), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 2, 3, 3, 4, 5, 6, 7, 8, 9, 10], 3), true);
}

#[test]
fn test_33() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 4), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45], 5), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 2, 5, 4, 6, 7, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 23, 22, 25, 24, 27, 26, 29, 28, 31, 30], 2), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60], 6), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36], 4), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::is_n_straight_hand(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 5), false);
}

#[test]
fn test_39() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34], 10), false);
}

#[test]
fn test_40() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15], 5), false);
}

#[test]
fn test_41() {
    assert_eq!(Solution::is_n_straight_hand(vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40], 5), false);
}

#[test]
fn test_42() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 5), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::is_n_straight_hand(vec![3, 3, 3, 2, 2, 1, 4, 4, 4], 3), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12], 4), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::is_n_straight_hand(vec![10, 10, 10, 10, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 13, 13, 14, 14, 14, 14], 4), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61], 6), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::is_n_straight_hand(vec![3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10], 5), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::is_n_straight_hand(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28], 7), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45], 7), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::is_n_straight_hand(vec![5, 6, 7, 8, 8, 9, 10, 11, 12, 12, 13, 14], 4), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 2), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::is_n_straight_hand(vec![3, 3, 3, 3, 4, 4, 5, 5, 5], 3), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10], 3), false);
}

#[test]
fn test_54() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], 5), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::is_n_straight_hand(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 10), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15], 3), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::is_n_straight_hand(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14], 5), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100], 5), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40], 5), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 3), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::is_n_straight_hand(vec![10, 10, 10, 10, 10, 11, 11, 11, 11, 11], 5), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100], 10), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8], 2), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 2, 5, 4, 6, 9, 8, 7, 12, 11, 10, 15, 14, 13, 18, 17, 16, 21, 20, 19, 24, 23, 22, 27, 26, 25, 30, 29, 28, 33, 32, 31, 36, 35, 34, 39, 38, 37, 42, 41, 40], 3), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 10), true);
}

#[test]
fn test_66() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50], 5), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 3), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10], 5), false);
}

#[test]
fn test_69() {
    assert_eq!(Solution::is_n_straight_hand(vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 5), false);
}

#[test]
fn test_70() {
    assert_eq!(Solution::is_n_straight_hand(vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21], 3), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4], 3), false);
}

#[test]
fn test_72() {
    assert_eq!(Solution::is_n_straight_hand(vec![3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10], 3), false);
}

#[test]
fn test_73() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 4), false);
}

#[test]
fn test_74() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 5), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 2), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 5), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::is_n_straight_hand(vec![100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115], 8), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10], 3), false);
}

#[test]
fn test_79() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 2, 4, 5, 6, 8, 7, 10, 9, 12, 11, 14, 13, 16, 15], 4), true);
}

#[test]
fn test_80() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 3, 3, 4, 4, 5, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11], 4), false);
}

#[test]
fn test_81() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6], 2), true);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_sliding_window(vec![5, 5, 5, 5, 5, 5, 5, 5, 5], 5), vec![5, 5, 5, 5, 5]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_sliding_window(vec![4, -2], 2), vec![4]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_sliding_window(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 4), vec![9, 8, 7, 6, 5, 4]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 2), vec![2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3), vec![3, 3, 2, 5]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_sliding_window(vec![7, 2, 4], 2), vec![7, 4]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_sliding_window(vec![9, 11], 2), vec![11]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, 2, 4, 5, 6, 4, 2, 3, 1, 5, 6, 7, 8, 9, 10], 4), vec![4, 5, 6, 6, 6, 6, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_sliding_window(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10], 3), vec![100, 90, 80, 70, 60, 50, 40, 30]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3), vec![3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_sliding_window(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10], 5), vec![100, 90, 80, 70, 60, 50]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_sliding_window(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 4), vec![-1, -2, -3, -4, -5, -6, -7]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_sliding_window(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 15), vec![1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), vec![5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_sliding_window(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 25), vec![1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_sliding_window(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 2), vec![10, 10, 10, 10, 10, 10, 10, 10, 10]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7, 1, 2, 4, 5, 6], 4), vec![3, 5, 5, 6, 7, 7, 7, 7, 5, 6]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 2, 3, 4, 5, 6, 6, 7, 8, 8, 9, 10, 10, 11], 6), vec![5, 6, 6, 7, 8, 8, 9, 10, 10, 11]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_sliding_window(vec![5, 3, 5, 2, 8, 1, 9, 6, 4, 7], 5), vec![8, 8, 9, 9, 9, 9]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_sliding_window(vec![5, 3, 4, 4, 1, 5, 3, 6, 9, 5, 12, 2, 8, 7], 5), vec![5, 5, 5, 6, 9, 9, 12, 12, 12, 12]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_sliding_window(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 10), vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_sliding_window(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 3), vec![3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_sliding_window(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 7), vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_sliding_window(vec![5, 3, 4, 2, 1, 8, 7, 6, 9, 10, 11], 5), vec![5, 8, 8, 8, 9, 10, 11]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_sliding_window(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 6), vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_sliding_window(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 2), vec![10, 9, 8, 7, 6, 5, 4, 3, 2]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 4, 3, 2, 1], 2), vec![2, 3, 4, 5, 5, 4, 3, 2]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_sliding_window(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3), vec![0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_sliding_window(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 5), vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_sliding_window(vec![2, 1, 3, 5, 4, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5], 5), vec![5, 6, 7, 8, 9, 10, 10, 10, 10, 10, 5]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_sliding_window(vec![1, -1, 3, -3, 5, -5, 6, -6, 7, -7], 5), vec![5, 5, 6, 6, 7, 7]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_sliding_window(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1], 5), vec![-6, -5, -4, -3, -2, -1]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_sliding_window(vec![5, 3, 8, 1, 9, 2, 7, 4, 6, 0], 3), vec![8, 8, 9, 9, 9, 7, 7, 6]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_sliding_window(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5], 10), vec![10, 9, 8, 7, 6, 5]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3], 4), vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_sliding_window(vec![10, -5, 3, 8, 2, -9, 15, 11, 1], 4), vec![10, 8, 8, 15, 15, 15]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4], 4), vec![4, 4, 4, 4, 3, 3, 4, 4, 4, 4, 3, 3, 4]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_sliding_window(vec![9, 11, 8, 9, 10, 7, 12], 4), vec![11, 11, 10, 12]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_sliding_window(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 5), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_sliding_window(vec![9, 11, 8, 9, 10, 7, 9, 6, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 5), vec![11, 11, 10, 10, 12, 12, 12, 12, 12, 11, 10, 9, 8, 7, 6, 5]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_sliding_window(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 3), vec![5, 5, 5, 5, 5, 5, 5, 5]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_sliding_window(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 4), vec![10, 9, 8, 7, 6, 5, 4]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_sliding_window(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90], 2), vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 5), vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_sliding_window(vec![9, 11, 8, 5, 7, 10, 6], 4), vec![11, 11, 10, 10]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_sliding_window(vec![10, 14, 13, 7, 10, 6, 5, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1], 4), vec![14, 14, 13, 10, 10, 9, 9, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 20), vec![20]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, 2, 1, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1], 5), vec![5, 5, 5, 5, 5, 9, 9, 9, 9, 9, 8, 7, 6, 5]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_sliding_window(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 2), vec![3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_sliding_window(vec![-7, -8, 7, 5, 7, 1, 6, 0, 1, 2, -7, -8, -2, -2, 0, 5, 0, 1, 2, -1, -4, -2, -1, -2, 4, 1, -5, 0, -7, -1, 10, 1, -6, 5, -8, 9, 10, 3, -4, -5, -9, -9, -3, -9, -8, -7, 7, -1, -7, -6, -6, 7, 0, 1, 4, 5, -5, -5, 2, -7, 9, 7, 9, 8, -9, -2, -5, -8, 2, -8, -6, -6, -6, 0, 1, 4, -8, -5, -6, -6, 6, -8, -8, -5, -6, -6, -7, -7, 7, -8, -6, -4, -6, -5, -8, -9, -6, -9, -7, -7, -8, -6, -7, -7, 7, -8, -5, -5, -5, 7, -7, 7, -7, -8, -6, -4, -6, -5, -8, -9, -6, -9, -7, -7, -8, -6, -7, -7, 7, -8, -5, -5, -5, 7, -7, 7, -7, -8, -6, -4, -6, -5], 25), vec![7, 7, 7, 7, 7, 6, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_sliding_window(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 6), vec![3, 3, 3, 3, 3]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_sliding_window(vec![8, 5, 10, 7, 9, 4, 15, 12, 90, 13], 3), vec![10, 10, 10, 9, 15, 15, 90, 90]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 6), vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_sliding_window(vec![25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1], 8), vec![25, 23, 21, 19, 17, 15]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_sliding_window(vec![10000, -10000, 5000, -5000, 2500, -2500, 1250, -1250, 625, -625, 312, -312, 156, -156, 78, -78, 39, -39, 19, -19, 9, -9, 4, -4, 2, -2, 1, -1], 5), vec![10000, 5000, 5000, 2500, 2500, 1250, 1250, 625, 625, 312, 312, 156, 156, 78, 78, 39, 39, 19, 19, 9, 9, 4, 4, 2]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_sliding_window(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 3), vec![1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_sliding_window(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20], 4), vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_sliding_window(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 7), vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3], 3), vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_sliding_window(vec![5, 3, 8, 7, 6, 5, 4, 3, 2, 1], 3), vec![8, 8, 8, 7, 6, 5, 4, 3]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7, 8, 1, 2, 3, 4, 5], 4), vec![3, 5, 5, 6, 7, 8, 8, 8, 8, 4, 5]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5], 3), vec![3, 4, 5, 5, 5, 4, 3, 2, 3, 4, 5, 5, 5, 4, 3, 2, 2, 3, 4, 5]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3], 5), vec![4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 3]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_sliding_window(vec![3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3], 4), vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25], 7), vec![13, 15, 17, 19, 21, 23, 25]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_sliding_window(vec![-10, -20, -30, -40, -50, -60, -70, -80, -90, -100], 3), vec![-10, -20, -30, -40, -50, -60, -70, -80]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3], 2), vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_sliding_window(vec![3, 2, 2, 3, 5, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19], 6), vec![5, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_sliding_window(vec![5, 3, 8, 2, 1, 4, 7, 6, 9], 4), vec![8, 8, 8, 7, 7, 9]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_sliding_window(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 4), vec![40, 50, 60, 70, 80, 90, 100]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_sliding_window(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 5), vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, 2, 1, 2, 3, 2, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1], 4), vec![3, 3, 3, 3, 3, 3, 3, 4, 5, 6, 6, 6, 6, 5, 4]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 7), vec![7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_sliding_window(vec![5, 3, 4, 2, 1, 6, 7, 8, 9, 10], 4), vec![5, 4, 6, 7, 8, 9, 10]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_sliding_window(vec![10000, -10000, 10000, -10000, 10000, -10000, 10000, -10000, 10000], 3), vec![10000, 10000, 10000, 10000, 10000, 10000, 10000]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_sliding_window(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5], 2), vec![1, 2, 2, 3, 3, 4, 4, 5, 5]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_sliding_window(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 3), vec![-1, -2, -3, -4, -5, -6, -7, -8]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_sliding_window(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5), vec![0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_sliding_window(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_sliding_window(vec![5, 4, 3, 2, 1, 2, 3, 4, 5], 3), vec![5, 4, 3, 2, 3, 4, 5]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_sliding_window(vec![10, 9, 2, 5, 3, 7, 101, 18], 3), vec![10, 9, 5, 7, 101, 101]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_sliding_window(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 7), vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_sliding_window(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 12), vec![20, 19, 18, 17, 16, 15, 14, 13, 12]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_sliding_window(vec![5, 3, 4, 2, 6, 1, 8, 9, 7], 4), vec![5, 6, 6, 8, 9, 9]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_sliding_window(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5), vec![0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_sliding_window(vec![9, 11, 8, 9, 10, 1, 7, 3, 8, 9], 4), vec![11, 11, 10, 10, 10, 8, 9]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_sliding_window(vec![9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 5), vec![14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10), vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_sliding_window(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10), vec![1]);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_sliding_window(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 5), vec![10, 9, 8, 7, 6, 5]);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_sliding_window(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 7), vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7]);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1], 5), vec![5, 5, 5, 5, 5, 4, 3, 4, 5, 5, 5, 5, 5]);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_sliding_window(vec![5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1], 3), vec![5, 4, 3, 2, 3, 4, 5, 5, 5, 4, 3]);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_sliding_window(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 3), vec![9, 8, 7, 6, 5, 4, 3]);
}

#[test]
fn test_97() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2), vec![2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_98() {
    assert_eq!(Solution::max_sliding_window(vec![9, 11, 8, 9, 10, 7, 10, 5, 6], 4), vec![11, 11, 10, 10, 10, 10]);
}

#[test]
fn test_99() {
    assert_eq!(Solution::max_sliding_window(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10], 5), vec![100, 90, 80, 70, 60, 50]);
}

#[test]
fn test_100() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 6), vec![6, 7, 8, 9, 10]);
}

#[test]
fn test_101() {
    assert_eq!(Solution::max_sliding_window(vec![5, 2, 2, 4, 1, 6, 1, 3, 2, 5, 2, 2, 4, 6, 1, 6, 3, 3, 5, 2, 1, 1, 4, 6, 3, 2, 1, 1, 5, 6, 1, 3, 5, 6, 3, 5, 6, 3, 6, 5, 4, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), vec![6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 8, 9, 10]);
}

#[test]
fn test_102() {
    assert_eq!(Solution::max_sliding_window(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 8), vec![15, 14, 13, 12, 11, 10, 9, 8]);
}

#[test]
fn test_103() {
    assert_eq!(Solution::max_sliding_window(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10), vec![0]);
}

#[test]
fn test_104() {
    assert_eq!(Solution::max_sliding_window(vec![1, 2, 2, 3, 4, 5, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10), vec![5, 5, 5, 5, 5, 5, 5, 4, 3, 2, 1]);
}

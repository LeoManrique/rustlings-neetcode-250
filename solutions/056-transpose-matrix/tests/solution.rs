include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::transpose(vec![vec![1]]), vec![vec![1]]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::transpose(vec![vec![1, 2], vec![3, 4], vec![5, 6]]), vec![vec![1, 3, 5], vec![2, 4, 6]]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::transpose(vec![vec![-1000, 1000], vec![2000, -2000]]), vec![vec![-1000, 2000], vec![1000, -2000]]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::transpose(vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6], vec![7], vec![8], vec![9], vec![10]]), vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]]), vec![vec![1, 6, 11, 16, 21], vec![2, 7, 12, 17, 22], vec![3, 8, 13, 18, 23], vec![4, 9, 14, 19, 24], vec![5, 10, 15, 20, 25]]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::transpose(vec![vec![-1, -2, -3], vec![-4, -5, -6], vec![-7, -8, -9]]), vec![vec![-1, -4, -7], vec![-2, -5, -8], vec![-3, -6, -9]]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::transpose(vec![vec![1000000000, -1000000000], vec![1000000000, 1000000000], vec![-1000000000, -1000000000]]), vec![vec![1000000000, 1000000000, -1000000000], vec![-1000000000, 1000000000, -1000000000]]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::transpose(vec![vec![-1, 0, 1], vec![-2, 0, 2], vec![-3, 0, 3]]), vec![vec![-1, -2, -3], vec![0, 0, 0], vec![1, 2, 3]]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::transpose(vec![vec![1000000000, -1000000000], vec![1000000000, -1000000000], vec![1000000000, -1000000000]]), vec![vec![1000000000, 1000000000, 1000000000], vec![-1000000000, -1000000000, -1000000000]]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::transpose(vec![vec![1, 0, -1, 2], vec![3, 4, 5, 6], vec![7, -8, 9, -10]]), vec![vec![1, 3, 7], vec![0, 4, -8], vec![-1, 5, 9], vec![2, 6, -10]]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::transpose(vec![vec![1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2], vec![3, 3, 3, 3, 3]]), vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::transpose(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]), vec![vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::transpose(vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6], vec![7], vec![8], vec![9], vec![10]]), vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::transpose(vec![vec![1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2], vec![3, 3, 3, 3, 3], vec![4, 4, 4, 4, 4], vec![5, 5, 5, 5, 5], vec![6, 6, 6, 6, 6]]), vec![vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6]]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20]]), vec![vec![1, 6, 11, 16], vec![2, 7, 12, 17], vec![3, 8, 13, 18], vec![4, 9, 14, 19], vec![5, 10, 15, 20]]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4, 5], vec![0, 0, 0, 0, 0], vec![9, 8, 7, 6, 5], vec![4, 3, 2, 1, 0]]), vec![vec![1, 0, 9, 4], vec![2, 0, 8, 3], vec![3, 0, 7, 2], vec![4, 0, 6, 1], vec![5, 0, 5, 0]]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::transpose(vec![vec![9]]), vec![vec![9]]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::transpose(vec![vec![1, -1, 2, -2], vec![-3, 3, -4, 4], vec![5, -5, 6, -6]]), vec![vec![1, -3, 5], vec![-1, 3, -5], vec![2, -4, 6], vec![-2, 4, -6]]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::transpose(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2], vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]]), vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::transpose(vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1], vec![0, -1, -2]]), vec![vec![9, 6, 3, 0], vec![8, 5, 2, -1], vec![7, 4, 1, -2]]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]]), vec![vec![1, 5, 9, 13], vec![2, 6, 10, 14], vec![3, 7, 11, 15], vec![4, 8, 12, 16]]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]]), vec![vec![1, 5, 9, 13], vec![2, 6, 10, 14], vec![3, 7, 11, 15], vec![4, 8, 12, 16]]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::transpose(vec![vec![-1, -2, -3], vec![-4, -5, -6], vec![-7, -8, -9], vec![-10, -11, -12]]), vec![vec![-1, -4, -7, -10], vec![-2, -5, -8, -11], vec![-3, -6, -9, -12]]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16], vec![17, 18, 19, 20]]), vec![vec![1, 5, 9, 13, 17], vec![2, 6, 10, 14, 18], vec![3, 7, 11, 15, 19], vec![4, 8, 12, 16, 20]]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::transpose(vec![vec![1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2], vec![3, 3, 3, 3, 3], vec![4, 4, 4, 4, 4], vec![5, 5, 5, 5, 5]]), vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]), vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6], vec![7], vec![8], vec![9], vec![10]]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::transpose(vec![vec![1000000, 2000000], vec![3000000, 4000000], vec![5000000, 6000000], vec![7000000, 8000000]]), vec![vec![1000000, 3000000, 5000000, 7000000], vec![2000000, 4000000, 6000000, 8000000]]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::transpose(vec![vec![7, 8, 9, 10], vec![11, 12, 13, 14], vec![15, 16, 17, 18]]), vec![vec![7, 11, 15], vec![8, 12, 16], vec![9, 13, 17], vec![10, 14, 18]]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::transpose(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]), vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::transpose(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]), vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::transpose(vec![vec![1000000000, 1000000000, 1000000000], vec![1000000000, 1000000000, 1000000000]]), vec![vec![1000000000, 1000000000], vec![1000000000, 1000000000], vec![1000000000, 1000000000]]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::transpose(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]), vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12], vec![13, 14, 15]]), vec![vec![1, 4, 7, 10, 13], vec![2, 5, 8, 11, 14], vec![3, 6, 9, 12, 15]]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::transpose(vec![vec![10, 20], vec![30, 40], vec![50, 60], vec![70, 80]]), vec![vec![10, 30, 50, 70], vec![20, 40, 60, 80]]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::transpose(vec![vec![10, 20, 30, 40, 50], vec![5, 15, 25, 35, 45], vec![1, 11, 21, 31, 41]]), vec![vec![10, 5, 1], vec![20, 15, 11], vec![30, 25, 21], vec![40, 35, 31], vec![50, 45, 41]]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::transpose(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]), vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]]), vec![vec![1, 11], vec![2, 12], vec![3, 13], vec![4, 14], vec![5, 15], vec![6, 16], vec![7, 17], vec![8, 18], vec![9, 19], vec![10, 20]]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::transpose(vec![vec![-1, -2, -3], vec![-4, -5, -6], vec![-7, -8, -9], vec![-10, -11, -12]]), vec![vec![-1, -4, -7, -10], vec![-2, -5, -8, -11], vec![-3, -6, -9, -12]]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::transpose(vec![vec![0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2, 2], vec![3, 3, 3, 3, 3, 3]]), vec![vec![0, 1, 2, 3], vec![0, 1, 2, 3], vec![0, 1, 2, 3], vec![0, 1, 2, 3], vec![0, 1, 2, 3], vec![0, 1, 2, 3]]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::transpose(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]), vec![vec![1, 5, 9], vec![2, 6, 10], vec![3, 7, 11], vec![4, 8, 12]]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::transpose(vec![vec![-999999999, -999999999], vec![-999999999, -999999999], vec![999999999, 999999999]]), vec![vec![-999999999, -999999999, 999999999], vec![-999999999, -999999999, 999999999]]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::transpose(vec![vec![-10, 20, -30, 40], vec![10, -20, 30, -40], vec![-5, 5, -15, 15], vec![25, -25, 35, -35]]), vec![vec![-10, 10, -5, 25], vec![20, -20, 5, -25], vec![-30, 30, -15, 35], vec![40, -40, 15, -35]]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::transpose(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14]]), vec![vec![1, 3, 5, 7, 9, 11, 13], vec![2, 4, 6, 8, 10, 12, 14]]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12], vec![13, 14, 15]]), vec![vec![1, 4, 7, 10, 13], vec![2, 5, 8, 11, 14], vec![3, 6, 9, 12, 15]]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5], vec![4, 5, 6], vec![5, 6, 7]]), vec![vec![1, 2, 3, 4, 5], vec![2, 3, 4, 5, 6], vec![3, 4, 5, 6, 7]]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::transpose(vec![vec![-1, 2, -3, 4], vec![5, -6, 7, -8], vec![9, -10, 11, -12]]), vec![vec![-1, 5, 9], vec![2, -6, -10], vec![-3, 7, 11], vec![4, -8, -12]]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]), vec![vec![1, 5, 9], vec![2, 6, 10], vec![3, 7, 11], vec![4, 8, 12]]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::transpose(vec![vec![0, 1, 2], vec![3, 0, 4], vec![5, 6, 0]]), vec![vec![0, 3, 5], vec![1, 0, 6], vec![2, 4, 0]]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::transpose(vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]]), vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18]]), vec![vec![1, 4, 7, 10, 13, 16], vec![2, 5, 8, 11, 14, 17], vec![3, 6, 9, 12, 15, 18]]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::transpose(vec![vec![1000000000, -1000000000], vec![1000000000, -1000000000], vec![1000000000, -1000000000]]), vec![vec![1000000000, 1000000000, 1000000000], vec![-1000000000, -1000000000, -1000000000]]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30]]), vec![vec![1, 11, 21], vec![2, 12, 22], vec![3, 13, 23], vec![4, 14, 24], vec![5, 15, 25], vec![6, 16, 26], vec![7, 17, 27], vec![8, 18, 28], vec![9, 19, 29], vec![10, 20, 30]]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]), vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6], vec![7], vec![8], vec![9], vec![10]]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::transpose(vec![vec![1000000000, 0], vec![-1000000000, 1000000000]]), vec![vec![1000000000, -1000000000], vec![0, 1000000000]]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::transpose(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1], vec![0, 0, 0]]), vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 1, 0]]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::transpose(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]]), vec![vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::transpose(vec![vec![-999999999, 999999999, -999999999], vec![999999999, -999999999, 999999999], vec![-999999999, 999999999, -999999999]]), vec![vec![-999999999, 999999999, -999999999], vec![999999999, -999999999, 999999999], vec![-999999999, 999999999, -999999999]]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::transpose(vec![vec![-999999999, 999999999], vec![0, 1], vec![2, -2], vec![3, -3]]), vec![vec![-999999999, 0, 2, 3], vec![999999999, 1, -2, -3]]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::transpose(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]), vec![vec![1, 3, 5, 7], vec![2, 4, 6, 8]]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::transpose(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]), vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::transpose(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]]), vec![vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::transpose(vec![vec![-999999999, 999999999], vec![-888888888, 888888888]]), vec![vec![-999999999, -888888888], vec![999999999, 888888888]]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::transpose(vec![vec![-1, 0, 1], vec![-2, 0, 2], vec![-3, 0, 3], vec![-4, 0, 4], vec![-5, 0, 5]]), vec![vec![-1, -2, -3, -4, -5], vec![0, 0, 0, 0, 0], vec![1, 2, 3, 4, 5]]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::transpose(vec![vec![1, 0, -1], vec![2, 0, -2], vec![3, 0, -3], vec![4, 0, -4]]), vec![vec![1, 2, 3, 4], vec![0, 0, 0, 0], vec![-1, -2, -3, -4]]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::transpose(vec![vec![1000000000, -1000000000], vec![-1000000000, 1000000000], vec![1000000000, -1000000000]]), vec![vec![1000000000, -1000000000, 1000000000], vec![-1000000000, 1000000000, -1000000000]]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::transpose(vec![vec![999999999, 888888888, 777777777], vec![666666666, 555555555, 444444444], vec![333333333, 222222222, 111111111]]), vec![vec![999999999, 666666666, 333333333], vec![888888888, 555555555, 222222222], vec![777777777, 444444444, 111111111]]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::transpose(vec![vec![10, 20, 30], vec![40, 50, 60], vec![70, 80, 90], vec![100, 110, 120]]), vec![vec![10, 40, 70, 100], vec![20, 50, 80, 110], vec![30, 60, 90, 120]]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::transpose(vec![vec![1, 2, 3, 4, 5]]), vec![vec![1], vec![2], vec![3], vec![4], vec![5]]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::transpose(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]), vec![vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::transpose(vec![vec![999, 998, 997], vec![996, 995, 994], vec![993, 992, 991], vec![990, 989, 988]]), vec![vec![999, 996, 993, 990], vec![998, 995, 992, 989], vec![997, 994, 991, 988]]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::transpose(vec![vec![1], vec![2], vec![3], vec![4], vec![5]]), vec![vec![1, 2, 3, 4, 5]]);
}

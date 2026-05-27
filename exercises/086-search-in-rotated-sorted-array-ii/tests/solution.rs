include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 6), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::search(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1], 2), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::search(vec![4, 5, 6, 6, 7, 0, 1, 2, 4, 4], 4), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::search(vec![3, 1], 2), false);
}

#[test]
fn test_5() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), false);
}

#[test]
fn test_6() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::search(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 3], 1), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::search(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1], 3), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::search(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 1], 1), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::search(vec![3, 1], 1), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::search(vec![1], 1), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::search(vec![5, 1, 3], 3), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
}

#[test]
fn test_14() {
    assert_eq!(Solution::search(vec![4, 5, 6, 6, 7, 0, 1, 2, 4, 4], 7), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::search(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 1), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::search(vec![5, 1, 3], 5), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::search(vec![1, 3, 5], 1), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::search(vec![1, 0, 1, 1, 1], 0), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::search(vec![1], 0), false);
}

#[test]
fn test_22() {
    assert_eq!(Solution::search(vec![1], 2), false);
}

#[test]
fn test_23() {
    assert_eq!(Solution::search(vec![1, 3, 5], 5), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::search(vec![3, 1], 3), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::search(vec![2, 2, 2, 2, 2, 3, 4, 2], 3), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::search(vec![11, 13, 15, 17, 19, 2, 4, 6, 8, 10], 7), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::search(vec![7, 8, 9, 1, 2, 3, 4, 5, 6], 9), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::search(vec![6, 7, 1, 2, 3, 4, 5], 3), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::search(vec![5, 5, 5, 1, 2, 3, 4, 5], 6), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2, 3], 6), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 0], 18), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2, 3], 10), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::search(vec![6, 7, 8, 9, 10, 11, 12, 1, 2, 3, 4, 5], 10), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::search(vec![11, 13, 15, 17, 19, 21, 3, 5, 7, 9], 3), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::search(vec![5, 6, 7, 8, 9, 0, 1, 2, 3, 4], 9), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::search(vec![1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 0], 0), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::search(vec![15, 16, 17, 18, 19, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14], 3), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::search(vec![10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 15), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::search(vec![5, 5, 5, 1, 2, 3, 4, 5], 1), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0], 0), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 0], 19), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::search(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 2), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::search(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::search(vec![2, 2, 2, 0, 2, 2], 0), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::search(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 2, 3, 4], 1), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::search(vec![2, 2, 2, 0, 1, 2], 3), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::search(vec![2, 2, 2, 0, 1, 2], 0), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::search(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 15), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::search(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 2), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::search(vec![5, 1, 3, 4, 5, 5, 5, 5, 5], 1), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::search(vec![5, 6, 7, 8, 9, 0, 1, 2, 3, 4], 1), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 0], 20), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::search(vec![3, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 1), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2, 3], 0), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::search(vec![2, 3, 4, 5, 6, 7, 8, 9, 0, 1], 0), true);
}

#[test]
fn test_56() {
    assert_eq!(Solution::search(vec![11, 13, 15, 17, 19, 2, 4, 6, 8, 10], 5), false);
}

#[test]
fn test_57() {
    assert_eq!(Solution::search(vec![9, 9, 9, 9, 9, 1, 1, 1, 1, 1], 1), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::search(vec![1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 3), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::search(vec![2, 2, 2, 2, 2, 2, 2, 3, 1, 2, 2, 2, 2, 2, 2, 2], 3), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::search(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 0, 8, 8, 8, 8, 8, 8, 8, 8, 8], 1), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::search(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1], 9), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::search(vec![4, 4, 4, 5, 6, 7, 8, 9, 10, 0, 1, 2, 3, 4, 4, 4, 4, 4], 0), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::search(vec![5, 5, 5, 5, 1, 5, 5, 5, 5, 5, 5, 5], 1), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::search(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4], 0), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::search(vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19], 25), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::search(vec![5, 6, 7, 8, 9, 10, 1, 2, 3, 4], 10), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7], 5), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::search(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 0, 8, 8, 8, 8, 8, 8, 8, 8, 8], 0), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::search(vec![3, 4, 5, 6, 1, 2], 1), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::search(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 7), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::search(vec![10, 10, 10, 10, 10, 1, 2, 3, 4, 5], 1), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::search(vec![10, 15, 20, 25, 30, 5, 10], 25), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::search(vec![11, 13, 15, 17, 19, 2, 4, 6, 8, 10], 17), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::search(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59], 29), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::search(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1], 2), true);
}

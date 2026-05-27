include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::can_finish(5, vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::can_finish(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::can_finish(3, vec![vec![1, 0], vec![2, 1]]), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::can_finish(1, vec![]), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::can_finish(20, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![7, 4], vec![8, 5], vec![8, 6], vec![9, 7], vec![10, 7], vec![11, 8], vec![12, 8], vec![13, 9], vec![14, 10], vec![14, 11], vec![14, 12], vec![15, 13], vec![16, 13], vec![17, 14], vec![18, 15], vec![18, 16], vec![19, 17], vec![19, 18]]), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![5, 4], vec![6, 4], vec![7, 5], vec![8, 5], vec![9, 6], vec![9, 7]]), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::can_finish(9, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![5, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![8, 5]]), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 1], vec![5, 2], vec![6, 3]]), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 3], vec![5, 3], vec![6, 4], vec![6, 5]]), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![4, 2], vec![5, 2], vec![6, 3], vec![7, 3], vec![8, 4], vec![8, 5], vec![9, 6], vec![9, 7]]), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::can_finish(9, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![4, 8]]), false);
}

#[test]
fn test_14() {
    assert_eq!(Solution::can_finish(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 0]]), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::can_finish(9, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![7, 4], vec![8, 5]]), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 2], vec![5, 4], vec![6, 4], vec![7, 5], vec![7, 6]]), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::can_finish(12, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![10, 4], vec![11, 5], vec![7, 9], vec![8, 10]]), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::can_finish(20, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![10, 9], vec![11, 10], vec![12, 11], vec![13, 12], vec![14, 13], vec![15, 14], vec![16, 15], vec![17, 16], vec![18, 17], vec![19, 18], vec![0, 19]]), false);
}

#[test]
fn test_19() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![5, 4], vec![6, 5]]), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::can_finish(9, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![0, 8]]), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::can_finish(13, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 1], vec![7, 1], vec![8, 2], vec![9, 2], vec![10, 3], vec![11, 3], vec![12, 4], vec![12, 5], vec![12, 6], vec![12, 7]]), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::can_finish(15, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![7, 4], vec![8, 5], vec![9, 5], vec![10, 6], vec![11, 6], vec![12, 7], vec![12, 8], vec![13, 9], vec![13, 10], vec![14, 11], vec![14, 12]]), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![4, 6], vec![5, 7]]), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![0, 9]]), false);
}

#[test]
fn test_25() {
    assert_eq!(Solution::can_finish(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![5, 3]]), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::can_finish(10, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 7], vec![5, 8], vec![6, 8], vec![7, 9], vec![8, 9]]), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::can_finish(15, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![10, 9], vec![11, 10], vec![12, 11], vec![13, 12], vec![14, 13]]), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::can_finish(6, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![5, 4]]), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::can_finish(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 0]]), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::can_finish(6, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4]]), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::can_finish(11, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 2], vec![6, 3], vec![7, 4], vec![8, 5], vec![9, 6], vec![10, 7], vec![10, 8]]), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::can_finish(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![5, 4]]), true);
}

#[test]
fn test_33() {
    assert_eq!(Solution::can_finish(6, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![0, 5]]), false);
}

#[test]
fn test_34() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![5, 6]]), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::can_finish(15, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![7, 4], vec![8, 5], vec![8, 6], vec![9, 7], vec![10, 7], vec![11, 8], vec![12, 8], vec![13, 9], vec![14, 10], vec![14, 11], vec![14, 12]]), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::can_finish(5, vec![vec![1, 0], vec![2, 1], vec![3, 4], vec![4, 3]]), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::can_finish(20, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 5], vec![1, 6], vec![2, 7], vec![2, 8], vec![3, 9], vec![3, 10], vec![4, 11], vec![4, 12], vec![5, 13], vec![5, 14], vec![6, 15], vec![6, 16], vec![7, 17], vec![8, 18], vec![9, 19], vec![10, 19], vec![11, 19], vec![12, 19], vec![13, 19], vec![14, 19], vec![15, 19], vec![16, 19], vec![17, 19], vec![18, 19]]), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![5, 0]]), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![6, 5]]), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![4, 2], vec![4, 3], vec![5, 2], vec![5, 3], vec![6, 4], vec![6, 5], vec![7, 6]]), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 2], vec![6, 3], vec![4, 6], vec![5, 6]]), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![7, 5], vec![8, 6], vec![9, 7], vec![8, 9]]), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![0, 6]]), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::can_finish(15, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![11, 4], vec![12, 5], vec![13, 6], vec![13, 7], vec![14, 8], vec![14, 9]]), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![5, 6]]), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::can_finish(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 4], vec![3, 5], vec![4, 5], vec![5, 6]]), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::can_finish(9, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![7, 4], vec![8, 5], vec![8, 6]]), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 1], vec![3, 0], vec![4, 1], vec![5, 2], vec![6, 3], vec![7, 3]]), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::can_finish(12, vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![4, 2], vec![5, 2], vec![6, 3], vec![7, 3], vec![8, 4], vec![8, 5], vec![9, 6], vec![9, 7], vec![10, 8], vec![11, 8]]), true);
}

#[test]
fn test_50() {
    assert_eq!(Solution::can_finish(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 0], vec![0, 3], vec![1, 4], vec![2, 5], vec![3, 6], vec![4, 7], vec![5, 8], vec![6, 9]]), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![0, 7]]), false);
}

#[test]
fn test_52() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![6, 7]]), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 2], vec![6, 3], vec![7, 4], vec![8, 5], vec![9, 6], vec![9, 7]]), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::can_finish(6, vec![vec![1, 0], vec![2, 1], vec![3, 0], vec![4, 3], vec![5, 3]]), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::can_finish(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![0, 9], vec![9, 1]]), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6]]), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![1, 9]]), false);
}

#[test]
fn test_58() {
    assert_eq!(Solution::can_finish(15, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 4], vec![3, 5], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14]]), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![9, 5]]), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::can_finish(6, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4]]), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![7, 5], vec![7, 6]]), true);
}

#[test]
fn test_62() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5]]), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::can_finish(10, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4], vec![4, 5], vec![5, 3], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 6]]), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 6], vec![1, 7], vec![6, 4], vec![7, 0], vec![0, 5]]), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::can_finish(15, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![10, 4], vec![11, 5], vec![12, 5], vec![13, 6], vec![14, 7], vec![14, 8], vec![14, 9]]), true);
}

#[test]
fn test_66() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 1], vec![5, 2], vec![6, 3], vec![7, 3], vec![8, 4], vec![9, 5]]), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::can_finish(6, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 2], vec![5, 4], vec![5, 3]]), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![5, 3], vec![5, 4], vec![6, 5]]), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::can_finish(7, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![6, 3]]), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 0], vec![7, 6]]), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::can_finish(20, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![7, 4], vec![8, 5], vec![9, 5], vec![10, 6], vec![11, 6], vec![12, 7], vec![12, 8], vec![13, 9], vec![13, 10], vec![14, 11], vec![14, 12], vec![15, 13], vec![16, 13], vec![17, 14], vec![18, 14], vec![19, 15], vec![19, 16]]), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::can_finish(11, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![7, 4], vec![8, 5], vec![8, 6], vec![9, 7], vec![10, 7]]), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::can_finish(12, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![10, 5], vec![11, 6], vec![11, 7]]), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::can_finish(9, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![0, 8], vec![8, 0]]), false);
}

#[test]
fn test_75() {
    assert_eq!(Solution::can_finish(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![7, 5], vec![8, 6], vec![9, 7], vec![0, 8]]), false);
}

#[test]
fn test_76() {
    assert_eq!(Solution::can_finish(20, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![10, 4], vec![11, 5], vec![12, 5], vec![13, 6], vec![14, 7], vec![15, 7], vec![16, 8], vec![17, 8], vec![18, 9], vec![19, 9], vec![15, 12], vec![16, 13], vec![17, 14], vec![18, 15], vec![19, 16]]), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::can_finish(8, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 2], vec![6, 3], vec![7, 4], vec![7, 5]]), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::can_finish(15, vec![vec![1, 0], vec![2, 1], vec![3, 0], vec![4, 1], vec![5, 2], vec![6, 3], vec![7, 4], vec![8, 5], vec![9, 6], vec![10, 7], vec![11, 8], vec![12, 9], vec![13, 10], vec![14, 11], vec![14, 12]]), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::can_finish(7, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 0]]), false);
}

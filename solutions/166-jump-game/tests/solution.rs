include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::can_jump(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::can_jump(vec![2, 5, 0, 0]), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::can_jump(vec![1, 1, 1, 1, 1]), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::can_jump(vec![0]), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::can_jump(vec![5, 0, 0, 0, 0]), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::can_jump(vec![1, 2, 3]), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::can_jump(vec![1, 0, 1, 0]), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::can_jump(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::can_jump(vec![1, 2, 3, 4, 5]), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::can_jump(vec![5, 9, 4, 2, 1]), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::can_jump(vec![2, 0, 0]), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::can_jump(vec![9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::can_jump(vec![1, 3, 2, 1, 0, 4]), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4, 2, 3, 1, 1, 1]), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::can_jump(vec![4, 2, 0, 0, 1, 1, 4, 4, 0, 4]), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::can_jump(vec![3, 0, 8, 2, 0, 0, 1, 1, 1, 1, 1, 1]), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::can_jump(vec![2, 0, 1, 0, 2, 0, 1, 0, 2, 0, 1, 0, 2, 0, 1, 0]), false);
}

#[test]
fn test_20() {
    assert_eq!(Solution::can_jump(vec![1, 0, 3, 4, 5, 0]), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::can_jump(vec![2, 5, 0, 0, 1, 1, 1, 1, 0, 2]), false);
}

#[test]
fn test_22() {
    assert_eq!(Solution::can_jump(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::can_jump(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::can_jump(vec![10, 0, 0, 0, 0, 0, 0, 0, 0, 1]), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::can_jump(vec![3, 0, 8, 2, 0, 0, 1]), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::can_jump(vec![2, 0, 0, 0, 1, 0]), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::can_jump(vec![2, 3, 0, 1, 4]), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::can_jump(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10]), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 3, 4, 2, 1, 0, 0, 0, 0]), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::can_jump(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::can_jump(vec![2, 5, 0, 1, 3, 2, 1]), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::can_jump(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::can_jump(vec![1, 3, 2, 0, 0, 0, 0, 0, 0, 1]), false);
}

#[test]
fn test_34() {
    assert_eq!(Solution::can_jump(vec![10, 0, 0, 0, 0, 0, 0, 0, 0, 9]), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::can_jump(vec![0, 2, 3, 1, 4, 2, 2, 1, 0, 1]), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::can_jump(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::can_jump(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), false);
}

#[test]
fn test_38() {
    assert_eq!(Solution::can_jump(vec![5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0]), false);
}

#[test]
fn test_39() {
    assert_eq!(Solution::can_jump(vec![3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), false);
}

#[test]
fn test_40() {
    assert_eq!(Solution::can_jump(vec![1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::can_jump(vec![4, 3, 2, 1, 0, 0, 0, 0, 0]), false);
}

#[test]
fn test_42() {
    assert_eq!(Solution::can_jump(vec![2, 0, 0, 0, 1]), false);
}

#[test]
fn test_43() {
    assert_eq!(Solution::can_jump(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::can_jump(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21]), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::can_jump(vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10]), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::can_jump(vec![1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1]), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::can_jump(vec![9, 0, 0, 0, 0, 0, 0, 0, 0, 0]), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::can_jump(vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::can_jump(vec![1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0]), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::can_jump(vec![1, 2, 0, 2, 0, 1, 0, 0, 1, 0]), false);
}

#[test]
fn test_52() {
    assert_eq!(Solution::can_jump(vec![1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1]), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), false);
}

#[test]
fn test_54() {
    assert_eq!(Solution::can_jump(vec![10, 2, 14, 1, 0, 0, 0, 0, 0, 0]), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::can_jump(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::can_jump(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6]), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::can_jump(vec![2, 0, 2, 0, 1, 1]), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::can_jump(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), false);
}

#[test]
fn test_59() {
    assert_eq!(Solution::can_jump(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), false);
}

#[test]
fn test_60() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4, 2, 1, 0, 1, 3, 1, 0, 1]), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::can_jump(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 0, 0, 1]), true);
}

#[test]
fn test_62() {
    assert_eq!(Solution::can_jump(vec![0, 2, 3]), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::can_jump(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0]), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::can_jump(vec![1, 1, 2, 2, 1, 1, 1, 1, 1, 0, 0]), false);
}

#[test]
fn test_65() {
    assert_eq!(Solution::can_jump(vec![4, 2, 0, 0, 1, 1, 4, 0, 0, 0, 1]), true);
}

#[test]
fn test_66() {
    assert_eq!(Solution::can_jump(vec![2, 5, 0, 0, 0, 0, 4, 1, 1, 1, 1]), true);
}

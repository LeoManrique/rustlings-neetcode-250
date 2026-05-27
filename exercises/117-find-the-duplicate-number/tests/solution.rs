include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 3, 3, 3]), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 2, 3, 4]), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_duplicate(vec![7, 9, 7, 4, 6, 2, 3, 8, 5, 1]), 7);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 4, 4, 5]), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_duplicate(vec![3, 3, 3, 3, 3]), 3);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_duplicate(vec![5, 1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 6]), 6);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 4]), 4);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_duplicate(vec![1, 1, 2, 3, 4, 5]), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_duplicate(vec![1, 1, 2, 3, 4]), 1);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2, 1]), 2);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2, 2]), 2);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_duplicate(vec![2, 1, 2, 3, 4, 5]), 2);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 5]), 5);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_duplicate(vec![1, 4, 4, 2, 3]), 4);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_duplicate(vec![2, 5, 9, 6, 9, 3, 8, 9, 7, 1]), 9);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_duplicate(vec![5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 2, 4, 5, 5, 6, 7, 8, 9]), 5);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 7]), 7);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 2, 5, 4, 6, 7, 8, 9, 10, 3]), 3);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10]), 10);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_duplicate(vec![2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 2]), 2);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2]), 2);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5]), 5);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_duplicate(vec![5, 1, 4, 2, 5, 3, 5, 5, 5, 5]), 5);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_duplicate(vec![5, 4, 3, 2, 1, 6, 7, 8, 9, 2, 2]), 2);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_duplicate(vec![2, 1, 4, 3, 5, 6, 7, 8, 9, 10, 2]), 2);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_duplicate(vec![8, 7, 3, 6, 4, 2, 5, 7, 1, 9]), 7);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_duplicate(vec![1, 5, 2, 3, 5, 4, 5, 5, 5, 5, 5]), 5);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_duplicate(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 1]), 1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_duplicate(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1]), 1);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_duplicate(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1]), 1);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 2);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_duplicate(vec![8, 9, 7, 6, 5, 4, 3, 2, 1, 8, 8]), 8);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_duplicate(vec![3, 2, 1, 4, 5, 6, 7, 8, 9, 10, 3]), 3);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 4]), 4);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]), 2);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_duplicate(vec![50000, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 50000]), 12);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_duplicate(vec![1, 5, 3, 4, 2, 5]), 5);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_duplicate(vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_duplicate(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 5);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 10]), 9);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1]), 1);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 8]), 8);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_duplicate(vec![100000, 99999, 99998, 99997, 99996, 99995, 99994, 99993, 99992, 99991, 99991]), 11);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_duplicate(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1]), 1);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9]), 9);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_duplicate(vec![10, 2, 3, 4, 5, 6, 7, 8, 9, 1, 1]), 1);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_duplicate(vec![5, 4, 3, 2, 1, 2, 6, 7, 8, 9, 10]), 2);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_duplicate(vec![7, 6, 5, 4, 3, 2, 1, 1]), 1);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_duplicate(vec![1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_duplicate(vec![8, 7, 6, 5, 4, 3, 2, 1, 1, 9, 10]), 1);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_duplicate(vec![100000, 1, 2, 3, 4, 5, 6, 7, 8, 9, 100000]), 11);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 6]), 6);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100]), 11);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_duplicate(vec![10, 8, 7, 4, 6, 2, 9, 5, 3, 10, 1]), 10);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 2, 5, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 5]), 5);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_duplicate(vec![5, 2, 4, 3, 1, 5, 5, 5, 5, 5, 5]), 5);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_duplicate(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 1]), 1);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_duplicate(vec![50000, 49999, 1, 2, 3, 4, 5, 6, 7, 8, 50000]), 11);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_duplicate(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]), 7);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 10]), 1);
}

#[test]
fn test_66() {
    assert_eq!(Solution::find_duplicate(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 5, 5]), 5);
}

#[test]
fn test_67() {
    assert_eq!(Solution::find_duplicate(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2]), 2);
}

#[test]
fn test_68() {
    assert_eq!(Solution::find_duplicate(vec![7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 7]), 11);
}

#[test]
fn test_69() {
    assert_eq!(Solution::find_duplicate(vec![5, 4, 3, 2, 1, 1, 6, 7, 8, 9, 10]), 1);
}

#[test]
fn test_70() {
    assert_eq!(Solution::find_duplicate(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 9]), 9);
}

#[test]
fn test_71() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 2, 4, 5, 6, 7, 8, 9, 10, 8, 8, 8, 8, 8]), 8);
}

#[test]
fn test_72() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 3]), 3);
}

#[test]
fn test_73() {
    assert_eq!(Solution::find_duplicate(vec![1, 100000, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 100000]), 21);
}

#[test]
fn test_74() {
    assert_eq!(Solution::find_duplicate(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 10]), 1);
}

#[test]
fn test_75() {
    assert_eq!(Solution::find_duplicate(vec![5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_76() {
    assert_eq!(Solution::find_duplicate(vec![99999, 99998, 99997, 99996, 99995, 99994, 99993, 99992, 99991, 99990, 99991]), 11);
}

#[test]
fn test_77() {
    assert_eq!(Solution::find_duplicate(vec![2, 3, 1, 4, 5, 6, 7, 8, 9, 10, 2]), 2);
}

#[test]
fn test_78() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 1]), 11);
}

#[test]
fn test_79() {
    assert_eq!(Solution::find_duplicate(vec![5, 4, 3, 2, 1, 6, 5, 7, 8, 9, 10]), 5);
}

#[test]
fn test_80() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 5, 4, 2, 5, 6]), 5);
}

#[test]
fn test_81() {
    assert_eq!(Solution::find_duplicate(vec![8, 9, 7, 6, 5, 4, 3, 2, 1, 8]), 8);
}

#[test]
fn test_82() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 3, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 3);
}

#[test]
fn test_83() {
    assert_eq!(Solution::find_duplicate(vec![10, 8, 9, 6, 3, 1, 2, 7, 4, 10, 5]), 10);
}

#[test]
fn test_84() {
    assert_eq!(Solution::find_duplicate(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 2]), 11);
}

#[test]
fn test_85() {
    assert_eq!(Solution::find_duplicate(vec![1, 5, 4, 2, 3, 6, 7, 8, 9, 9, 10]), 9);
}

#[test]
fn test_86() {
    assert_eq!(Solution::find_duplicate(vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), 9);
}

#[test]
fn test_87() {
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 5, 5, 5, 5, 5, 5]), 5);
}

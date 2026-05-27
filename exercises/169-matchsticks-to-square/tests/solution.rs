include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::makesquare(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3]), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::makesquare(vec![10, 10, 10, 10]), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::makesquare(vec![5, 5, 5, 5]), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::makesquare(vec![1, 3, 3, 3, 4]), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 2, 2, 2, 2]), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 2]), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5]), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 12]), false);
}

#[test]
fn test_11() {
    assert_eq!(Solution::makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4]), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 4]), false);
}

#[test]
fn test_13() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5]), false);
}

#[test]
fn test_14() {
    assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8]), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::makesquare(vec![2, 2, 2, 2, 2, 2, 2, 2]), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::makesquare(vec![7, 7, 7, 7, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), false);
}

#[test]
fn test_18() {
    assert_eq!(Solution::makesquare(vec![10, 20, 30, 40, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), false);
}

#[test]
fn test_19() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1]), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::makesquare(vec![1, 3, 3, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::makesquare(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), false);
}

#[test]
fn test_22() {
    assert_eq!(Solution::makesquare(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8]), false);
}

#[test]
fn test_23() {
    assert_eq!(Solution::makesquare(vec![7, 8, 10, 10, 10, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12]), false);
}

#[test]
fn test_24() {
    assert_eq!(Solution::makesquare(vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]), false);
}

#[test]
fn test_25() {
    assert_eq!(Solution::makesquare(vec![20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 80]), false);
}

#[test]
fn test_26() {
    assert_eq!(Solution::makesquare(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16]), false);
}

#[test]
fn test_29() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 6]), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 5]), false);
}

#[test]
fn test_31() {
    assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), false);
}

#[test]
fn test_32() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 30]), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::makesquare(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]), false);
}

#[test]
fn test_34() {
    assert_eq!(Solution::makesquare(vec![100000000, 100000000, 100000000, 100000000, 25000000, 25000000, 25000000, 25000000]), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::makesquare(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::makesquare(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8]), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::makesquare(vec![15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 1]), false);
}

#[test]
fn test_38() {
    assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8]), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::makesquare(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 11, 12, 13, 14, 15]), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::makesquare(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), false);
}

#[test]
fn test_41() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4]), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5]), false);
}

#[test]
fn test_43() {
    assert_eq!(Solution::makesquare(vec![2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9]), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::makesquare(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1]), false);
}

#[test]
fn test_45() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3]), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::makesquare(vec![10, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::makesquare(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 8]), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 100]), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::makesquare(vec![10, 10, 10, 10, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2]), false);
}

#[test]
fn test_52() {
    assert_eq!(Solution::makesquare(vec![100, 100, 100, 100, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25]), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::makesquare(vec![100000000, 100000000, 100000000, 100000000, 25000000, 25000000, 25000000, 25000000, 25000000, 25000000, 25000000, 25000000, 25000000, 25000000, 25000000]), false);
}

#[test]
fn test_54() {
    assert_eq!(Solution::makesquare(vec![100000000, 100000000, 100000000, 100000000, 1]), false);
}

#[test]
fn test_55() {
    assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6]), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::makesquare(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]), false);
}

#[test]
fn test_57() {
    assert_eq!(Solution::makesquare(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 5]), false);
}

#[test]
fn test_58() {
    assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), false);
}

#[test]
fn test_59() {
    assert_eq!(Solution::makesquare(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1]), false);
}

#[test]
fn test_60() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3]), true);
}

#[test]
fn test_62() {
    assert_eq!(Solution::makesquare(vec![100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000, 100000000]), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::makesquare(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::makesquare(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::makesquare(vec![6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6]), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4]), false);
}

#[test]
fn test_67() {
    assert_eq!(Solution::makesquare(vec![10, 20, 30, 40, 10, 20, 30, 40]), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 28]), false);
}

#[test]
fn test_69() {
    assert_eq!(Solution::makesquare(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 1]), false);
}

#[test]
fn test_70() {
    assert_eq!(Solution::makesquare(vec![2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5]), false);
}

#[test]
fn test_71() {
    assert_eq!(Solution::makesquare(vec![15, 15, 15, 15, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), false);
}

#[test]
fn test_72() {
    assert_eq!(Solution::makesquare(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 15]), false);
}

#[test]
fn test_73() {
    assert_eq!(Solution::makesquare(vec![10, 20, 30, 40, 50, 15, 5, 25, 35]), false);
}

#[test]
fn test_74() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::makesquare(vec![8, 8, 8, 8, 8, 8, 8, 8, 1, 1, 1, 1, 1, 1, 1]), false);
}

#[test]
fn test_77() {
    assert_eq!(Solution::makesquare(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 32]), false);
}

#[test]
fn test_78() {
    assert_eq!(Solution::makesquare(vec![15, 15, 15, 15, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), false);
}

#[test]
fn test_79() {
    assert_eq!(Solution::makesquare(vec![100000000, 100000000, 100000000, 100000000]), true);
}

#[test]
fn test_80() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 1]), false);
}

#[test]
fn test_81() {
    assert_eq!(Solution::makesquare(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 20]), false);
}

#[test]
fn test_82() {
    assert_eq!(Solution::makesquare(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 1]), false);
}

#[test]
fn test_83() {
    assert_eq!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 1]), false);
}

#[test]
fn test_84() {
    assert_eq!(Solution::makesquare(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 20]), false);
}

#[test]
fn test_85() {
    assert_eq!(Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]), true);
}

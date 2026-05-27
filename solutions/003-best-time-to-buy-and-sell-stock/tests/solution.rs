include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 2]), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_profit(vec![1]), 0);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_profit(vec![2, 1, 2, 1, 0, 1, 2]), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_profit(vec![2, 1]), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 4);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_profit(vec![1, 2]), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_profit(vec![8, 9, 7, 9, 8, 7, 9, 10, 7, 9, 8, 10, 11, 10, 12]), 5);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 4, 3, 2, 1, 5, 4, 3, 2, 1, 6]), 5);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 1, 2, 3, 8, 2, 10]), 9);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_profit(vec![8, 6, 7, 3, 3, 5, 1, 0, 6, 5]), 6);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_profit(vec![1, 5, 3, 8, 12, 10, 9, 15, 18, 5, 7]), 17);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3, 1, 5, 3, 7, 10, 4, 8, 12, 6, 14, 2, 18, 9, 20, 1, 22, 10, 24, 5, 26, 11, 28, 12, 30, 13, 32, 14, 34, 15, 36, 16, 38, 17, 40]), 40);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_profit(vec![8, 3, 6, 2, 8, 8, 8, 4, 2, 0, 9, 5, 7, 6, 2]), 9);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_profit(vec![4, 1, 2, 3, 5, 6, 1, 2, 3, 1, 5, 6, 7, 8, 1, 2, 3, 1]), 7);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_profit(vec![897, 456, 680, 509, 535, 695, 890, 456, 509, 535, 695, 890, 456, 509, 535, 695]), 434);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_profit(vec![100, 180, 260, 40, 310, 535, 695, 10, 1]), 655);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_profit(vec![100, 90, 80, 50, 20, 10]), 0);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6, 4, 5, 6, 7, 5, 6, 7, 8]), 7);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3]), 4);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_profit(vec![100, 180, 260, 40, 310, 535, 695]), 655);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_profit(vec![1, 9, 2, 8, 3, 7, 4, 6, 5, 10]), 9);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_profit(vec![1, 9, 6, 9, 1, 7, 1, 1, 5, 9, 9, 9, 8, 9, 8, 9, 10, 9, 8, 9, 10]), 9);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 8);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 0);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 6, 8, 9, 3, 4, 5, 1]), 5);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 10, 2, 8, 15]), 14);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_profit(vec![5, 2, 3, 4, 1, 6, 8, 7, 8, 9]), 8);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_profit(vec![100, 180, 260, 310, 40, 535, 695]), 655);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_profit(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 9);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_profit(vec![10, 10, 10, 5, 5, 5, 10, 10, 15, 15]), 10);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_profit(vec![8, 5, 12, 9, 19, 1, 7, 17, 3, 18]), 17);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_profit(vec![30, 15, 50, 10, 60, 35, 100, 40, 90, 50, 120, 60, 130, 70, 140, 80, 150, 90]), 140);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_profit(vec![50, 20, 30, 10, 50, 20, 30, 10, 50, 20, 30, 10, 50, 20, 30, 10, 50, 20, 30, 10, 50]), 40);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_profit(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_profit(vec![8, 4, 6, 2, 3, 10, 14, 11, 13]), 12);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_profit(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 0);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_profit(vec![1, 5, 3, 7, 10, 4, 8, 12, 6, 14, 2, 18, 9, 20, 1, 22, 10, 24, 5, 26, 11, 28, 12, 30, 13, 32, 14, 34, 15, 36]), 35);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_profit(vec![5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 0);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 8, 9, 2, 1, 5, 6, 3, 10]), 9);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_profit(vec![10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20]), 10);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 5, 7, 11, 8, 12, 14, 15]), 14);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3]), 2);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_profit(vec![10, 1, 1, 6, 9, 1, 2, 1, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_profit(vec![1, 5, 3, 8, 12, 7, 9, 2, 3, 15, 10]), 14);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_profit(vec![8, 6, 7, 8, 4, 9, 1, 9, 4, 5]), 8);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_profit(vec![1, 2, 1, 3, 1, 2, 3, 1, 2, 3, 4]), 3);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_profit(vec![5, 2, 3, 0, 3, 1, 6, 2, 8, 3, 4]), 8);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 10, 2, 3, 4, 5, 6, 7, 8, 9, 1]), 9);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_profit(vec![1, 5, 3, 6, 7, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_profit(vec![10, 1, 1, 1, 10, 1, 1, 1, 10, 1, 1, 1, 10, 1, 1, 1]), 9);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_profit(vec![1, 9, 1, 9, 1, 9, 1, 9, 1, 9]), 8);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_profit(vec![100, 200, 300, 100, 150, 200, 250, 100, 150, 200, 250, 300, 100, 150, 200, 250]), 200);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9, 3, 5, 2, 10, 1, 3, 7, 2, 4, 9, 1, 6, 3, 5]), 9);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_profit(vec![1, 9, 18, 2, 7, 21, 12, 17, 6, 19]), 20);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_profit(vec![1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1]), 2);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 9);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_profit(vec![5, 11, 3, 50, 60, 90, 70, 80, 65, 30, 55, 95]), 92);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 3, 3, 5, 10, 0, 6, 2, 9, 10, 4, 7, 1]), 10);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_profit(vec![2, 2, 5, 11, 1, 3, 4, 11, 1, 2, 11, 3, 4]), 10);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_profit(vec![1, 7, 2, 9, 4, 6, 3, 8, 5, 10]), 9);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_profit(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 1);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_profit(vec![10, 22, 5, 75, 65, 80]), 75);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0, 9]), 9);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_profit(vec![1, 9, 6, 9, 2, 8, 7, 1, 9, 4, 9, 2, 3, 8, 1, 6, 5, 8, 1, 1, 5]), 8);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_profit(vec![5, 3, 6, 7, 2, 8, 1, 4, 9, 10, 1, 11]), 10);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_profit(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 0);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 4, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 8);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_profit(vec![1, 9, 6, 9, 1, 7, 1, 1, 5, 9, 9, 9, 8]), 8);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_profit(vec![1, 9, 6, 9, 1, 7, 1, 1, 5, 9, 9, 9]), 8);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_profit(vec![5, 2, 6, 1, 9, 12, 9, 12, 12, 1, 5, 3, 3, 5, 2, 8, 10, 9, 1, 2]), 11);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9, 3, 5, 10, 6, 2, 3, 1, 5, 20, 3, 1, 2, 3, 10]), 19);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_profit(vec![10, 7, 5, 8, 11, 9]), 6);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_profit(vec![1, 5, 3, 7, 10, 4, 8, 12, 6, 14, 2, 18, 9, 20, 1, 22, 10, 24, 5, 26, 11, 28, 12, 30, 13, 32]), 31);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 4);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0, 1, 2, 4, 2, 5, 7, 2, 4, 9, 0, 1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 9);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_profit(vec![1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100]), 99);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]), 3);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_profit(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_profit(vec![5, 9, 1, 6, 2, 8, 3, 7, 4, 10]), 9);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_profit(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 0);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_profit(vec![10, 1, 1, 10, 1, 1, 10, 1, 1, 10, 1, 1, 10, 1, 1, 10, 1, 1, 10, 1, 1, 10, 1, 1]), 9);
}

#[test]
fn test_97() {
    assert_eq!(Solution::max_profit(vec![310, 310, 275, 275, 260, 260, 260, 230, 230, 230]), 0);
}

#[test]
fn test_98() {
    assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 8);
}

#[test]
fn test_99() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 4, 7, 5, 8, 11, 9, 13]), 12);
}

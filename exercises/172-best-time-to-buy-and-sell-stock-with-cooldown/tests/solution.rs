include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::max_profit(vec![1, 2, 4]), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2, 3, 1, 4, 2, 5]), 7);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_profit(vec![2, 1, 4, 5, 2, 9, 7]), 10);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_profit(vec![1]), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3]), 7);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_profit(vec![3, 2, 1]), 0);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_profit(vec![8, 9, 7, 9, 10, 1, 2, 3, 4, 1, 5]), 8);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 3, 3, 2, 3, 5, 8, 3, 8, 2, 6]), 10);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9]), 8);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_profit(vec![6, 1, 3, 2, 4, 7]), 6);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_profit(vec![5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3]), 9);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_profit(vec![8, 9, 2, 8, 4, 9]), 7);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_profit(vec![5, 2, 3, 4, 1, 6, 7, 8, 9, 1]), 9);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3, 1, 4, 5, 4, 7, 8, 9, 3, 2, 1, 0, 12, 14, 16, 18, 20, 19, 18, 17, 16, 15, 14]), 33);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5]), 12);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 3, 3, 2, 3, 5, 0, 0, 3, 1, 4, 0, 2, 1, 2, 0, 1, 3]), 11);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1]), 12);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2, 3, 4, 5, 0, 6, 7, 8, 0, 9]), 21);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 9, 2, 5, 8]), 12);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_profit(vec![100, 50, 200, 10, 150, 80, 120, 20, 180, 100, 140, 50, 250, 200, 300, 150, 350, 300, 400, 250, 450, 400, 500, 350, 550, 500, 600, 450, 650]), 1310);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_profit(vec![1, 5, 3, 8, 12, 6, 2, 9, 15, 10, 13, 7, 16, 11]), 33);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_profit(vec![5, 1, 1, 2, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 8);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]), 8);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 19);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 19);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_profit(vec![10, 1, 1, 6, 9, 10, 7, 12, 6, 7, 5, 18, 9]), 26);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_profit(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 27);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_profit(vec![10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]), 45);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 8, 7, 6, 5, 4, 3, 2, 1, 9]), 15);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2, 5, 0, 6, 1, 8]), 13);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_profit(vec![10, 20, 10, 10, 10, 20, 30, 10, 40, 50, 10, 60, 70, 10, 80, 90, 10]), 180);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 3, 3, 2, 3, 5, 7, 8, 9, 10, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 9);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_profit(vec![1, 4, 2, 7, 4, 8, 1, 10, 12, 20, 2]), 25);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11]), 10);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_profit(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 3);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_profit(vec![1, 2, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 5);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_profit(vec![1, 5, 1, 3, 1, 7, 1, 9, 1, 11, 1, 13, 1, 15, 1, 17, 1, 19, 1, 21]), 60);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 2, 8]), 11);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_profit(vec![1, 1, 0, 0, 1, 1, 0, 0, 1, 1]), 2);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_profit(vec![10, 5, 20, 1, 15, 8, 12, 2, 18, 10, 14, 5, 25, 20, 30, 15, 35, 30, 40, 25, 45, 40, 50, 35, 55, 50, 60, 45, 65]), 131);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 1, 2, 1, 2, 3, 4, 1, 2]), 6);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 3, 3, 2, 3, 5, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 14);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2, 3, 4, 5, 0, 6]), 11);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_profit(vec![6, 1, 3, 2, 4, 7]), 6);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 8, 2, 10, 3, 5]), 13);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 3, 2, 3, 0, 4, 2, 0, 1, 2, 3, 4, 5, 0, 0, 0, 0, 0]), 13);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5]), 16);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_profit(vec![1, 2, 5, 0, 1, 8, 0, 3, 4, 0, 5, 2, 3, 8, 9, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 36);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_profit(vec![1, 4, 2, 7, 2, 8, 3, 9, 1, 10, 1, 11, 1, 12, 1, 13]), 34);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_profit(vec![1, 5, 2, 3, 7, 1, 4, 2, 8, 3, 5]), 14);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_profit(vec![1, 2, 5, 0, 2, 1, 2, 0, 2, 1, 0, 2, 3, 4, 5, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 21);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_profit(vec![100, 180, 260, 310, 40, 535, 695, 200, 400, 310, 350, 410, 300, 500, 600, 500, 400, 600, 700, 800, 900, 1000, 500, 1100, 1200, 1300]), 2455);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_profit(vec![5, 2, 5, 4, 6, 9, 1, 3, 2, 8, 1, 7, 6, 3, 8, 9, 10, 5]), 20);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 0, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 0, 6, 7, 8, 9]), 24);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 5, 2]), 5);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]), 9);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_profit(vec![3, 1, 4, 8, 7, 2, 5, 10, 9, 6, 11]), 20);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 1, 5, 3, 6, 4, 1, 5, 3, 6, 4, 1, 5, 3, 6, 4]), 20);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_profit(vec![1, 4, 2, 10, 7, 5, 15, 13, 9, 11]), 21);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_profit(vec![10, 20, 10, 5, 2, 3, 10, 15, 20, 25, 30, 35, 40, 45, 50]), 58);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_profit(vec![10, 14, 14, 14, 13, 10, 9, 8, 7, 6, 5, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 19);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_profit(vec![100, 90, 80, 50, 25, 20, 10, 5, 3, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2, 1, 3, 4, 1, 5, 0, 6, 3]), 11);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 6, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 4);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_profit(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 4);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_profit(vec![2, 3, 5, 0, 1, 8, 0, 3, 4, 0, 5, 2, 3, 8, 9, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6, 7]), 30);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 11);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2, 5, 0, 4, 6, 0, 3, 8, 0, 10]), 20);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_profit(vec![1, 4, 2, 7, 2, 4, 1, 6, 1, 5, 2, 7, 2, 4, 1, 6, 1, 5, 2, 7, 2, 4, 1, 6, 1, 5, 2, 7, 2, 4, 1]), 36);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_profit(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 0);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 12);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_profit(vec![5, 2, 5, 0, 1, 8, 0, 3, 4, 0, 5, 2, 3, 8, 9, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 32);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 6, 8, 7, 3, 5, 1, 9]), 12);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 9);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]), 11);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]), 12);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5]), 14);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_profit(vec![6, 1, 3, 2, 8, 0, 4, 2, 5, 10, 1, 3]), 15);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_profit(vec![2, 1, 4, 5, 2, 9, 7, 10, 3, 12, 8, 15]), 22);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_profit(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 5);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 3, 7, 8, 2, 5, 10, 4]), 17);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 8, 4, 9, 0, 5, 6, 7, 8, 9, 10, 11, 12]), 19);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_profit(vec![10, 1, 2, 8, 4, 9, 5, 6, 3, 10]), 15);
}

#[test]
fn test_97() {
    assert_eq!(Solution::max_profit(vec![10, 5, 0, 2, 1, 7, 5, 3, 2, 8]), 13);
}

#[test]
fn test_98() {
    assert_eq!(Solution::max_profit(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 5);
}

#[test]
fn test_99() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_100() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 18);
}

#[test]
fn test_101() {
    assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3, 8, 1, 2, 4, 5]), 15);
}

#[test]
fn test_102() {
    assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3, 4, 7, 8, 0, 5, 2, 3, 8, 9, 1, 2, 3, 4, 5]), 25);
}

#[test]
fn test_103() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2, 5, 6, 1, 2, 3, 4, 5, 6]), 11);
}

#[test]
fn test_104() {
    assert_eq!(Solution::max_profit(vec![10, 1, 1, 6, 9, 1, 6, 1, 4, 7]), 14);
}

#[test]
fn test_105() {
    assert_eq!(Solution::max_profit(vec![10, 1, 1, 6, 9, 10, 3, 1, 3, 9, 5, 5, 5, 4, 5, 5, 6, 6, 6, 8]), 21);
}

#[test]
fn test_106() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_107() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2, 1, 2, 0, 2, 1, 0, 2, 3, 4, 5, 0, 0, 1, 2, 3, 4]), 14);
}

#[test]
fn test_108() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 3, 3, 2, 3, 5, 7, 8, 5, 3, 5, 7, 8]), 11);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::candy(vec![50, 40, 30, 20, 10]), 15);
}

#[test]
fn test_2() {
    assert_eq!(Solution::candy(vec![1]), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::candy(vec![1, 3, 4, 5, 2]), 11);
}

#[test]
fn test_4() {
    assert_eq!(Solution::candy(vec![1, 1, 1, 1]), 4);
}

#[test]
fn test_5() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 1]), 24);
}

#[test]
fn test_6() {
    assert_eq!(Solution::candy(vec![5, 4, 3, 2, 1]), 15);
}

#[test]
fn test_7() {
    assert_eq!(Solution::candy(vec![10, 20, 30, 40, 50, 45, 35, 25, 15, 5]), 31);
}

#[test]
fn test_8() {
    assert_eq!(Solution::candy(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 66);
}

#[test]
fn test_9() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 2, 3, 1]), 8);
}

#[test]
fn test_10() {
    assert_eq!(Solution::candy(vec![1, 3, 4, 3, 2]), 9);
}

#[test]
fn test_11() {
    assert_eq!(Solution::candy(vec![1, 6, 10, 8, 7, 3, 2]), 18);
}

#[test]
fn test_12() {
    assert_eq!(Solution::candy(vec![10, 20, 30, 40, 50]), 15);
}

#[test]
fn test_13() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 1, 2, 3, 4, 3, 2, 1]), 22);
}

#[test]
fn test_14() {
    assert_eq!(Solution::candy(vec![1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_15() {
    assert_eq!(Solution::candy(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 55);
}

#[test]
fn test_16() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 4, 5, 6]), 13);
}

#[test]
fn test_17() {
    assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
}

#[test]
fn test_18() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
}

#[test]
fn test_19() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 2, 1]), 7);
}

#[test]
fn test_20() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 100);
}

#[test]
fn test_21() {
    assert_eq!(Solution::candy(vec![10, 20, 15, 10, 15, 20, 10]), 13);
}

#[test]
fn test_22() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_23() {
    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
}

#[test]
fn test_24() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 0]), 31);
}

#[test]
fn test_25() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 3, 3, 4, 5, 5, 6]), 15);
}

#[test]
fn test_26() {
    assert_eq!(Solution::candy(vec![1, 6, 10, 8, 7, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 123);
}

#[test]
fn test_27() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 2, 1, 2, 3, 2, 1]), 15);
}

#[test]
fn test_28() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6]), 16);
}

#[test]
fn test_29() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 66);
}

#[test]
fn test_30() {
    assert_eq!(Solution::candy(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 109);
}

#[test]
fn test_31() {
    assert_eq!(Solution::candy(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 19);
}

#[test]
fn test_32() {
    assert_eq!(Solution::candy(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 55);
}

#[test]
fn test_33() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 3, 2, 2, 3, 2, 2, 1, 2, 2, 3, 2, 2, 1, 2, 2]), 26);
}

#[test]
fn test_34() {
    assert_eq!(Solution::candy(vec![10, 10, 10, 10, 10]), 5);
}

#[test]
fn test_35() {
    assert_eq!(Solution::candy(vec![1, 2, 1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7, 9, 8, 10, 9, 8]), 31);
}

#[test]
fn test_36() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1]), 48);
}

#[test]
fn test_37() {
    assert_eq!(Solution::candy(vec![1, 1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]), 84);
}

#[test]
fn test_38() {
    assert_eq!(Solution::candy(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 15);
}

#[test]
fn test_39() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 259);
}

#[test]
fn test_40() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 2, 1]), 7);
}

#[test]
fn test_41() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 2, 3, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 47);
}

#[test]
fn test_42() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 33);
}

#[test]
fn test_43() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7, 9, 8, 10, 9, 11, 10]), 28);
}

#[test]
fn test_44() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 2, 3, 1, 2, 3, 4, 3, 2, 1]), 23);
}

#[test]
fn test_45() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 124);
}

#[test]
fn test_46() {
    assert_eq!(Solution::candy(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10);
}

#[test]
fn test_47() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5]), 45);
}

#[test]
fn test_48() {
    assert_eq!(Solution::candy(vec![5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5]), 60);
}

#[test]
fn test_49() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 1, 2, 1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 57);
}

#[test]
fn test_50() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 100);
}

#[test]
fn test_51() {
    assert_eq!(Solution::candy(vec![3, 3, 3, 3, 3, 2, 1, 1, 2, 3, 3, 3, 3, 3, 4, 5, 4, 3, 2, 1]), 37);
}

#[test]
fn test_52() {
    assert_eq!(Solution::candy(vec![10, 20, 10, 50, 20, 30, 10]), 10);
}

#[test]
fn test_53() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 154);
}

#[test]
fn test_54() {
    assert_eq!(Solution::candy(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 210);
}

#[test]
fn test_55() {
    assert_eq!(Solution::candy(vec![5, 3, 8, 6, 7, 2, 4, 1]), 12);
}

#[test]
fn test_56() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4, 3, 2, 3, 4]), 66);
}

#[test]
fn test_57() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 3, 3, 3, 2, 1]), 13);
}

#[test]
fn test_58() {
    assert_eq!(Solution::candy(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), 20);
}

#[test]
fn test_59() {
    assert_eq!(Solution::candy(vec![20, 19, 20, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 193);
}

#[test]
fn test_60() {
    assert_eq!(Solution::candy(vec![10, 9, 8, 7, 6, 5, 6, 7, 8, 9, 10]), 41);
}

#[test]
fn test_61() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 110);
}

#[test]
fn test_62() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 4, 3, 4, 5, 4, 3, 4, 5, 4, 3, 4, 5, 4, 3, 4, 5, 4, 3, 4, 5, 4, 3, 4, 5, 4, 3, 4, 5, 4, 3, 4, 5, 4, 3, 4, 5, 4, 3]), 90);
}

#[test]
fn test_63() {
    assert_eq!(Solution::candy(vec![5, 2, 3, 1, 4, 6, 1]), 12);
}

#[test]
fn test_64() {
    assert_eq!(Solution::candy(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5]), 69);
}

#[test]
fn test_65() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 3, 3, 4, 4, 3, 2, 1, 1, 2, 3, 3, 4, 4]), 29);
}

#[test]
fn test_66() {
    assert_eq!(Solution::candy(vec![3, 3, 3, 2, 1, 2, 3, 3, 3]), 15);
}

#[test]
fn test_67() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 3, 2, 3, 4, 5, 6, 7]), 33);
}

#[test]
fn test_68() {
    assert_eq!(Solution::candy(vec![5, 3, 8, 6, 7, 2, 4, 1]), 12);
}

#[test]
fn test_69() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 2, 3, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]), 43);
}

#[test]
fn test_70() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1]), 46);
}

#[test]
fn test_71() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 210);
}

#[test]
fn test_72() {
    assert_eq!(Solution::candy(vec![3, 2, 1, 4, 3, 5, 4, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 66);
}

#[test]
fn test_73() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]), 22);
}

#[test]
fn test_74() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 3, 3, 3, 2, 1, 2, 3, 4, 4, 3, 2, 1]), 32);
}

#[test]
fn test_75() {
    assert_eq!(Solution::candy(vec![5, 4, 3, 2, 1, 2, 3, 4, 5]), 29);
}

#[test]
fn test_76() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 3, 4, 5, 5, 4, 3, 2, 1]), 28);
}

#[test]
fn test_77() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 49);
}

#[test]
fn test_78() {
    assert_eq!(Solution::candy(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 109);
}

#[test]
fn test_79() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 225);
}

#[test]
fn test_80() {
    assert_eq!(Solution::candy(vec![1, 1, 1, 1, 1, 1, 1, 1, 1]), 9);
}

#[test]
fn test_81() {
    assert_eq!(Solution::candy(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 21);
}

#[test]
fn test_82() {
    assert_eq!(Solution::candy(vec![7, 3, 5, 4, 6, 5, 4, 3, 5, 6, 7]), 25);
}

#[test]
fn test_83() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 2, 3, 1, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 72);
}

#[test]
fn test_84() {
    assert_eq!(Solution::candy(vec![5, 3, 8, 6, 7, 9, 2]), 12);
}

#[test]
fn test_85() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 400);
}

#[test]
fn test_86() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 2, 3, 4, 4, 3, 2, 1]), 20);
}

#[test]
fn test_87() {
    assert_eq!(Solution::candy(vec![1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1]), 38);
}

#[test]
fn test_88() {
    assert_eq!(Solution::candy(vec![10, 20, 15, 10, 5, 10, 20, 30, 25, 20, 15, 10, 5]), 37);
}

#[test]
fn test_89() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7, 9, 8, 10, 9, 11, 10]), 28);
}

#[test]
fn test_90() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 3, 2, 3, 4, 5, 6, 5, 4, 5, 6, 7, 8, 7, 6, 7, 8, 9]), 56);
}

#[test]
fn test_91() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3]), 38);
}

#[test]
fn test_92() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]), 36);
}

#[test]
fn test_93() {
    assert_eq!(Solution::candy(vec![3, 2, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]), 41);
}

#[test]
fn test_94() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 106);
}

#[test]
fn test_95() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4, 3, 4]), 34);
}

#[test]
fn test_96() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 5]), 16);
}

#[test]
fn test_97() {
    assert_eq!(Solution::candy(vec![1, 3, 2, 3, 1, 5, 2, 4, 1]), 13);
}

#[test]
fn test_98() {
    assert_eq!(Solution::candy(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 21);
}

#[test]
fn test_99() {
    assert_eq!(Solution::candy(vec![5, 3, 4, 2, 1, 6, 7, 8, 9, 1]), 24);
}

#[test]
fn test_100() {
    assert_eq!(Solution::candy(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6]), 15);
}

#[test]
fn test_101() {
    assert_eq!(Solution::candy(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), 10);
}

#[test]
fn test_102() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1]), 56);
}

#[test]
fn test_103() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1]), 60);
}

#[test]
fn test_104() {
    assert_eq!(Solution::candy(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1]), 26);
}

#[test]
fn test_105() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3]), 29);
}

#[test]
fn test_106() {
    assert_eq!(Solution::candy(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 29);
}

#[test]
fn test_107() {
    assert_eq!(Solution::candy(vec![5, 3, 1, 2, 5, 4, 3, 2, 1]), 23);
}

#[test]
fn test_108() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 48);
}

#[test]
fn test_109() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 207);
}

#[test]
fn test_110() {
    assert_eq!(Solution::candy(vec![3, 2, 1, 4, 5, 6, 5, 4, 3, 2, 1]), 32);
}

#[test]
fn test_111() {
    assert_eq!(Solution::candy(vec![1, 2, 3, 4, 5, 4, 3, 2, 1]), 25);
}

#[test]
fn test_112() {
    assert_eq!(Solution::candy(vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1]), 12);
}

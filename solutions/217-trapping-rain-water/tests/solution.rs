include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::trap(vec![3, 1, 2, 1, 4, 3, 2, 1, 5]), 11);
}

#[test]
fn test_2() {
    assert_eq!(Solution::trap(vec![3, 0, 1, 3, 0, 1, 3]), 10);
}

#[test]
fn test_3() {
    assert_eq!(Solution::trap(vec![5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_4() {
    assert_eq!(Solution::trap(vec![1]), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::trap(vec![2, 0, 2]), 2);
}

#[test]
fn test_6() {
    assert_eq!(Solution::trap(vec![0, 0, 1, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_7() {
    assert_eq!(Solution::trap(vec![1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::trap(vec![1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 1, 2]), 12);
}

#[test]
fn test_9() {
    assert_eq!(Solution::trap(vec![1, 2, 1, 0, 1, 0, 1, 0, 1, 2, 1]), 10);
}

#[test]
fn test_10() {
    assert_eq!(Solution::trap(vec![0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::trap(vec![0, 0, 0, 0]), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::trap(vec![5, 4, 1, 2]), 1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::trap(vec![3, 0, 1, 3, 0, 1, 1, 3, 2, 1, 2, 1]), 13);
}

#[test]
fn test_15() {
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}

#[test]
fn test_16() {
    assert_eq!(Solution::trap(vec![3, 0, 0, 2, 0, 4]), 10);
}

#[test]
fn test_17() {
    assert_eq!(Solution::trap(vec![5, 4, 3, 2, 1, 2, 3, 4, 5]), 16);
}

#[test]
fn test_18() {
    assert_eq!(Solution::trap(vec![1, 0, 2, 0, 1, 0, 3, 1, 0, 1, 2]), 10);
}

#[test]
fn test_19() {
    assert_eq!(Solution::trap(vec![0, 2, 0, 2, 0]), 2);
}

#[test]
fn test_20() {
    assert_eq!(Solution::trap(vec![1, 0, 0, 0, 1]), 3);
}

#[test]
fn test_21() {
    assert_eq!(Solution::trap(vec![3, 0, 1, 3, 0, 1, 2, 1, 2, 1]), 9);
}

#[test]
fn test_22() {
    assert_eq!(Solution::trap(vec![0, 5, 0, 5, 0]), 5);
}

#[test]
fn test_23() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}

#[test]
fn test_24() {
    assert_eq!(Solution::trap(vec![1, 0, 1, 0, 1]), 2);
}

#[test]
fn test_25() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5]), 0);
}

#[test]
fn test_26() {
    assert_eq!(Solution::trap(vec![2, 1, 0, 2]), 3);
}

#[test]
fn test_27() {
    assert_eq!(Solution::trap(vec![10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 10]), 81);
}

#[test]
fn test_28() {
    assert_eq!(Solution::trap(vec![0, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 0]), 36);
}

#[test]
fn test_29() {
    assert_eq!(Solution::trap(vec![10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10, 0, 10]), 60);
}

#[test]
fn test_30() {
    assert_eq!(Solution::trap(vec![6, 4, 2, 0, 3, 2, 0, 3, 1, 4, 5, 3, 2, 7, 5, 3, 0, 1, 2, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2, 1]), 52);
}

#[test]
fn test_31() {
    assert_eq!(Solution::trap(vec![10, 0, 10, 0, 10, 0, 10, 0, 10]), 40);
}

#[test]
fn test_32() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 25);
}

#[test]
fn test_33() {
    assert_eq!(Solution::trap(vec![100, 80, 60, 40, 20, 0, 20, 40, 60, 80, 100]), 500);
}

#[test]
fn test_34() {
    assert_eq!(Solution::trap(vec![2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 6);
}

#[test]
fn test_35() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1]), 12);
}

#[test]
fn test_36() {
    assert_eq!(Solution::trap(vec![10, 9, 8, 7, 6, 5, 6, 7, 8, 9, 10]), 25);
}

#[test]
fn test_37() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_38() {
    assert_eq!(Solution::trap(vec![0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0]), 50);
}

#[test]
fn test_39() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 8);
}

#[test]
fn test_40() {
    assert_eq!(Solution::trap(vec![1, 2, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 0, 3, 2, 1, 2, 1, 5]), 21);
}

#[test]
fn test_41() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 0, 1]), 13);
}

#[test]
fn test_42() {
    assert_eq!(Solution::trap(vec![6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6]), 25);
}

#[test]
fn test_43() {
    assert_eq!(Solution::trap(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 19);
}

#[test]
fn test_44() {
    assert_eq!(Solution::trap(vec![0, 1, 2, 1, 0, 1, 3, 1, 0, 1, 2, 1, 0]), 8);
}

#[test]
fn test_45() {
    assert_eq!(Solution::trap(vec![10, 1, 1, 1, 1, 1, 1, 1, 1, 10]), 72);
}

#[test]
fn test_46() {
    assert_eq!(Solution::trap(vec![0, 1, 2, 3, 4, 5, 0, 5, 4, 3, 2, 1, 0]), 5);
}

#[test]
fn test_47() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1, 0, 0, 0, 1, 2, 3, 4, 5]), 35);
}

#[test]
fn test_48() {
    assert_eq!(Solution::trap(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 0);
}

#[test]
fn test_49() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 4, 2, 1, 3, 2, 1, 2, 1]), 15);
}

#[test]
fn test_50() {
    assert_eq!(Solution::trap(vec![10, 9, 8, 7, 6, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 25);
}

#[test]
fn test_51() {
    assert_eq!(Solution::trap(vec![5, 2, 1, 2, 1, 5, 1, 2, 1, 5, 1, 2, 1, 2, 1, 5]), 43);
}

#[test]
fn test_52() {
    assert_eq!(Solution::trap(vec![2, 1, 0, 1, 2]), 4);
}

#[test]
fn test_53() {
    assert_eq!(Solution::trap(vec![10, 0, 10, 0, 10, 0, 10]), 30);
}

#[test]
fn test_54() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1]), 9);
}

#[test]
fn test_55() {
    assert_eq!(Solution::trap(vec![1, 0, 2, 0, 1, 0, 1, 3, 2, 1, 2, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 42);
}

#[test]
fn test_56() {
    assert_eq!(Solution::trap(vec![2, 8, 5, 5, 5, 9, 8, 9, 2]), 10);
}

#[test]
fn test_57() {
    assert_eq!(Solution::trap(vec![1, 7, 8, 8, 6, 4, 3, 1, 1, 0, 1, 7, 6, 5, 4, 3, 2, 1, 0, 1, 0, 1, 2, 1, 1, 1, 1, 1, 1, 1]), 40);
}

#[test]
fn test_58() {
    assert_eq!(Solution::trap(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_59() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 3, 2, 1, 0, 1, 0, 1, 2, 3, 2, 1]), 14);
}

#[test]
fn test_60() {
    assert_eq!(Solution::trap(vec![5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0]), 18);
}

#[test]
fn test_61() {
    assert_eq!(Solution::trap(vec![4, 2, 0, 6, 2, 3, 8, 0, 4, 4, 1, 2, 2, 2, 3, 3, 4, 0, 1, 0, 1, 2, 1, 2, 1, 1, 1, 2, 1, 2]), 39);
}

#[test]
fn test_62() {
    assert_eq!(Solution::trap(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_63() {
    assert_eq!(Solution::trap(vec![2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0]), 18);
}

#[test]
fn test_64() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 0, 1, 2, 0, 1, 2, 0, 1, 2]), 9);
}

#[test]
fn test_65() {
    assert_eq!(Solution::trap(vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), 23);
}

#[test]
fn test_66() {
    assert_eq!(Solution::trap(vec![0, 5, 4, 3, 2, 1, 2, 3, 4, 5, 0, 5, 4, 3, 2, 1, 2, 3, 4, 5]), 37);
}

#[test]
fn test_67() {
    assert_eq!(Solution::trap(vec![2, 1, 0, 1, 2, 1, 0, 1, 2, 1]), 8);
}

#[test]
fn test_68() {
    assert_eq!(Solution::trap(vec![0, 1, 2, 3, 2, 1, 0, 1, 2, 1, 0]), 4);
}

#[test]
fn test_69() {
    assert_eq!(Solution::trap(vec![1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0]), 16);
}

#[test]
fn test_70() {
    assert_eq!(Solution::trap(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 81);
}

#[test]
fn test_71() {
    assert_eq!(Solution::trap(vec![3, 0, 1, 3, 0, 1, 2, 0, 2]), 10);
}

#[test]
fn test_72() {
    assert_eq!(Solution::trap(vec![5, 0, 3, 0, 0, 0, 2, 0, 4, 0, 0, 1, 0, 0, 5]), 55);
}

#[test]
fn test_73() {
    assert_eq!(Solution::trap(vec![3, 0, 3, 0, 3, 0, 3, 0, 3, 0]), 12);
}

#[test]
fn test_74() {
    assert_eq!(Solution::trap(vec![0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5]), 50);
}

#[test]
fn test_75() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 9);
}

#[test]
fn test_76() {
    assert_eq!(Solution::trap(vec![0, 2, 0, 2, 0, 3, 0, 3, 0, 2, 0, 2]), 11);
}

#[test]
fn test_77() {
    assert_eq!(Solution::trap(vec![3, 0, 1, 0, 2, 0, 1, 0, 3, 0, 2, 0, 1, 0]), 20);
}

#[test]
fn test_78() {
    assert_eq!(Solution::trap(vec![5, 2, 1, 2, 1, 5, 1, 2, 1, 2, 1, 5, 1, 2, 1, 2, 1, 5, 1, 2, 1, 2, 1, 5, 1, 2, 1, 2, 1, 5]), 86);
}

#[test]
fn test_79() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 0, 1, 2, 1, 0, 2, 1, 0]), 13);
}

#[test]
fn test_80() {
    assert_eq!(Solution::trap(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 4);
}

#[test]
fn test_81() {
    assert_eq!(Solution::trap(vec![0, 1, 2, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 0, 3, 2, 1, 2, 1, 0, 5, 0, 4]), 28);
}

#[test]
fn test_82() {
    assert_eq!(Solution::trap(vec![0, 2, 0, 2, 0, 3, 0, 3, 0, 4]), 10);
}

#[test]
fn test_83() {
    assert_eq!(Solution::trap(vec![5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 0, 5, 4, 3, 2, 1]), 30);
}

#[test]
fn test_84() {
    assert_eq!(Solution::trap(vec![1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7, 9, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 52);
}

#[test]
fn test_85() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 6);
}

#[test]
fn test_86() {
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5, 0, 5, 2, 3, 0, 2, 4, 0, 5]), 33);
}

#[test]
fn test_87() {
    assert_eq!(Solution::trap(vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0]), 5);
}

#[test]
fn test_88() {
    assert_eq!(Solution::trap(vec![3, 2, 1, 2, 3]), 4);
}

#[test]
fn test_89() {
    assert_eq!(Solution::trap(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 7);
}

#[test]
fn test_90() {
    assert_eq!(Solution::trap(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 9, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 35);
}

#[test]
fn test_91() {
    assert_eq!(Solution::trap(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 9);
}

#[test]
fn test_92() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 12);
}

#[test]
fn test_93() {
    assert_eq!(Solution::trap(vec![4, 2, 3, 0, 3, 5, 3, 2, 1, 5, 3, 0, 3, 5, 3, 0, 3, 5, 2, 1]), 35);
}

#[test]
fn test_94() {
    assert_eq!(Solution::trap(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 5);
}

#[test]
fn test_95() {
    assert_eq!(Solution::trap(vec![7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7]), 36);
}

#[test]
fn test_96() {
    assert_eq!(Solution::trap(vec![3, 2, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 6);
}

#[test]
fn test_97() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 4, 2, 1, 0, 1, 3]), 19);
}

#[test]
fn test_98() {
    assert_eq!(Solution::trap(vec![1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1]), 12);
}

#[test]
fn test_99() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 0, 1, 0, 2, 0, 3, 0, 2, 0, 1, 0, 3]), 24);
}

#[test]
fn test_100() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_101() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7]), 49);
}

#[test]
fn test_102() {
    assert_eq!(Solution::trap(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 100);
}

#[test]
fn test_103() {
    assert_eq!(Solution::trap(vec![3, 0, 1, 0, 2, 0, 4, 0, 3, 0, 5, 0]), 21);
}

#[test]
fn test_104() {
    assert_eq!(Solution::trap(vec![5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5]), 35);
}

#[test]
fn test_105() {
    assert_eq!(Solution::trap(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 0);
}

#[test]
fn test_106() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 0, 3, 2, 1, 2, 1]), 15);
}

#[test]
fn test_107() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 9);
}

#[test]
fn test_108() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1, 0, 1, 0, 1, 3, 2, 1, 2, 1]), 22);
}

#[test]
fn test_109() {
    assert_eq!(Solution::trap(vec![2, 1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2]), 13);
}

#[test]
fn test_110() {
    assert_eq!(Solution::trap(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 6);
}

#[test]
fn test_111() {
    assert_eq!(Solution::trap(vec![5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5]), 60);
}

#[test]
fn test_112() {
    assert_eq!(Solution::trap(vec![1, 3, 2, 4, 1, 3, 1, 4, 5, 2, 2, 1]), 8);
}

#[test]
fn test_113() {
    assert_eq!(Solution::trap(vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1]), 12);
}

#[test]
fn test_114() {
    assert_eq!(Solution::trap(vec![5, 5, 5, 5, 5, 1, 5, 5, 5, 5, 5]), 4);
}

#[test]
fn test_115() {
    assert_eq!(Solution::trap(vec![5, 2, 1, 2, 1, 5]), 14);
}

#[test]
fn test_116() {
    assert_eq!(Solution::trap(vec![0, 2, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 7);
}

#[test]
fn test_117() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_118() {
    assert_eq!(Solution::trap(vec![2, 0, 2, 0, 2, 0, 2]), 6);
}

#[test]
fn test_119() {
    assert_eq!(Solution::trap(vec![0, 1, 2, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 7);
}

#[test]
fn test_120() {
    assert_eq!(Solution::trap(vec![0, 2, 0, 2, 0, 3, 0, 3, 0, 2, 0, 2, 0]), 11);
}

#[test]
fn test_121() {
    assert_eq!(Solution::trap(vec![5, 5, 1, 1, 1, 5, 1, 1, 1, 5, 1, 1, 1, 5, 5]), 36);
}

#[test]
fn test_122() {
    assert_eq!(Solution::trap(vec![3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0]), 0);
}

#[test]
fn test_123() {
    assert_eq!(Solution::trap(vec![6, 4, 2, 0, 3, 0, 1, 4, 6, 2, 3, 5, 1, 0, 5, 4, 3, 2, 1, 0]), 42);
}

#[test]
fn test_124() {
    assert_eq!(Solution::trap(vec![5, 0, 3, 0, 0, 5, 0, 0, 2, 4]), 27);
}

#[test]
fn test_125() {
    assert_eq!(Solution::trap(vec![3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1]), 16);
}

#[test]
fn test_126() {
    assert_eq!(Solution::trap(vec![5, 5, 1, 7, 1, 1, 5, 2, 7, 6]), 23);
}

#[test]
fn test_127() {
    assert_eq!(Solution::trap(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 9);
}

#[test]
fn test_128() {
    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

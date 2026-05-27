include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 3, 4, 4, 5]), 4);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_profit(vec![1, 2, 1, 2, 1, 2, 1]), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_profit(vec![6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_profit(vec![1, 9, 6, 9, 1, 7, 1, 1, 5, 9, 9, 9]), 25);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 3, 0, 4]), 6);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_profit(vec![1, 2]), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_profit(vec![1]), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9]), 13);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3]), 7);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 3, 4, 5]), 4);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 2]), 0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_profit(vec![1, 100]), 99);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_profit(vec![1, 9, 6, 9, 1, 7, 1, 1, 5, 9, 9, 9, 8, 9]), 26);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 2, 3, 4, 5, 6]), 7);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 8);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 2, 8, 3, 10, 2, 5, 8, 12]), 30);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 3, 4, 3, 4, 5, 4, 5, 6, 5, 6, 7, 6, 7, 8, 7, 8, 9, 8, 9, 10]), 16);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_profit(vec![30, 28, 25, 22, 20, 18, 15, 13, 10, 8, 5, 3, 1, 0, 2, 4, 6, 8]), 8);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1]), 12);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]), 15);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_profit(vec![2, 1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7, 9, 8, 10, 9, 11, 10, 12, 11, 13, 12, 14, 13, 15]), 26);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_profit(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 0);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_profit(vec![100, 101, 102, 98, 97, 96, 100, 105, 110, 108, 107, 109, 112, 115, 118, 120, 119, 117, 116, 115, 114, 113, 112, 111, 110, 109]), 29);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 19);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 10, 8, 12, 15, 10, 18, 14, 20, 25, 22, 27, 30]), 47);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_profit(vec![5, 2, 4, 9, 8, 10, 3, 2, 6, 4, 8]), 17);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 6, 8, 7, 4, 12, 14, 13, 11, 10, 8, 6, 4, 2, 0]), 14);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_profit(vec![1, 5, 3, 8, 12, 4, 7, 15, 10, 18, 14, 20, 25, 22, 27, 30]), 51);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_profit(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49]), 48);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_profit(vec![5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 8);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_profit(vec![5, 3, 2, 6, 7, 8, 4, 5, 6, 1, 2, 3]), 10);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_profit(vec![5, 2, 3, 7, 6, 1, 8, 4, 9]), 17);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_profit(vec![30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_profit(vec![10, 7, 5, 8, 11, 9]), 6);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 2, 0, 2, 4, 6, 8, 10, 12, 14, 16]), 16);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 3, 3, 2, 3, 5, 0, 0, 1, 8]), 11);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2]), 10);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_profit(vec![80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80]), 0);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_profit(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 10);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 5, 7, 8, 9, 10, 12, 11, 13, 15, 14, 16, 17]), 19);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9]), 8);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 2, 3, 6, 8, 5, 8, 10]), 11);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_profit(vec![5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8]), 8);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_profit(vec![5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_profit(vec![5, 3, 5, 10, 4, 8, 2, 6, 7, 2, 3, 1, 5, 9, 10, 3, 2, 1, 5, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 44);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_profit(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_profit(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_profit(vec![10, 7, 5, 8, 11, 9, 12, 10, 14, 13, 15]), 15);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7, 9, 8, 10, 9, 11, 10, 12, 11, 13, 12, 14, 13, 15, 14]), 26);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 4, 5, 3, 6, 7, 5, 8, 9, 7, 10, 11, 9]), 17);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 1);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_profit(vec![7, 1, 3, 4, 6, 2, 5, 7, 8, 1, 9, 11, 13, 10]), 23);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_profit(vec![2, 1, 2, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5]), 26);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_profit(vec![1, 5, 1, 3, 1, 5, 1, 3, 1, 5]), 16);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 3, 0, 1, 2, 1, 2, 3]), 6);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4, 4, 5, 2, 3, 0, 4, 2, 3]), 15);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9, 5, 10, 6, 11, 7, 12, 8, 13, 9, 14, 10, 15, 11, 16, 12, 17, 13, 18, 14, 19, 15, 20]), 68);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_profit(vec![10, 9, 10, 8, 11, 7, 12, 6, 13, 5, 14, 4, 15, 3, 16, 2, 17, 1]), 64);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 23);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_profit(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]), 28);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_profit(vec![8, 6, 4, 3, 3, 5, 7, 9, 10, 8, 11, 13, 12, 15, 17]), 17);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_profit(vec![1, 2, 1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7, 9, 8, 10, 9, 11, 10, 12, 11, 13, 12, 14, 13, 15]), 27);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 6]), 5);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 2, 3, 4, 4, 5, 5, 5, 6, 7, 8, 8, 8, 9, 10, 10, 11, 12]), 11);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_profit(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80]), 0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10]), 9);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 3, 4, 5, 6, 5, 6, 7, 8, 9, 10, 11, 12, 13, 12, 14, 15]), 16);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_profit(vec![10, 7, 15, 8, 12, 9, 20, 18, 25, 17]), 30);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_profit(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, 10, 20, 30]), 30);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_profit(vec![7, 2, 5, 3, 10, 1, 3, 8, 1, 9, 2, 6]), 29);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]), 12);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 3, 4, 5, 4, 5, 6, 5, 6, 7, 6, 7, 8, 7, 8, 9, 8, 9, 10]), 18);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 19);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 10, 8, 6, 8, 12, 10, 8, 10, 14, 12, 10, 12, 16, 14, 12, 14, 18, 16, 14, 16]), 39);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_profit(vec![1, 5, 3, 8, 12, 10, 14, 18, 22, 20, 25, 28, 26, 30, 32, 31, 35, 38]), 46);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]), 10);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4, 5, 8, 2, 9, 1, 5, 7, 3, 8]), 29);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_profit(vec![2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1]), 9);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]), 8);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5]), 14);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_profit(vec![8, 12, 4, 6, 10, 3, 12, 7, 9, 8]), 21);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]), 13);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_profit(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81]), 0);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 9);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 2, 3, 2, 1, 2]), 5);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7, 9, 8, 10, 9, 11, 10, 12, 11, 13, 12, 14, 13, 15]), 26);
}

#[test]
fn test_97() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1]), 8);
}

#[test]
fn test_98() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 9);
}

#[test]
fn test_99() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 17);
}

#[test]
fn test_100() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1]), 15);
}

#[test]
fn test_101() {
    assert_eq!(Solution::max_profit(vec![10, 5, 10, 20, 10, 15, 10, 25, 15, 30, 20, 35, 25, 40, 30]), 80);
}

#[test]
fn test_102() {
    assert_eq!(Solution::max_profit(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 101]), 1);
}

#[test]
fn test_103() {
    assert_eq!(Solution::max_profit(vec![5, 3, 6, 7, 4, 3, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 21);
}

#[test]
fn test_104() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9, 5, 10, 6, 11, 7, 12, 8, 13, 9, 14, 10, 15, 11, 16]), 48);
}

#[test]
fn test_105() {
    assert_eq!(Solution::max_profit(vec![2, 2, 5, 0, 1, 5, 0, 3, 1, 2, 5]), 15);
}

#[test]
fn test_106() {
    assert_eq!(Solution::max_profit(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, 10, 20, 30, 40, 50]), 50);
}

#[test]
fn test_107() {
    assert_eq!(Solution::max_profit(vec![10, 7, 5, 8, 11, 9]), 6);
}

#[test]
fn test_108() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 8);
}

#[test]
fn test_109() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 4, 5, 7, 6, 8, 9, 11, 10, 12, 14, 13, 15]), 18);
}

#[test]
fn test_110() {
    assert_eq!(Solution::max_profit(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 10);
}

#[test]
fn test_111() {
    assert_eq!(Solution::max_profit(vec![1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100]), 990);
}

#[test]
fn test_112() {
    assert_eq!(Solution::max_profit(vec![1, 2, 2, 6, 3, 5, 4, 8, 7, 9, 10, 11, 10, 12, 13, 14, 15, 16]), 21);
}

#[test]
fn test_113() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 2, 1, 4, 5, 4, 3, 2, 1, 6, 7, 6, 5, 4, 3, 2, 1]), 12);
}

#[test]
fn test_114() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5]), 4);
}

#[test]
fn test_115() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 20, 15, 16, 14, 22, 25, 30, 35, 40]), 46);
}

#[test]
fn test_116() {
    assert_eq!(Solution::max_profit(vec![7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 23);
}

#[test]
fn test_117() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 19);
}

#[test]
fn test_118() {
    assert_eq!(Solution::max_profit(vec![1, 5, 3, 9, 7, 8, 4, 10, 1, 12, 11, 13]), 30);
}

#[test]
fn test_119() {
    assert_eq!(Solution::max_profit(vec![5, 3, 5, 2, 8, 4, 9, 10, 1, 11, 13, 12, 14, 15, 13, 16, 18, 17, 20, 22]), 39);
}

#[test]
fn test_120() {
    assert_eq!(Solution::max_profit(vec![10, 7, 5, 8, 11, 9, 12, 15, 13, 17, 20, 18]), 19);
}

#[test]
fn test_121() {
    assert_eq!(Solution::max_profit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_122() {
    assert_eq!(Solution::max_profit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_123() {
    assert_eq!(Solution::max_profit(vec![10, 1, 10, 1, 10, 1, 10, 1, 10, 1]), 36);
}

#[test]
fn test_124() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16]), 23);
}

#[test]
fn test_125() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 14);
}

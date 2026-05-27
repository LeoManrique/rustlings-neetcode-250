include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::longest_consecutive(vec![-1, -2, -3, -4]), 4);
}

#[test]
fn test_2() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 12, 3, 9, 7, 8, 11]), 6);
}

#[test]
fn test_3() {
    assert_eq!(Solution::longest_consecutive(vec![1, 9, 3, 10, 4, 20, 2]), 4);
}

#[test]
fn test_4() {
    assert_eq!(Solution::longest_consecutive(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 10);
}

#[test]
fn test_5() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 12, 3, 14, 7, 16, 20, 18, 11, 9, 8, 13, 6, 4, 19, 15, 17, 2, 1]), 20);
}

#[test]
fn test_6() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_7() {
    assert_eq!(Solution::longest_consecutive(vec![1]), 1);
}

#[test]
fn test_8() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 12, 3, 7, 8, 9, 2, 1, 0]), 4);
}

#[test]
fn test_9() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, -1000000000, 500000000, -500000000]), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::longest_consecutive(vec![-1, -2, -3, -4, -5]), 5);
}

#[test]
fn test_11() {
    assert_eq!(Solution::longest_consecutive(vec![0, 0, 0, 0, 0]), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), 1);
}

#[test]
fn test_13() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_14() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 12, 3, 5, 7, 9, 11, 13, 15, 17, 19, 2, 4, 6, 8]), 12);
}

#[test]
fn test_15() {
    assert_eq!(Solution::longest_consecutive(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), 10);
}

#[test]
fn test_16() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
}

#[test]
fn test_17() {
    assert_eq!(Solution::longest_consecutive(vec![]), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
}

#[test]
fn test_19() {
    assert_eq!(Solution::longest_consecutive(vec![5, 4, 3, 2, 1]), 5);
}

#[test]
fn test_20() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 0, 1]), 3);
}

#[test]
fn test_21() {
    assert_eq!(Solution::longest_consecutive(vec![1, 0, 1, 2]), 3);
}

#[test]
fn test_22() {
    assert_eq!(Solution::longest_consecutive(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 10);
}

#[test]
fn test_23() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, -1000000000, 0]), 1);
}

#[test]
fn test_24() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 2, 3, 3, 4, 4, 5, 5]), 5);
}

#[test]
fn test_25() {
    assert_eq!(Solution::longest_consecutive(vec![5]), 1);
}

#[test]
fn test_26() {
    assert_eq!(Solution::longest_consecutive(vec![-1, 0, 1]), 3);
}

#[test]
fn test_27() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 50);
}

#[test]
fn test_28() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110]), 11);
}

#[test]
fn test_29() {
    assert_eq!(Solution::longest_consecutive(vec![-10, -5, -3, -2, -1, 0, 1, 2, 3, 4, 5]), 9);
}

#[test]
fn test_30() {
    assert_eq!(Solution::longest_consecutive(vec![500000000, 500000001, 500000002, 500000003, 500000004, 500000005, 500000006, 500000007, 500000008, 500000009]), 10);
}

#[test]
fn test_31() {
    assert_eq!(Solution::longest_consecutive(vec![50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64]), 15);
}

#[test]
fn test_32() {
    assert_eq!(Solution::longest_consecutive(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6, 11, 20, 12, 19, 13, 18, 14, 17, 15, 16, 21, 30, 22, 29, 23, 28, 24, 27, 25, 26]), 30);
}

#[test]
fn test_33() {
    assert_eq!(Solution::longest_consecutive(vec![100, 200, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 20);
}

#[test]
fn test_34() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, -1000000000, 1000000001, -1000000001, 0, 1, -1, 2, -2]), 5);
}

#[test]
fn test_35() {
    assert_eq!(Solution::longest_consecutive(vec![999999999, 1000000000, 1000000001, 1000000002, 1000000003]), 5);
}

#[test]
fn test_36() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 5, 6, 7, 10, 11, 12, 15, 16, 17, 20, 21, 22, 25, 26, 27, 30, 31, 32]), 3);
}

#[test]
fn test_37() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 0, 5, 3, 15, 10, 5, 3, 6, 7, 100, 200, 300, 400, 500, 600, 700, 800, 900]), 3);
}

#[test]
fn test_38() {
    assert_eq!(Solution::longest_consecutive(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000, 2100, 2200, 2300, 2400, 2500, 2600, 2700, 2800, 2900, 3000]), 1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::longest_consecutive(vec![-10, -5, -3, -1, -2, 0, -8, -9, -7, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 14);
}

#[test]
fn test_40() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 5, 6, 7, 9, 10, 11, 13, 14, 15, 17, 18, 19]), 3);
}

#[test]
fn test_41() {
    assert_eq!(Solution::longest_consecutive(vec![100, 200, 300, 400, 500, 101, 102, 103, 104, 105]), 6);
}

#[test]
fn test_42() {
    assert_eq!(Solution::longest_consecutive(vec![5, 1, 3, 2, 4, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 14);
}

#[test]
fn test_43() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]), 1);
}

#[test]
fn test_44() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), 1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 10);
}

#[test]
fn test_46() {
    assert_eq!(Solution::longest_consecutive(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81]), 20);
}

#[test]
fn test_47() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 30);
}

#[test]
fn test_48() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59]), 1);
}

#[test]
fn test_49() {
    assert_eq!(Solution::longest_consecutive(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10]), 10);
}

#[test]
fn test_50() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49]), 1);
}

#[test]
fn test_51() {
    assert_eq!(Solution::longest_consecutive(vec![-1000000000, -999999999, -999999998, -999999997, -999999996, -999999995, -999999994, -999999993, -999999992, -999999991]), 10);
}

#[test]
fn test_52() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 0, -5, -10, -15, -20, -25, -30]), 1);
}

#[test]
fn test_53() {
    assert_eq!(Solution::longest_consecutive(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 21);
}

#[test]
fn test_54() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, 1000000001, 999999999, 999999998, 999999997]), 5);
}

#[test]
fn test_55() {
    assert_eq!(Solution::longest_consecutive(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 101, 102, 103, 104, 105]), 6);
}

#[test]
fn test_56() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, 1000000001, 1000000002, 1000000003, 1000000004]), 5);
}

#[test]
fn test_57() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, 1000000002, 1000000001, 1000000003, 1000000004, 1000000005, 1000000006, 1000000007, 1000000008, 1000000009]), 10);
}

#[test]
fn test_58() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]), 10);
}

#[test]
fn test_59() {
    assert_eq!(Solution::longest_consecutive(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 101, 201, 301, 401, 501, 601, 701, 801, 901, 1001]), 2);
}

#[test]
fn test_60() {
    assert_eq!(Solution::longest_consecutive(vec![10, 15, 1, 3, 2, 8, 7, 4, 12, 14, 11, 6, 9, 5]), 12);
}

#[test]
fn test_61() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20]), 6);
}

#[test]
fn test_62() {
    assert_eq!(Solution::longest_consecutive(vec![1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009]), 10);
}

#[test]
fn test_63() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25]), 7);
}

#[test]
fn test_64() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384]), 2);
}

#[test]
fn test_65() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]), 1);
}

#[test]
fn test_66() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 5, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 11);
}

#[test]
fn test_67() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75]), 1);
}

#[test]
fn test_68() {
    assert_eq!(Solution::longest_consecutive(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 10);
}

#[test]
fn test_69() {
    assert_eq!(Solution::longest_consecutive(vec![1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 31);
}

#[test]
fn test_70() {
    assert_eq!(Solution::longest_consecutive(vec![10, 9, 2, 5, 3, 7, 101, 18, 11, 12, 13, 14, 15, 16, 17, 8, 6, 4, 1, 0]), 19);
}

#[test]
fn test_71() {
    assert_eq!(Solution::longest_consecutive(vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 12);
}

#[test]
fn test_72() {
    assert_eq!(Solution::longest_consecutive(vec![100, 200, 300, 400, 500, 101, 201, 301, 401, 501]), 2);
}

#[test]
fn test_73() {
    assert_eq!(Solution::longest_consecutive(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20, -21, -22, -23, -24, -25, -26, -27, -28, -29, -30]), 30);
}

#[test]
fn test_74() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 31);
}

#[test]
fn test_75() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, 999999999, 1000000001, 1000000002, 1000000003, 1000000004]), 6);
}

#[test]
fn test_76() {
    assert_eq!(Solution::longest_consecutive(vec![5, 3, 8, 1, 2, 7, 4, 6, 9, 0, 11, 12, 13, 14, 15]), 10);
}

#[test]
fn test_77() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 25);
}

#[test]
fn test_78() {
    assert_eq!(Solution::longest_consecutive(vec![5, 100, 50, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49]), 11);
}

#[test]
fn test_79() {
    assert_eq!(Solution::longest_consecutive(vec![50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
}

#[test]
fn test_80() {
    assert_eq!(Solution::longest_consecutive(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31]), 20);
}

#[test]
fn test_81() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), 10);
}

#[test]
fn test_82() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204]), 5);
}

#[test]
fn test_83() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, 999999999, 1000000001, 1000000002, 1000000003]), 5);
}

#[test]
fn test_84() {
    assert_eq!(Solution::longest_consecutive(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96, 98]), 1);
}

#[test]
fn test_85() {
    assert_eq!(Solution::longest_consecutive(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10]), 10);
}

#[test]
fn test_86() {
    assert_eq!(Solution::longest_consecutive(vec![-5, -3, -4, -2, -1, 0, 1, 2, 3, 4, 5]), 11);
}

#[test]
fn test_87() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 30);
}

#[test]
fn test_88() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93, 95, 97, 99]), 1);
}

#[test]
fn test_89() {
    assert_eq!(Solution::longest_consecutive(vec![5, 3, 8, 9, 11, 12, 13, 14, 10]), 7);
}

#[test]
fn test_90() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 15, 3, 8, 2, 20, 25, 18, 7, 6, 11, 12, 13, 14, 16, 17, 19, 4, 9]), 19);
}

#[test]
fn test_91() {
    assert_eq!(Solution::longest_consecutive(vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000]), 1);
}

#[test]
fn test_92() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), 1);
}

#[test]
fn test_93() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21]), 1);
}

#[test]
fn test_94() {
    assert_eq!(Solution::longest_consecutive(vec![-10, -8, -6, -4, -2, 0, 2, 4, 6, 8, 10]), 1);
}

#[test]
fn test_95() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25]), 1);
}

#[test]
fn test_96() {
    assert_eq!(Solution::longest_consecutive(vec![5, 3, 1, 2, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 15);
}

#[test]
fn test_97() {
    assert_eq!(Solution::longest_consecutive(vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 31);
}

#[test]
fn test_98() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 11);
}

#[test]
fn test_99() {
    assert_eq!(Solution::longest_consecutive(vec![5, 2, 9, 1, 5, 6, 7, 3, 8, 4, 10, 11, 12, 13, 14]), 14);
}

#[test]
fn test_100() {
    assert_eq!(Solution::longest_consecutive(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 21);
}

#[test]
fn test_101() {
    assert_eq!(Solution::longest_consecutive(vec![42, 41, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53]), 13);
}

#[test]
fn test_102() {
    assert_eq!(Solution::longest_consecutive(vec![5, 3, 8, 6, 2, 7, 4, 1, 0, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 20);
}

#[test]
fn test_103() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, 1000000001, 1000000002, 1000000003, 1000000004, 1000000005, 1000000006, 1000000007, 1000000008, 1000000009, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_104() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 20);
}

#[test]
fn test_105() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 21);
}

#[test]
fn test_106() {
    assert_eq!(Solution::longest_consecutive(vec![5, 1, 9, 11, 7, 3, 8, 4, 2, 10, 6, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 20);
}

#[test]
fn test_107() {
    assert_eq!(Solution::longest_consecutive(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5]), 5);
}

#[test]
fn test_108() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, -1000000000, 2147483647, -2147483648, 0]), 1);
}

#[test]
fn test_109() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, 1000000001, 1000000002, 1000000003, 1000000004, 1000000005]), 6);
}

#[test]
fn test_110() {
    assert_eq!(Solution::longest_consecutive(vec![5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5]), 11);
}

#[test]
fn test_111() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 0, -5, -10, 15, 20, 25, 30, 35, 40]), 1);
}

#[test]
fn test_112() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, -1000000000, 500000000, 500000001, 500000002, -1000000001]), 3);
}

#[test]
fn test_113() {
    assert_eq!(Solution::longest_consecutive(vec![500000000, 500000001, 500000002, 500000003, 500000004, 500000005, 500000006, 500000007, 500000008, 500000009]), 10);
}

#[test]
fn test_114() {
    assert_eq!(Solution::longest_consecutive(vec![-10, -5, -3, -2, -1, 0, 1, 2, 3, 4, 5, 10, 11, 12]), 9);
}

#[test]
fn test_115() {
    assert_eq!(Solution::longest_consecutive(vec![5, 3, 8, 4, 7, 6, 9, 1, 2, 0]), 10);
}

#[test]
fn test_116() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 9, 1, 11, 8, 6, 7, 3, 4, 2]), 11);
}

#[test]
fn test_117() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]), 20);
}

#[test]
fn test_118() {
    assert_eq!(Solution::longest_consecutive(vec![-1, -2, -3, -4, -5, 0, 1, 2, 3, 4, 5]), 11);
}

#[test]
fn test_119() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 9, 1, 11, 5, 7, 8, 2, 12, 3, 6, 4]), 12);
}

#[test]
fn test_120() {
    assert_eq!(Solution::longest_consecutive(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 10);
}

#[test]
fn test_121() {
    assert_eq!(Solution::longest_consecutive(vec![100, 200, 300, 400, 500, 501, 502, 503, 504, 505]), 6);
}

#[test]
fn test_122() {
    assert_eq!(Solution::longest_consecutive(vec![5, 4, 3, 2, 1, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 10);
}

#[test]
fn test_123() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 9, 1, 11, 13, 8, 2, 4, 6, 7, 3]), 11);
}

#[test]
fn test_124() {
    assert_eq!(Solution::longest_consecutive(vec![-1, -2, -3, 0, 1, 2, 3, 4, 5]), 9);
}

#[test]
fn test_125() {
    assert_eq!(Solution::longest_consecutive(vec![1000000000, 1000000001, 1000000002, 1000000003, 1000000004, 1000000005, 1000000006, 1000000007, 1000000008, 1000000009]), 10);
}

#[test]
fn test_126() {
    assert_eq!(Solution::longest_consecutive(vec![10, 5, 0, 5, 3, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60]), 1);
}

#[test]
fn test_127() {
    assert_eq!(Solution::longest_consecutive(vec![1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984, 983, 982, 981, 980]), 21);
}

#[test]
fn test_128() {
    assert_eq!(Solution::longest_consecutive(vec![-1, -2, -3, -4, 0, 1, 2, 3, 4, 5]), 10);
}

#[test]
fn test_129() {
    assert_eq!(Solution::longest_consecutive(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 1);
}

#[test]
fn test_130() {
    assert_eq!(Solution::longest_consecutive(vec![5, 3, 8, 9, 11, 1, 7, 10, 12, 6, 2, 4]), 12);
}

#[test]
fn test_131() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 20]), 20);
}

#[test]
fn test_132() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110]), 11);
}

#[test]
fn test_133() {
    assert_eq!(Solution::longest_consecutive(vec![200, 199, 198, 197, 196, 195, 194, 193, 192, 191, 190]), 11);
}

#[test]
fn test_134() {
    assert_eq!(Solution::longest_consecutive(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_135() {
    assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 2, 4, 6, 8, 7, 10, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 20);
}

#[test]
fn test_136() {
    assert_eq!(Solution::longest_consecutive(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 21);
}

#[test]
fn test_137() {
    assert_eq!(Solution::longest_consecutive(vec![500000000, 500000001, 500000002, 500000003, 500000004, 500000005, 500000006, 500000007, 500000008, 500000009, 500000010]), 11);
}

#[test]
fn test_138() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 15, 16, 17, 18, 20, 21, 22, 23, 24]), 5);
}

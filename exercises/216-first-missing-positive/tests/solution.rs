include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::first_missing_positive(vec![1000000, -1000000, 500000]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::first_missing_positive(vec![1000000, -1000000, 1]), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::first_missing_positive(vec![1, 1, 1, 1]), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::first_missing_positive(vec![1, 1, 2, 2]), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::first_missing_positive(vec![2147483647, 1, 2, 0]), 3);
}

#[test]
fn test_6() {
    assert_eq!(Solution::first_missing_positive(vec![-1, -2, -3]), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
}

#[test]
fn test_8() {
    assert_eq!(Solution::first_missing_positive(vec![1]), 2);
}

#[test]
fn test_9() {
    assert_eq!(Solution::first_missing_positive(vec![2]), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::first_missing_positive(vec![0, -1, -2]), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::first_missing_positive(vec![1, 3, 2]), 4);
}

#[test]
fn test_13() {
    assert_eq!(Solution::first_missing_positive(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 2);
}

#[test]
fn test_14() {
    assert_eq!(Solution::first_missing_positive(vec![]), 1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::first_missing_positive(vec![0]), 1);
}

#[test]
fn test_16() {
    assert_eq!(Solution::first_missing_positive(vec![2147483647, -2147483648, 0]), 1);
}

#[test]
fn test_17() {
    assert_eq!(Solution::first_missing_positive(vec![0, -1, -2, -3]), 1);
}

#[test]
fn test_18() {
    assert_eq!(Solution::first_missing_positive(vec![1000000, -1000000, 0]), 1);
}

#[test]
fn test_19() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3]), 4);
}

#[test]
fn test_20() {
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
}

#[test]
fn test_21() {
    assert_eq!(Solution::first_missing_positive(vec![1000000, 1000001, 1000002]), 1);
}

#[test]
fn test_22() {
    assert_eq!(Solution::first_missing_positive(vec![2, 2]), 1);
}

#[test]
fn test_23() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
}

#[test]
fn test_24() {
    assert_eq!(Solution::first_missing_positive(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 1, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 11);
}

#[test]
fn test_25() {
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 1);
}

#[test]
fn test_26() {
    assert_eq!(Solution::first_missing_positive(vec![2, 2, 3, 1, 2, 2, 3, 1, 2, 2, 3, 1, 2, 2, 3, 1]), 4);
}

#[test]
fn test_27() {
    assert_eq!(Solution::first_missing_positive(vec![-10, 10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 11);
}

#[test]
fn test_28() {
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1, 2, 5]), 6);
}

#[test]
fn test_29() {
    assert_eq!(Solution::first_missing_positive(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20]), 1);
}

#[test]
fn test_30() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 23, 24, 25]), 21);
}

#[test]
fn test_31() {
    assert_eq!(Solution::first_missing_positive(vec![2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 1]), 11);
}

#[test]
fn test_32() {
    assert_eq!(Solution::first_missing_positive(vec![100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000, 900000, 1000000]), 1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::first_missing_positive(vec![3, 1, -1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]), 20);
}

#[test]
fn test_34() {
    assert_eq!(Solution::first_missing_positive(vec![2147483647, -2147483648, 0, 1, 2, 3, 4, 5]), 6);
}

#[test]
fn test_35() {
    assert_eq!(Solution::first_missing_positive(vec![5, 3, 5, 2, 3, 3, 9, 0, 123, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
}

#[test]
fn test_36() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100]), 101);
}

#[test]
fn test_37() {
    assert_eq!(Solution::first_missing_positive(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 21);
}

#[test]
fn test_38() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0, 4, 6, 3, 8, 5, 7]), 9);
}

#[test]
fn test_39() {
    assert_eq!(Solution::first_missing_positive(vec![2, 3, 1, -1, -2, -3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 21);
}

#[test]
fn test_40() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 0, -1, -2, -3, -4, -5]), 21);
}

#[test]
fn test_41() {
    assert_eq!(Solution::first_missing_positive(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0, 5, 3, 7, 8, 9, 4, 6]), 10);
}

#[test]
fn test_43() {
    assert_eq!(Solution::first_missing_positive(vec![5, 3, 2, 1, 4, 6, 8, 7, 10, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 41);
}

#[test]
fn test_44() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 0]), 30);
}

#[test]
fn test_45() {
    assert_eq!(Solution::first_missing_positive(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 11);
}

#[test]
fn test_46() {
    assert_eq!(Solution::first_missing_positive(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]), 20);
}

#[test]
fn test_47() {
    assert_eq!(Solution::first_missing_positive(vec![-10, -20, -30, -40, -50, 100, 200, 300, 400, 500]), 1);
}

#[test]
fn test_48() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0]), 11);
}

#[test]
fn test_49() {
    assert_eq!(Solution::first_missing_positive(vec![-5, -4, -3, -2, -1]), 1);
}

#[test]
fn test_50() {
    assert_eq!(Solution::first_missing_positive(vec![100000, 100001, 100002, 100003, 100004, 100005, 100006, 100007, 100008, 100009, 100010, 100011, 100012, 100013, 100014, 100015, 100016, 100017, 100018, 100019]), 1);
}

#[test]
fn test_51() {
    assert_eq!(Solution::first_missing_positive(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 1);
}

#[test]
fn test_52() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22]), 21);
}

#[test]
fn test_53() {
    assert_eq!(Solution::first_missing_positive(vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40]), 1);
}

#[test]
fn test_54() {
    assert_eq!(Solution::first_missing_positive(vec![2, 3, -7, 6, 8, 1, -10, 15]), 4);
}

#[test]
fn test_55() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 3);
}

#[test]
fn test_56() {
    assert_eq!(Solution::first_missing_positive(vec![0, 2, 2, 1, 3, 5, 4]), 6);
}

#[test]
fn test_57() {
    assert_eq!(Solution::first_missing_positive(vec![0, -1, -2, -3, -4, -5, -6, -7, -8, -9]), 1);
}

#[test]
fn test_58() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]), 3);
}

#[test]
fn test_59() {
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1, 5, 2]), 6);
}

#[test]
fn test_60() {
    assert_eq!(Solution::first_missing_positive(vec![3, 2, 1, 5, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 31);
}

#[test]
fn test_61() {
    assert_eq!(Solution::first_missing_positive(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 2);
}

#[test]
fn test_62() {
    assert_eq!(Solution::first_missing_positive(vec![100000, 99999, 99998, 99997, 99996, 99995, 99994, 99993, 99992, 99991]), 1);
}

#[test]
fn test_63() {
    assert_eq!(Solution::first_missing_positive(vec![0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 1);
}

#[test]
fn test_64() {
    assert_eq!(Solution::first_missing_positive(vec![5, 3, 1, 2, 4, 6, 8, 7, 10, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 31);
}

#[test]
fn test_65() {
    assert_eq!(Solution::first_missing_positive(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), 1);
}

#[test]
fn test_66() {
    assert_eq!(Solution::first_missing_positive(vec![100000, 100001, 100002, 100003, 100004, 100005, 100006, 100007, 100008, 100009, 1]), 2);
}

#[test]
fn test_67() {
    assert_eq!(Solution::first_missing_positive(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 1);
}

#[test]
fn test_68() {
    assert_eq!(Solution::first_missing_positive(vec![100000, 100001, 100002, 100003, 100004, 100005, 100006, 100007, 100008, 100009, -1]), 1);
}

#[test]
fn test_69() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6]), 7);
}

#[test]
fn test_70() {
    assert_eq!(Solution::first_missing_positive(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4]), 11);
}

#[test]
fn test_71() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 23, 24, 25, 26, 27, 28, 29, 30]), 22);
}

#[test]
fn test_72() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 26);
}

#[test]
fn test_73() {
    assert_eq!(Solution::first_missing_positive(vec![1000000, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0]), 11);
}

#[test]
fn test_74() {
    assert_eq!(Solution::first_missing_positive(vec![-100000, -99999, -99998, -99997, -99996, -99995, -99994, -99993, -99992, -99991, -99990, -99989, -99988, -99987, -99986, -99985, -99984, -99983, -99982, -99981]), 1);
}

#[test]
fn test_75() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 41);
}

#[test]
fn test_76() {
    assert_eq!(Solution::first_missing_positive(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40]), 1);
}

#[test]
fn test_77() {
    assert_eq!(Solution::first_missing_positive(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 11);
}

#[test]
fn test_78() {
    assert_eq!(Solution::first_missing_positive(vec![5, 3, 4, 1, 2, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 6);
}

#[test]
fn test_79() {
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1, 5, 6, 2]), 7);
}

#[test]
fn test_80() {
    assert_eq!(Solution::first_missing_positive(vec![5, 3, 1, 2, 4, 6, 8, 7, 9, 11, 10, 13, 12, 14, 15, 16, 17, 18, 19, 20]), 21);
}

#[test]
fn test_81() {
    assert_eq!(Solution::first_missing_positive(vec![10000, 10001, 10002, 10003, 10004, 10005, 10006, 10007, 10008, 10009]), 1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 31);
}

#[test]
fn test_83() {
    assert_eq!(Solution::first_missing_positive(vec![2147483647, -2147483648, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 10);
}

#[test]
fn test_84() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 21);
}

#[test]
fn test_85() {
    assert_eq!(Solution::first_missing_positive(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 11);
}

#[test]
fn test_86() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 4);
}

#[test]
fn test_87() {
    assert_eq!(Solution::first_missing_positive(vec![2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9]), 1);
}

#[test]
fn test_88() {
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1, 5, 2, 7, 8, 9, 10]), 6);
}

#[test]
fn test_89() {
    assert_eq!(Solution::first_missing_positive(vec![100, 4, 200, 1, 3, 2]), 5);
}

#[test]
fn test_90() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 16);
}

#[test]
fn test_91() {
    assert_eq!(Solution::first_missing_positive(vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90]), 1);
}

#[test]
fn test_92() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 32]), 31);
}

#[test]
fn test_93() {
    assert_eq!(Solution::first_missing_positive(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
}

#[test]
fn test_94() {
    assert_eq!(Solution::first_missing_positive(vec![2147483647, -2147483648, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
}

#[test]
fn test_95() {
    assert_eq!(Solution::first_missing_positive(vec![101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200]), 1);
}

#[test]
fn test_96() {
    assert_eq!(Solution::first_missing_positive(vec![100000, 100001, 100002, 100003, 100004, 100005, 100006, 100007, 100008, 100009, 100010]), 1);
}

#[test]
fn test_97() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, -1, -2, -3]), 11);
}

#[test]
fn test_98() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 21);
}

#[test]
fn test_99() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11]), 10);
}

#[test]
fn test_100() {
    assert_eq!(Solution::first_missing_positive(vec![2, 3, 4, 5, 6, 7, 8, 9, 10]), 1);
}

#[test]
fn test_101() {
    assert_eq!(Solution::first_missing_positive(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), 1);
}

#[test]
fn test_102() {
    assert_eq!(Solution::first_missing_positive(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 2);
}

#[test]
fn test_103() {
    assert_eq!(Solution::first_missing_positive(vec![5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5]), 6);
}

#[test]
fn test_104() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 3);
}

#[test]
fn test_105() {
    assert_eq!(Solution::first_missing_positive(vec![5, 3, 1, 4, 2, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 21);
}

#[test]
fn test_106() {
    assert_eq!(Solution::first_missing_positive(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), 2);
}

#[test]
fn test_107() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0, 0, 0, 0, 0]), 3);
}

#[test]
fn test_108() {
    assert_eq!(Solution::first_missing_positive(vec![3, 5, -2, 1, 4, 2, 3, 6]), 7);
}

#[test]
fn test_109() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0, 4, 5, 6, 7, 8, 9, 10]), 3);
}

#[test]
fn test_110() {
    assert_eq!(Solution::first_missing_positive(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), 1);
}

#[test]
fn test_111() {
    assert_eq!(Solution::first_missing_positive(vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 1);
}

#[test]
fn test_112() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21]), 20);
}

#[test]
fn test_113() {
    assert_eq!(Solution::first_missing_positive(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 1);
}

#[test]
fn test_114() {
    assert_eq!(Solution::first_missing_positive(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 1);
}

#[test]
fn test_115() {
    assert_eq!(Solution::first_missing_positive(vec![2, 3, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 1);
}

#[test]
fn test_116() {
    assert_eq!(Solution::first_missing_positive(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5]), 6);
}

#[test]
fn test_117() {
    assert_eq!(Solution::first_missing_positive(vec![1, 1, 1, 1, 1, 1]), 2);
}

#[test]
fn test_118() {
    assert_eq!(Solution::first_missing_positive(vec![0, 0, 0, 0, 0, 0]), 1);
}

#[test]
fn test_119() {
    assert_eq!(Solution::first_missing_positive(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 2);
}

#[test]
fn test_120() {
    assert_eq!(Solution::first_missing_positive(vec![0, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
}

#[test]
fn test_121() {
    assert_eq!(Solution::first_missing_positive(vec![1, 1000000, 2, 999999, 3, 999998, 4, 999997, 5, 999996, 6, 999995, 7, 999994, 8, 999993, 9, 999992, 10, 999991, 11, 999990, 12, 999989, 13, 999988, 14, 999987, 15, 999986, 16, 999985, 17, 999984, 18, 999983, 19, 999982, 20, 999981, 21, 999980, 22, 999979, 23, 999978, 24, 999977, 25, 999976, 26, 999975, 27, 999974, 28, 999973, 29, 999972, 30, 999971, 31, 999970, 32, 999969, 33, 999968, 34, 999967, 35, 999966, 36, 999965, 37, 999964, 38, 999963, 39, 999962, 40, 999961, 41, 999960, 42, 999959, 43, 999958, 44, 999957, 45, 999956, 46, 999955, 47, 999954, 48, 999953, 49, 999952, 50, 999951]), 51);
}

#[test]
fn test_122() {
    assert_eq!(Solution::first_missing_positive(vec![3, 5, -7, 1, 2, 4, 6, 8, 9, 10]), 7);
}

#[test]
fn test_123() {
    assert_eq!(Solution::first_missing_positive(vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
}

#[test]
fn test_124() {
    assert_eq!(Solution::first_missing_positive(vec![5, 1, 4, 3, 2]), 6);
}

#[test]
fn test_125() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, -1, -2, -3, -4, -5]), 11);
}

#[test]
fn test_126() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, -1, -2, -3, -4, -5]), 16);
}

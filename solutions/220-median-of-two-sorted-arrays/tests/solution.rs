include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100, 200, 300], vec![150, 250, 350]), 225.0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4, 5, 6, 7, 8, 9, 10]), 5.5);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1000000], vec![-1000000]), 0.0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19]), 10.0);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7], vec![2, 4, 6, 8]), 4.5);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![10, 20, 30], vec![5, 15, 25, 35, 45]), 22.5);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 10.5);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]), 5.5);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![0, 4, 5, 6]), 3.0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![5, 6], vec![1, 2, 3, 4, 7, 8]), 4.5);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7], vec![2, 4, 6, 8, 10]), 5.0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6], vec![0]), 3.0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4]), 2.5);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3]), 2.0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5], vec![2, 4, 6]), 3.5);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4]), 2.5);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![10, 20, 30, 40, 50], vec![5, 15, 25, 35, 45]), 27.5);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], vec![2, 4, 6, 8, 10]), 10.5);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1000000, 1000001, 1000002, 1000003, 1000004], vec![999999, 1000000, 1000005, 1000010]), 1000002.0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-1000000, -999999, -999998], vec![-1000001, -999997, -999995, -999993, -999991]), -999997.5);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![5, 9, 11, 19], vec![1, 3, 4, 6, 7, 8, 10, 12, 13, 14, 15, 16, 17, 18]), 10.5);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-5, -3, -1], vec![-6, -4, -2, 0]), -3.0);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-10, -5, 0, 5, 10], vec![-7, -3, 2, 7, 12]), 1.0);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]), 15.5);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![2, 4, 6, 8, 10]), 8.0);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-1000000], vec![0, 1000000]), 0.0);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 10.5);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-10, -9, -8, -7, -6], vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]), -2.5);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 10.5);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]), 15.0);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 5, 9, 13], vec![2, 6, 10, 14, 18]), 9.0);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 8.0);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 8.0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), 0.0);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]), 10.5);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-5, -3, -1, 1, 3, 5], vec![-6, -4, -2, 0, 2, 4, 6]), 0.0);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10, 12, 14, 16]), 7.0);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 10.5);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![1000000]), 500000.5);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![5, 15, 25, 35, 45], vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 37.5);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], vec![26, 27, 28, 29, 30]), 15.5);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-1000000, -999999, -999998], vec![-1000001, -1000000, -999999, -999998]), -999999.0);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20]), 10.5);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]), 13.0);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 25.5);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15], vec![2, 4, 6, 8, 10, 12, 14, 16]), 8.5);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 20.5);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![0, 2, 4, 6, 8]), 4.5);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100, 200, 300, 400, 500], vec![5, 15, 25, 35, 45, 55]), 55.0);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 20.5);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1, 3, 5, 7, 9]), 5.0);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]), 15.5);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40]), 20.5);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100, 200, 300, 400, 500], vec![50, 150, 250, 350, 450, 550]), 300.0);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 15.5);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40]), 20.5);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![10, 20, 30, 40, 50], vec![5, 15, 25, 35, 45, 55]), 30.0);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 4.0);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-1, -3, -5, -7, -9], vec![-2, -4, -6, -8, -10]), -5.5);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11], vec![2, 4, 6, 8, 10, 12]), 6.5);
}

#[test]
fn test_66() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100, 200, 300, 400, 500], vec![50, 150, 250, 350, 450, 550, 650]), 325.0);
}

#[test]
fn test_67() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 15.5);
}

#[test]
fn test_68() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000], vec![500, 1500, 2500, 3500, 4500, 5500, 6500, 7500, 8500, 9500, 10500]), 5500.0);
}

#[test]
fn test_69() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-10, -8, -6, -4, -2], vec![-9, -7, -5, -3, -1]), -5.5);
}

#[test]
fn test_70() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1000000], vec![999999]), 999999.5);
}

#[test]
fn test_71() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-5, -3, -1, 1, 3], vec![-10, -8, -6, -4, -2]), -3.5);
}

#[test]
fn test_72() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4, 5]), 3.0);
}

#[test]
fn test_73() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 5, 9, 13, 17], vec![2, 6, 10, 14, 18]), 9.5);
}

#[test]
fn test_74() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1000000, 1000001, 1000002], vec![999999, 1000000, 1000001, 1000002]), 1000001.0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100000, 100001, 100002, 100003, 100004], vec![100005, 100006, 100007, 100008, 100009]), 100004.5);
}

#[test]
fn test_76() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-5, -4, -3, -2, -1], vec![0, 1, 2, 3, 4, 5]), 0.0);
}

#[test]
fn test_77() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100000], vec![-100000, -99999, -99998, -99997, -99996, -99995]), -99997.0);
}

#[test]
fn test_78() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![]), 5.5);
}

#[test]
fn test_79() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![]), 3.0);
}

#[test]
fn test_80() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], vec![2, 4, 6, 8, 10]), 15.0);
}

#[test]
fn test_81() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]), 5.5);
}

#[test]
fn test_82() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4, 5]), 3.0);
}

#[test]
fn test_83() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-5, -3, 0, 8], vec![-10, -4, 2, 6, 12]), 0.0);
}

#[test]
fn test_84() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 13.0);
}

#[test]
fn test_85() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-1, 3, 5, 7, 9], vec![-2, -4, -6, -8, -10]), -5.5);
}

#[test]
fn test_86() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50]), 25.5);
}

#[test]
fn test_87() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 2, 3, 3], vec![2, 2, 3, 3, 4, 4, 5, 5]), 3.0);
}

#[test]
fn test_88() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100, 200, 300], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 6.5);
}

#[test]
fn test_89() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 10.5);
}

#[test]
fn test_90() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5.5);
}

#[test]
fn test_91() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6, 7, 8, 9, 10]), 5.5);
}

#[test]
fn test_92() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], vec![]), 13.0);
}

#[test]
fn test_93() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100, 200, 300, 400, 500], vec![50, 150, 250, 350, 450, 550]), 300.0);
}

#[test]
fn test_94() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-5, 0, 3, 8, 12], vec![-10, -1, 2, 4, 9, 14]), 3.0);
}

#[test]
fn test_95() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![50, 60, 70, 80, 90], vec![10, 20, 30, 40]), 50.0);
}

#[test]
fn test_96() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40]), 18.0);
}

#[test]
fn test_97() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 5, 7, 9, 11, 13], vec![2, 4, 6, 8, 10, 12, 14]), 8.0);
}

#[test]
fn test_98() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-10, -5, 0, 5, 10], vec![-20, -15, -1, 1, 6, 11, 15, 20]), 1.0);
}

#[test]
fn test_99() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11, 12]), 6.5);
}

#[test]
fn test_100() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-1, -2, -3, -4, -5], vec![-10, -9, -8, -7, -6]), -3.5);
}

#[test]
fn test_101() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![0, 0, 0, 0], vec![0, 0, 0, 0]), 0.0);
}

#[test]
fn test_102() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![5, 15, 25, 35, 45], vec![10, 20, 30, 40, 50, 60]), 30.0);
}

#[test]
fn test_103() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100, 200, 300], vec![50, 150, 250, 350]), 200.0);
}

#[test]
fn test_104() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26]), 12.0);
}

#[test]
fn test_105() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1.0);
}

#[test]
fn test_106() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![-100000, -99999, -99998], vec![-99997, -99996, -99995, -99994, -99993, -99992]), -99996.0);
}

#[test]
fn test_107() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![]), 10.5);
}

#[test]
fn test_108() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![]), 3.0);
}

#[test]
fn test_109() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53]), 19.0);
}

#[test]
fn test_110() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 2, 3, 4], vec![2, 2, 3, 4, 5]), 2.5);
}

#[test]
fn test_111() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10, 12, 14]), 6.5);
}

#[test]
fn test_112() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![100, 200, 300, 400], vec![50, 150, 250, 350, 450, 550]), 275.0);
}

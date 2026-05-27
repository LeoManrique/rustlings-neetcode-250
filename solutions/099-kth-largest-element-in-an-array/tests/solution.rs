include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_kth_largest(vec![7, 6, 5, 4, 3, 2, 1], 5), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_kth_largest(vec![7, 6, 5, 4, 3, 2, 1], 3), 5);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_kth_largest(vec![10000, -10000, 0], 2), 0);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_kth_largest(vec![-1, -2, -3, -4, -5], 2), -2);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_kth_largest(vec![1], 1), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_kth_largest(vec![10000, 9999, 9998, 9997, 9996, 9995, 9994, 9993, 9992, 9991, 9990, 9989, 9988, 9987, 9986, 9985, 9984, 9983, 9982, 9981], 10), 9991);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_kth_largest(vec![1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984, 983, 982, 981], 15), 986);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_kth_largest(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], 20), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_kth_largest(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 20), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_kth_largest(vec![-10000, -9999, -9998, -9997, -9996, -9995, -9994, -9993, -9992, -9991], 3), -9993);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_kth_largest(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 10), 600);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_kth_largest(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 20), 6);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_kth_largest(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10], 1), 100);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 2, 3, 4, 4, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 10], 25), 4);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_kth_largest(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200], 15), 60);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_kth_largest(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 18), 3);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 10), 6);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 15), 6);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_kth_largest(vec![9, 1, 8, 2, 7, 3, 6, 4, 5], 3), 7);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_kth_largest(vec![-10000, -10000, -10000, -10000, -10000, -10000, -10000, -10000, -10000, -10000], 10), -10000);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_kth_largest(vec![-10000, 10000, -9999, 9999, -9998, 9998, -9997, 9997, -9996, 9996], 5), 9996);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_kth_largest(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15], 7), -7);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 20), 11);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_kth_largest(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50], 25), 26);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_kth_largest(vec![9999, 9998, 9997, 9996, 9995, 9994, 9993, 9992, 9991, 9990], 5), 9995);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 1), 20);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_kth_largest(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10], 5), 60);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_kth_largest(vec![2, 2, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10, 1, 11, 1], 5), 7);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6], 10), 5);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_kth_largest(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1), 0);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_kth_largest(vec![10000, 9999, 9998, 9997, 9996, 9995, 9994, 9993, 9992, 9991, 9990, 9989, 9988, 9987, 9986, 9985, 9984, 9983, 9982, 9981], 15), 9986);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_kth_largest(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20], 10), -10);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 19), 2);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_kth_largest(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 15), 5);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_kth_largest(vec![10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000], 1), 10000);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10), 11);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_kth_largest(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 5), -5);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_kth_largest(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], 7), 27);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_kth_largest(vec![-10000, -9999, -9998, -9997, -9996, -9995, -9994, -9993, -9992, -9991, -9990, -9989, -9988, -9987, -9986, -9985, -9984, -9983, -9982, -9981], 15), -9995);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_kth_largest(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 7), -7);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_kth_largest(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40], 10), 22);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 17), 14);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_kth_largest(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 1), 5);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_kth_largest(vec![1, 10000, 2, 9999, 3, 9998, 4, 9997, 5, 9996, 6, 9995, 7, 9994, 8, 9993, 9, 9992, 10, 9991], 7), 9994);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_kth_largest(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], 10), 60);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_kth_largest(vec![25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 18), 8);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 20), 1);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_kth_largest(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, -10, -20, -30, -40, -50, -60, -70, -80, -90, -100], 12), -10);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_kth_largest(vec![10000, 9999, 9998, 9997, 9996, 9995, 9994, 9993, 9992, 9991, 9990], 10), 9991);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_kth_largest(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 14), 7);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 25), 6);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_kth_largest(vec![10000, 5000, 0, -5000, -10000, 10000, 5000, 0, -5000, -10000, 10000, 5000, 0, -5000, -10000], 8), 0);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_kth_largest(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], 15), 11);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_kth_largest(vec![10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000], 15), 10000);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_kth_largest(vec![-10000, -9999, -9998, -9997, -9996, -9995, -9994, -9993, -9992, -9991, -9990], 8), -9997);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_kth_largest(vec![0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20], 10), -9);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_kth_largest(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1), 1);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_kth_largest(vec![20, 20, 19, 19, 18, 18, 17, 17, 16, 16, 15, 15, 14, 14, 13, 13, 12, 12, 11, 11], 18), 12);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_kth_largest(vec![104, 103, 102, 101, 100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85], 15), 90);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_kth_largest(vec![9, 3, 7, 6, 2, 5, 8, 4, 1, 10], 3), 8);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_kth_largest(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5], 10), 0);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_kth_largest(vec![1, 3, 2, 6, 4, 5, 8, 7, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], 12), 14);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6], 7), 5);
}

#[test]
fn test_66() {
    assert_eq!(Solution::find_kth_largest(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 30), 21);
}

#[test]
fn test_67() {
    assert_eq!(Solution::find_kth_largest(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 5), 5);
}

#[test]
fn test_68() {
    assert_eq!(Solution::find_kth_largest(vec![100, 90, 90, 80, 80, 70, 70, 60, 60, 50, 50, 40, 40, 30, 30, 20, 20, 10, 10, 0, 0, -10, -10, -20, -20, -30, -30, -40, -40], 18), 10);
}

#[test]
fn test_69() {
    assert_eq!(Solution::find_kth_largest(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10], 3), 7);
}

#[test]
fn test_70() {
    assert_eq!(Solution::find_kth_largest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 8), 8);
}

#[test]
fn test_71() {
    assert_eq!(Solution::find_kth_largest(vec![10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000], 5), 10000);
}

#[test]
fn test_72() {
    assert_eq!(Solution::find_kth_largest(vec![5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 19), 1);
}

#[test]
fn test_73() {
    assert_eq!(Solution::find_kth_largest(vec![9999, 9998, 9997, 9996, 9995, 9994, 9993, 9992, 9991, 9990, 9989, 9988, 9987, 9986, 9985, 9984, 9983, 9982, 9981, 9980], 10), 9990);
}

#[test]
fn test_74() {
    assert_eq!(Solution::find_kth_largest(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 20), 0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::find_kth_largest(vec![-10000, -9999, -9998, -9997, -9996, -9995, -9994, -9993, -9992, -9991, -9990, -9989, -9988, -9987, -9986, -9985, -9984, -9983, -9982, -9981], 10), -9990);
}

#[test]
fn test_76() {
    assert_eq!(Solution::find_kth_largest(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 3), 5);
}

#[test]
fn test_77() {
    assert_eq!(Solution::find_kth_largest(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 25), 5);
}

#[test]
fn test_78() {
    assert_eq!(Solution::find_kth_largest(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 10), 5);
}

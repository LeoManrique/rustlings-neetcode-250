include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::subarray_sum(vec![2, 2, 2, 2, 2], 4), 4);
}

#[test]
fn test_2() {
    assert_eq!(Solution::subarray_sum(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 15), 8);
}

#[test]
fn test_3() {
    assert_eq!(Solution::subarray_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), 55);
}

#[test]
fn test_4() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 1, 2, 1], 3), 4);
}

#[test]
fn test_5() {
    assert_eq!(Solution::subarray_sum(vec![100, 200, 300, 400, 500], 1500), 1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::subarray_sum(vec![1000, -1000, 1000, -1000, 1000], 0), 6);
}

#[test]
fn test_7() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5], 9), 2);
}

#[test]
fn test_8() {
    assert_eq!(Solution::subarray_sum(vec![1], 1), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
}

#[test]
fn test_10() {
    assert_eq!(Solution::subarray_sum(vec![100, -100, 200, -200, 300, -300], 0), 6);
}

#[test]
fn test_11() {
    assert_eq!(Solution::subarray_sum(vec![-1, -1, 1], 0), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::subarray_sum(vec![10, 20, 30, 40, 50], 100), 1);
}

#[test]
fn test_13() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 15), 3);
}

#[test]
fn test_14() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 5, -2, 3], 3), 3);
}

#[test]
fn test_15() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 2, -2, 3, -3, 4, -4], 0), 10);
}

#[test]
fn test_16() {
    assert_eq!(Solution::subarray_sum(vec![1, 0, -1, 0, -1, 0, 1], 0), 8);
}

#[test]
fn test_17() {
    assert_eq!(Solution::subarray_sum(vec![10, 20, 30, 40, 50], 150), 1);
}

#[test]
fn test_18() {
    assert_eq!(Solution::subarray_sum(vec![3, 4, 7, 2, -3, 1, 4, 2], 7), 4);
}

#[test]
fn test_19() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 0], 0), 3);
}

#[test]
fn test_20() {
    assert_eq!(Solution::subarray_sum(vec![-1, -2, -3], -6), 1);
}

#[test]
fn test_21() {
    assert_eq!(Solution::subarray_sum(vec![-1, -1, 1, 1, 0], 0), 4);
}

#[test]
fn test_22() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
}

#[test]
fn test_23() {
    assert_eq!(Solution::subarray_sum(vec![-1, -1, 2], 1), 1);
}

#[test]
fn test_24() {
    assert_eq!(Solution::subarray_sum(vec![1000, -1000, 1000, -1000, 1000], 0), 6);
}

#[test]
fn test_25() {
    assert_eq!(Solution::subarray_sum(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 25), 16);
}

#[test]
fn test_26() {
    assert_eq!(Solution::subarray_sum(vec![1000, -1000, 2000, -2000, 3000], 0), 3);
}

#[test]
fn test_27() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 0, 1, -1, 1, -1], 0), 13);
}

#[test]
fn test_28() {
    assert_eq!(Solution::subarray_sum(vec![1000, 2000, 3000, 4000, 5000], 10000), 1);
}

#[test]
fn test_29() {
    assert_eq!(Solution::subarray_sum(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50], 1000), 1);
}

#[test]
fn test_30() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 1, -1, 1, -1, 1, -1], 0), 16);
}

#[test]
fn test_31() {
    assert_eq!(Solution::subarray_sum(vec![1, 3, -2, 5, 6, -4, 2, 3], 5), 3);
}

#[test]
fn test_32() {
    assert_eq!(Solution::subarray_sum(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 25), 6);
}

#[test]
fn test_33() {
    assert_eq!(Solution::subarray_sum(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 15), 8);
}

#[test]
fn test_34() {
    assert_eq!(Solution::subarray_sum(vec![10, 20, 30, -10, -20, -30, 40, 50, -40, -50], 30), 5);
}

#[test]
fn test_35() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 0, 1, -1, 2], 0), 7);
}

#[test]
fn test_36() {
    assert_eq!(Solution::subarray_sum(vec![5, 5, 5, 5, 5, 5, 5], 10), 6);
}

#[test]
fn test_37() {
    assert_eq!(Solution::subarray_sum(vec![100, -100, 50, 50, -50, 50, -100, 100], 0), 10);
}

#[test]
fn test_38() {
    assert_eq!(Solution::subarray_sum(vec![-10, 0, 10, -10, 0, 10], 0), 9);
}

#[test]
fn test_39() {
    assert_eq!(Solution::subarray_sum(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 5000), 0);
}

#[test]
fn test_40() {
    assert_eq!(Solution::subarray_sum(vec![10, -5, 10, -5, 10], 5), 4);
}

#[test]
fn test_41() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 1, -1, 1, -1], 0), 9);
}

#[test]
fn test_42() {
    assert_eq!(Solution::subarray_sum(vec![100, 200, 300, 400, 500], 1000), 1);
}

#[test]
fn test_43() {
    assert_eq!(Solution::subarray_sum(vec![0, 0, 0, 0, 0, 0, 0], 0), 28);
}

#[test]
fn test_44() {
    assert_eq!(Solution::subarray_sum(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10], -15), 3);
}

#[test]
fn test_45() {
    assert_eq!(Solution::subarray_sum(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 100), 1);
}

#[test]
fn test_46() {
    assert_eq!(Solution::subarray_sum(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 1500), 3);
}

#[test]
fn test_47() {
    assert_eq!(Solution::subarray_sum(vec![1, -2, 1, 2, -1, 2, 3, 4, -2, 1], 3), 5);
}

#[test]
fn test_48() {
    assert_eq!(Solution::subarray_sum(vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10], -3), 4);
}

#[test]
fn test_49() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), 2);
}

#[test]
fn test_50() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5, -5, -4, -3, -2, -1], 5), 3);
}

#[test]
fn test_51() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), 16);
}

#[test]
fn test_52() {
    assert_eq!(Solution::subarray_sum(vec![1000, 1000, 1000, 1000, 1000, 1000, 1000], 3000), 5);
}

#[test]
fn test_53() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2], 3), 9);
}

#[test]
fn test_54() {
    assert_eq!(Solution::subarray_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), 55);
}

#[test]
fn test_55() {
    assert_eq!(Solution::subarray_sum(vec![0, 0, 0, 0, 0], 0), 15);
}

#[test]
fn test_56() {
    assert_eq!(Solution::subarray_sum(vec![-10, -20, -30, -40, -50], -70), 1);
}

#[test]
fn test_57() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), 6);
}

#[test]
fn test_58() {
    assert_eq!(Solution::subarray_sum(vec![1, 3, 4, 5, 7, 8, 10, 11, 12, 13], 20), 2);
}

#[test]
fn test_59() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 0, 0, 1, -1], 0), 11);
}

#[test]
fn test_60() {
    assert_eq!(Solution::subarray_sum(vec![10, -5, 2, -3, 1, 5, -2], 7), 1);
}

#[test]
fn test_61() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 0, 0, 1], 0), 7);
}

#[test]
fn test_62() {
    assert_eq!(Solution::subarray_sum(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 2500), 1);
}

#[test]
fn test_63() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), 6);
}

#[test]
fn test_64() {
    assert_eq!(Solution::subarray_sum(vec![10, -10, 10, -10, 10, -10, 10, -10, 10, -10], 0), 25);
}

#[test]
fn test_65() {
    assert_eq!(Solution::subarray_sum(vec![-10, 100, -100, 50, -50, 25, -25], 0), 6);
}

#[test]
fn test_66() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5], 0), 15);
}

#[test]
fn test_67() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, -6, 2, 3, 4, -10, 2, 3, 4, 5], 5), 6);
}

#[test]
fn test_68() {
    assert_eq!(Solution::subarray_sum(vec![-10, -20, 10, 20, 10, -10, -20, 10, 20], 0), 6);
}

#[test]
fn test_69() {
    assert_eq!(Solution::subarray_sum(vec![1000, -1000, 1000, -1000, 1000, -1000, 1000, -1000], 0), 16);
}

#[test]
fn test_70() {
    assert_eq!(Solution::subarray_sum(vec![10, 20, 30, 40, 50], 70), 1);
}

#[test]
fn test_71() {
    assert_eq!(Solution::subarray_sum(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 150), 3);
}

#[test]
fn test_72() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 15), 3);
}

#[test]
fn test_73() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 15), 3);
}

#[test]
fn test_74() {
    assert_eq!(Solution::subarray_sum(vec![-100, -200, -300, -400, -500, -600, -700, -800, -900, -1000], -5000), 0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1], 0), 25);
}

#[test]
fn test_76() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 5, -2, 3], 3), 3);
}

#[test]
fn test_77() {
    assert_eq!(Solution::subarray_sum(vec![1000, -1000, 1000, -1000, 1000, -1000, 1000, -1000, 1000, -1000], 0), 25);
}

#[test]
fn test_78() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 50), 2);
}

#[test]
fn test_79() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 1, -1, 1, -1, 1], 0), 12);
}

#[test]
fn test_80() {
    assert_eq!(Solution::subarray_sum(vec![100, 200, 300, 400, 500, 600], 1500), 2);
}

#[test]
fn test_81() {
    assert_eq!(Solution::subarray_sum(vec![5, 5, 5, 5, 5], 15), 3);
}

#[test]
fn test_82() {
    assert_eq!(Solution::subarray_sum(vec![0, 0, 0, 0, 0], 0), 15);
}

#[test]
fn test_83() {
    assert_eq!(Solution::subarray_sum(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1], 2), 0);
}

#[test]
fn test_84() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, -1, 2, -1, 1], 2), 5);
}

#[test]
fn test_85() {
    assert_eq!(Solution::subarray_sum(vec![-1, 2, -3, 4, -5, 6, -7, 8, -9], -5), 3);
}

#[test]
fn test_86() {
    assert_eq!(Solution::subarray_sum(vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10], 5), 3);
}

#[test]
fn test_87() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 30), 2);
}

#[test]
fn test_88() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5], 9), 2);
}

#[test]
fn test_89() {
    assert_eq!(Solution::subarray_sum(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 25), 6);
}

#[test]
fn test_90() {
    assert_eq!(Solution::subarray_sum(vec![-1000, 1000, -1000, 1000, -1000, 1000], 0), 9);
}

#[test]
fn test_91() {
    assert_eq!(Solution::subarray_sum(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 50), 1);
}

#[test]
fn test_92() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 1, 2, 1], 3), 4);
}

#[test]
fn test_93() {
    assert_eq!(Solution::subarray_sum(vec![3, 4, 7, 2, -3, 1, 4, 2, 0, 1], 7), 7);
}

#[test]
fn test_94() {
    assert_eq!(Solution::subarray_sum(vec![10, 2, -2, -20, 10], -10), 3);
}

#[test]
fn test_95() {
    assert_eq!(Solution::subarray_sum(vec![0, 0, 0, 0, 0, 0], 0), 21);
}

#[test]
fn test_96() {
    assert_eq!(Solution::subarray_sum(vec![1000, -1000, 1000, -1000, 1000, -1000], 0), 9);
}

#[test]
fn test_97() {
    assert_eq!(Solution::subarray_sum(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20], 30), 3);
}

#[test]
fn test_98() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5], 15), 1);
}

#[test]
fn test_99() {
    assert_eq!(Solution::subarray_sum(vec![-10, -20, -30, -40, -50, -60, -70, -80, -90, -100], -150), 3);
}

#[test]
fn test_100() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 100), 1);
}

#[test]
fn test_101() {
    assert_eq!(Solution::subarray_sum(vec![-1, 0, 1, -1, 0, 1], 0), 9);
}

#[test]
fn test_102() {
    assert_eq!(Solution::subarray_sum(vec![5, -5, 5, -5, 5, -5, 5], 5), 10);
}

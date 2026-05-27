include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9], 4, 6), vec![3, 5, 7, 9]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9], 2, 6), vec![5, 7]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9], 1, 6), vec![5]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9], 5, 6), vec![1, 3, 5, 7, 9]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 2, 3), vec![2, 3]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 1, 1], 3, 1), vec![1, 1, 1]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9], 5, 8), vec![1, 3, 5, 7, 9]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3), vec![1, 2, 3, 4]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 3, 3, 4, 5], 4, 2), vec![1, 2, 2, 2]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 2, 3, 4, 5], 4, -1), vec![1, 1, 2, 3]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 1, 2), vec![2]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 5, 3), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9], 3, 6), vec![3, 5, 7]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 3, 6), vec![3, 4, 5]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_closest_elements(vec![0, 0, 1, 2, 3, 3, 4, 7, 7, 8], 3, 5), vec![3, 3, 4]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 3, 4, 5], 3, 2), vec![2, 2, 2]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 5, 5, 5, 8, 9], 3, 4), vec![5, 5, 5]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1, 5), vec![5]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_closest_elements(vec![0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6], 7, 3), vec![2, 2, 2, 3, 3, 3, 4]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30], 7, 13), vec![6, 8, 10, 12, 14, 16, 18]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 100, 101, 102], 3, 99), vec![100, 101, 102]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_closest_elements(vec![-100, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90], 5, -90), vec![-94, -93, -92, -91, -90]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_closest_elements(vec![100, 101, 102, 103, 104, 105, 106, 107, 108, 109], 4, 90), vec![100, 101, 102, 103]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -5, -2, 0, 3, 8, 12, 16, 20], 5, -1), vec![-10, -5, -2, 0, 3]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_closest_elements(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 4, 5), vec![10, 10, 10, 10]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -5, 0, 5, 10], 3, -6), vec![-10, -5, 0]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 5), vec![3, 4, 5, 6, 7]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 7, 7, 8, 8, 9, 9, 10], 9, 4), vec![3, 3, 3, 3, 4, 4, 4, 5, 5]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4, 5), vec![3, 4, 5, 6]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5], 5, 6), vec![4, 4, 5, 5, 5]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10, 0), vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 7, 15), vec![4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 20, 10), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21], 5, 10), vec![5, 7, 9, 11, 13]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_closest_elements(vec![-100, -50, -30, -20, -10, 0, 10, 20, 30, 50, 100], 5, -25), vec![-50, -30, -20, -10, 0]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 4, 12), vec![9, 11, 13, 15]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 7, 10), vec![7, 8, 9, 10, 11, 12, 13]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_closest_elements(vec![-100, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90], 5, -99), vec![-100, -99, -98, -97, -96]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 3), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31], 5, 28), vec![23, 25, 27, 29, 31]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -8, -6, -4, -2, 0, 2, 4, 6, 8, 10], 5, 1), vec![-4, -2, 0, 2, 4]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 4, 10), vec![7, 9, 11, 13]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_closest_elements(vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2], 5, 1), vec![1, 1, 1, 1, 1]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_closest_elements(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 7, 0), vec![5, 5, 5, 5, 5, 5, 5]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1], 4, -5), vec![-7, -6, -5, -4]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3], 6, 1), vec![1, 1, 1, 1, 1, 2]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5], 5, 0), vec![1, 1, 1, 2, 2]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 6, 14), vec![9, 11, 13, 15, 17, 19]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 10, 8), vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10, 1), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 0), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5], 5, 3), vec![2, 2, 3, 3, 3]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 7, 0), vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9], 4, 1), vec![1, 1, 1, 2]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 5), vec![3, 4, 5, 6, 7]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 3, 3, 4, 5], 3, 5), vec![3, 4, 5]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 1, 1, 1, 1, 1], 4, 1), vec![1, 1, 1, 1]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0], 5, 0), vec![-4, -3, -2, -1, 0]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4, 11), vec![7, 8, 9, 10]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 2, 3, 3, 4, 4, 5, 5, 5, 6, 6, 7, 8, 9, 9, 10], 8, 5), vec![3, 4, 4, 5, 5, 5, 6, 6]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_closest_elements(vec![0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6], 15, 3), vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5, 1), vec![1, 1, 1, 1, 1]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 15), vec![6, 7, 8, 9, 10]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -5, -3, -1, 0, 1, 3, 5, 7, 10], 4, 12), vec![3, 5, 7, 10]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 0), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::find_closest_elements(vec![1, 4, 6, 8, 10, 12, 14, 16, 18, 20], 4, 15), vec![12, 14, 16, 18]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::find_closest_elements(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10, 10), vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10], 10, 5), vec![4, 4, 4, 4, 5, 5, 5, 5, 5, 6]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 6, 7], 8, 4), vec![3, 3, 3, 4, 4, 5, 5, 5]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 8, 16), vec![8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3, 0), vec![1, 2, 3]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::find_closest_elements(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 7, 5), vec![2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::find_closest_elements(vec![-100, -90, -80, -70, -60, -50, -40, -30, -20, -10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 10, 5), vec![-40, -30, -20, -10, 0, 10, 20, 30, 40, 50]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3, 15), vec![8, 9, 10]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 11), vec![6, 7, 8, 9, 10]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21], 5, 12), vec![7, 9, 11, 13, 15]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::find_closest_elements(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10, 15), vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 3, 2), vec![1, 1, 1]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4, 10), vec![7, 8, 9, 10]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 3, 4, 5, 5, 6, 7, 8, 8, 9], 4, 4), vec![3, 4, 5, 5]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -5, -3, -1, 0, 1, 2, 3, 5, 7], 3, -4), vec![-5, -3, -1]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::find_closest_elements(vec![1, 10, 100, 1000, 10000, 100000, 1000000], 2, 10000), vec![1000, 10000]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -5, -2, 0, 1, 3, 5, 8, 12], 3, -3), vec![-5, -2, 0]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 3, 3, 3, 4, 5, 6], 4, 3), vec![3, 3, 3, 3]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 3, 3, 4, 5], 3, 1), vec![1, 2, 2]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 2, 2, 3, 4, 5, 6, 7, 8], 5, 2), vec![1, 1, 1, 2, 2]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3, 11), vec![8, 9, 10]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 5, 10), vec![5, 7, 9, 11, 13]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5, 1), vec![1, 1, 1, 1, 1]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3, 0), vec![1, 2, 3]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 3, 4, 5, 5, 5, 6, 6, 7, 8, 8, 9], 6, 5), vec![4, 5, 5, 5, 6, 6]);
}

#[test]
fn test_92() {
    assert_eq!(Solution::find_closest_elements(vec![-100, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90], 5, -94), vec![-96, -95, -94, -93, -92]);
}

#[test]
fn test_93() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21], 6, 10), vec![5, 7, 9, 11, 13, 15]);
}

#[test]
fn test_94() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 2, 2, 3, 3, 4, 5, 5, 5, 6, 7, 8, 9], 5, 4), vec![3, 3, 4, 5, 5]);
}

#[test]
fn test_95() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3, 12), vec![8, 9, 10]);
}

#[test]
fn test_96() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 6, 5), vec![2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_97() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 7, 10), vec![7, 8, 9, 10, 11, 12, 13]);
}

#[test]
fn test_98() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2, 5), vec![4, 5]);
}

#[test]
fn test_99() {
    assert_eq!(Solution::find_closest_elements(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 3, 55), vec![40, 50, 60]);
}

#[test]
fn test_100() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 7, 7), vec![4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_101() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25], 5, 12), vec![7, 9, 11, 13, 15]);
}

#[test]
fn test_102() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 6, 5), vec![2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_103() {
    assert_eq!(Solution::find_closest_elements(vec![-20, -15, -10, -5, 0, 5, 10, 15, 20], 4, 0), vec![-10, -5, 0, 5]);
}

#[test]
fn test_104() {
    assert_eq!(Solution::find_closest_elements(vec![0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6], 10, 3), vec![1, 2, 2, 2, 3, 3, 3, 4, 4, 4]);
}

#[test]
fn test_105() {
    assert_eq!(Solution::find_closest_elements(vec![0, 1, 1, 1, 1, 2, 3, 4, 4, 4, 5, 5, 6, 7, 8, 9], 6, 0), vec![0, 1, 1, 1, 1, 2]);
}

#[test]
fn test_106() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 15, 10), vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]);
}

#[test]
fn test_107() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 6, 7, 8, 9, 10], 3, 5), vec![3, 6, 7]);
}

#[test]
fn test_108() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -5, -2, -1, 0, 1, 2, 3, 5, 6, 7, 10, 15], 5, -3), vec![-5, -2, -1, 0, 1]);
}

#[test]
fn test_109() {
    assert_eq!(Solution::find_closest_elements(vec![0, 0, 1, 1, 1, 1, 2, 2, 3, 3], 5, 1), vec![0, 1, 1, 1, 1]);
}

#[test]
fn test_110() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 7, 11), vec![4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_111() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3, -2), vec![1, 2, 3]);
}

#[test]
fn test_112() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 6, 9, 12, 15, 18, 21, 24], 5, 16), vec![9, 12, 15, 18, 21]);
}

#[test]
fn test_113() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -5, -3, -1, 0, 1, 3, 5, 7, 10], 5, -2), vec![-5, -3, -1, 0, 1]);
}

#[test]
fn test_114() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -5, -1, 0, 1, 5, 10], 3, -3), vec![-5, -1, 0]);
}

#[test]
fn test_115() {
    assert_eq!(Solution::find_closest_elements(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5, 2), vec![1, 1, 1, 1, 1]);
}

#[test]
fn test_116() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 3, 3, 5, 5, 5, 7, 7, 9], 4, 6), vec![5, 5, 5, 7]);
}

#[test]
fn test_117() {
    assert_eq!(Solution::find_closest_elements(vec![1, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5], 7, 4), vec![4, 4, 4, 4, 4, 4, 4]);
}

#[test]
fn test_118() {
    assert_eq!(Solution::find_closest_elements(vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60], 5, 53), vec![40, 45, 50, 55, 60]);
}

#[test]
fn test_119() {
    assert_eq!(Solution::find_closest_elements(vec![-5, -3, -2, -1, 0, 1, 2, 3, 4, 5], 6, -4), vec![-5, -3, -2, -1, 0, 1]);
}

#[test]
fn test_120() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10, 5), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_121() {
    assert_eq!(Solution::find_closest_elements(vec![0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5], 12, 2), vec![0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3]);
}

#[test]
fn test_122() {
    assert_eq!(Solution::find_closest_elements(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10, 5), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_123() {
    assert_eq!(Solution::find_closest_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 6, 15), vec![12, 13, 14, 15, 16, 17]);
}

#[test]
fn test_124() {
    assert_eq!(Solution::find_closest_elements(vec![-10, -5, 0, 3, 9, 12], 3, -7), vec![-10, -5, 0]);
}

#[test]
fn test_125() {
    assert_eq!(Solution::find_closest_elements(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5, 0), vec![0, 0, 0, 0, 0]);
}

#[test]
fn test_126() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 4, 10), vec![7, 9, 11, 13]);
}

#[test]
fn test_127() {
    assert_eq!(Solution::find_closest_elements(vec![-50, -40, -30, -20, -10, 0, 10, 20, 30, 40, 50], 6, 15), vec![-10, 0, 10, 20, 30, 40]);
}

#[test]
fn test_128() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 4, 5, 8, 10, 15, 20, 30, 50, 100], 4, 12), vec![5, 8, 10, 15]);
}

#[test]
fn test_129() {
    assert_eq!(Solution::find_closest_elements(vec![1, 3, 3, 4, 4, 4, 4, 5, 6, 8, 9], 5, 4), vec![3, 4, 4, 4, 4]);
}

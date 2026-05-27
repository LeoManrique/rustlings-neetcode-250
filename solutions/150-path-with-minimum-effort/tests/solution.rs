include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![4, 3, 4, 10, 5, 5, 9, 2], vec![10, 8, 2, 10, 9, 7, 5, 6], vec![5, 8, 1, 10, 10, 7, 7, 2], vec![5, 10, 2, 8, 3, 9, 7, 9], vec![7, 6, 6, 8, 6, 6, 4, 4], vec![4, 9, 6, 10, 2, 1, 2, 7], vec![5, 10, 3, 7, 7, 5, 10, 10]]), 4);
}

#[test]
fn test_2() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![4, 3, 4, 10, 5, 5, 9, 2], vec![10, 8, 2, 10, 9, 7, 5, 6], vec![5, 8, 10, 10, 10, 7, 4, 2], vec![5, 1, 3, 1, 1, 3, 1, 9], vec![6, 4, 10, 6, 10, 9, 4, 6]]), 5);
}

#[test]
fn test_3() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![7, 1, 4, 10, 10, 6, 9, 4, 10, 9, 7], vec![1, 2, 5, 9, 1, 5, 7, 5, 4, 10, 7], vec![6, 9, 8, 10, 9, 9, 9, 8, 10, 10, 8], vec![2, 7, 6, 8, 5, 5, 9, 9, 10, 10, 7], vec![5, 7, 6, 8, 3, 5, 6, 7, 1, 8, 6], vec![8, 8, 10, 10, 4, 3, 10, 7, 10, 10, 10], vec![9, 8, 9, 1, 2, 6, 3, 6, 8, 2, 2], vec![6, 9, 10, 9, 1, 6, 6, 6, 10, 8, 9], vec![4, 6, 10, 7, 9, 8, 3, 5, 6, 10, 2], vec![8, 1, 8, 5, 6, 5, 10, 8, 10, 10, 9]]), 6);
}

#[test]
fn test_4() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 10], vec![10, 1]]), 9);
}

#[test]
fn test_5() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 1, 1, 1], vec![1, 2, 1, 2, 1], vec![1, 2, 1, 2, 1], vec![1, 2, 1, 2, 1], vec![1, 1, 1, 2, 1]]), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 10, 6, 7, 9, 10, 4, 9]]), 9);
}

#[test]
fn test_7() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]), 1);
}

#[test]
fn test_8() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 1], vec![1, 1]]), 0);
}

#[test]
fn test_9() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 10, 6], vec![1, 3, 5], vec![4, 8, 2]]), 3);
}

#[test]
fn test_10() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]]), 2);
}

#[test]
fn test_11() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 9, 20, 8, 15], vec![9, 10, 5, 11, 9], vec![8, 7, 2, 15, 12], vec![1, 11, 1, 1, 12], vec![20, 7, 11, 10, 2]]), 8);
}

#[test]
fn test_12() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![4, 3, 4, 10, 5, 5, 9, 2], vec![10, 8, 2, 10, 9, 7, 5, 6], vec![5, 8, 1, 10, 10, 7, 7, 2], vec![10, 1, 3, 1, 1, 6, 6, 9], vec![6, 10, 6, 3, 9, 4, 9, 7], vec![1, 10, 6, 1, 2, 3, 8, 1], vec![6, 6, 9, 2, 3, 8, 1, 7], vec![10, 4, 6, 10, 4, 3, 8, 1]]), 6);
}

#[test]
fn test_13() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![5, 8, 4, 1], vec![9, 6, 6, 7], vec![1, 6, 1, 1], vec![4, 9, 9, 1]]), 5);
}

#[test]
fn test_14() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 1, 3, 4, 5], vec![1, 2, 1, 2, 2, 2], vec![2, 2, 2, 2, 2, 2], vec![1, 1, 1, 1, 1, 1], vec![3, 3, 3, 3, 3, 3]]), 2);
}

#[test]
fn test_15() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3, 4, 5], vec![4, 3, 2, 1, 6], vec![7, 8, 9, 10, 11], vec![12, 13, 14, 15, 16]]), 5);
}

#[test]
fn test_16() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![4, 3, 4, 10, 5], vec![6, 7, 3, 9, 2], vec![3, 8, 1, 3, 4], vec![10, 6, 3, 2, 20], vec![6, 1, 1, 5, 4]]), 3);
}

#[test]
fn test_17() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 6, 5, 1, 3], vec![4, 7, 1, 4, 2], vec![6, 2, 5, 3, 1], vec![4, 1, 2, 1, 2], vec![2, 6, 3, 1, 1]]), 3);
}

#[test]
fn test_18() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 100, 3, 4], vec![2, 3, 100, 4, 5], vec![3, 4, 100, 5, 6], vec![4, 5, 100, 6, 7], vec![5, 6, 100, 7, 8]]), 94);
}

#[test]
fn test_19() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![5, 6, 2, 4, 6], vec![4, 8, 3, 3, 8], vec![8, 9, 4, 7, 10], vec![10, 9, 2, 3, 8], vec![5, 1, 5, 8, 1]]), 7);
}

#[test]
fn test_20() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![4, 3, 2, 1], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]]), 4);
}

#[test]
fn test_21() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]]), 0);
}

#[test]
fn test_22() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 6, 3, 1, 2], vec![2, 2, 5, 1, 3], vec![6, 1, 1, 1, 1], vec![2, 1, 1, 2, 1], vec![3, 1, 1, 1, 1]]), 1);
}

#[test]
fn test_23() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 5, 9, 8, 11, 9, 4], vec![3, 9, 4, 8, 15, 9, 1], vec![2, 9, 2, 10, 15, 1, 6], vec![9, 10, 2, 10, 10, 2, 3], vec![10, 8, 2, 10, 7, 2, 8], vec![6, 8, 10, 10, 8, 10, 3], vec![1, 7, 5, 8, 3, 7, 1]]), 5);
}

#[test]
fn test_24() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 1, 1, 1, 1, 1], vec![1, 100000, 1, 100000, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 100000, 1, 100000, 1, 1], vec![1, 1, 1, 1, 1, 1]]), 0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![10, 1, 1, 1, 1], vec![1, 1, 1, 1, 10], vec![1, 1, 10, 1, 1], vec![1, 10, 1, 10, 1], vec![10, 1, 1, 1, 10]]), 9);
}

#[test]
fn test_26() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 6, 1, 1, 5], vec![1, 2, 9, 1, 5], vec![1, 3, 1, 8, 5], vec![1, 2, 1, 2, 5], vec![1, 2, 1, 2, 1]]), 1);
}

#[test]
fn test_27() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1000000, 999999, 999998, 999997], vec![999996, 999995, 999994, 999993], vec![999992, 999991, 999990, 999989], vec![999988, 999987, 999986, 999985]]), 4);
}

#[test]
fn test_28() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 3, 5, 7, 9, 11, 13], vec![2, 4, 6, 8, 10, 12, 14], vec![3, 5, 7, 9, 11, 13, 15], vec![4, 6, 8, 10, 12, 14, 16], vec![5, 7, 9, 11, 13, 15, 17]]), 2);
}

#[test]
fn test_29() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]), 1);
}

#[test]
fn test_30() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]), 1);
}

#[test]
fn test_31() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2], vec![2, 1, 2, 1, 2, 1, 2, 1, 2, 1]]), 1);
}

#[test]
fn test_32() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 10, 10, 10, 1], vec![10, 1, 1, 1, 1], vec![10, 10, 10, 1, 10], vec![10, 1, 1, 1, 10], vec![1, 1, 1, 10, 1]]), 9);
}

#[test]
fn test_33() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 100, 100, 100], vec![100, 100, 100, 100], vec![100, 100, 100, 100], vec![100, 100, 100, 1]]), 99);
}

#[test]
fn test_34() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![10, 15, 5], vec![20, 10, 5], vec![10, 10, 10]]), 5);
}

#[test]
fn test_35() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1], vec![1, 2, 3, 3, 3, 3, 3, 3, 2, 1], vec![1, 2, 3, 4, 4, 4, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 5, 5, 4, 3, 1], vec![1, 2, 3, 4, 5, 6, 5, 4, 3, 1], vec![1, 2, 3, 4, 5, 5, 5, 4, 3, 1], vec![1, 2, 3, 4, 4, 4, 4, 3, 2, 1], vec![1, 2, 3, 3, 3, 3, 3, 3, 2, 1], vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1]]), 0);
}

#[test]
fn test_36() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![9, 8, 7, 6, 5, 4, 3, 2, 1], vec![8, 7, 6, 5, 4, 3, 2, 1, 9], vec![7, 6, 5, 4, 3, 2, 1, 9, 8], vec![6, 5, 4, 3, 2, 1, 9, 8, 7], vec![5, 4, 3, 2, 1, 9, 8, 7, 6], vec![4, 3, 2, 1, 9, 8, 7, 6, 5], vec![3, 2, 1, 9, 8, 7, 6, 5, 4], vec![2, 1, 9, 8, 7, 6, 5, 4, 3], vec![1, 9, 8, 7, 6, 5, 4, 3, 2]]), 8);
}

#[test]
fn test_37() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 100, 1], vec![1, 100, 1], vec![1, 100, 1], vec![1, 100, 1], vec![1, 1, 1]]), 0);
}

#[test]
fn test_38() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2], vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3], vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4], vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], vec![6, 6, 6, 6, 6, 6, 6, 6, 6, 6]]), 1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![10, 20, 30, 40, 50], vec![15, 25, 35, 45, 55], vec![10, 20, 30, 40, 50], vec![5, 15, 25, 35, 45], vec![1, 11, 21, 31, 41]]), 10);
}

#[test]
fn test_40() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 6, 7, 5, 3, 1, 2, 6, 3, 10], vec![10, 1, 5, 8, 2, 10, 4, 10, 8, 7], vec![1, 9, 3, 5, 7, 4, 5, 10, 9, 8], vec![10, 8, 7, 5, 1, 10, 1, 10, 3, 7], vec![1, 10, 8, 1, 3, 8, 10, 7, 1, 10], vec![1, 5, 4, 4, 3, 2, 8, 3, 9, 9], vec![9, 6, 7, 4, 3, 2, 1, 8, 7, 6], vec![5, 2, 1, 3, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10]]), 9);
}

#[test]
fn test_41() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 6, 3, 8, 5, 4, 2, 7, 9], vec![6, 3, 8, 5, 4, 2, 7, 9, 1], vec![3, 8, 5, 4, 2, 7, 9, 1, 6], vec![8, 5, 4, 2, 7, 9, 1, 6, 3], vec![5, 4, 2, 7, 9, 1, 6, 3, 8], vec![4, 2, 7, 9, 1, 6, 3, 8, 5], vec![2, 7, 9, 1, 6, 3, 8, 5, 4], vec![7, 9, 1, 6, 3, 8, 5, 4, 2], vec![9, 1, 6, 3, 8, 5, 4, 2, 7]]), 8);
}

#[test]
fn test_42() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![5, 6, 7, 8, 9], vec![4, 3, 2, 1, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]]), 5);
}

#[test]
fn test_43() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![6, 5, 4], vec![7, 8, 9], vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18]]), 3);
}

#[test]
fn test_44() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![4, 3, 4, 10, 5, 5, 9, 2], vec![10, 8, 2, 10, 9, 7, 5, 6], vec![5, 5, 1, 3, 1, 3, 7, 6], vec![8, 1, 2, 1, 6, 8, 5, 1], vec![2, 1, 1, 8, 1, 5, 6, 1], vec![1, 1, 8, 1, 5, 9, 1, 4], vec![3, 1, 2, 5, 4, 8, 3, 1]]), 4);
}

#[test]
fn test_45() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]]), 5);
}

#[test]
fn test_46() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 9, 9, 1], vec![9, 0, 0, 9], vec![9, 0, 0, 9], vec![1, 9, 9, 1]]), 8);
}

#[test]
fn test_47() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![8, 5, 2], vec![6, 7, 3], vec![5, 1, 4], vec![6, 9, 8], vec![9, 3, 7]]), 3);
}

#[test]
fn test_48() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![7, 9, 6, 9, 6], vec![6, 6, 8, 6, 6], vec![6, 9, 7, 9, 6], vec![9, 6, 9, 9, 6], vec![6, 9, 8, 9, 6], vec![6, 6, 9, 6, 6]]), 2);
}

#[test]
fn test_49() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![5, 3, 1], vec![6, 4, 2], vec![7, 5, 3], vec![8, 6, 4], vec![9, 7, 5], vec![10, 8, 6]]), 2);
}

#[test]
fn test_50() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![20, 21, 22, 23, 24, 25], vec![19, 18, 17, 16, 15, 14], vec![13, 12, 11, 10, 9, 8], vec![7, 6, 5, 4, 3, 2], vec![1, 2, 3, 4, 5, 6]]), 6);
}

#[test]
fn test_51() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]]), 0);
}

#[test]
fn test_52() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3, 4, 5], vec![10, 9, 8, 7, 6], vec![11, 12, 13, 14, 15], vec![20, 19, 18, 17, 16], vec![21, 22, 23, 24, 25]]), 1);
}

#[test]
fn test_53() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2], vec![2, 1, 2, 1, 2, 1, 2, 1, 2, 1], vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2], vec![2, 1, 2, 1, 2, 1, 2, 1, 2, 1], vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]]), 1);
}

#[test]
fn test_54() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 10, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 10, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 10, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 10, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]]), 0);
}

#[test]
fn test_55() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![10, 8, 9, 10, 12], vec![13, 14, 15, 16, 17], vec![18, 19, 20, 21, 22], vec![23, 24, 25, 26, 27], vec![28, 29, 30, 31, 32]]), 5);
}

#[test]
fn test_56() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![5, 8, 1, 3, 5, 6], vec![3, 5, 4, 5, 6, 6], vec![1, 3, 5, 8, 2, 2], vec![6, 6, 3, 3, 2, 6], vec![8, 5, 1, 1, 2, 5], vec![9, 5, 3, 3, 4, 1]]), 3);
}

#[test]
fn test_57() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![10, 18, 16, 19], vec![6, 13, 8, 15], vec![7, 19, 18, 17], vec![4, 9, 9, 18], vec![3, 5, 12, 9]]), 4);
}

#[test]
fn test_58() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10], vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10], vec![1, 3, 5, 7, 9]]), 2);
}

#[test]
fn test_59() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 3, 1, 3, 1, 3], vec![2, 1, 2, 1, 2, 1], vec![3, 2, 3, 2, 3, 2], vec![1, 3, 1, 3, 1, 3], vec![2, 1, 2, 1, 2, 1]]), 2);
}

#[test]
fn test_60() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7], vec![7, 1, 7, 7, 7, 7, 7, 7, 7, 7], vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7], vec![7, 7, 7, 1, 7, 7, 7, 7, 7, 7], vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7], vec![7, 7, 7, 7, 7, 1, 7, 7, 7, 7], vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7], vec![7, 7, 7, 7, 7, 7, 7, 1, 7, 7], vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7], vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 1]]), 6);
}

#[test]
fn test_61() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![10, 11, 12, 13, 14], vec![9, 8, 7, 6, 5], vec![4, 3, 2, 1, 100], vec![101, 102, 103, 104, 105], vec![106, 107, 108, 109, 110]]), 95);
}

#[test]
fn test_62() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![5, 3, 1, 1, 1], vec![4, 2, 1, 1, 1], vec![4, 2, 1, 1, 1], vec![4, 2, 1, 1, 1], vec![4, 3, 2, 2, 1]]), 1);
}

#[test]
fn test_63() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 2, 1, 2, 1, 2, 1], vec![2, 1, 1, 2, 1, 2, 1, 2], vec![1, 2, 2, 1, 2, 1, 2, 1], vec![2, 1, 1, 2, 1, 2, 1, 2], vec![1, 2, 2, 1, 2, 1, 2, 1]]), 1);
}

#[test]
fn test_64() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18], vec![19, 20, 21], vec![22, 23, 24], vec![25, 26, 27], vec![28, 29, 30]]), 3);
}

#[test]
fn test_65() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3, 4, 5], vec![6, 5, 4, 3, 2], vec![3, 4, 5, 6, 7], vec![8, 7, 6, 5, 4], vec![9, 10, 11, 12, 13]]), 1);
}

#[test]
fn test_66() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 5, 5, 1], vec![5, 1, 5, 5], vec![1, 5, 1, 5], vec![5, 1, 5, 1]]), 4);
}

#[test]
fn test_67() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 6, 3, 7, 2, 8], vec![4, 5, 2, 6, 8, 5], vec![3, 9, 4, 10, 12, 1], vec![9, 3, 8, 11, 14, 6], vec![12, 11, 9, 13, 15, 7]]), 5);
}

#[test]
fn test_68() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 9, 3, 1, 2], vec![9, 9, 9, 1, 1], vec![1, 9, 9, 9, 1], vec![1, 1, 1, 1, 1], vec![2, 1, 1, 1, 3]]), 8);
}

#[test]
fn test_69() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![5, 5, 5, 5, 5], vec![5, 4, 4, 4, 5], vec![5, 4, 3, 4, 5], vec![5, 4, 4, 4, 5], vec![5, 5, 5, 5, 5]]), 0);
}

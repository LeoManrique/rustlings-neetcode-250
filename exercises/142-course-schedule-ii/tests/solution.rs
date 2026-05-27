include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1]]), vec![0, 1, 2, 3]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_order(7, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2]]), vec![0, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_order(5, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 3]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_order(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![2, 1]]), vec![0, 4, 5, 1, 2, 3]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_order(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]]), vec![]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_order(3, vec![vec![1, 0], vec![1, 2], vec![0, 1]]), vec![]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_order(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), vec![2, 1, 0]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_order(3, vec![vec![1, 0], vec![0, 1]]), vec![]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_order(5, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]), vec![0, 4, 1, 2, 3]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_order(8, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 2], vec![5, 3], vec![6, 3], vec![7, 4], vec![7, 5]]), vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_order(5, vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]), vec![0, 4, 1, 2, 3]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]), vec![0, 1, 2, 3]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_order(3, vec![vec![1, 0], vec![2, 1]]), vec![0, 1, 2]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_order(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 3], vec![5, 4]]), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_order(1, vec![]), vec![0]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_order(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![5, 4]]), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_order(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![5, 3]]), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_order(9, vec![vec![1, 0], vec![2, 1], vec![2, 0], vec![3, 2], vec![4, 2], vec![5, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 6]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![10, 4], vec![10, 5], vec![11, 6], vec![11, 7], vec![11, 8], vec![11, 9]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_order(11, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 1], vec![6, 1], vec![7, 2], vec![8, 2], vec![9, 3], vec![10, 3]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_order(11, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![9, 5], vec![10, 9]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_order(9, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 2], vec![6, 3], vec![7, 4], vec![7, 5], vec![8, 6], vec![8, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_order(20, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![11, 4], vec![12, 5], vec![13, 5], vec![14, 6], vec![15, 6], vec![16, 7], vec![17, 7], vec![18, 8], vec![19, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_order(20, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![10, 5], vec![11, 6], vec![11, 7], vec![12, 8], vec![12, 9], vec![13, 10], vec![13, 11], vec![14, 12], vec![14, 13], vec![15, 14], vec![16, 14], vec![17, 15], vec![17, 16], vec![18, 17], vec![19, 18]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![4, 2], vec![5, 2], vec![6, 3], vec![7, 3], vec![8, 4], vec![8, 5], vec![9, 6], vec![9, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_order(20, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![7, 5], vec![8, 6], vec![9, 7], vec![10, 8], vec![11, 9], vec![12, 10], vec![13, 11], vec![14, 12], vec![15, 13], vec![16, 14], vec![17, 15], vec![18, 16], vec![19, 17], vec![18, 19]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19, 18]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_order(11, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![8, 4], vec![9, 5], vec![9, 6], vec![10, 7], vec![10, 8], vec![10, 9]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_order(9, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![5, 1], vec![5, 2], vec![5, 3], vec![6, 1], vec![6, 2], vec![6, 3], vec![7, 4], vec![7, 5], vec![7, 6], vec![8, 4], vec![8, 5], vec![8, 6]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![4, 2], vec![4, 3], vec![5, 4], vec![6, 4], vec![7, 5], vec![8, 6], vec![9, 7], vec![9, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_order(9, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![0, 8]]), vec![]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![10, 4], vec![11, 5], vec![12, 5], vec![13, 6], vec![13, 7], vec![14, 8], vec![14, 9], vec![14, 10], vec![14, 11]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_order(14, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 1], vec![6, 2], vec![7, 3], vec![8, 4], vec![9, 5], vec![10, 6], vec![11, 7], vec![12, 8], vec![13, 9], vec![13, 10], vec![13, 11], vec![13, 12]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![10, 9], vec![11, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_order(20, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 1], vec![4, 3], vec![5, 2], vec![5, 3], vec![6, 3], vec![6, 4], vec![7, 3], vec![7, 4], vec![7, 5], vec![8, 4], vec![8, 5], vec![8, 6], vec![9, 4], vec![9, 5], vec![9, 6], vec![9, 7], vec![10, 4], vec![10, 5], vec![10, 6], vec![10, 7], vec![10, 8], vec![11, 4], vec![11, 5], vec![11, 6], vec![11, 7], vec![11, 8], vec![11, 9], vec![12, 5], vec![12, 6], vec![12, 7], vec![12, 8], vec![12, 9], vec![12, 10], vec![12, 11], vec![13, 6], vec![13, 7], vec![13, 8], vec![13, 9], vec![13, 10], vec![13, 11], vec![13, 12], vec![14, 7], vec![14, 8], vec![14, 9], vec![14, 10], vec![14, 11], vec![14, 12], vec![14, 13], vec![15, 8], vec![15, 9], vec![15, 10], vec![15, 11], vec![15, 12], vec![15, 13], vec![15, 14], vec![16, 9], vec![16, 10], vec![16, 11], vec![16, 12], vec![16, 13], vec![16, 14], vec![16, 15], vec![17, 10], vec![17, 11], vec![17, 12], vec![17, 13], vec![17, 14], vec![17, 15], vec![17, 16], vec![18, 11], vec![18, 12], vec![18, 13], vec![18, 14], vec![18, 15], vec![18, 16], vec![18, 17], vec![19, 12], vec![19, 13], vec![19, 14], vec![19, 15], vec![19, 16], vec![19, 17], vec![19, 18]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![0, 9]]), vec![]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 5], vec![8, 5], vec![9, 7], vec![9, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![11, 4], vec![12, 5], vec![13, 6], vec![13, 7], vec![14, 8], vec![14, 9], vec![14, 10], vec![14, 11], vec![14, 12], vec![14, 13]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_order(6, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![5, 4]]), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![10, 5], vec![11, 6], vec![10, 11]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 10]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_order(20, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![10, 4], vec![11, 5], vec![12, 5], vec![13, 6], vec![14, 6], vec![15, 7], vec![16, 7], vec![17, 8], vec![18, 8], vec![19, 9], vec![19, 10], vec![19, 11], vec![19, 12], vec![19, 13], vec![19, 14], vec![19, 15], vec![19, 16], vec![19, 17], vec![19, 18]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![10, 9], vec![11, 10], vec![12, 11], vec![13, 12], vec![14, 13]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![4, 2], vec![4, 3], vec![5, 4], vec![6, 4], vec![7, 5], vec![7, 6], vec![8, 5], vec![8, 6], vec![8, 7], vec![9, 7], vec![10, 7], vec![10, 8], vec![11, 7], vec![11, 8], vec![11, 9], vec![11, 10], vec![12, 7], vec![12, 8], vec![12, 9], vec![12, 10], vec![12, 11], vec![13, 9], vec![13, 10], vec![13, 11], vec![13, 12], vec![14, 11], vec![14, 12], vec![14, 13]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_order(6, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4]]), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 1], vec![6, 2], vec![7, 3], vec![7, 4], vec![8, 5], vec![8, 6], vec![9, 7], vec![9, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![10, 9], vec![11, 10], vec![0, 11]]), vec![]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_order(20, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![11, 4], vec![12, 5], vec![13, 5], vec![14, 6], vec![15, 6], vec![16, 7], vec![17, 7], vec![18, 8], vec![19, 8], vec![19, 16]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![10, 5], vec![10, 6], vec![10, 7], vec![10, 8], vec![10, 9], vec![11, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_order(8, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![5, 1], vec![5, 2], vec![5, 3], vec![6, 4], vec![6, 5], vec![7, 6]]), vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![1, 2], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 3], vec![5, 3], vec![6, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 7], vec![9, 8]]), vec![0, 2, 1, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_order(18, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![5, 1], vec![5, 3], vec![6, 2], vec![6, 4], vec![7, 3], vec![7, 4], vec![8, 5], vec![8, 6], vec![9, 5], vec![9, 6], vec![10, 7], vec![10, 8], vec![11, 7], vec![11, 8], vec![12, 9], vec![12, 10], vec![13, 10], vec![13, 11], vec![14, 12], vec![14, 13], vec![15, 13], vec![16, 14], vec![16, 15], vec![17, 15], vec![17, 16]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_order(5, vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![3, 2], vec![4, 2], vec![4, 3]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_order(8, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![7, 4], vec![7, 5], vec![7, 6]]), vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 4], vec![9, 5], vec![9, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 3], vec![6, 3], vec![7, 4], vec![8, 4], vec![9, 7], vec![10, 7], vec![11, 8], vec![11, 9]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 4], vec![9, 6], vec![9, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![4, 2], vec![5, 2], vec![6, 3], vec![7, 3], vec![8, 4], vec![9, 4], vec![10, 5], vec![11, 5], vec![12, 6], vec![13, 7], vec![14, 7], vec![13, 14]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 13]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![10, 5], vec![11, 6], vec![11, 7], vec![11, 8], vec![11, 9], vec![11, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![9, 4], vec![9, 5], vec![9, 6], vec![9, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![5, 3], vec![6, 3], vec![7, 4], vec![8, 5], vec![9, 5], vec![10, 6], vec![11, 6], vec![12, 7], vec![13, 8], vec![14, 9], vec![14, 10], vec![14, 11], vec![14, 12], vec![14, 13]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![10, 5], vec![11, 6], vec![11, 7], vec![10, 8], vec![10, 9], vec![10, 11]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 10]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![9, 5], vec![10, 6], vec![10, 7], vec![11, 8], vec![11, 9], vec![11, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 2], vec![6, 3], vec![7, 4], vec![8, 5], vec![9, 6], vec![7, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 8, 9, 7]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_order(8, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![5, 1], vec![5, 2], vec![5, 3], vec![6, 4], vec![7, 5], vec![7, 6]]), vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_order(20, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![11, 4], vec![12, 5], vec![13, 5], vec![14, 6], vec![15, 6], vec![16, 7], vec![17, 7], vec![18, 8], vec![19, 8], vec![19, 9], vec![18, 10], vec![17, 11], vec![16, 12], vec![15, 13], vec![14, 14], vec![13, 15], vec![12, 16], vec![11, 17], vec![10, 18], vec![9, 19], vec![8, 19]]), vec![]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![5, 9]]), vec![]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![9, 5], vec![10, 6], vec![10, 7], vec![11, 8], vec![11, 9], vec![12, 10], vec![13, 10], vec![14, 11], vec![14, 12], vec![14, 13]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![5, 6], vec![7, 8], vec![9, 7]]), vec![0, 1, 2, 3, 4, 6, 8, 5, 7, 9]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![11, 4], vec![12, 5], vec![13, 5], vec![14, 6], vec![14, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![2, 1], vec![3, 0], vec![3, 1], vec![3, 2], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4], vec![6, 5], vec![7, 0], vec![7, 1], vec![7, 2], vec![7, 3], vec![7, 4], vec![7, 5], vec![7, 6], vec![8, 0], vec![8, 1], vec![8, 2], vec![8, 3], vec![8, 4], vec![8, 5], vec![8, 6], vec![8, 7], vec![9, 0], vec![9, 1], vec![9, 2], vec![9, 3], vec![9, 4], vec![9, 5], vec![9, 6], vec![9, 7], vec![9, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::find_order(8, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![7, 5]]), vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::find_order(13, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![7, 5], vec![8, 6], vec![9, 7], vec![10, 8], vec![11, 9], vec![12, 10], vec![12, 11]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::find_order(9, vec![vec![1, 0], vec![2, 1], vec![2, 0], vec![3, 2], vec![3, 1], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::find_order(12, vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![4, 2], vec![5, 2], vec![6, 3], vec![7, 3], vec![8, 6], vec![9, 7], vec![10, 8], vec![10, 9], vec![11, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::find_order(6, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4]]), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![5, 1], vec![5, 2], vec![5, 3], vec![6, 4], vec![7, 4], vec![8, 5], vec![9, 5], vec![10, 5], vec![11, 6], vec![12, 7], vec![13, 8], vec![14, 9], vec![14, 10], vec![14, 11], vec![14, 12], vec![14, 13]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::find_order(9, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![8, 3]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::find_order(20, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 1], vec![6, 1], vec![7, 1], vec![8, 2], vec![9, 2], vec![10, 2], vec![11, 3], vec![12, 3], vec![13, 4], vec![14, 4], vec![15, 5], vec![16, 5], vec![17, 6], vec![18, 6], vec![19, 7], vec![19, 8], vec![19, 9], vec![19, 10], vec![19, 11], vec![19, 12], vec![19, 13], vec![19, 14], vec![19, 15], vec![19, 16], vec![19, 17], vec![19, 18]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::find_order(9, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 2], vec![6, 3], vec![7, 3], vec![8, 4], vec![8, 5], vec![8, 6], vec![8, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::find_order(13, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![9, 5], vec![10, 6], vec![10, 7], vec![11, 8], vec![11, 9], vec![12, 10], vec![12, 11]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 3], vec![9, 4], vec![10, 4], vec![11, 5], vec![12, 5], vec![13, 6], vec![14, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![4, 2], vec![5, 2], vec![6, 3], vec![7, 3], vec![8, 4], vec![8, 5], vec![9, 6], vec![9, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::find_order(15, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![10, 4], vec![11, 4], vec![12, 5], vec![13, 5], vec![14, 6], vec![13, 7], vec![12, 8], vec![11, 9], vec![10, 10], vec![9, 11], vec![8, 12], vec![7, 13], vec![6, 14]]), vec![]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 1], vec![5, 2], vec![6, 3], vec![6, 4], vec![7, 5], vec![8, 6], vec![9, 7], vec![9, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 2], vec![5, 3], vec![6, 3], vec![7, 4], vec![7, 5], vec![8, 6], vec![9, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::find_order(11, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4], vec![6, 5], vec![7, 6], vec![8, 7], vec![9, 8], vec![10, 9]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::find_order(16, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 1], vec![5, 2], vec![6, 1], vec![6, 3], vec![7, 1], vec![7, 4], vec![8, 2], vec![8, 3], vec![8, 4], vec![9, 2], vec![9, 3], vec![9, 4], vec![10, 3], vec![10, 4], vec![11, 4], vec![12, 5], vec![12, 6], vec![12, 7], vec![12, 8], vec![13, 5], vec![13, 6], vec![13, 7], vec![13, 8], vec![13, 9], vec![14, 6], vec![14, 7], vec![14, 8], vec![14, 9], vec![14, 10], vec![14, 11], vec![15, 7], vec![15, 8], vec![15, 9], vec![15, 10], vec![15, 11], vec![15, 12], vec![15, 13], vec![15, 14]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 3], vec![5, 3], vec![6, 4], vec![6, 5], vec![7, 6], vec![8, 6], vec![9, 7], vec![9, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::find_order(9, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 3], vec![8, 4], vec![8, 5], vec![8, 6], vec![8, 7]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 1], vec![5, 1], vec![6, 2], vec![7, 2], vec![8, 3], vec![9, 3], vec![4, 6], vec![5, 7], vec![8, 4], vec![8, 5], vec![9, 8]]), vec![0, 1, 2, 3, 6, 7, 4, 5, 8, 9]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::find_order(10, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![6, 2], vec![7, 3], vec![8, 4], vec![9, 5], vec![9, 6]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

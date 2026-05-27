include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::majority_element(vec![1, 1, 2, 2, 2, 2, 2]), 2);
}

#[test]
fn test_2() {
    assert_eq!(Solution::majority_element(vec![1, 1, 1, 1, 2, 2, 3]), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::majority_element(vec![-1, 100, -1, -1, -1, -1, 1, 1, 1, 1]), -1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::majority_element(vec![1, 2, 3, 4, 5, 4, 4, 4, 4]), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::majority_element(vec![4, 4, 4, 4, 2, 2, 3]), 4);
}

#[test]
fn test_7() {
    assert_eq!(Solution::majority_element(vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1, 1, 1, 1, 1]), 1000000000);
}

#[test]
fn test_8() {
    assert_eq!(Solution::majority_element(vec![5, 5, 4, 5, 5, 6, 5, 5]), 5);
}

#[test]
fn test_9() {
    assert_eq!(Solution::majority_element(vec![1, 2, 3, 4, 5, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::majority_element(vec![1000000000, -1000000000, 1000000000, 1000000000, 1000000000]), 1000000000);
}

#[test]
fn test_11() {
    assert_eq!(Solution::majority_element(vec![1, 1, 1, 1, 2, 2, 3, 3, 3, 3, 3]), 3);
}

#[test]
fn test_12() {
    assert_eq!(Solution::majority_element(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 5);
}

#[test]
fn test_13() {
    assert_eq!(Solution::majority_element(vec![7, 7, 7, 7, 7, 7, 7, 7, 0, 0, 0, 0, 0]), 7);
}

#[test]
fn test_14() {
    assert_eq!(Solution::majority_element(vec![-1000000000, -1000000000, -1000000000, -1000000000, -1000000000, -1000000000, 1, 1, 1, 1, 1]), -1000000000);
}

#[test]
fn test_15() {
    assert_eq!(Solution::majority_element(vec![1, 1, 2, 2, 1, 1, 1]), 1);
}

#[test]
fn test_16() {
    assert_eq!(Solution::majority_element(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8]), 8);
}

#[test]
fn test_17() {
    assert_eq!(Solution::majority_element(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_18() {
    assert_eq!(Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_19() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
}

#[test]
fn test_20() {
    assert_eq!(Solution::majority_element(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_21() {
    assert_eq!(Solution::majority_element(vec![4, 4, 4, 4, 4, 4, 5, 5, 5, 5]), 4);
}

#[test]
fn test_22() {
    assert_eq!(Solution::majority_element(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8]), 7);
}

#[test]
fn test_23() {
    assert_eq!(Solution::majority_element(vec![4, 4, 4, 4, 1, 2, 3]), 4);
}

#[test]
fn test_24() {
    assert_eq!(Solution::majority_element(vec![100, 200, 100, 300, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]), 100);
}

#[test]
fn test_25() {
    assert_eq!(Solution::majority_element(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]), 9);
}

#[test]
fn test_26() {
    assert_eq!(Solution::majority_element(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_27() {
    assert_eq!(Solution::majority_element(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 0);
}

#[test]
fn test_28() {
    assert_eq!(Solution::majority_element(vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1, 2, 3]), 1000000000);
}

#[test]
fn test_29() {
    assert_eq!(Solution::majority_element(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 1, 1, 1, 1, 1, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]), 7);
}

#[test]
fn test_30() {
    assert_eq!(Solution::majority_element(vec![1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10, 1, 1]), 1);
}

#[test]
fn test_31() {
    assert_eq!(Solution::majority_element(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_32() {
    assert_eq!(Solution::majority_element(vec![-1, -1, -1, -1, -1, 1, 2, 3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]), -1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::majority_element(vec![-1, 100, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]), -1);
}

#[test]
fn test_35() {
    assert_eq!(Solution::majority_element(vec![1, 2, 3, 4, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_36() {
    assert_eq!(Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_37() {
    assert_eq!(Solution::majority_element(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_38() {
    assert_eq!(Solution::majority_element(vec![1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10, 1, 11, 1, 12, 1, 13, 1, 14, 1, 15, 1, 16, 1, 17, 1, 18, 1, 19, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_40() {
    assert_eq!(Solution::majority_element(vec![1000000000, 1000000000, 1000000000, 500000000, 500000000, 500000000, 1000000000, 1000000000, 1000000000, 1000000000]), 1000000000);
}

#[test]
fn test_41() {
    assert_eq!(Solution::majority_element(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 20, 20, 20, 20, 20, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_42() {
    assert_eq!(Solution::majority_element(vec![5, 4, 3, 2, 1, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 5);
}

#[test]
fn test_43() {
    assert_eq!(Solution::majority_element(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 8, 7]), 7);
}

#[test]
fn test_44() {
    assert_eq!(Solution::majority_element(vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3]), 2);
}

#[test]
fn test_45() {
    assert_eq!(Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

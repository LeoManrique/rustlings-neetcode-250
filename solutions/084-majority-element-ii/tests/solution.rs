include!("../src/lib.rs");

#[test]
fn test_1() {
    let mut a = Solution::majority_element(vec![3, 2, 3]);
    a.sort();
    let mut b: Vec<i32> = vec![3];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_2() {
    let mut a = Solution::majority_element(vec![1]);
    a.sort();
    let mut b: Vec<i32> = vec![1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_3() {
    let mut a = Solution::majority_element(vec![1, 2]);
    a.sort();
    let mut b: Vec<i32> = vec![2, 1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_4() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_5() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 1, 1]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_6() {
    let mut a = Solution::majority_element(vec![3, 3, 4, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
    a.sort();
    let mut b: Vec<i32> = vec![3];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_7() {
    let mut a = Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]);
    a.sort();
    let mut b: Vec<i32> = vec![2, 1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_8() {
    let mut a = Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
    a.sort();
    let mut b: Vec<i32> = vec![2, 1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_9() {
    let mut a = Solution::majority_element(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_10() {
    let mut a = Solution::majority_element(vec![10, 10, 10, 10, 10, 10, 10, 2, 3, 4, 5, 6, 7, 8, 9]);
    a.sort();
    let mut b: Vec<i32> = vec![10];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_11() {
    let mut a = Solution::majority_element(vec![2, 2, 1, 3]);
    a.sort();
    let mut b: Vec<i32> = vec![2];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_12() {
    let mut a = Solution::majority_element(vec![4, 1, 2, 3, 4, 2, 3, 4, 5, 6, 7, 8, 9, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]);
    a.sort();
    let mut b: Vec<i32> = vec![4];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_13() {
    let mut a = Solution::majority_element(vec![-1, -2, -3, -1, -1, -1, 0]);
    a.sort();
    let mut b: Vec<i32> = vec![-1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_14() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1]);
    a.sort();
    let mut b: Vec<i32> = vec![1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_15() {
    let mut a = Solution::majority_element(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    a.sort();
    let mut b: Vec<i32> = vec![5, 1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_16() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_17() {
    let mut a = Solution::majority_element(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
    a.sort();
    let mut b: Vec<i32> = vec![2, 1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_18() {
    let mut a = Solution::majority_element(vec![1000000000, -1000000000, 1000000000, -1000000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    a.sort();
    let mut b: Vec<i32> = vec![0];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_19() {
    let mut a = Solution::majority_element(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]);
    a.sort();
    let mut b: Vec<i32> = vec![7];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_20() {
    let mut a = Solution::majority_element(vec![1000000000, -1000000000, 1000000000, -1000000000, 1000000000, 1000000000, 1000000000, 1000000000]);
    a.sort();
    let mut b: Vec<i32> = vec![1000000000];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_21() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_22() {
    let mut a = Solution::majority_element(vec![10, 20, 10, 10, 30, 20, 10, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20]);
    a.sort();
    let mut b: Vec<i32> = vec![20];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_23() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_24() {
    let mut a = Solution::majority_element(vec![10, 20, 10, 10, 30, 10, 40, 10, 50, 10, 60, 10, 70, 10, 80, 10, 90, 10, 10, 10, 10, 10]);
    a.sort();
    let mut b: Vec<i32> = vec![10];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_25() {
    let mut a = Solution::majority_element(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_26() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 1, 1, 1, 1, 1, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_27() {
    let mut a = Solution::majority_element(vec![1000000000, -1000000000, 1000000000, -1000000000, 1000000000, -1000000000, 1000000000, -1000000000, 1000000000, -1000000000, 1000000000, -1000000000, 1000000000]);
    a.sort();
    let mut b: Vec<i32> = vec![1000000000, -1000000000];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_28() {
    let mut a = Solution::majority_element(vec![1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]);
    a.sort();
    let mut b: Vec<i32> = vec![7];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_29() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    a.sort();
    let mut b: Vec<i32> = vec![1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_30() {
    let mut a = Solution::majority_element(vec![1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_31() {
    let mut a = Solution::majority_element(vec![1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6]);
    a.sort();
    let mut b: Vec<i32> = vec![6];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_32() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_33() {
    let mut a = Solution::majority_element(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_34() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_35() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_36() {
    let mut a = Solution::majority_element(vec![10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30, 10, 20, 30]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_37() {
    let mut a = Solution::majority_element(vec![10, 10, 20, 20, 30, 30, 40, 40, 50, 50, 60, 60, 70, 70, 80, 80, 90, 90, 10, 20, 30, 40, 50, 60, 70, 80, 90, 10, 20, 30, 40, 50, 60, 70, 80, 90, 10, 20, 30, 40, 50, 60, 70, 80, 90]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_38() {
    let mut a = Solution::majority_element(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
    a.sort();
    let mut b: Vec<i32> = vec![9];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_39() {
    let mut a = Solution::majority_element(vec![-1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3, -1, -2, -3]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_40() {
    let mut a = Solution::majority_element(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_41() {
    let mut a = Solution::majority_element(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_42() {
    let mut a = Solution::majority_element(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_43() {
    let mut a = Solution::majority_element(vec![42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42]);
    a.sort();
    let mut b: Vec<i32> = vec![42];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_44() {
    let mut a = Solution::majority_element(vec![100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200]);
    a.sort();
    let mut b: Vec<i32> = vec![100, 200];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_45() {
    let mut a = Solution::majority_element(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]);
    a.sort();
    let mut b: Vec<i32> = vec![7];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_46() {
    let mut a = Solution::majority_element(vec![10, 20, 10, 10, 30, 10, 40, 10, 10, 50, 60, 10, 10, 70, 10, 10, 80, 10, 10, 90, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]);
    a.sort();
    let mut b: Vec<i32> = vec![10];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_47() {
    let mut a = Solution::majority_element(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![5];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_48() {
    let mut a = Solution::majority_element(vec![100, 200, 300, 100, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200, 100, 200]);
    a.sort();
    let mut b: Vec<i32> = vec![100, 200];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_49() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_50() {
    let mut a = Solution::majority_element(vec![1000000000, 1000000000, 1000000000, 1000000000, -1000000000, -1000000000, -1000000000, -1000000000]);
    a.sort();
    let mut b: Vec<i32> = vec![1000000000, -1000000000];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_51() {
    let mut a = Solution::majority_element(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    a.sort();
    let mut b: Vec<i32> = vec![0];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_52() {
    let mut a = Solution::majority_element(vec![9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3, 9, 3]);
    a.sort();
    let mut b: Vec<i32> = vec![9, 3];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_53() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_54() {
    let mut a = Solution::majority_element(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![5];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_55() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_56() {
    let mut a = Solution::majority_element(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]);
    a.sort();
    let mut b: Vec<i32> = vec![7];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_57() {
    let mut a = Solution::majority_element(vec![-1, -2, -3, -4, -1, -2, -3, -4, -1, -2, -3, -4, -1, -2, -3, -4, -1, -2, -3, -4, -1, -2, -3, -4, -1, -2, -3, -4, -1, -1, -1, -1]);
    a.sort();
    let mut b: Vec<i32> = vec![-1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_58() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2]);
    a.sort();
    let mut b: Vec<i32> = vec![2, 1];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_59() {
    let mut a = Solution::majority_element(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![5];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_60() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_61() {
    let mut a = Solution::majority_element(vec![10, 10, 20, 20, 30, 30, 10, 10, 20, 20, 30, 30, 10, 10, 20, 20, 30, 30, 10, 10, 20, 20, 30, 30, 10, 10, 20, 20, 30]);
    a.sort();
    let mut b: Vec<i32> = vec![10, 20];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_62() {
    let mut a = Solution::majority_element(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_63() {
    let mut a = Solution::majority_element(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_64() {
    let mut a = Solution::majority_element(vec![5, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![5];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_65() {
    let mut a = Solution::majority_element(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_66() {
    let mut a = Solution::majority_element(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]);
    a.sort();
    let mut b: Vec<i32> = vec![7];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_67() {
    let mut a = Solution::majority_element(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26, 27, 27, 28, 28, 29, 29, 30, 30]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_68() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_69() {
    let mut a = Solution::majority_element(vec![999999999, -999999999, 0, 999999999, -999999999, 0, 999999999, -999999999, 0, 999999999, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    a.sort();
    let mut b: Vec<i32> = vec![0];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_70() {
    let mut a = Solution::majority_element(vec![-1, -1, -1, -1, -1, -2, -2, -3, -3, -3, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4]);
    a.sort();
    let mut b: Vec<i32> = vec![-4];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_71() {
    let mut a = Solution::majority_element(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![5];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_72() {
    let mut a = Solution::majority_element(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_73() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_74() {
    let mut a = Solution::majority_element(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
    a.sort();
    let mut b: Vec<i32> = vec![2];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_75() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_76() {
    let mut a = Solution::majority_element(vec![1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_77() {
    let mut a = Solution::majority_element(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    a.sort();
    let mut b: Vec<i32> = vec![];
    b.sort();
    assert_eq!(a, b);
}

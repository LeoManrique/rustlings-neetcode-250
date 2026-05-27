include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5], vec![2, 2, 2, 2, 2], vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5]]), vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 2, 3, 5], vec![3, 2, 3, 4, 4], vec![2, 4, 5, 3, 1], vec![6, 7, 1, 4, 5], vec![5, 1, 1, 2, 4]]), vec![vec![0, 4], vec![1, 3], vec![1, 4], vec![2, 2], vec![3, 0], vec![3, 1], vec![4, 0]]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![10, 10, 10], vec![10, 1, 10], vec![10, 10, 10]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![1, 0], vec![1, 2], vec![2, 0], vec![2, 1], vec![2, 2]]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1]]), vec![vec![0, 0]]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![3, 3, 3, 3, 3], vec![3, 2, 2, 2, 3], vec![3, 2, 1, 2, 3], vec![3, 2, 2, 2, 3], vec![3, 3, 3, 3, 3]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 0], vec![1, 4], vec![2, 0], vec![2, 4], vec![3, 0], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2], vec![2, 3]]), vec![vec![0, 1], vec![1, 0], vec![1, 1]]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5], vec![16, 17, 18, 19, 6], vec![15, 24, 25, 20, 7], vec![14, 23, 22, 21, 8], vec![13, 12, 11, 10, 9]]), vec![vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]), vec![vec![0, 2], vec![1, 0], vec![1, 1], vec![1, 2], vec![2, 0], vec![2, 1], vec![2, 2]]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 1, 1, 1, 1, 1, 1], vec![1, 2, 2, 2, 2, 2, 1], vec![1, 2, 3, 3, 3, 2, 1], vec![1, 2, 3, 4, 3, 2, 1], vec![1, 2, 3, 3, 3, 2, 1], vec![1, 2, 2, 2, 2, 2, 1], vec![1, 1, 1, 1, 1, 1, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5], vec![5, 6], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4], vec![6, 5], vec![6, 6]]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 3, 4, 5, 6, 7, 8, 9], vec![3, 4, 5, 6, 7, 8, 9, 10], vec![4, 5, 6, 7, 8, 9, 10, 11], vec![5, 6, 7, 8, 9, 10, 11, 12]]), vec![vec![0, 7], vec![1, 7], vec![2, 7], vec![3, 7], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7]]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3], vec![6, 5, 4], vec![7, 8, 9]]), vec![vec![0, 2], vec![1, 0], vec![1, 1], vec![1, 2], vec![2, 0], vec![2, 1], vec![2, 2]]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![10, 10, 10, 10, 10, 10], vec![10, 20, 20, 20, 20, 10], vec![10, 20, 30, 30, 20, 10], vec![10, 20, 30, 30, 20, 10], vec![10, 20, 20, 20, 20, 10], vec![10, 10, 10, 10, 10, 10]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5]]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![5, 4, 5, 4, 5], vec![4, 3, 4, 3, 4], vec![5, 4, 5, 4, 5], vec![4, 3, 4, 3, 4], vec![5, 4, 5, 4, 5]]), vec![vec![0, 4], vec![4, 0]]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20], vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11]]), vec![vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6]]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5], vec![2, 3, 4, 5, 6], vec![3, 4, 5, 6, 7], vec![4, 5, 6, 7, 8], vec![5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8], vec![3, 4, 5, 6, 7], vec![2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5]]), vec![vec![0, 4], vec![1, 4], vec![2, 4], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4], vec![7, 0], vec![7, 1], vec![7, 2], vec![7, 3], vec![7, 4], vec![8, 0], vec![8, 1], vec![8, 2], vec![8, 3], vec![8, 4]]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2], vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3], vec![13, 12, 11, 10, 9, 8, 7, 6, 5, 4], vec![14, 13, 12, 11, 10, 9, 8, 7, 6, 5], vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6], vec![16, 15, 14, 13, 12, 11, 10, 9, 8, 7], vec![17, 16, 15, 14, 13, 12, 11, 10, 9, 8], vec![18, 17, 16, 15, 14, 13, 12, 11, 10, 9], vec![19, 18, 17, 16, 15, 14, 13, 12, 11, 10]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5], vec![5, 6], vec![5, 7], vec![5, 8], vec![5, 9], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4], vec![6, 5], vec![6, 6], vec![6, 7], vec![6, 8], vec![6, 9], vec![7, 0], vec![7, 1], vec![7, 2], vec![7, 3], vec![7, 4], vec![7, 5], vec![7, 6], vec![7, 7], vec![7, 8], vec![7, 9], vec![8, 0], vec![8, 1], vec![8, 2], vec![8, 3], vec![8, 4], vec![8, 5], vec![8, 6], vec![8, 7], vec![8, 8], vec![8, 9], vec![9, 0], vec![9, 1], vec![9, 2], vec![9, 3], vec![9, 4], vec![9, 5], vec![9, 6], vec![9, 7], vec![9, 8], vec![9, 9]]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]]), vec![vec![0, 10], vec![0, 11], vec![0, 12], vec![0, 13], vec![0, 14], vec![0, 15], vec![0, 16], vec![0, 17], vec![0, 18], vec![0, 19], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![2, 10], vec![2, 11], vec![2, 12], vec![2, 13], vec![2, 14], vec![2, 15], vec![2, 16], vec![2, 17], vec![2, 18], vec![2, 19], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9], vec![4, 10], vec![4, 11], vec![4, 12], vec![4, 13], vec![4, 14], vec![4, 15], vec![4, 16], vec![4, 17], vec![4, 18], vec![4, 19]]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![10, 10, 10, 10], vec![10, 9, 8, 7], vec![10, 8, 7, 6], vec![10, 7, 6, 5], vec![10, 6, 5, 4], vec![10, 5, 4, 3], vec![10, 4, 3, 2], vec![10, 3, 2, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 0], vec![7, 0]]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![5, 6, 7, 8, 9], vec![4, 5, 6, 7, 8], vec![3, 4, 5, 6, 7], vec![2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5]]), vec![vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4]]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]), vec![vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9]]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 2, 3, 5, 6], vec![3, 2, 3, 4, 4, 5], vec![2, 4, 5, 3, 1, 6], vec![6, 7, 1, 4, 5, 7], vec![5, 1, 1, 2, 4, 8], vec![9, 8, 7, 6, 5, 4]]), vec![vec![0, 5], vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5], vec![5, 0]]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 1, 1, 1, 1, 1], vec![1, 2, 2, 2, 2, 1], vec![1, 2, 3, 3, 2, 1], vec![1, 2, 3, 3, 2, 1], vec![1, 2, 2, 2, 2, 1], vec![1, 1, 1, 1, 1, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5]]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![6, 5, 4], vec![3, 2, 1]]), vec![vec![0, 2], vec![1, 2], vec![2, 0], vec![2, 1], vec![2, 2], vec![3, 0], vec![4, 0]]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![5, 5, 5, 5, 5], vec![5, 1, 2, 3, 5], vec![5, 4, 3, 2, 5], vec![5, 3, 3, 3, 5], vec![5, 5, 5, 5, 5]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 0], vec![1, 4], vec![2, 0], vec![2, 4], vec![3, 0], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30], vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40], vec![41, 42, 43, 44, 45, 46, 47, 48, 49, 50], vec![51, 52, 53, 54, 55, 56, 57, 58, 59, 60], vec![61, 62, 63, 64, 65, 66, 67, 68, 69, 70], vec![71, 72, 73, 74, 75, 76, 77, 78, 79, 80], vec![81, 82, 83, 84, 85, 86, 87, 88, 89, 90], vec![91, 92, 93, 94, 95, 96, 97, 98, 99, 100]]), vec![vec![0, 9], vec![1, 9], vec![2, 9], vec![3, 9], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9], vec![8, 9], vec![9, 0], vec![9, 1], vec![9, 2], vec![9, 3], vec![9, 4], vec![9, 5], vec![9, 6], vec![9, 7], vec![9, 8], vec![9, 9]]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![5, 4, 5], vec![1, 2, 6], vec![7, 4, 6]]), vec![vec![0, 2], vec![1, 2], vec![2, 0], vec![2, 1], vec![2, 2]]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![3, 3, 3, 3, 3, 3], vec![3, 0, 0, 0, 0, 3], vec![3, 0, 9, 8, 0, 3], vec![3, 0, 7, 6, 0, 3], vec![3, 0, 0, 0, 0, 3], vec![3, 3, 3, 3, 3, 3]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 0], vec![1, 5], vec![2, 0], vec![2, 5], vec![3, 0], vec![3, 5], vec![4, 0], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5]]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 2, 3, 5, 6, 7], vec![3, 2, 3, 4, 4, 5, 6], vec![2, 4, 5, 3, 1, 4, 5], vec![6, 7, 1, 4, 5, 6, 7], vec![5, 1, 1, 2, 4, 5, 6], vec![5, 1, 1, 2, 4, 5, 6]]), vec![vec![0, 6], vec![1, 6], vec![2, 2], vec![3, 0], vec![3, 1], vec![4, 0], vec![5, 0]]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1], vec![2, 3, 4, 5, 6], vec![6, 5, 4, 3, 2], vec![3, 4, 5, 6, 7]]), vec![vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![2, 2], vec![2, 3], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![3, 3, 3, 3, 3, 3], vec![3, 2, 2, 2, 2, 3], vec![3, 2, 1, 1, 2, 3], vec![3, 2, 1, 1, 2, 3], vec![3, 2, 2, 2, 2, 3], vec![3, 3, 3, 3, 3, 3]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 0], vec![1, 5], vec![2, 0], vec![2, 5], vec![3, 0], vec![3, 5], vec![4, 0], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5]]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![10, 10, 10, 10, 10, 10], vec![10, 1, 1, 1, 1, 10], vec![10, 1, 2, 2, 1, 10], vec![10, 1, 2, 2, 1, 10], vec![10, 1, 1, 1, 1, 10], vec![10, 10, 10, 10, 10, 10]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 0], vec![1, 5], vec![2, 0], vec![2, 5], vec![3, 0], vec![3, 5], vec![4, 0], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5]]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![19, 17, 15, 13, 11, 9, 7, 5, 3, 1], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]]), vec![vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9]]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![10, 20, 10, 20, 10], vec![20, 30, 20, 30, 20], vec![10, 20, 10, 20, 10], vec![20, 30, 20, 30, 20], vec![10, 20, 10, 20, 10]]), vec![vec![0, 3], vec![0, 4], vec![1, 3], vec![1, 4], vec![3, 0], vec![3, 1], vec![4, 0], vec![4, 1]]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]), vec![vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9]]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 1, 1, 1, 1, 1], vec![1, 2, 2, 2, 2, 1], vec![1, 2, 3, 3, 2, 1], vec![1, 2, 3, 3, 2, 1], vec![1, 2, 2, 2, 2, 1], vec![1, 1, 1, 1, 1, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5]]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 2, 1, 2], vec![2, 2, 2, 2, 2], vec![2, 2, 1, 2, 2], vec![1, 2, 2, 1, 2], vec![2, 2, 2, 2, 2]]), vec![vec![0, 1], vec![0, 2], vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 0], vec![2, 1], vec![2, 3], vec![2, 4], vec![3, 1], vec![3, 2], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![5, 4, 5], vec![1, 2, 6], vec![7, 4, 8], vec![9, 1, 2], vec![3, 6, 7], vec![4, 6, 8]]), vec![vec![0, 2], vec![1, 2], vec![2, 2], vec![4, 1], vec![4, 2], vec![5, 0], vec![5, 1], vec![5, 2]]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![10, 11, 16, 21, 17], vec![14, 17, 24, 25, 22], vec![13, 18, 23, 24, 21], vec![12, 19, 22, 23, 20], vec![11, 16, 19, 20, 19]]), vec![vec![0, 3], vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3]]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1], vec![1, 2, 3, 3, 3, 3, 3, 3, 2, 1], vec![1, 2, 3, 4, 4, 4, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 5, 4, 3, 1], vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 4, 4, 4, 3, 2, 1], vec![1, 2, 3, 3, 3, 3, 3, 3, 2, 1], vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5], vec![5, 6], vec![5, 7], vec![5, 8], vec![5, 9], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4], vec![6, 5], vec![6, 6], vec![6, 7], vec![6, 8], vec![6, 9], vec![7, 0], vec![7, 1], vec![7, 2], vec![7, 3], vec![7, 4], vec![7, 5], vec![7, 6], vec![7, 7], vec![7, 8], vec![7, 9], vec![8, 0], vec![8, 1], vec![8, 2], vec![8, 3], vec![8, 4], vec![8, 5], vec![8, 6], vec![8, 7], vec![8, 8], vec![8, 9], vec![9, 0], vec![9, 1], vec![9, 2], vec![9, 3], vec![9, 4], vec![9, 5], vec![9, 6], vec![9, 7], vec![9, 8], vec![9, 9], vec![10, 0], vec![10, 1], vec![10, 2], vec![10, 3], vec![10, 4], vec![10, 5], vec![10, 6], vec![10, 7], vec![10, 8], vec![10, 9]]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5, 6], vec![6, 5, 4, 3, 2, 1], vec![2, 3, 4, 5, 6, 7], vec![7, 6, 5, 4, 3, 2], vec![3, 4, 5, 6, 7, 8], vec![8, 7, 6, 5, 4, 3]]), vec![vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 0], vec![1, 1], vec![1, 2], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 0], vec![3, 1], vec![3, 2], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2]]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1], vec![1, 2, 3, 3, 3, 3, 3, 3, 2, 1], vec![1, 2, 3, 4, 4, 4, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 4, 4, 4, 3, 2, 1], vec![1, 2, 3, 3, 3, 3, 3, 3, 2, 1], vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5], vec![5, 6], vec![5, 7], vec![5, 8], vec![5, 9], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4], vec![6, 5], vec![6, 6], vec![6, 7], vec![6, 8], vec![6, 9], vec![7, 0], vec![7, 1], vec![7, 2], vec![7, 3], vec![7, 4], vec![7, 5], vec![7, 6], vec![7, 7], vec![7, 8], vec![7, 9], vec![8, 0], vec![8, 1], vec![8, 2], vec![8, 3], vec![8, 4], vec![8, 5], vec![8, 6], vec![8, 7], vec![8, 8], vec![8, 9], vec![9, 0], vec![9, 1], vec![9, 2], vec![9, 3], vec![9, 4], vec![9, 5], vec![9, 6], vec![9, 7], vec![9, 8], vec![9, 9]]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 5, 5, 5, 5, 5, 5, 5, 5, 5], vec![2, 3, 4, 5, 5, 5, 5, 5, 5, 5], vec![2, 2, 3, 4, 5, 5, 5, 5, 5, 5], vec![2, 2, 2, 3, 4, 5, 5, 5, 5, 5], vec![2, 2, 2, 2, 3, 4, 5, 5, 5, 5], vec![2, 2, 2, 2, 2, 3, 4, 5, 5, 5], vec![2, 2, 2, 2, 2, 2, 3, 4, 5, 5], vec![2, 2, 2, 2, 2, 2, 2, 3, 4, 5], vec![2, 2, 2, 2, 2, 2, 2, 2, 3, 4], vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 3]]), vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5], vec![5, 6], vec![5, 7], vec![5, 8], vec![5, 9], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4], vec![6, 5], vec![6, 6], vec![6, 7], vec![6, 8], vec![6, 9], vec![7, 0], vec![7, 1], vec![7, 2], vec![7, 3], vec![7, 4], vec![7, 5], vec![7, 6], vec![7, 7], vec![7, 8], vec![7, 9], vec![8, 0], vec![8, 1], vec![8, 2], vec![8, 3], vec![8, 4], vec![8, 5], vec![8, 6], vec![8, 7], vec![8, 8], vec![8, 9], vec![9, 0], vec![9, 1], vec![9, 2], vec![9, 3], vec![9, 4], vec![9, 5], vec![9, 6], vec![9, 7], vec![9, 8], vec![9, 9]]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 2, 3, 5, 6], vec![3, 2, 3, 4, 4, 5], vec![2, 4, 5, 3, 1, 4], vec![6, 7, 1, 4, 5, 6], vec![5, 1, 1, 2, 4, 5], vec![5, 1, 1, 2, 4, 1]]), vec![vec![0, 5], vec![1, 5], vec![2, 2], vec![3, 0], vec![3, 1], vec![4, 0], vec![5, 0]]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 2, 3, 4, 5], vec![2, 3, 4, 5, 6, 7], vec![3, 4, 5, 6, 7, 8], vec![4, 5, 6, 7, 8, 9], vec![5, 6, 7, 8, 9, 10], vec![6, 7, 8, 9, 10, 11]]), vec![vec![0, 5], vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5]]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5], vec![10, 9, 8, 7, 6], vec![11, 12, 13, 14, 15], vec![20, 19, 18, 17, 16], vec![21, 22, 23, 24, 25]]), vec![vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]), vec![vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9]]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 2, 1], vec![4, 5, 6, 5, 4], vec![7, 8, 9, 8, 7], vec![4, 5, 6, 5, 4], vec![1, 2, 3, 2, 1]]), vec![vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![4, 0], vec![4, 1], vec![4, 2]]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![5, 5, 5, 5, 5, 5, 5], vec![5, 1, 1, 1, 1, 1, 5], vec![5, 1, 2, 2, 2, 1, 5], vec![5, 1, 2, 3, 2, 1, 5], vec![5, 1, 2, 2, 2, 1, 5], vec![5, 1, 1, 1, 1, 1, 5], vec![5, 5, 5, 5, 5, 5, 5]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![1, 0], vec![1, 6], vec![2, 0], vec![2, 6], vec![3, 0], vec![3, 6], vec![4, 0], vec![4, 6], vec![5, 0], vec![5, 6], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4], vec![6, 5], vec![6, 6]]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10], vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10], vec![1, 3, 5, 7, 9]]), vec![vec![0, 4], vec![1, 4], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 2, 3, 5, 6], vec![3, 2, 3, 4, 4, 5], vec![2, 4, 5, 3, 1, 2], vec![6, 7, 1, 4, 5, 3], vec![5, 1, 1, 2, 4, 1], vec![4, 3, 2, 1, 1, 1]]), vec![vec![0, 5], vec![1, 5], vec![3, 0], vec![3, 1], vec![4, 0], vec![5, 0]]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 1, 1, 1, 1], vec![1, 2, 2, 2, 1], vec![1, 2, 3, 2, 1], vec![1, 2, 2, 2, 1], vec![1, 1, 1, 1, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 1, 1, 1, 1, 1], vec![1, 2, 2, 2, 2, 1], vec![1, 2, 3, 3, 2, 1], vec![1, 2, 2, 2, 2, 1], vec![1, 1, 1, 1, 1, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5]]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], vec![5, 1, 1, 1, 1, 1, 1, 1, 1, 5], vec![5, 1, 2, 2, 2, 2, 2, 2, 1, 5], vec![5, 1, 2, 3, 3, 3, 3, 2, 1, 5], vec![5, 1, 2, 3, 4, 4, 3, 2, 1, 5], vec![5, 1, 2, 3, 4, 5, 4, 3, 2, 5], vec![5, 1, 2, 3, 4, 4, 3, 2, 1, 5], vec![5, 1, 2, 3, 3, 3, 3, 2, 1, 5], vec![5, 1, 2, 2, 2, 2, 2, 2, 1, 5], vec![5, 1, 1, 1, 1, 1, 1, 1, 1, 5], vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 9], vec![2, 0], vec![2, 9], vec![3, 0], vec![3, 9], vec![4, 0], vec![4, 9], vec![5, 0], vec![5, 9], vec![6, 0], vec![6, 9], vec![7, 0], vec![7, 9], vec![8, 0], vec![8, 9], vec![9, 0], vec![9, 9], vec![10, 0], vec![10, 1], vec![10, 2], vec![10, 3], vec![10, 4], vec![10, 5], vec![10, 6], vec![10, 7], vec![10, 8], vec![10, 9]]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3], vec![6, 5, 4], vec![7, 8, 9], vec![12, 15, 14], vec![13, 16, 17]]), vec![vec![0, 2], vec![1, 0], vec![1, 1], vec![1, 2], vec![2, 0], vec![2, 1], vec![2, 2], vec![3, 0], vec![3, 1], vec![3, 2], vec![4, 0], vec![4, 1], vec![4, 2]]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 2, 3, 5], vec![3, 2, 3, 4, 4], vec![2, 4, 8, 3, 1], vec![6, 7, 1, 4, 5], vec![5, 1, 1, 2, 4], vec![4, 3, 2, 1, 0]]), vec![vec![0, 4], vec![1, 3], vec![1, 4], vec![2, 2], vec![3, 0], vec![3, 1], vec![4, 0], vec![5, 0]]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![10, 10, 10, 10], vec![10, 20, 20, 10], vec![10, 20, 10, 10], vec![10, 10, 10, 10]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3]]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![3, 3, 3, 3], vec![3, 0, 0, 3], vec![3, 0, 0, 3], vec![3, 3, 3, 3]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 0], vec![1, 3], vec![2, 0], vec![2, 3], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3]]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![5, 1, 3, 4, 5], vec![6, 2, 3, 4, 6], vec![1, 2, 3, 4, 7], vec![2, 3, 4, 5, 8], vec![3, 4, 5, 6, 9]]), vec![vec![0, 4], vec![1, 4], vec![2, 4], vec![3, 4], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1], vec![1, 2, 3, 3, 3, 3, 3, 3, 2, 1], vec![1, 2, 3, 4, 4, 4, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 4, 3, 2, 2, 1], vec![1, 2, 3, 4, 4, 3, 2, 2, 1, 1], vec![1, 2, 3, 3, 3, 2, 2, 1, 1, 1], vec![1, 2, 2, 2, 2, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]]), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![4, 9], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5], vec![5, 6], vec![5, 7], vec![5, 8], vec![5, 9], vec![6, 0], vec![6, 1], vec![6, 2], vec![6, 3], vec![6, 4], vec![6, 5], vec![6, 6], vec![6, 7], vec![6, 8], vec![6, 9], vec![7, 0], vec![7, 1], vec![7, 2], vec![7, 3], vec![7, 4], vec![7, 5], vec![7, 6], vec![7, 7], vec![7, 8], vec![7, 9], vec![8, 0], vec![8, 1], vec![8, 2], vec![8, 3], vec![8, 4], vec![8, 5], vec![8, 6], vec![8, 7], vec![8, 8], vec![8, 9], vec![9, 0], vec![9, 1], vec![9, 2], vec![9, 3], vec![9, 4], vec![9, 5], vec![9, 6], vec![9, 7], vec![9, 8], vec![9, 9]]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 4, 3, 4, 5, 6], vec![2, 4, 3, 4, 5, 6], vec![3, 4, 3, 4, 5, 6], vec![4, 4, 3, 4, 5, 6], vec![5, 4, 3, 4, 5, 6], vec![6, 4, 3, 4, 5, 6]]), vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5]]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::pacific_atlantic(vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5]]), vec![vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 0], vec![1, 1], vec![1, 2], vec![2, 2], vec![2, 3], vec![2, 4], vec![3, 0], vec![3, 1], vec![3, 2], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4]]);
}

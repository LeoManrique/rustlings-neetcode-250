include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![4, 5], vec![5, 1]]), vec![1, 4]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![3, 4], vec![1, 2], vec![2, 4], vec![3, 5], vec![2, 5]]), vec![2, 5]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 1]]), vec![3, 1]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![1, 3]]), vec![1, 3]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 4], vec![3, 4], vec![1, 3], vec![1, 2], vec![4, 5]]), vec![1, 3]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]), vec![2, 3]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 5]]), vec![4, 5]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]]), vec![1, 4]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![1, 3], vec![4, 5], vec![5, 6], vec![4, 6]]), vec![1, 3]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 3]]), vec![5, 3]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 3], vec![3, 4], vec![1, 5], vec![3, 5], vec![2, 4]]), vec![3, 5]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![1, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 6], vec![1, 7], vec![2, 8], vec![3, 9], vec![4, 10], vec![5, 6], vec![7, 9], vec![8, 10], vec![1, 3], vec![2, 4], vec![5, 7], vec![6, 8], vec![9, 11], vec![10, 12], vec![11, 13], vec![12, 14], vec![13, 15], vec![14, 1], vec![15, 2]]), vec![5, 1]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![8, 9], vec![10, 11], vec![12, 13], vec![14, 15], vec![1, 8]]), vec![8, 9]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 1], vec![1, 3]]), vec![9, 1]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 4], vec![2, 5], vec![3, 5], vec![4, 6], vec![5, 6]]), vec![2, 4]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 5], vec![3, 6], vec![4, 7], vec![1, 8]]), vec![10, 5]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![2, 5], vec![3, 6]]), vec![5, 1]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![1, 20]]), vec![1, 20]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![20, 1], vec![1, 2]]), vec![20, 1]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 1]]), vec![15, 1]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![1, 10], vec![5, 7]]), vec![1, 10]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 1], vec![5, 8]]), vec![10, 1]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 1], vec![3, 6], vec![4, 7], vec![1, 8], vec![2, 9], vec![3, 10], vec![5, 8]]), vec![10, 1]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![1, 10], vec![5, 10]]), vec![1, 10]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![8, 9], vec![10, 11], vec![12, 13], vec![14, 15], vec![9, 15], vec![1, 15], vec![2, 6], vec![3, 8], vec![4, 10], vec![5, 12], vec![6, 14], vec![7, 15]]), vec![8, 9]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 1], vec![5, 7]]), vec![10, 1]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![4, 5], vec![5, 6]]), vec![4, 5]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![8, 16], vec![8, 17], vec![9, 18], vec![9, 19], vec![10, 20], vec![10, 21], vec![11, 22], vec![11, 23], vec![12, 24], vec![12, 25], vec![13, 26], vec![13, 27], vec![14, 28], vec![14, 29], vec![15, 30], vec![15, 31], vec![16, 17], vec![18, 19], vec![20, 21], vec![22, 23], vec![24, 25], vec![26, 27], vec![28, 29], vec![30, 31]]), vec![16, 17]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![20, 5]]), vec![20, 5]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 4], vec![3, 5], vec![4, 5]]), vec![3, 4]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 1]]), vec![1, 4]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![8, 9], vec![10, 11], vec![12, 13], vec![14, 15], vec![15, 1]]), vec![8, 9]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![2, 5]]), vec![5, 1]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 3], vec![5, 9]]), vec![10, 3]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![1, 3]]), vec![5, 1]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 5]]), vec![10, 5]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 4], vec![3, 5], vec![4, 5], vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6]]), vec![2, 3]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 10]]), vec![5, 10]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 5], vec![3, 6], vec![4, 7], vec![1, 8], vec![2, 9]]), vec![10, 5]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 1], vec![1, 10]]), vec![12, 1]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 4], vec![6, 8]]), vec![10, 4]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![20, 1]]), vec![20, 1]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 5]]), vec![5, 1]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![5, 7]]), vec![5, 7]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 5], vec![3, 6], vec![4, 7], vec![1, 8], vec![2, 9], vec![3, 10]]), vec![10, 5]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![1, 5]]), vec![1, 5]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![2, 4], vec![1, 3]]), vec![1, 4]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![8, 9], vec![10, 11], vec![12, 13], vec![14, 15], vec![9, 15], vec![1, 15]]), vec![8, 9]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 1], vec![1, 3]]), vec![6, 1]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10], vec![9, 10]]), vec![9, 10]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![4, 5], vec![5, 6], vec![6, 1]]), vec![1, 4]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![8, 16], vec![8, 17], vec![9, 18], vec![9, 19], vec![10, 20], vec![10, 21], vec![11, 22], vec![11, 23], vec![12, 24], vec![12, 25], vec![13, 26], vec![13, 27], vec![14, 28], vec![14, 29], vec![15, 30], vec![15, 31], vec![16, 32], vec![16, 33], vec![17, 34], vec![17, 35], vec![18, 36], vec![18, 37], vec![19, 38], vec![19, 39], vec![20, 40], vec![20, 41], vec![21, 42], vec![21, 43], vec![22, 44], vec![22, 45], vec![23, 46], vec![23, 47], vec![24, 48], vec![24, 49], vec![25, 50], vec![25, 51], vec![26, 52], vec![26, 53], vec![27, 54], vec![27, 55], vec![28, 56], vec![28, 57], vec![29, 58], vec![29, 59], vec![30, 60], vec![30, 61], vec![31, 62], vec![31, 63], vec![32, 64], vec![32, 65], vec![33, 66], vec![33, 67], vec![34, 68], vec![34, 69], vec![35, 70], vec![35, 71], vec![36, 72], vec![36, 73], vec![37, 74], vec![37, 75], vec![38, 76], vec![38, 77], vec![39, 78], vec![39, 79], vec![40, 80], vec![40, 81], vec![41, 82], vec![41, 83], vec![42, 84], vec![42, 85], vec![43, 86], vec![43, 87], vec![44, 88], vec![44, 89], vec![45, 90], vec![45, 91], vec![46, 92], vec![46, 93], vec![47, 94], vec![47, 95], vec![48, 96], vec![48, 97], vec![49, 98], vec![49, 99], vec![50, 100], vec![50, 1], vec![1, 50]]), vec![50, 1]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 1], vec![1, 3]]), vec![16, 1]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![1, 10]]), vec![1, 10]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![1, 11], vec![1, 3], vec![4, 7]]), vec![1, 11]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![1, 10], vec![4, 6], vec![7, 9]]), vec![1, 10]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 1], vec![1, 10], vec![2, 11], vec![3, 12], vec![4, 13], vec![5, 14], vec![6, 15], vec![7, 1], vec![8, 2], vec![9, 3], vec![10, 4], vec![11, 5], vec![12, 6], vec![13, 7], vec![14, 8], vec![15, 9]]), vec![15, 1]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![1, 3], vec![3, 5], vec![5, 2]]), vec![5, 1]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 1], vec![2, 5]]), vec![6, 1]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 1]]), vec![10, 1]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![1, 10], vec![1, 11], vec![1, 12], vec![1, 13], vec![1, 14], vec![1, 15], vec![1, 16], vec![1, 17], vec![1, 18], vec![1, 19], vec![1, 20], vec![1, 21], vec![1, 22], vec![1, 23], vec![1, 24], vec![1, 25], vec![1, 26], vec![1, 27], vec![1, 28], vec![1, 29], vec![1, 30], vec![1, 31], vec![1, 32], vec![1, 33], vec![1, 34], vec![1, 35], vec![1, 36], vec![1, 37], vec![1, 38], vec![1, 39], vec![1, 40], vec![1, 41], vec![1, 42], vec![1, 43], vec![1, 44], vec![1, 45], vec![1, 46], vec![1, 47], vec![1, 48], vec![1, 49], vec![1, 50], vec![1, 51], vec![1, 52], vec![1, 53], vec![1, 54], vec![1, 55], vec![1, 56], vec![1, 57], vec![1, 58], vec![1, 59], vec![1, 60], vec![1, 61], vec![1, 62], vec![1, 63], vec![1, 64], vec![1, 65], vec![1, 66], vec![1, 67], vec![1, 68], vec![1, 69], vec![1, 70], vec![1, 71], vec![1, 72], vec![1, 73], vec![1, 74], vec![1, 75], vec![1, 76], vec![1, 77], vec![1, 78], vec![1, 79], vec![1, 80], vec![1, 81], vec![1, 82], vec![1, 83], vec![1, 84], vec![1, 85], vec![1, 86], vec![1, 87], vec![1, 88], vec![1, 89], vec![1, 90], vec![1, 91], vec![1, 92], vec![1, 93], vec![1, 94], vec![1, 95], vec![1, 96], vec![1, 97], vec![1, 98], vec![1, 99], vec![1, 100], vec![2, 100]]), vec![2, 100]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 1], vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10], vec![7, 1], vec![8, 2], vec![9, 3], vec![10, 4]]), vec![10, 1]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![20, 21], vec![21, 22], vec![22, 23], vec![23, 24], vec![24, 25], vec![25, 26], vec![26, 27], vec![27, 28], vec![28, 29], vec![29, 30], vec![30, 31], vec![31, 32], vec![32, 33], vec![33, 34], vec![34, 35], vec![35, 36], vec![36, 37], vec![37, 38], vec![38, 39], vec![39, 40], vec![40, 1], vec![1, 3]]), vec![40, 1]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![1, 10], vec![2, 4], vec![3, 5], vec![6, 8], vec![7, 9]]), vec![1, 10]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![20, 10]]), vec![20, 10]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 5]]), vec![2, 3]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![1, 10], vec![2, 5]]), vec![1, 10]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![2, 4]]), vec![5, 1]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![8, 9], vec![10, 11], vec![12, 13], vec![14, 15], vec![1, 15]]), vec![8, 9]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 5], vec![3, 6]]), vec![10, 5]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 1], vec![1, 5]]), vec![2, 3]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![8, 9], vec![10, 11], vec![12, 13], vec![14, 15], vec![9, 15]]), vec![8, 9]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 6]]), vec![5, 1]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 5], vec![3, 6], vec![4, 7]]), vec![10, 5]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 1], vec![1, 4]]), vec![13, 1]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![1, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 6], vec![1, 7], vec![2, 8], vec![3, 9], vec![4, 10], vec![5, 6], vec![7, 9], vec![8, 10]]), vec![5, 1]);
}

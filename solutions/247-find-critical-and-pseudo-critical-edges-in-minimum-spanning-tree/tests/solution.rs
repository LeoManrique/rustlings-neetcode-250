include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(3, vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 2]]), vec![vec![0, 1], vec![]]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(5, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 2], vec![0, 3, 2], vec![0, 4, 3], vec![3, 4, 3], vec![1, 4, 6]]), vec![vec![0, 1], vec![2, 3, 4, 5]]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(6, vec![vec![0, 1, 1], vec![0, 2, 2], vec![1, 2, 3], vec![1, 3, 4], vec![2, 3, 5], vec![3, 4, 6], vec![4, 5, 7]]), vec![vec![0, 1, 3, 5, 6], vec![]]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(6, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 0, 5], vec![1, 3, 6]]), vec![vec![0, 1, 2, 3, 4], vec![]]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(4, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]]), vec![vec![], vec![0, 1, 2, 3]]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(3, vec![vec![0, 1, 1], vec![1, 2, 2], vec![2, 0, 2]]), vec![vec![0], vec![1, 2]]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(3, vec![vec![0, 1, 1], vec![1, 2, 2], vec![0, 2, 3]]), vec![vec![0, 1], vec![]]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(7, vec![vec![0, 1, 1], vec![0, 2, 1], vec![0, 3, 1], vec![1, 2, 1], vec![1, 3, 1], vec![2, 3, 1], vec![3, 4, 2], vec![4, 5, 2], vec![5, 6, 2], vec![6, 3, 2]]), vec![vec![], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(7, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 1], vec![4, 5, 1], vec![5, 6, 1], vec![6, 0, 1]]), vec![vec![], vec![0, 1, 2, 3, 4, 5, 6]]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(5, vec![vec![0, 1, 2], vec![0, 3, 6], vec![1, 2, 5], vec![1, 3, 8], vec![1, 4, 9], vec![2, 4, 7], vec![3, 4, 4]]), vec![vec![0, 6, 2, 1], vec![]]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(6, vec![vec![0, 1, 4], vec![0, 2, 4], vec![1, 2, 2], vec![1, 3, 3], vec![1, 4, 2], vec![2, 3, 1], vec![3, 4, 2], vec![3, 5, 3], vec![4, 5, 3]]), vec![vec![5], vec![2, 4, 6, 7, 8, 0, 1]]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(7, vec![vec![0, 1, 1], vec![0, 2, 2], vec![0, 3, 3], vec![1, 2, 4], vec![1, 3, 5], vec![1, 4, 6], vec![2, 3, 7], vec![2, 4, 8], vec![2, 5, 9], vec![3, 4, 10], vec![3, 5, 11], vec![3, 6, 12], vec![4, 5, 13], vec![4, 6, 14], vec![5, 6, 15], vec![0, 4, 16], vec![0, 5, 17], vec![0, 6, 18], vec![1, 5, 19], vec![1, 6, 20], vec![2, 6, 21]]), vec![vec![0, 1, 2, 5, 8, 11], vec![]]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(12, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 1], vec![4, 5, 1], vec![5, 6, 1], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1], vec![10, 11, 1], vec![11, 0, 1], vec![0, 6, 2], vec![1, 7, 2], vec![2, 8, 2], vec![3, 9, 2], vec![4, 10, 2], vec![5, 11, 2]]), vec![vec![], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 10], vec![0, 2, 20], vec![0, 3, 30], vec![1, 2, 10], vec![1, 3, 20], vec![1, 4, 30], vec![2, 3, 10], vec![2, 4, 20], vec![2, 5, 30], vec![3, 4, 10], vec![3, 5, 20], vec![3, 6, 30], vec![4, 5, 10], vec![4, 6, 20], vec![4, 7, 30], vec![5, 6, 10], vec![5, 7, 20], vec![5, 8, 30], vec![6, 7, 10], vec![6, 8, 20], vec![6, 9, 30], vec![7, 8, 10], vec![7, 9, 20], vec![8, 9, 10]]), vec![vec![0, 3, 6, 9, 12, 15, 18, 21, 23], vec![]]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 1], vec![0, 2, 2], vec![0, 3, 3], vec![1, 2, 4], vec![1, 3, 5], vec![2, 3, 6], vec![2, 4, 7], vec![3, 4, 8], vec![4, 5, 9], vec![4, 6, 10], vec![5, 6, 11], vec![5, 7, 12], vec![6, 7, 13], vec![6, 8, 14], vec![7, 8, 15], vec![7, 9, 16], vec![8, 9, 17]]), vec![vec![0, 1, 2, 6, 8, 9, 11, 13, 15], vec![]]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(12, vec![vec![0, 1, 3], vec![0, 2, 4], vec![0, 3, 5], vec![1, 2, 2], vec![1, 3, 6], vec![1, 4, 7], vec![2, 3, 8], vec![2, 4, 9], vec![2, 5, 10], vec![3, 4, 11], vec![3, 5, 12], vec![3, 6, 13], vec![4, 5, 14], vec![4, 6, 15], vec![4, 7, 16], vec![5, 6, 17], vec![5, 7, 18], vec![5, 8, 19], vec![6, 7, 20], vec![6, 8, 21], vec![6, 9, 22], vec![7, 8, 23], vec![7, 9, 24], vec![7, 10, 25], vec![8, 9, 26], vec![8, 10, 27], vec![8, 11, 28], vec![9, 10, 29], vec![9, 11, 30], vec![10, 11, 31]]), vec![vec![3, 0, 2, 5, 8, 11, 14, 17, 20, 23, 26], vec![]]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(9, vec![vec![0, 1, 100], vec![0, 2, 90], vec![0, 3, 95], vec![1, 2, 85], vec![1, 3, 80], vec![1, 4, 75], vec![2, 3, 65], vec![2, 4, 60], vec![2, 5, 55], vec![3, 4, 50], vec![3, 5, 45], vec![3, 6, 40], vec![4, 5, 35], vec![4, 6, 30], vec![4, 7, 25], vec![5, 6, 20], vec![5, 7, 15], vec![5, 8, 10], vec![6, 7, 5], vec![6, 8, 1], vec![7, 8, 50]]), vec![vec![19, 18, 17, 14, 11, 8, 5, 1], vec![]]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(12, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 3, 30], vec![1, 4, 40], vec![2, 5, 50], vec![2, 6, 60], vec![3, 7, 70], vec![3, 8, 80], vec![4, 9, 90], vec![4, 10, 100], vec![5, 11, 110], vec![6, 11, 120], vec![7, 8, 130], vec![8, 9, 140], vec![9, 10, 150], vec![10, 11, 160], vec![0, 3, 170], vec![1, 5, 180], vec![2, 7, 190], vec![3, 9, 200], vec![4, 11, 210]]), vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![]]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(15, vec![vec![0, 1, 10], vec![0, 2, 20], vec![0, 3, 30], vec![1, 2, 25], vec![1, 4, 35], vec![2, 3, 40], vec![2, 5, 45], vec![3, 4, 50], vec![3, 6, 55], vec![4, 5, 60], vec![5, 6, 65], vec![6, 7, 70], vec![6, 8, 75], vec![7, 8, 80], vec![7, 9, 85], vec![8, 9, 90], vec![9, 10, 95], vec![9, 11, 100], vec![10, 11, 105], vec![10, 12, 110], vec![11, 12, 115], vec![12, 13, 120], vec![12, 14, 125], vec![13, 14, 130]]), vec![vec![0, 1, 2, 4, 6, 8, 11, 12, 14, 16, 17, 19, 21, 22], vec![]]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(8, vec![vec![0, 1, 2], vec![1, 2, 2], vec![2, 3, 3], vec![3, 4, 4], vec![4, 5, 5], vec![5, 6, 6], vec![6, 7, 7], vec![0, 7, 8], vec![1, 3, 9], vec![2, 4, 10], vec![3, 5, 11], vec![4, 6, 12], vec![5, 7, 13], vec![0, 3, 14], vec![1, 4, 15], vec![2, 5, 16], vec![3, 6, 17], vec![4, 7, 18]]), vec![vec![0, 1, 2, 3, 4, 5, 6], vec![]]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(8, vec![vec![0, 1, 5], vec![0, 2, 20], vec![0, 3, 30], vec![1, 2, 15], vec![1, 4, 10], vec![2, 3, 10], vec![2, 4, 25], vec![2, 5, 35], vec![3, 4, 5], vec![3, 6, 15], vec![4, 5, 20], vec![4, 6, 10], vec![4, 7, 25], vec![5, 6, 30], vec![5, 7, 15], vec![6, 7, 20]]), vec![vec![0, 8, 4, 5, 11, 14], vec![10, 15]]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(8, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![0, 7, 7], vec![1, 3, 8], vec![2, 4, 9], vec![3, 5, 10], vec![4, 6, 11]]), vec![vec![0, 1, 2, 3, 4, 5, 6], vec![]]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(6, vec![vec![0, 1, 100], vec![0, 2, 50], vec![0, 3, 20], vec![1, 2, 60], vec![1, 3, 40], vec![1, 4, 70], vec![2, 3, 30], vec![2, 4, 80], vec![2, 5, 90], vec![3, 4, 5], vec![3, 5, 10], vec![4, 5, 100]]), vec![vec![9, 10, 2, 6, 4], vec![]]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(8, vec![vec![0, 1, 1], vec![0, 2, 2], vec![0, 3, 3], vec![1, 2, 4], vec![1, 3, 5], vec![1, 4, 6], vec![2, 3, 7], vec![2, 4, 8], vec![2, 5, 9], vec![3, 4, 10], vec![3, 5, 11], vec![3, 6, 12], vec![4, 5, 13], vec![4, 6, 14], vec![4, 7, 15], vec![5, 6, 16], vec![5, 7, 17], vec![6, 7, 18]]), vec![vec![0, 1, 2, 5, 8, 11, 14], vec![]]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(8, vec![vec![0, 1, 10], vec![0, 2, 6], vec![0, 3, 5], vec![1, 3, 15], vec![2, 3, 4], vec![2, 4, 11], vec![3, 4, 9], vec![3, 5, 10], vec![4, 5, 13], vec![4, 6, 14], vec![5, 6, 16], vec![5, 7, 7], vec![6, 7, 18]]), vec![vec![4, 2, 11, 6, 0, 7, 9], vec![]]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(12, vec![vec![0, 1, 2], vec![0, 2, 4], vec![1, 3, 3], vec![1, 4, 5], vec![2, 3, 6], vec![2, 4, 7], vec![3, 5, 8], vec![4, 5, 9], vec![5, 6, 10], vec![6, 7, 11], vec![6, 8, 12], vec![7, 9, 13], vec![8, 9, 14], vec![9, 10, 15], vec![10, 11, 16], vec![11, 0, 17], vec![2, 7, 18], vec![3, 8, 19], vec![4, 9, 20], vec![5, 10, 21]]), vec![vec![0, 2, 1, 3, 6, 8, 9, 10, 11, 13, 14], vec![]]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 2, 5], vec![1, 3, 15], vec![2, 3, 25], vec![2, 4, 10], vec![3, 4, 30], vec![3, 5, 15], vec![4, 5, 5], vec![4, 6, 20], vec![5, 6, 25], vec![5, 7, 10], vec![6, 7, 30], vec![6, 8, 15], vec![7, 8, 5], vec![7, 9, 20], vec![8, 9, 25]]), vec![vec![2, 8, 14, 0, 5, 11, 13, 15], vec![3, 7]]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(9, vec![vec![0, 1, 5], vec![0, 2, 6], vec![0, 3, 7], vec![1, 2, 8], vec![1, 3, 9], vec![2, 3, 10], vec![2, 4, 11], vec![3, 4, 12], vec![3, 5, 13], vec![4, 5, 14], vec![4, 6, 15], vec![5, 6, 16], vec![5, 7, 17], vec![6, 7, 18], vec![6, 8, 19], vec![7, 8, 20], vec![0, 4, 21], vec![1, 5, 22], vec![2, 6, 23], vec![3, 7, 24], vec![4, 8, 25]]), vec![vec![0, 1, 2, 6, 8, 10, 12, 14], vec![]]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 10], vec![0, 2, 15], vec![0, 3, 20], vec![1, 2, 35], vec![1, 3, 25], vec![2, 3, 30], vec![4, 5, 10], vec![4, 6, 20], vec![4, 7, 30], vec![5, 6, 25], vec![5, 7, 35], vec![6, 7, 40], vec![8, 9, 5], vec![8, 7, 15], vec![7, 9, 10]]), vec![vec![12, 0, 6, 14, 1, 13, 2, 7, 4, 9, 5, 8, 3, 10, 11], vec![]]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(15, vec![vec![0, 1, 5], vec![0, 2, 7], vec![0, 3, 9], vec![1, 2, 3], vec![1, 3, 2], vec![1, 4, 6], vec![2, 3, 1], vec![2, 4, 4], vec![2, 5, 8], vec![3, 4, 11], vec![3, 5, 13], vec![3, 6, 15], vec![4, 5, 12], vec![4, 6, 14], vec![5, 6, 16], vec![5, 7, 18], vec![5, 8, 20], vec![6, 7, 21], vec![6, 8, 22], vec![7, 8, 23], vec![7, 9, 25], vec![7, 10, 27], vec![8, 9, 24], vec![8, 10, 26], vec![9, 10, 28], vec![9, 11, 29], vec![9, 12, 31], vec![10, 11, 30], vec![10, 12, 32], vec![11, 12, 33], vec![11, 13, 35], vec![11, 14, 37], vec![12, 13, 34], vec![12, 14, 36], vec![13, 14, 38]]), vec![vec![6, 4, 7, 0, 8, 13, 15, 16, 22, 23, 25, 26, 32, 33], vec![]]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(8, vec![vec![0, 1, 1], vec![0, 2, 2], vec![1, 3, 3], vec![1, 4, 4], vec![2, 5, 5], vec![2, 6, 6], vec![3, 6, 7], vec![3, 7, 8], vec![4, 7, 9], vec![5, 7, 10], vec![0, 3, 11], vec![1, 5, 12], vec![2, 4, 13], vec![6, 7, 14]]), vec![vec![0, 1, 2, 3, 4, 5, 7], vec![]]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(9, vec![vec![0, 1, 5], vec![0, 2, 6], vec![0, 3, 7], vec![1, 2, 8], vec![1, 3, 9], vec![1, 4, 10], vec![2, 3, 11], vec![2, 4, 12], vec![2, 5, 13], vec![3, 4, 14], vec![3, 5, 15], vec![3, 6, 16], vec![4, 5, 17], vec![4, 6, 18], vec![4, 7, 19], vec![5, 6, 20], vec![5, 7, 21], vec![5, 8, 22], vec![6, 7, 23], vec![7, 8, 24]]), vec![vec![0, 1, 2, 5, 8, 11, 14, 17], vec![]]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 10], vec![0, 2, 10], vec![1, 3, 10], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10], vec![5, 6, 10], vec![6, 7, 10], vec![7, 8, 10], vec![8, 9, 10], vec![9, 0, 10], vec![0, 3, 20], vec![1, 4, 20], vec![2, 5, 20], vec![3, 6, 20], vec![4, 7, 20], vec![5, 8, 20], vec![6, 9, 20], vec![7, 0, 20], vec![8, 1, 20], vec![9, 2, 20]]), vec![vec![], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 4], vec![0, 2, 8], vec![1, 2, 2], vec![1, 3, 6], vec![2, 3, 3], vec![3, 4, 5], vec![4, 5, 9], vec![5, 6, 10], vec![6, 7, 1], vec![7, 8, 7], vec![8, 9, 11], vec![9, 0, 12], vec![1, 4, 13], vec![2, 5, 14]]), vec![vec![8, 2, 4, 0, 5, 9, 6, 7, 10], vec![]]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(12, vec![vec![0, 1, 1], vec![0, 2, 2], vec![0, 3, 3], vec![1, 2, 4], vec![1, 3, 5], vec![1, 4, 6], vec![2, 3, 7], vec![2, 4, 8], vec![2, 5, 9], vec![3, 4, 10], vec![3, 5, 11], vec![3, 6, 12], vec![4, 5, 13], vec![4, 6, 14], vec![5, 6, 15], vec![6, 7, 16], vec![6, 8, 17], vec![7, 8, 18], vec![7, 9, 19], vec![8, 9, 20], vec![8, 10, 21], vec![9, 10, 22], vec![9, 11, 23], vec![10, 11, 24]]), vec![vec![0, 1, 2, 5, 8, 11, 15, 16, 18, 20, 22], vec![]]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 1], vec![0, 2, 2], vec![1, 2, 3], vec![1, 3, 4], vec![2, 3, 5], vec![3, 4, 6], vec![4, 5, 7], vec![5, 6, 8], vec![6, 7, 9], vec![7, 8, 10], vec![8, 9, 11], vec![0, 9, 15], vec![2, 8, 12], vec![4, 7, 13]]), vec![vec![0, 1, 3, 5, 6, 7, 8, 9, 10], vec![]]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(11, vec![vec![0, 1, 2], vec![0, 2, 3], vec![0, 3, 4], vec![0, 4, 5], vec![1, 2, 6], vec![1, 3, 7], vec![1, 4, 8], vec![1, 5, 9], vec![2, 3, 10], vec![2, 4, 11], vec![2, 5, 12], vec![2, 6, 13], vec![3, 4, 14], vec![3, 5, 15], vec![3, 6, 16], vec![3, 7, 17], vec![4, 5, 18], vec![4, 6, 19], vec![4, 7, 20], vec![4, 8, 21], vec![5, 6, 22], vec![5, 7, 23], vec![5, 8, 24], vec![5, 9, 25], vec![6, 7, 26], vec![6, 8, 27], vec![6, 9, 28], vec![6, 10, 29], vec![7, 8, 30], vec![7, 9, 31], vec![7, 10, 32], vec![8, 9, 33], vec![8, 10, 34], vec![9, 10, 35]]), vec![vec![0, 1, 2, 3, 7, 11, 15, 19, 23, 27], vec![]]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(8, vec![vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5], vec![4, 5, 6], vec![5, 6, 7], vec![6, 7, 8], vec![7, 0, 9], vec![0, 3, 10], vec![1, 4, 11], vec![2, 5, 12], vec![3, 6, 13], vec![4, 7, 14], vec![5, 0, 15], vec![6, 1, 16], vec![7, 2, 17]]), vec![vec![0, 1, 2, 3, 4, 5, 6], vec![]]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(9, vec![vec![0, 1, 1], vec![0, 2, 1], vec![0, 3, 1], vec![1, 4, 1], vec![1, 5, 1], vec![2, 4, 1], vec![2, 6, 1], vec![3, 5, 1], vec![3, 6, 1], vec![4, 7, 1], vec![5, 7, 1], vec![6, 7, 1], vec![7, 8, 1], vec![0, 4, 1], vec![0, 5, 1], vec![0, 6, 1], vec![1, 6, 1], vec![1, 7, 1], vec![2, 5, 1], vec![2, 7, 1], vec![3, 4, 1], vec![3, 7, 1], vec![4, 6, 1], vec![5, 6, 1], vec![4, 8, 1], vec![5, 8, 1], vec![6, 8, 1]]), vec![vec![], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(7, vec![vec![0, 1, 10], vec![1, 2, 10], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10], vec![5, 6, 10], vec![6, 0, 10], vec![0, 3, 20], vec![1, 4, 20], vec![2, 5, 20], vec![3, 6, 20], vec![4, 0, 20], vec![5, 1, 20], vec![6, 2, 20]]), vec![vec![], vec![0, 1, 2, 3, 4, 5, 6]]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(9, vec![vec![0, 1, 1], vec![0, 2, 2], vec![0, 3, 3], vec![0, 4, 4], vec![0, 5, 5], vec![0, 6, 6], vec![0, 7, 7], vec![0, 8, 8], vec![1, 2, 9], vec![2, 3, 10], vec![3, 4, 11], vec![4, 5, 12], vec![5, 6, 13], vec![6, 7, 14], vec![7, 8, 15], vec![8, 1, 16], vec![1, 3, 17], vec![3, 5, 18], vec![5, 7, 19], vec![7, 1, 20], vec![1, 4, 21], vec![4, 6, 22], vec![6, 8, 23], vec![8, 2, 24], vec![2, 4, 25], vec![4, 7, 26], vec![7, 3, 27], vec![3, 6, 28], vec![6, 1, 29], vec![1, 5, 30], vec![5, 2, 31], vec![2, 7, 32], vec![7, 4, 33], vec![4, 8, 34], vec![8, 3, 35], vec![3, 1, 36]]), vec![vec![0, 1, 2, 3, 4, 5, 6, 7], vec![]]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(8, vec![vec![0, 1, 1], vec![0, 2, 1], vec![0, 3, 1], vec![0, 4, 1], vec![0, 5, 1], vec![0, 6, 1], vec![0, 7, 1], vec![1, 2, 2], vec![2, 3, 2], vec![3, 4, 2], vec![4, 5, 2], vec![5, 6, 2], vec![6, 7, 2], vec![7, 1, 2], vec![1, 3, 2], vec![3, 5, 2], vec![5, 7, 2], vec![7, 2, 2], vec![2, 4, 2], vec![4, 6, 2], vec![6, 1, 2], vec![1, 4, 2], vec![4, 7, 2], vec![7, 3, 2], vec![3, 6, 2], vec![6, 2, 2], vec![2, 5, 2], vec![5, 3, 2], vec![3, 7, 2], vec![7, 4, 2], vec![4, 1, 2], vec![1, 5, 2], vec![5, 2, 2], vec![2, 6, 2], vec![6, 3, 2], vec![3, 1, 2], vec![1, 6, 2], vec![6, 4, 2], vec![4, 2, 2], vec![2, 7, 2], vec![7, 5, 2], vec![5, 4, 2], vec![4, 3, 2], vec![3, 2, 2], vec![2, 1, 2], vec![1, 7, 2], vec![7, 6, 2], vec![6, 5, 2], vec![5, 4, 2], vec![4, 3, 2], vec![3, 1, 2], vec![1, 6, 2], vec![6, 4, 2], vec![4, 2, 2], vec![2, 7, 2], vec![7, 5, 2], vec![5, 3, 2], vec![3, 7, 2]]), vec![vec![0, 1, 2, 3, 4, 5, 6], vec![]]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(12, vec![vec![0, 1, 1], vec![0, 2, 2], vec![0, 3, 3], vec![1, 4, 4], vec![1, 5, 5], vec![2, 6, 6], vec![2, 7, 7], vec![3, 8, 8], vec![3, 9, 9], vec![4, 10, 10], vec![5, 11, 11], vec![6, 11, 12], vec![7, 10, 13], vec![8, 9, 14], vec![9, 10, 15], vec![10, 11, 16]]), vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![]]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 1], vec![4, 5, 1], vec![5, 6, 1], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 0, 1], vec![0, 5, 2], vec![1, 6, 2], vec![2, 7, 2], vec![3, 8, 2], vec![4, 9, 2]]), vec![vec![], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(12, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9], vec![10, 11, 10], vec![11, 0, 11], vec![0, 2, 12], vec![1, 3, 13], vec![2, 4, 14], vec![3, 5, 15], vec![4, 6, 16], vec![5, 7, 17], vec![6, 8, 18], vec![7, 9, 19], vec![8, 10, 20], vec![9, 11, 21], vec![10, 0, 22], vec![11, 1, 23], vec![0, 3, 24], vec![1, 4, 25], vec![2, 5, 26], vec![3, 6, 27], vec![4, 7, 28], vec![5, 8, 29], vec![6, 9, 30], vec![7, 10, 31], vec![8, 11, 32], vec![9, 0, 33], vec![10, 1, 34], vec![11, 2, 35], vec![0, 4, 36], vec![1, 5, 37], vec![2, 6, 38], vec![3, 7, 39], vec![4, 8, 40], vec![5, 9, 41], vec![6, 10, 42], vec![7, 11, 43], vec![8, 0, 44], vec![9, 1, 45], vec![10, 2, 46], vec![11, 3, 47], vec![0, 5, 48], vec![1, 6, 49], vec![2, 7, 50], vec![3, 8, 51], vec![4, 9, 52], vec![5, 10, 53], vec![6, 11, 54], vec![7, 0, 55], vec![8, 1, 56], vec![9, 2, 57], vec![10, 3, 58], vec![11, 4, 59]]), vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![]]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(8, vec![vec![0, 1, 2], vec![0, 2, 3], vec![0, 3, 4], vec![1, 2, 1], vec![1, 3, 5], vec![1, 4, 6], vec![2, 3, 7], vec![2, 4, 8], vec![2, 5, 9], vec![3, 4, 10], vec![3, 5, 11], vec![3, 6, 12], vec![4, 5, 13], vec![4, 6, 14], vec![4, 7, 15], vec![5, 6, 16], vec![5, 7, 17], vec![6, 7, 18]]), vec![vec![3, 0, 2, 5, 8, 11, 14], vec![]]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(15, vec![vec![0, 1, 1], vec![1, 2, 2], vec![2, 3, 3], vec![3, 4, 4], vec![4, 5, 5], vec![5, 6, 6], vec![6, 7, 7], vec![7, 8, 8], vec![8, 9, 9], vec![9, 10, 10], vec![10, 11, 11], vec![11, 12, 12], vec![12, 13, 13], vec![13, 14, 14], vec![14, 0, 15], vec![0, 7, 16], vec![1, 8, 17], vec![2, 9, 18], vec![3, 10, 19], vec![4, 11, 20], vec![5, 12, 21], vec![6, 13, 22], vec![7, 14, 23], vec![8, 0, 24], vec![9, 1, 25], vec![10, 2, 26], vec![11, 3, 27], vec![12, 4, 28], vec![13, 5, 29], vec![14, 6, 30]]), vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13], vec![]]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(9, vec![vec![0, 1, 1], vec![0, 2, 2], vec![1, 2, 3], vec![1, 3, 4], vec![2, 3, 5], vec![2, 4, 6], vec![3, 4, 7], vec![3, 5, 8], vec![4, 5, 9], vec![4, 6, 10], vec![5, 6, 11], vec![5, 7, 12], vec![6, 7, 13], vec![6, 8, 14], vec![7, 8, 15], vec![0, 4, 16], vec![1, 5, 17], vec![2, 6, 18], vec![3, 7, 19], vec![0, 5, 20], vec![1, 6, 21], vec![2, 7, 22], vec![3, 8, 23], vec![0, 6, 24], vec![1, 7, 25], vec![2, 8, 26]]), vec![vec![0, 1, 3, 5, 7, 9, 11, 13], vec![]]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(9, vec![vec![0, 1, 1], vec![0, 2, 10], vec![0, 3, 10], vec![1, 2, 10], vec![1, 3, 10], vec![2, 3, 10], vec![2, 4, 1], vec![2, 5, 10], vec![3, 4, 10], vec![3, 5, 10], vec![4, 5, 1], vec![4, 6, 10], vec![4, 7, 10], vec![5, 6, 10], vec![5, 7, 10], vec![6, 7, 1], vec![6, 8, 10], vec![7, 8, 10]]), vec![vec![0, 6, 10, 15], vec![1, 2, 3, 4, 5, 8, 9, 11, 12, 13, 14, 16, 17]]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(9, vec![vec![0, 1, 2], vec![0, 2, 3], vec![0, 3, 4], vec![1, 2, 5], vec![1, 3, 6], vec![2, 3, 7], vec![3, 4, 8], vec![4, 5, 9], vec![5, 6, 10], vec![6, 7, 11], vec![7, 8, 12], vec![8, 4, 13], vec![0, 5, 14], vec![1, 6, 15], vec![2, 7, 16]]), vec![vec![0, 1, 2, 6, 7, 8, 9, 10], vec![]]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(12, vec![vec![0, 1, 1], vec![0, 2, 2], vec![0, 3, 3], vec![1, 2, 4], vec![1, 3, 5], vec![1, 4, 6], vec![2, 3, 7], vec![2, 4, 8], vec![2, 5, 9], vec![3, 4, 10], vec![3, 5, 11], vec![3, 6, 12], vec![4, 5, 13], vec![4, 6, 14], vec![4, 7, 15], vec![5, 6, 16], vec![5, 7, 17], vec![5, 8, 18], vec![6, 7, 19], vec![6, 8, 20], vec![6, 9, 21], vec![7, 8, 22], vec![7, 9, 23], vec![7, 10, 24], vec![8, 9, 25], vec![8, 10, 26], vec![8, 11, 27], vec![9, 10, 28], vec![9, 11, 29], vec![10, 11, 30]]), vec![vec![0, 1, 2, 5, 8, 11, 14, 17, 20, 23, 26], vec![]]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 1], vec![1, 2, 2], vec![2, 3, 3], vec![3, 4, 4], vec![4, 5, 5], vec![5, 6, 6], vec![6, 7, 7], vec![7, 8, 8], vec![8, 9, 9], vec![9, 0, 10], vec![0, 5, 11], vec![1, 6, 12], vec![2, 7, 13], vec![3, 8, 14], vec![4, 9, 15]]), vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8], vec![]]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(9, vec![vec![0, 1, 2], vec![1, 2, 2], vec![2, 3, 2], vec![3, 4, 2], vec![4, 5, 2], vec![5, 6, 2], vec![6, 7, 2], vec![7, 8, 2], vec![8, 0, 2], vec![0, 2, 3], vec![1, 3, 3], vec![2, 4, 3], vec![3, 5, 3], vec![4, 6, 3], vec![5, 7, 3], vec![6, 8, 3], vec![7, 0, 3], vec![8, 1, 3]]), vec![vec![], vec![0, 1, 2, 3, 4, 5, 6, 7, 8]]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(15, vec![vec![0, 1, 1], vec![0, 2, 2], vec![0, 3, 3], vec![1, 2, 4], vec![1, 3, 5], vec![1, 4, 6], vec![2, 3, 7], vec![2, 4, 8], vec![2, 5, 9], vec![3, 4, 10], vec![3, 5, 11], vec![3, 6, 12], vec![4, 5, 13], vec![4, 6, 14], vec![4, 7, 15], vec![5, 6, 16], vec![5, 7, 17], vec![5, 8, 18], vec![6, 7, 19], vec![6, 8, 20], vec![6, 9, 21], vec![7, 8, 22], vec![7, 9, 23], vec![7, 10, 24], vec![8, 9, 25], vec![8, 10, 26], vec![8, 11, 27], vec![9, 10, 28], vec![9, 11, 29], vec![10, 11, 30], vec![11, 12, 31], vec![11, 13, 32], vec![11, 14, 33], vec![12, 13, 34], vec![12, 14, 35], vec![13, 14, 36]]), vec![vec![0, 1, 2, 5, 8, 11, 14, 17, 20, 23, 26, 30, 31, 32], vec![]]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_critical_and_pseudo_critical_edges(10, vec![vec![0, 1, 2], vec![0, 2, 3], vec![0, 3, 4], vec![1, 2, 1], vec![1, 4, 5], vec![1, 5, 6], vec![2, 3, 2], vec![2, 5, 3], vec![3, 6, 7], vec![3, 7, 8], vec![4, 5, 2], vec![4, 8, 9], vec![5, 6, 4], vec![5, 9, 10], vec![6, 7, 5], vec![7, 8, 6], vec![8, 9, 7]]), vec![vec![3, 0, 6, 10, 7, 12, 14, 15, 16], vec![]]);
}

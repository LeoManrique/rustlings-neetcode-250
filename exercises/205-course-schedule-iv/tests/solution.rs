include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 4], vec![4, 0]]), vec![true, false]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 3], vec![1, 3], vec![0, 2]]), vec![true, true, true]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 4]], vec![vec![0, 3], vec![1, 4], vec![2, 3]]), vec![true, true, false]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2]], vec![vec![0, 2], vec![2, 0]]), vec![true, false]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::check_if_prerequisite(2, vec![], vec![vec![1, 0], vec![0, 1]]), vec![false, false]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]], vec![vec![0, 3], vec![0, 4], vec![3, 4]]), vec![true, true, false]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]], vec![vec![0, 3], vec![0, 4], vec![1, 4], vec![2, 3]]), vec![true, true, true, true]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 2], vec![1, 0], vec![2, 0]], vec![vec![1, 0], vec![1, 2]]), vec![true, true]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]), vec![false, true]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 3], vec![1, 2], vec![2, 0], vec![3, 0]]), vec![true, true, false, false]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::check_if_prerequisite(9, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 8]], vec![vec![0, 8], vec![1, 7], vec![2, 6], vec![3, 5], vec![4, 5], vec![0, 4], vec![1, 2]]), vec![true, true, true, false, false, true, false]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::check_if_prerequisite(15, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![5, 12], vec![6, 13], vec![6, 14]], vec![vec![0, 7], vec![0, 8], vec![0, 9], vec![0, 10], vec![0, 11], vec![0, 12], vec![0, 13], vec![0, 14], vec![1, 11], vec![1, 12], vec![2, 7], vec![2, 8]]), vec![true, true, true, true, true, true, true, true, false, false, false, false]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::check_if_prerequisite(15, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14]], vec![vec![0, 14], vec![1, 13], vec![2, 12], vec![3, 11], vec![4, 10], vec![5, 9], vec![6, 8], vec![7, 7]]), vec![true, true, true, true, true, true, true, true]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 7], vec![5, 7], vec![6, 7]], vec![vec![0, 7], vec![1, 5], vec![2, 4], vec![3, 6], vec![5, 7]]), vec![true, false, false, false, true]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]], vec![vec![0, 9], vec![1, 8], vec![2, 7], vec![3, 6], vec![4, 5], vec![5, 4]]), vec![true, true, true, true, true, false]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9]], vec![vec![0, 9], vec![1, 8], vec![2, 7], vec![3, 6], vec![4, 5], vec![0, 3], vec![1, 2]]), vec![true, true, false, false, false, true, false]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::check_if_prerequisite(12, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 10], vec![6, 11], vec![7, 11], vec![8, 11], vec![9, 11]], vec![vec![0, 7], vec![0, 8], vec![0, 9], vec![0, 10], vec![0, 11], vec![1, 10], vec![1, 11], vec![2, 10], vec![2, 11], vec![3, 11], vec![4, 11]]), vec![true, true, true, true, true, false, true, true, true, true, true]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::check_if_prerequisite(9, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 7], vec![4, 8]], vec![vec![0, 7], vec![0, 8], vec![1, 7], vec![1, 8], vec![2, 7], vec![2, 8], vec![0, 3], vec![0, 4]]), vec![true, true, true, true, false, false, true, true]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9], vec![8, 9]], vec![vec![0, 9], vec![1, 9], vec![2, 9], vec![3, 9], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9], vec![8, 9]]), vec![true, true, true, true, true, true, true, true, true]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::check_if_prerequisite(9, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 7], vec![5, 7], vec![6, 8]], vec![vec![0, 7], vec![0, 8], vec![1, 5], vec![1, 6], vec![2, 7], vec![2, 8], vec![3, 7], vec![3, 8], vec![4, 7], vec![5, 7], vec![6, 8]]), vec![true, true, false, false, true, true, true, true, true, true, true]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]], vec![vec![0, 9], vec![1, 9], vec![2, 9], vec![3, 9], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9], vec![8, 9]]), vec![true, true, true, true, true, true, true, true, true]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![1, 4], vec![2, 3], vec![0, 2]]), vec![true, true, true, true]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::check_if_prerequisite(12, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![5, 10], vec![6, 11]], vec![vec![0, 11], vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6], vec![0, 5], vec![1, 2]]), vec![true, false, false, true, false, false, true, false]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 6], vec![4, 6], vec![5, 6]], vec![vec![0, 6], vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6], vec![1, 5], vec![0, 4], vec![0, 3]]), vec![true, true, true, true, true, true, false, true, true]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::check_if_prerequisite(9, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8]], vec![vec![0, 3], vec![0, 7], vec![1, 8], vec![2, 6], vec![3, 5]]), vec![true, true, true, true, false]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::check_if_prerequisite(9, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![0, 4], vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8]], vec![vec![0, 8], vec![1, 7], vec![2, 6], vec![3, 5], vec![4, 4], vec![5, 3], vec![6, 2], vec![7, 1], vec![8, 0]]), vec![true, true, true, true, true, false, false, false, false]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6]], vec![vec![0, 3], vec![1, 5], vec![2, 6], vec![3, 6]]), vec![true, true, true, true]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]], vec![vec![0, 9], vec![1, 8], vec![2, 7], vec![3, 6], vec![4, 5]]), vec![true, true, true, true, true]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 4], vec![2, 5], vec![4, 6], vec![5, 6]], vec![vec![0, 6], vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6], vec![1, 5]]), vec![true, true, true, false, true, true, false]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]], vec![vec![0, 7], vec![1, 6], vec![2, 5], vec![3, 4], vec![0, 3]]), vec![true, true, true, true, true]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 5], vec![4, 5]], vec![vec![0, 5], vec![1, 4], vec![2, 4], vec![3, 5], vec![1, 3], vec![2, 3]]), vec![true, true, true, true, true, true]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5]], vec![vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 2], vec![1, 5], vec![2, 3], vec![2, 4]]), vec![true, true, true, false, false, false, false]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 5], vec![3, 6], vec![4, 6], vec![5, 6], vec![6, 7]], vec![vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 6], vec![1, 7], vec![2, 6], vec![2, 7]]), vec![true, true, true, true, true, true, true]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 2], vec![1, 2], vec![2, 3], vec![3, 5], vec![3, 6], vec![6, 7], vec![7, 5]], vec![vec![0, 5], vec![0, 7], vec![1, 5], vec![1, 7], vec![2, 5], vec![2, 7], vec![3, 7]]), vec![true, true, true, true, true, true, true]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 7], vec![5, 8], vec![6, 8], vec![7, 9], vec![8, 9]], vec![vec![0, 9], vec![1, 9], vec![2, 9], vec![0, 8], vec![0, 7], vec![0, 6], vec![0, 5], vec![0, 4], vec![0, 3], vec![0, 2], vec![0, 1], vec![1, 8], vec![1, 7], vec![1, 6], vec![1, 5], vec![1, 4], vec![2, 7], vec![2, 6], vec![2, 5], vec![3, 6], vec![3, 5], vec![4, 6], vec![4, 5]]), vec![true, true, true, true, true, true, true, true, true, true, true, false, true, false, false, true, false, true, true, false, false, false, false]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::check_if_prerequisite(12, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![5, 10], vec![6, 11], vec![7, 10], vec![8, 11]], vec![vec![0, 10], vec![0, 11], vec![1, 10], vec![1, 11], vec![2, 10], vec![2, 11], vec![3, 10], vec![3, 11], vec![4, 10], vec![4, 11]]), vec![true, true, true, true, true, true, true, true, false, false]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]], vec![vec![0, 7], vec![1, 6], vec![2, 5], vec![3, 4], vec![4, 3]]), vec![true, true, true, true, false]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![1, 4], vec![2, 3], vec![0, 2], vec![3, 5]]), vec![true, true, true, true, true]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![1, 5], vec![2, 5], vec![0, 4], vec![1, 4], vec![2, 4]]), vec![true, true, true, true, true, true]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![0, 7], vec![1, 6], vec![2, 5], vec![3, 4]], vec![vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![0, 7], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![4, 5], vec![4, 6], vec![4, 7], vec![5, 6], vec![5, 7], vec![6, 7]]), vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]], vec![vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![1, 5], vec![1, 6]]), vec![true, true, true, true, false, false]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]], vec![vec![0, 7], vec![1, 6], vec![2, 5], vec![3, 4], vec![0, 2], vec![1, 3]]), vec![true, true, true, true, true, true]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 9], vec![1, 8], vec![2, 7], vec![3, 6], vec![4, 5], vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 8], vec![1, 7], vec![2, 6], vec![3, 5], vec![4, 6]]), vec![true, true, true, true, false]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]], vec![vec![0, 7], vec![1, 7], vec![2, 7], vec![3, 7], vec![4, 7], vec![5, 7]]), vec![true, true, true, true, true, true]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 5], vec![4, 5], vec![5, 6]], vec![vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![1, 5], vec![1, 6], vec![2, 5], vec![2, 6], vec![3, 6], vec![4, 6]]), vec![true, true, true, true, true, true, true, true, true, true]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::check_if_prerequisite(11, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10]], vec![vec![0, 10], vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6], vec![5, 5], vec![6, 4], vec![7, 3], vec![8, 2], vec![9, 1]]), vec![true, true, true, true, true, true, false, false, false, false]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![0, 4], vec![1, 4], vec![2, 5]]), vec![true, true, true, true]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![0, 2], vec![2, 4], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![2, 5], vec![3, 5], vec![0, 3], vec![5, 0]]), vec![true, true, true, false, false]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::check_if_prerequisite(15, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14]], vec![vec![0, 14], vec![1, 13], vec![2, 12], vec![3, 11], vec![4, 10], vec![5, 9], vec![6, 8], vec![0, 10], vec![10, 0]]), vec![true, true, true, true, true, true, true, true, false]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]], vec![vec![0, 9], vec![2, 8], vec![4, 7], vec![6, 5], vec![0, 8]]), vec![true, true, true, false, true]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::check_if_prerequisite(9, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 8], vec![6, 8]], vec![vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![2, 3], vec![2, 4], vec![2, 7], vec![2, 8], vec![3, 8], vec![4, 8], vec![5, 6], vec![6, 7]]), vec![true, true, true, true, true, true, false, false, true, true, false, false, false, true, false, true, false, false]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 2], vec![1, 2], vec![2, 3], vec![2, 4], vec![3, 5], vec![4, 5], vec![5, 6], vec![5, 7]], vec![vec![0, 5], vec![0, 6], vec![0, 7], vec![1, 5], vec![1, 6], vec![1, 7], vec![2, 6], vec![2, 7], vec![3, 6], vec![3, 7], vec![4, 6], vec![4, 7]]), vec![true, true, true, true, true, true, true, true, true, true, true, true]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![1, 5], vec![2, 5], vec![0, 4]]), vec![true, true, true, true]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 5], vec![1, 6], vec![2, 7], vec![2, 8], vec![3, 9], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]], vec![vec![0, 9], vec![0, 8], vec![1, 7], vec![2, 6], vec![3, 5]]), vec![true, true, true, false, false]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::check_if_prerequisite(12, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![0, 5], vec![1, 6], vec![2, 7], vec![3, 8], vec![4, 9], vec![5, 10], vec![6, 11]], vec![vec![0, 11], vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6], vec![0, 7], vec![1, 8], vec![2, 7], vec![3, 6], vec![4, 5], vec![5, 4]]), vec![true, true, true, true, true, true, true, true, true, true, true, false]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 4], vec![3, 5], vec![3, 6], vec![4, 5], vec![4, 6], vec![5, 6]], vec![vec![0, 5], vec![0, 6], vec![1, 5], vec![1, 6], vec![2, 5], vec![2, 6], vec![3, 6]]), vec![true, true, true, true, true, true, true]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]], vec![vec![0, 9], vec![0, 8], vec![1, 6], vec![3, 9], vec![5, 4]]), vec![true, true, true, true, false]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![0, 3], vec![1, 4], vec![2, 5]], vec![vec![0, 6], vec![1, 5], vec![2, 4], vec![0, 5], vec![3, 4], vec![0, 4], vec![4, 6], vec![1, 3]]), vec![true, true, true, true, true, true, true, true]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::check_if_prerequisite(15, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 5], vec![1, 6], vec![2, 7], vec![2, 8], vec![3, 9], vec![3, 10], vec![4, 11], vec![4, 12], vec![5, 13], vec![5, 14]], vec![vec![0, 13], vec![0, 14], vec![1, 13], vec![1, 14], vec![2, 13], vec![2, 14], vec![3, 13], vec![3, 14], vec![4, 13], vec![4, 14]]), vec![true, true, true, true, false, false, false, false, false, false]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 4], vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 4]], vec![vec![0, 2], vec![0, 3], vec![1, 4], vec![2, 3], vec![3, 2]]), vec![true, true, true, false, false]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 5], vec![1, 6], vec![2, 7], vec![2, 8], vec![3, 9]], vec![vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![1, 9], vec![2, 9], vec![3, 5], vec![3, 6], vec![4, 7]]), vec![true, true, true, true, true, false, false, false, false, false]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::check_if_prerequisite(12, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![6, 11], vec![7, 11], vec![8, 11], vec![9, 11], vec![10, 11]], vec![vec![0, 11], vec![1, 11], vec![2, 11], vec![3, 11], vec![4, 11], vec![5, 11], vec![6, 11], vec![7, 11], vec![8, 11], vec![9, 11], vec![10, 11]]), vec![true, true, true, true, true, true, true, true, true, true, true]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![5, 9], vec![6, 9]], vec![vec![0, 9], vec![1, 8], vec![2, 7], vec![3, 6], vec![4, 5]]), vec![true, true, false, false, false]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![2, 6]], vec![vec![0, 7], vec![1, 6], vec![2, 5], vec![3, 4]]), vec![true, true, false, false]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::check_if_prerequisite(15, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 11], vec![6, 12], vec![7, 13], vec![8, 13], vec![9, 13], vec![10, 14], vec![11, 14], vec![12, 14]], vec![vec![0, 7], vec![0, 8], vec![0, 9], vec![0, 10], vec![0, 11], vec![0, 12], vec![0, 13], vec![0, 14]]), vec![true, true, true, true, true, true, true, true]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::check_if_prerequisite(12, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11]], vec![vec![0, 11], vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6]]), vec![true, true, true, true, true, true]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![0, 5], vec![1, 6], vec![2, 7], vec![3, 8], vec![4, 9]], vec![vec![0, 9], vec![1, 8], vec![2, 7], vec![3, 6], vec![4, 5], vec![0, 8], vec![1, 7], vec![2, 6], vec![3, 5], vec![0, 7], vec![1, 6], vec![2, 5], vec![3, 4]]), vec![true, true, true, true, true, true, true, true, true, true, true, true, true]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 4], vec![5, 6]], vec![vec![0, 4], vec![0, 5], vec![0, 6], vec![1, 3], vec![1, 5], vec![1, 6], vec![2, 5], vec![2, 6], vec![3, 5], vec![4, 6]]), vec![true, true, true, true, false, false, true, true, false, false]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 2], vec![1, 2], vec![2, 3], vec![2, 4], vec![3, 5], vec![4, 5], vec![5, 6], vec![5, 7], vec![6, 8], vec![7, 9]], vec![vec![0, 5], vec![1, 6], vec![2, 8], vec![3, 9], vec![4, 7], vec![5, 8]]), vec![true, true, true, true, true, true]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]], vec![vec![0, 3], vec![0, 5], vec![1, 6], vec![3, 5], vec![4, 6]]), vec![true, true, false, false, false]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::check_if_prerequisite(12, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11]], vec![vec![0, 11], vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6], vec![0, 9], vec![1, 8], vec![2, 7], vec![3, 6], vec![4, 5], vec![0, 8], vec![1, 7], vec![2, 6], vec![3, 5], vec![0, 7], vec![1, 6], vec![2, 5], vec![3, 4]]), vec![true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::check_if_prerequisite(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]], vec![vec![0, 9], vec![1, 9], vec![2, 9], vec![3, 9], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9]]), vec![true, true, true, true, true, true, true, true]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 7], vec![5, 7], vec![6, 7]], vec![vec![0, 7], vec![1, 7], vec![2, 7], vec![3, 7], vec![4, 7], vec![5, 7], vec![6, 7]]), vec![true, true, true, true, true, true, true]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 2]], vec![vec![0, 2], vec![1, 0], vec![2, 0]]), vec![false, false, false]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 3], vec![3, 0], vec![1, 2]]), vec![true, false, true]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]], vec![vec![0, 3], vec![0, 4], vec![1, 2], vec![3, 4]]), vec![true, true, false, false]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 4], vec![1, 4]], vec![vec![0, 2], vec![2, 0], vec![0, 4], vec![4, 0]]), vec![true, false, true, false]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![5, 0], vec![1, 4], vec![4, 1]]), vec![true, false, true, false]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![], vec![vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]]), vec![false, false, false, false]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0], vec![2, 0]], vec![vec![0, 1], vec![2, 1], vec![1, 2]]), vec![false, false, false]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]]), vec![false, false, false, false]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![1, 0], vec![0, 2], vec![2, 3], vec![3, 4]], vec![vec![1, 4], vec![0, 4], vec![0, 3], vec![0, 2]]), vec![true, true, true, true]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![0, 2]], vec![vec![0, 1], vec![1, 2], vec![0, 2], vec![3, 0]]), vec![true, true, true, false]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 2]], vec![vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]]), vec![false, false, false, false]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 4], vec![4, 5]], vec![vec![1, 5], vec![1, 4], vec![2, 5], vec![3, 5], vec![4, 5]]), vec![true, true, true, true, true]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![2, 3]], vec![vec![0, 3], vec![1, 3], vec![0, 2]]), vec![false, false, false]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 3], vec![2, 3]]), vec![false, false, false, false, false]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 3], vec![1, 2]]), vec![true, true]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 4], vec![4, 0], vec![1, 3], vec![3, 1]]), vec![true, false, true, false]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![3, 0]]), vec![true, true, true, false]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 4], vec![2, 4], vec![3, 5], vec![4, 5]], vec![vec![1, 5], vec![2, 5], vec![3, 4]]), vec![true, true, false]);
}

#[test]
fn test_92() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 4], vec![4, 0], vec![1, 3]]), vec![true, false, true]);
}

#[test]
fn test_93() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![5, 3]], vec![vec![5, 0], vec![5, 1], vec![4, 2], vec![4, 3], vec![2, 1]]), vec![true, true, false, false, false]);
}

#[test]
fn test_94() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![0, 1]], vec![vec![0, 1], vec![1, 0], vec![1, 2]]), vec![true, false, false]);
}

#[test]
fn test_95() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]], vec![vec![0, 3], vec![3, 0], vec![1, 2], vec![2, 1]]), vec![true, false, false, false]);
}

#[test]
fn test_96() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]], vec![vec![0, 3], vec![0, 4], vec![1, 2]]), vec![true, true, false]);
}

#[test]
fn test_97() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![], vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 0]]), vec![false, false, false, false]);
}

#[test]
fn test_98() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![2, 3]], vec![vec![0, 2], vec![1, 3], vec![0, 3], vec![2, 1]]), vec![false, false, false, false]);
}

#[test]
fn test_99() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 3], vec![0, 2], vec![1, 3], vec![2, 0]]), vec![true, true, true, false]);
}

#[test]
fn test_100() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2], vec![5, 3]], vec![vec![0, 1], vec![0, 3], vec![0, 5], vec![1, 2], vec![2, 3], vec![3, 4]]), vec![false, false, false, false, false, false]);
}

#[test]
fn test_101() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![1, 0], vec![2, 0], vec![3, 1]], vec![vec![0, 3], vec![1, 3], vec![2, 3]]), vec![false, false, false]);
}

#[test]
fn test_102() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![0, 2]]), vec![false, true, false, false]);
}

#[test]
fn test_103() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], vec![vec![0, 1], vec![1, 3], vec![0, 3]]), vec![false, false, false]);
}

#[test]
fn test_104() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![1, 5], vec![2, 4], vec![3, 5], vec![0, 5]]), vec![true, true, true, false]);
}

#[test]
fn test_105() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 2]], vec![vec![0, 1], vec![1, 0], vec![2, 0]]), vec![false, false, false]);
}

#[test]
fn test_106() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![], vec![vec![0, 1], vec![1, 0], vec![2, 3], vec![3, 2]]), vec![false, false, false, false]);
}

#[test]
fn test_107() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![0, 2]], vec![vec![0, 1], vec![1, 2], vec![0, 2], vec![2, 0], vec![1, 0], vec![2, 1]]), vec![true, true, true, false, false, false]);
}

#[test]
fn test_108() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3], vec![1, 4]], vec![vec![0, 1], vec![1, 0], vec![2, 4]]), vec![false, false, true]);
}

#[test]
fn test_109() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![2, 3]], vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 0]]), vec![true, false, true, false]);
}

#[test]
fn test_110() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0], vec![1, 2]], vec![vec![0, 1], vec![1, 2], vec![0, 2]]), vec![false, true, false]);
}

#[test]
fn test_111() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![5, 4]], vec![vec![0, 5], vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5]]), vec![false, false, false, false, false]);
}

#[test]
fn test_112() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], vec![vec![0, 3], vec![1, 3], vec![2, 3]]), vec![false, false, false]);
}

#[test]
fn test_113() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![1, 5], vec![2, 5]]), vec![true, true, true]);
}

#[test]
fn test_114() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 4], vec![2, 4], vec![3, 4]], vec![vec![0, 4], vec![1, 4], vec![2, 4], vec![3, 4], vec![4, 0]]), vec![true, true, true, true, false]);
}

#[test]
fn test_115() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 4], vec![4, 0], vec![1, 3], vec![3, 0]]), vec![true, false, true, false]);
}

#[test]
fn test_116() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![5, 0], vec![2, 3], vec![3, 2]]), vec![true, false, true, false]);
}

#[test]
fn test_117() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![5, 4]], vec![vec![0, 5], vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5]]), vec![false, false, false, false, false]);
}

#[test]
fn test_118() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 1], vec![0, 4], vec![1, 3], vec![2, 4], vec![4, 0]]), vec![true, true, true, true, false]);
}

#[test]
fn test_119() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2]], vec![vec![0, 5], vec![1, 4], vec![2, 5], vec![0, 3], vec![3, 2]]), vec![false, false, false, false, false]);
}

#[test]
fn test_120() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 4], vec![1, 4], vec![0, 3], vec![1, 3], vec![0, 2], vec![1, 2], vec![2, 4], vec![0, 1], vec![2, 3], vec![4, 0]]), vec![true, true, true, true, true, true, true, true, true, false]);
}

#[test]
fn test_121() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![0, 1]], vec![vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]]), vec![true, false, false, false]);
}

#[test]
fn test_122() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2]], vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]), vec![false, false, false, false, false, false]);
}

#[test]
fn test_123() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 4], vec![4, 0], vec![2, 3]]), vec![true, false, true]);
}

#[test]
fn test_124() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 3], vec![3, 0], vec![1, 2], vec![0, 2]]), vec![true, false, true, true]);
}

#[test]
fn test_125() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]], vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![3, 0]]), vec![true, true, true, false]);
}

#[test]
fn test_126() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![2, 5]]), vec![true, true]);
}

#[test]
fn test_127() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![], vec![vec![1, 0], vec![0, 1], vec![2, 1]]), vec![false, false, false]);
}

#[test]
fn test_128() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0]], vec![vec![0, 1], vec![2, 0], vec![1, 2]]), vec![false, false, false]);
}

#[test]
fn test_129() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0], vec![2, 0]], vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 0]]), vec![false, true, false, false, true]);
}

#[test]
fn test_130() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0], vec![1, 2]], vec![vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]]), vec![false, true, false, false]);
}

#[test]
fn test_131() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 2]], vec![vec![0, 2], vec![1, 3], vec![2, 0], vec![0, 1]]), vec![true, false, false, true]);
}

#[test]
fn test_132() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![3, 4], vec![3, 5]], vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 1]]), vec![true, true, true, false]);
}

#[test]
fn test_133() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5]], vec![vec![1, 4], vec![2, 5], vec![3, 4], vec![0, 5]]), vec![true, false, false, false]);
}

#[test]
fn test_134() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0], vec![1, 2]], vec![vec![0, 1], vec![1, 2], vec![2, 0]]), vec![false, true, false]);
}

#[test]
fn test_135() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]], vec![vec![0, 2], vec![3, 4]]), vec![true, true]);
}

#[test]
fn test_136() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![], vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]]), vec![false, false, false, false]);
}

#[test]
fn test_137() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 2], vec![0, 2]], vec![vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]]), vec![false, false, true, false]);
}

#[test]
fn test_138() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3]], vec![vec![0, 5], vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5]]), vec![false, false, false, false, false]);
}

#[test]
fn test_139() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 4], vec![4, 0], vec![1, 3], vec![2, 4]]), vec![true, false, true, true]);
}

#[test]
fn test_140() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![4, 3], vec![5, 4]], vec![vec![0, 5], vec![1, 5], vec![2, 5], vec![3, 5]]), vec![false, false, false, false]);
}

#[test]
fn test_141() {
    assert_eq!(Solution::check_if_prerequisite(7, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![6, 4], vec![6, 5]], vec![vec![0, 6], vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6]]), vec![false, false, false, false, false, false]);
}

#[test]
fn test_142() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0]], vec![vec![2, 0], vec![2, 1]]), vec![false, false]);
}

#[test]
fn test_143() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0], vec![1, 2]], vec![vec![0, 1], vec![1, 0], vec![0, 2]]), vec![false, true, false]);
}

#[test]
fn test_144() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![2, 3], vec![1, 3], vec![2, 4], vec![3, 5]], vec![vec![0, 3], vec![2, 5], vec![0, 5], vec![1, 2]]), vec![true, true, true, false]);
}

#[test]
fn test_145() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3], vec![3, 4]], vec![vec![0, 4], vec![1, 4], vec![2, 4]]), vec![true, true, true]);
}

#[test]
fn test_146() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5]], vec![vec![0, 3], vec![0, 4], vec![1, 5], vec![2, 3]]), vec![true, true, false, false]);
}

#[test]
fn test_147() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 0]], vec![vec![1, 4], vec![4, 1], vec![0, 3], vec![3, 2]]), vec![true, false, false, false]);
}

#[test]
fn test_148() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 2], vec![2, 0]], vec![vec![1, 0], vec![2, 0], vec![0, 1]]), vec![true, true, false]);
}

#[test]
fn test_149() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 2]], vec![vec![0, 1], vec![0, 2], vec![1, 0]]), vec![false, false, false]);
}

#[test]
fn test_150() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]), vec![false, true]);
}

#[test]
fn test_151() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![0, 1], vec![0, 2]], vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 0], vec![0, 2]]), vec![true, false, false, false, false, true]);
}

#[test]
fn test_152() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![], vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]]), vec![false, false, false, false]);
}

#[test]
fn test_153() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], vec![vec![0, 1], vec![1, 3], vec![0, 3], vec![3, 0]]), vec![false, false, false, true]);
}

#[test]
fn test_154() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]], vec![vec![0, 5], vec![1, 4], vec![2, 3], vec![0, 2], vec![3, 1]]), vec![true, true, true, true, false]);
}

#[test]
fn test_155() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 2]], vec![vec![0, 2], vec![1, 0], vec![2, 1]]), vec![false, false, false]);
}

#[test]
fn test_156() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]], vec![vec![1, 3], vec![2, 3], vec![3, 0], vec![3, 1]]), vec![true, true, false, false]);
}

#[test]
fn test_157() {
    assert_eq!(Solution::check_if_prerequisite(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]], vec![vec![0, 7], vec![1, 7], vec![2, 7], vec![3, 7], vec![4, 7], vec![5, 7], vec![6, 7]]), vec![true, true, true, true, true, true, true]);
}

#[test]
fn test_158() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3], vec![3, 0]]), vec![false, false, false, false, true]);
}

#[test]
fn test_159() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 4], vec![2, 4], vec![3, 4]], vec![vec![0, 4], vec![1, 2], vec![2, 0], vec![3, 1]]), vec![true, false, false, false]);
}

#[test]
fn test_160() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![2, 3], vec![3, 0]]), vec![false, false, false, false, true]);
}

#[test]
fn test_161() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4]], vec![vec![0, 3], vec![0, 4]]), vec![true, true]);
}

#[test]
fn test_162() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 4]], vec![vec![0, 4], vec![0, 3], vec![1, 4], vec![2, 3], vec![3, 4]]), vec![true, true, true, false, false]);
}

#[test]
fn test_163() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![], vec![vec![0, 1], vec![1, 2], vec![2, 0]]), vec![false, false, false]);
}

#[test]
fn test_164() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 2], vec![1, 0]], vec![vec![0, 1], vec![1, 0], vec![2, 0]]), vec![false, true, false]);
}

#[test]
fn test_165() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0], vec![2, 1]], vec![vec![0, 1], vec![1, 2], vec![0, 2]]), vec![false, false, false]);
}

#[test]
fn test_166() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0], vec![1, 2]], vec![vec![0, 1], vec![1, 2]]), vec![false, true]);
}

#[test]
fn test_167() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![3, 0]]), vec![false, false, false, true]);
}

#[test]
fn test_168() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![1, 0], vec![2, 0], vec![3, 0]], vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]), vec![false, false, false, false]);
}

#[test]
fn test_169() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3], vec![2, 4]], vec![vec![0, 3], vec![0, 4], vec![1, 4], vec![2, 0]]), vec![true, true, false, false]);
}

#[test]
fn test_170() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5]], vec![vec![1, 4], vec![4, 1], vec![1, 5], vec![5, 1]]), vec![true, false, true, false]);
}

#[test]
fn test_171() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![vec![1, 0], vec![1, 2]], vec![vec![0, 1], vec![1, 0], vec![1, 2]]), vec![false, true, true]);
}

#[test]
fn test_172() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![], vec![vec![0, 1], vec![1, 0], vec![1, 2]]), vec![false, false, false]);
}

#[test]
fn test_173() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 1], vec![1, 3], vec![0, 3]]), vec![true, true, true]);
}

#[test]
fn test_174() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![0, 1], vec![2, 3], vec![4, 5]], vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![3, 4]]), vec![true, true, true, false]);
}

#[test]
fn test_175() {
    assert_eq!(Solution::check_if_prerequisite(3, vec![], vec![vec![0, 1], vec![1, 0], vec![2, 1]]), vec![false, false, false]);
}

#[test]
fn test_176() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 2], vec![5, 3], vec![5, 4]], vec![vec![0, 5], vec![1, 5], vec![2, 5], vec![3, 5]]), vec![false, false, false, false]);
}

#[test]
fn test_177() {
    assert_eq!(Solution::check_if_prerequisite(6, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![4, 1], vec![5, 2]], vec![vec![3, 0], vec![5, 0], vec![4, 2], vec![1, 2]]), vec![true, true, false, false]);
}

#[test]
fn test_178() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3], vec![1, 4], vec![2, 4]], vec![vec![0, 4], vec![1, 4], vec![2, 3], vec![3, 4]]), vec![true, true, true, false]);
}

#[test]
fn test_179() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![2, 0], vec![1, 0], vec![3, 1], vec![3, 2]], vec![vec![0, 1], vec![2, 0], vec![0, 3], vec![3, 0], vec![2, 3]]), vec![false, true, false, true, false]);
}

#[test]
fn test_180() {
    assert_eq!(Solution::check_if_prerequisite(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 3], vec![1, 3], vec![0, 2], vec![3, 0]]), vec![true, true, true, false]);
}

#[test]
fn test_181() {
    assert_eq!(Solution::check_if_prerequisite(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![vec![0, 1], vec![1, 3], vec![0, 4], vec![2, 3], vec![4, 2]]), vec![true, true, true, true, false]);
}

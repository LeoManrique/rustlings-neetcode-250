include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_min_height_trees(2, vec![vec![0, 1]]), vec![0, 1]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_min_height_trees(10, vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 4], vec![5, 6], vec![5, 7], vec![8, 5], vec![8, 9]]), vec![4, 5]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_min_height_trees(6, vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]), vec![3, 4]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_min_height_trees(5, vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 4]]), vec![0]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_min_height_trees(10, vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 4], vec![5, 6], vec![5, 7], vec![8, 5], vec![9, 5]]), vec![4]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_min_height_trees(7, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6]]), vec![1, 2]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]), vec![1]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_min_height_trees(9, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![3, 7], vec![3, 8]]), vec![0]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_min_height_trees(12, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11]]), vec![1, 2]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_min_height_trees(10, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9]]), vec![0, 1]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_min_height_trees(10, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![8, 9]]), vec![1]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_min_height_trees(12, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11]]), vec![5, 6]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_min_height_trees(10, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![3, 9]]), vec![0]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_min_height_trees(9, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 8]]), vec![0, 1]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_min_height_trees(9, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![5, 7], vec![5, 8]]), vec![0, 2]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_min_height_trees(13, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 7], vec![3, 8], vec![3, 9], vec![4, 10], vec![5, 11], vec![6, 12]]), vec![0]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_min_height_trees(12, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10], vec![9, 11]]), vec![1]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_min_height_trees(18, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 0]]), vec![]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_min_height_trees(15, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 7], vec![3, 8], vec![3, 9], vec![4, 10], vec![5, 11], vec![6, 12], vec![7, 13], vec![8, 14]]), vec![0]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_min_height_trees(18, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![10, 16], vec![10, 17]]), vec![1, 2]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_min_height_trees(8, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![5, 7]]), vec![0, 2]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_min_height_trees(12, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![6, 8], vec![6, 9], vec![7, 10], vec![7, 11]]), vec![1, 3]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_min_height_trees(20, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 10], vec![3, 11], vec![3, 12], vec![4, 13], vec![4, 14], vec![5, 15], vec![5, 16], vec![6, 17], vec![6, 18], vec![7, 19]]), vec![0]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_min_height_trees(14, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10], vec![7, 11], vec![8, 12], vec![9, 13], vec![10, 11]]), vec![4, 5]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_min_height_trees(15, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![5, 12], vec![6, 13], vec![6, 14]]), vec![1, 2]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_min_height_trees(12, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10], vec![7, 11]]), vec![1]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_min_height_trees(18, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![4, 9], vec![5, 10], vec![5, 11], vec![6, 12], vec![6, 13], vec![7, 14], vec![7, 15], vec![9, 16], vec![10, 17]]), vec![1, 2]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_min_height_trees(11, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10]]), vec![0]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_min_height_trees(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]]), vec![4, 5]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_min_height_trees(12, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 7], vec![3, 8], vec![3, 9], vec![3, 10], vec![4, 11]]), vec![0, 1]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_min_height_trees(14, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![5, 12], vec![6, 13]]), vec![0]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_min_height_trees(11, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![6, 8], vec![6, 9], vec![9, 10]]), vec![3]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_min_height_trees(8, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![3, 7]]), vec![0]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_min_height_trees(13, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![5, 12]]), vec![0]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_min_height_trees(15, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10], vec![7, 11], vec![8, 12], vec![9, 13], vec![10, 14]]), vec![1]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_min_height_trees(11, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 7], vec![3, 8], vec![3, 9], vec![4, 10]]), vec![0, 1]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_min_height_trees(12, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11]]), vec![0]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_min_height_trees(18, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17]]), vec![8, 9]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_min_height_trees(8, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7]]), vec![0, 1]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_min_height_trees(11, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10]]), vec![0, 1]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_min_height_trees(18, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![5, 12], vec![6, 13], vec![6, 14], vec![7, 15], vec![8, 16], vec![9, 17]]), vec![0, 1]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_min_height_trees(9, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8]]), vec![4]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_min_height_trees(13, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 7], vec![3, 8], vec![3, 9], vec![3, 10], vec![4, 11], vec![5, 12]]), vec![0, 1]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_min_height_trees(15, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![5, 12], vec![6, 13], vec![6, 14]]), vec![0]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_min_height_trees(11, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10]]), vec![1, 2]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_min_height_trees(25, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![2, 7], vec![2, 8], vec![2, 9], vec![3, 10], vec![3, 11], vec![3, 12], vec![4, 13], vec![4, 14], vec![5, 15], vec![5, 16], vec![6, 17], vec![6, 18], vec![6, 19], vec![7, 20], vec![7, 21], vec![8, 22], vec![9, 23], vec![9, 24]]), vec![0]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_min_height_trees(13, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10], vec![7, 11], vec![8, 12], vec![9, 10], vec![11, 12]]), vec![]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_min_height_trees(16, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![5, 12], vec![6, 13], vec![6, 14], vec![7, 15]]), vec![0, 1]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_min_height_trees(20, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![2, 9], vec![2, 10], vec![2, 11], vec![2, 12], vec![3, 13], vec![3, 14], vec![3, 15], vec![3, 16], vec![4, 17], vec![4, 18], vec![4, 19]]), vec![0]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_min_height_trees(15, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10], vec![7, 11], vec![8, 12], vec![9, 13], vec![10, 14]]), vec![0]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_min_height_trees(10, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9]]), vec![0]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_min_height_trees(20, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![2, 7], vec![2, 8], vec![3, 9], vec![3, 10], vec![3, 11], vec![4, 12], vec![4, 13], vec![5, 14], vec![5, 15], vec![6, 16], vec![6, 17], vec![7, 18], vec![8, 19]]), vec![0]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_min_height_trees(10, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![4, 9]]), vec![0, 1]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_min_height_trees(20, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19]]), vec![9, 10]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_min_height_trees(20, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![5, 12], vec![6, 13], vec![6, 14], vec![7, 15], vec![7, 16], vec![8, 17], vec![8, 18], vec![9, 19]]), vec![0, 1]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_min_height_trees(9, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5], vec![4, 6], vec![6, 7], vec![6, 8]]), vec![2, 4]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_min_height_trees(9, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8]]), vec![0]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_min_height_trees(25, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 7], vec![3, 8], vec![4, 9], vec![4, 10], vec![5, 11], vec![5, 12], vec![6, 13], vec![6, 14], vec![7, 15], vec![7, 16], vec![8, 17], vec![8, 18], vec![9, 19], vec![9, 20], vec![10, 21], vec![10, 22], vec![11, 23], vec![11, 24]]), vec![0]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_min_height_trees(15, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 7], vec![3, 8], vec![3, 9], vec![3, 10], vec![4, 11], vec![5, 12], vec![7, 13], vec![7, 14]]), vec![0]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_min_height_trees(10, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 7], vec![3, 8], vec![3, 9]]), vec![0]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_min_height_trees(11, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10]]), vec![1]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_min_height_trees(15, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10], vec![9, 11], vec![10, 12], vec![11, 13], vec![12, 14]]), vec![1, 2]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_min_height_trees(12, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![0, 11]]), vec![]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_min_height_trees(14, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13]]), vec![6, 7]);
}

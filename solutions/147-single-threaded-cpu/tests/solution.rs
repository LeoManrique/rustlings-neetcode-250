include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::get_order(vec![vec![1, 3], vec![2, 2], vec![3, 1], vec![4, 4]]), vec![0, 2, 1, 3]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::get_order(vec![vec![1, 3], vec![2, 5], vec![8, 2], vec![7, 4], vec![10, 2]]), vec![0, 1, 2, 4, 3]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::get_order(vec![vec![1, 3], vec![2, 2], vec![3, 1], vec![4, 4], vec![5, 5]]), vec![0, 2, 1, 3, 4]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::get_order(vec![vec![3, 1], vec![1, 2], vec![2, 4], vec![1, 4]]), vec![1, 0, 2, 3]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::get_order(vec![vec![5, 2], vec![3, 1], vec![1, 4], vec![4, 3], vec![2, 5]]), vec![2, 1, 0, 3, 4]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::get_order(vec![vec![5, 5], vec![8, 2], vec![1, 9], vec![3, 8]]), vec![2, 1, 0, 3]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::get_order(vec![vec![9, 3], vec![3, 7], vec![8, 10], vec![4, 3], vec![5, 3]]), vec![1, 0, 3, 4, 2]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::get_order(vec![vec![2, 1], vec![3, 1], vec![1, 2], vec![7, 3], vec![8, 4], vec![9, 5]]), vec![2, 0, 1, 3, 4, 5]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::get_order(vec![vec![19, 13], vec![16, 9], vec![21, 10], vec![32, 25], vec![37, 4], vec![49, 24], vec![2, 15], vec![38, 41], vec![37, 34], vec![33, 6], vec![45, 4], vec![18, 18], vec![46, 39], vec![12, 24]]), vec![6, 1, 2, 9, 4, 10, 0, 11, 5, 13, 3, 8, 12, 7]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::get_order(vec![vec![5, 2], vec![7, 2], vec![9, 4], vec![6, 3], vec![8, 2]]), vec![0, 1, 4, 3, 2]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::get_order(vec![vec![7, 10], vec![7, 12], vec![7, 5], vec![7, 4], vec![7, 2]]), vec![4, 3, 2, 0, 1]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::get_order(vec![vec![5, 2], vec![1, 2], vec![3, 1], vec![2, 1]]), vec![1, 2, 3, 0]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]), vec![0, 2, 3, 1]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::get_order(vec![vec![1, 3], vec![2, 5], vec![8, 2], vec![7, 4], vec![6, 1], vec![5, 6], vec![4, 3], vec![3, 2], vec![9, 5], vec![10, 4]]), vec![0, 7, 4, 6, 2, 3, 9, 1, 8, 5]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::get_order(vec![vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6], vec![5, 5], vec![6, 4], vec![7, 3], vec![8, 2], vec![9, 1], vec![10, 10]]), vec![0, 8, 7, 6, 5, 4, 3, 2, 1, 9]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::get_order(vec![vec![100, 1], vec![100, 1], vec![100, 1], vec![100, 1], vec![100, 1], vec![100, 1], vec![100, 1], vec![100, 1], vec![100, 1], vec![100, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10], vec![11, 11], vec![12, 12], vec![13, 13], vec![14, 14], vec![15, 15]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![2, 10], vec![3, 10], vec![4, 10], vec![5, 10]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6], vec![6, 5], vec![7, 4], vec![8, 3], vec![9, 2], vec![10, 1]]), vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![2, 3], vec![4, 5], vec![8, 9], vec![16, 17], vec![32, 33], vec![64, 65], vec![128, 129], vec![256, 257], vec![512, 513], vec![1024, 1025], vec![2048, 2049], vec![4096, 4097], vec![8192, 8193], vec![16384, 16385]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::get_order(vec![vec![1, 100], vec![101, 1], vec![201, 10], vec![301, 100], vec![401, 10], vec![501, 1], vec![601, 100], vec![701, 10], vec![801, 1], vec![901, 100]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 2], vec![1, 3], vec![2, 4], vec![1, 5], vec![2, 6], vec![1, 7], vec![2, 8], vec![1, 9], vec![2, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::get_order(vec![vec![1, 100], vec![2, 200], vec![3, 300], vec![4, 400], vec![5, 500], vec![6, 600], vec![7, 700], vec![8, 800], vec![9, 900], vec![10, 1000]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10]]), vec![0, 10, 1, 11, 2, 12, 3, 13, 4, 14, 5, 15, 6, 16, 7, 17, 8, 18, 9, 19]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::get_order(vec![vec![1, 1000000000], vec![2, 1000000000], vec![3, 1000000000], vec![4, 1000000000], vec![5, 1000000000]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::get_order(vec![vec![1, 100], vec![200, 1], vec![300, 1], vec![400, 1], vec![500, 1], vec![600, 1], vec![700, 1], vec![800, 1], vec![900, 1], vec![1000, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::get_order(vec![vec![1, 5], vec![2, 4], vec![3, 3], vec![4, 2], vec![5, 1], vec![6, 1], vec![7, 2], vec![8, 3], vec![9, 4], vec![10, 5]]), vec![0, 4, 5, 3, 6, 2, 7, 1, 8, 9]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![10, 10], vec![20, 10], vec![30, 10], vec![40, 10], vec![50, 10], vec![60, 10], vec![70, 10], vec![80, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::get_order(vec![vec![2, 5], vec![1, 3], vec![2, 2], vec![4, 1], vec![5, 10], vec![7, 1], vec![8, 2], vec![9, 3], vec![10, 4]]), vec![1, 3, 2, 5, 6, 7, 8, 0, 4]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::get_order(vec![vec![1, 5], vec![2, 4], vec![3, 3], vec![4, 2], vec![5, 1], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7]]), vec![5, 6, 3, 4, 10, 2, 7, 11, 15, 1, 8, 12, 16, 0, 9, 13, 17, 14, 18, 19]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![10, 1], vec![100, 1], vec![1000, 1], vec![10000, 1], vec![100000, 1], vec![1000000, 1], vec![10000000, 1], vec![100000000, 1], vec![1000000000, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::get_order(vec![vec![5, 10], vec![5, 10], vec![5, 10], vec![5, 10], vec![5, 10], vec![5, 10], vec![5, 10], vec![5, 10], vec![5, 10], vec![5, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::get_order(vec![vec![10, 100], vec![100, 10], vec![200, 20], vec![300, 30], vec![400, 40], vec![500, 50], vec![600, 60], vec![700, 70], vec![800, 80], vec![900, 90]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![1, 6], vec![2, 7], vec![3, 8], vec![4, 9], vec![5, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::get_order(vec![vec![1, 5], vec![1, 4], vec![1, 3], vec![1, 2], vec![1, 1], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5]]), vec![4, 5, 3, 6, 2, 7, 1, 8, 0, 9]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::get_order(vec![vec![1, 3], vec![1, 2], vec![1, 1], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![1, 10]]), vec![2, 1, 0, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![2, 1], vec![2, 1], vec![2, 1], vec![3, 3], vec![3, 3], vec![3, 3], vec![4, 4], vec![4, 4], vec![4, 4], vec![5, 5], vec![5, 5], vec![5, 5]]), vec![0, 3, 4, 5, 1, 2, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![1000000000, 1000000000], vec![2, 2], vec![999999999, 999999999], vec![3, 3], vec![999999998, 999999998], vec![4, 4], vec![999999997, 999999997], vec![5, 5], vec![999999996, 999999996]]), vec![0, 2, 4, 6, 8, 9, 7, 5, 3, 1]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::get_order(vec![vec![10, 10], vec![10, 10], vec![10, 10], vec![10, 10], vec![10, 10], vec![10, 10], vec![10, 10], vec![10, 10], vec![10, 10], vec![10, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![2, 1], vec![3, 2], vec![4, 1], vec![5, 2], vec![6, 1], vec![7, 2], vec![8, 1], vec![9, 2], vec![10, 1], vec![1, 2], vec![2, 1], vec![3, 2], vec![4, 1], vec![5, 2], vec![6, 1], vec![7, 2], vec![8, 1], vec![9, 2], vec![10, 1]]), vec![0, 1, 3, 11, 5, 13, 7, 15, 9, 17, 19, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![10, 2], vec![20, 3], vec![30, 4], vec![40, 5], vec![50, 6], vec![60, 7], vec![70, 8], vec![80, 9], vec![90, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::get_order(vec![vec![1, 100], vec![2, 99], vec![3, 98], vec![4, 97], vec![5, 96], vec![6, 95], vec![7, 94], vec![8, 93], vec![9, 92], vec![10, 91]]), vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::get_order(vec![vec![1000000000, 1], vec![1000000000, 2], vec![1000000000, 3], vec![1000000000, 4], vec![1000000000, 5]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1], vec![6, 1], vec![7, 1], vec![8, 1], vec![9, 1], vec![10, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::get_order(vec![vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6], vec![5, 5], vec![6, 4], vec![7, 3], vec![8, 2], vec![9, 1], vec![10, 0]]), vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![1, 2], vec![1, 4], vec![1, 8], vec![1, 16], vec![1, 32], vec![1, 64], vec![1, 128], vec![1, 256], vec![1, 512]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1], vec![6, 1], vec![7, 1], vec![8, 1], vec![9, 1], vec![10, 1], vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6], vec![6, 5], vec![7, 4], vec![8, 3], vec![9, 2], vec![10, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::get_order(vec![vec![5, 10], vec![10, 5], vec![15, 1], vec![20, 2], vec![25, 8], vec![30, 3], vec![35, 4], vec![40, 9], vec![45, 6], vec![50, 7]]), vec![0, 2, 1, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::get_order(vec![vec![1, 1000000000], vec![2, 1000000000], vec![3, 1000000000], vec![4, 1000000000], vec![5, 1000000000], vec![6, 1000000000], vec![7, 1000000000], vec![8, 1000000000], vec![9, 1000000000], vec![10, 1000000000]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![4, 2], vec![5, 2], vec![6, 2], vec![7, 2], vec![8, 2], vec![9, 2], vec![10, 2]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![1000000000, 1], vec![999999999, 2], vec![2, 2], vec![999999998, 3], vec![3, 3], vec![999999997, 4], vec![4, 4], vec![999999996, 5], vec![5, 5]]), vec![0, 3, 5, 7, 9, 8, 1, 2, 4, 6]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1], vec![6, 1], vec![7, 1], vec![8, 1], vec![9, 1], vec![10, 1], vec![1, 2], vec![2, 2], vec![3, 2], vec![4, 2], vec![5, 2], vec![6, 2], vec![7, 2], vec![8, 2], vec![9, 2], vec![10, 2]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![1, 3], vec![2, 1], vec![2, 2], vec![3, 3], vec![3, 4], vec![4, 1], vec![4, 2], vec![5, 3], vec![5, 4], vec![6, 1], vec![6, 2], vec![7, 3], vec![7, 4], vec![8, 1], vec![8, 2], vec![9, 3], vec![9, 4], vec![10, 1], vec![10, 2]]), vec![0, 2, 6, 3, 10, 14, 7, 18, 11, 15, 19, 1, 4, 8, 12, 16, 5, 9, 13, 17]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::get_order(vec![vec![1, 100], vec![2, 50], vec![3, 25], vec![4, 12], vec![5, 6], vec![6, 3], vec![7, 2], vec![8, 1]]), vec![0, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::get_order(vec![vec![1, 20], vec![5, 10], vec![10, 5], vec![15, 2], vec![20, 1], vec![25, 1], vec![30, 1], vec![35, 1], vec![40, 1], vec![45, 1]]), vec![0, 4, 3, 2, 5, 6, 1, 7, 8, 9]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::get_order(vec![vec![100, 100], vec![150, 100], vec![200, 100], vec![250, 100], vec![300, 100], vec![350, 100], vec![400, 100], vec![450, 100], vec![500, 100]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![3, 1], vec![5, 1], vec![7, 1], vec![9, 1], vec![11, 1], vec![13, 1], vec![15, 1], vec![17, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::get_order(vec![vec![3, 3], vec![3, 3], vec![3, 3], vec![3, 3], vec![3, 3], vec![3, 3], vec![3, 3], vec![3, 3], vec![3, 3], vec![3, 3], vec![4, 1]]), vec![0, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::get_order(vec![vec![2, 1], vec![4, 1], vec![6, 1], vec![8, 1], vec![10, 1], vec![1, 2], vec![3, 2], vec![5, 2], vec![7, 2], vec![9, 2], vec![11, 2], vec![13, 2], vec![15, 2], vec![17, 2], vec![19, 2]]), vec![5, 0, 1, 6, 2, 3, 7, 4, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::get_order(vec![vec![10, 10], vec![20, 20], vec![30, 30], vec![40, 40], vec![50, 50], vec![60, 60], vec![70, 70], vec![80, 80], vec![90, 90], vec![100, 100]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![1, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![1, 10], vec![1, 10], vec![1, 10], vec![1, 10], vec![1, 10], vec![1, 10]]), vec![0, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::get_order(vec![vec![10, 10], vec![15, 5], vec![20, 1], vec![25, 2], vec![30, 3], vec![35, 4], vec![40, 5], vec![45, 6], vec![50, 7]]), vec![0, 2, 1, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::get_order(vec![vec![1, 100], vec![10, 100], vec![20, 100], vec![30, 100], vec![40, 100], vec![50, 100], vec![60, 100], vec![70, 100], vec![80, 100], vec![90, 100]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::get_order(vec![vec![1, 5], vec![3, 3], vec![5, 1], vec![7, 2], vec![9, 4], vec![11, 6], vec![13, 5], vec![15, 3], vec![17, 2], vec![19, 1]]), vec![0, 2, 3, 1, 4, 7, 9, 8, 6, 5]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::get_order(vec![vec![1000000000, 1], vec![1000000000, 1], vec![1000000000, 1], vec![1000000000, 1], vec![1000000000, 1]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![11, 1], vec![21, 1], vec![31, 1], vec![41, 1], vec![51, 1], vec![61, 1], vec![71, 1], vec![81, 1], vec![91, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::get_order(vec![vec![10, 1], vec![11, 2], vec![12, 3], vec![13, 4], vec![14, 5], vec![15, 6], vec![16, 7], vec![17, 8], vec![18, 9], vec![19, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![2, 1], vec![2, 1], vec![2, 1], vec![2, 1], vec![2, 1]]), vec![0, 5, 6, 7, 8, 9, 1, 2, 3, 4]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![10, 3], vec![20, 4], vec![30, 5], vec![40, 6], vec![50, 7], vec![60, 8], vec![70, 9], vec![80, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![1, 10], vec![1, 100], vec![1, 1000], vec![1, 10000], vec![1, 100000], vec![1, 1000000], vec![1, 10000000], vec![1, 100000000], vec![1, 1000000000]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::get_order(vec![vec![2, 1], vec![2, 1], vec![2, 1], vec![2, 1], vec![2, 1], vec![2, 1], vec![2, 1], vec![2, 1], vec![2, 1], vec![2, 1], vec![3, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![3, 1], vec![5, 3], vec![7, 4], vec![9, 2], vec![11, 1], vec![13, 5], vec![15, 3], vec![17, 2], vec![19, 1]]), vec![0, 1, 2, 3, 5, 4, 7, 8, 9, 6]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::get_order(vec![vec![9, 5], vec![1, 5], vec![8, 5], vec![3, 5], vec![2, 5], vec![4, 5], vec![6, 5], vec![7, 5], vec![10, 5], vec![11, 5]]), vec![1, 3, 0, 2, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![2, 1], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![10, 2], vec![100, 3], vec![1000, 4], vec![10000, 5], vec![100000, 6], vec![1000000, 7], vec![10000000, 8], vec![100000000, 9], vec![1000000000, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::get_order(vec![vec![10, 3], vec![20, 2], vec![30, 1], vec![40, 4], vec![50, 5], vec![60, 6]]), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![3, 3], vec![5, 5], vec![7, 7], vec![9, 9], vec![2, 2], vec![4, 4], vec![6, 6], vec![8, 8], vec![10, 10]]), vec![0, 5, 1, 6, 2, 7, 3, 8, 4, 9]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::get_order(vec![vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6], vec![5, 5], vec![6, 4], vec![7, 3], vec![8, 2], vec![9, 1]]), vec![0, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![1, 9], vec![1, 8], vec![1, 7], vec![1, 6], vec![1, 5], vec![1, 4], vec![1, 3], vec![1, 2], vec![1, 1], vec![2, 10], vec![2, 9], vec![2, 8], vec![2, 7], vec![2, 6], vec![2, 5], vec![2, 4], vec![2, 3], vec![2, 2], vec![2, 1]]), vec![9, 19, 8, 18, 7, 17, 6, 16, 5, 15, 4, 14, 3, 13, 2, 12, 1, 11, 0, 10]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1], vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![3, 3], vec![5, 5], vec![7, 7], vec![9, 9], vec![11, 11], vec![13, 13], vec![15, 15], vec![17, 17], vec![19, 19]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![2, 4], vec![3, 8], vec![4, 16], vec![5, 32], vec![6, 64], vec![7, 128], vec![8, 256], vec![9, 512], vec![10, 1024]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::get_order(vec![vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![10, 10], vec![20, 10], vec![30, 10], vec![40, 10], vec![50, 10]]), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_92() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![2, 1], vec![1, 2], vec![2, 1], vec![1, 2], vec![2, 1], vec![1, 2], vec![2, 1], vec![1, 2], vec![2, 1]]), vec![0, 1, 3, 5, 7, 9, 2, 4, 6, 8]);
}

#[test]
fn test_93() {
    assert_eq!(Solution::get_order(vec![vec![10, 2], vec![10, 2], vec![10, 2], vec![10, 2], vec![10, 2], vec![10, 2], vec![10, 2], vec![10, 2], vec![10, 2], vec![10, 2]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_94() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2], vec![1, 1], vec![2, 2]]), vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]);
}

#[test]
fn test_95() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![1, 1], vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 3], vec![1, 4], vec![1, 4], vec![1, 5], vec![1, 5]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_96() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![10, 1], vec![20, 1], vec![30, 1], vec![40, 1], vec![50, 1], vec![60, 1], vec![70, 1], vec![80, 1], vec![90, 1], vec![100, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_97() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![2, 5], vec![3, 15], vec![4, 20], vec![5, 1], vec![6, 1], vec![7, 1], vec![8, 1], vec![9, 1], vec![10, 1]]), vec![0, 4, 5, 6, 7, 8, 9, 1, 2, 3]);
}

#[test]
fn test_98() {
    assert_eq!(Solution::get_order(vec![vec![1000000000, 1000000000], vec![1000000001, 1000000000], vec![1000000002, 1000000000], vec![1000000003, 1000000000], vec![1000000004, 1000000000]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_99() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![3, 3], vec![5, 2], vec![7, 1], vec![9, 2], vec![11, 3], vec![13, 1], vec![15, 2], vec![17, 3], vec![19, 1], vec![21, 2], vec![23, 3], vec![25, 1], vec![27, 2], vec![29, 3]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_100() {
    assert_eq!(Solution::get_order(vec![vec![10, 1], vec![20, 1], vec![30, 1], vec![40, 1], vec![50, 1], vec![60, 1], vec![70, 1], vec![80, 1], vec![90, 1], vec![100, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_101() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![5, 1], vec![3, 2], vec![2, 3], vec![7, 4], vec![6, 5], vec![8, 6], vec![9, 7], vec![10, 8]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_102() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1], vec![6, 1], vec![7, 1], vec![8, 1], vec![9, 1], vec![10, 1], vec![11, 1], vec![12, 1], vec![13, 1], vec![14, 1], vec![15, 1]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_103() {
    assert_eq!(Solution::get_order(vec![vec![1, 1000000000], vec![2, 900000000], vec![3, 800000000], vec![4, 700000000], vec![5, 600000000], vec![6, 500000000], vec![7, 400000000], vec![8, 300000000], vec![9, 200000000], vec![10, 100000000]]), vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_104() {
    assert_eq!(Solution::get_order(vec![vec![10, 10], vec![20, 5], vec![30, 15], vec![40, 1], vec![50, 20]]), vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_105() {
    assert_eq!(Solution::get_order(vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2], vec![1, 3], vec![2, 3], vec![1, 4], vec![2, 4], vec![1, 5], vec![2, 5]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_106() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![1, 10]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_107() {
    assert_eq!(Solution::get_order(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20], vec![21, 22], vec![23, 24], vec![25, 26], vec![27, 28], vec![29, 30]]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_108() {
    assert_eq!(Solution::get_order(vec![vec![1, 9], vec![1, 8], vec![1, 7], vec![1, 6], vec![1, 5], vec![1, 4], vec![1, 3], vec![1, 2], vec![1, 1]]), vec![8, 7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_109() {
    assert_eq!(Solution::get_order(vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6], vec![6, 5], vec![7, 4], vec![8, 3], vec![9, 2], vec![10, 1], vec![11, 10], vec![12, 9], vec![13, 8], vec![14, 7], vec![15, 6], vec![16, 5], vec![17, 4], vec![18, 3], vec![19, 2], vec![20, 1]]), vec![0, 9, 8, 7, 6, 19, 18, 17, 16, 5, 15, 4, 14, 3, 13, 2, 12, 1, 11, 10]);
}

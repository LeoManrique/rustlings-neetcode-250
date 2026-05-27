include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::merge(vec![1, 3, 5, 7, 9, 0, 0, 0, 0, 0], 5, vec![2, 4, 6, 8, 10], 5), None);
}

#[test]
fn test_2() {
    assert_eq!(Solution::merge(vec![1], 1, vec![], 0), None);
}

#[test]
fn test_3() {
    assert_eq!(Solution::merge(vec![0], 0, vec![1], 1), None);
}

#[test]
fn test_4() {
    assert_eq!(Solution::merge(vec![4, 5, 6, 0, 0, 0], 3, vec![1, 2, 3], 3), None);
}

#[test]
fn test_5() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3), None);
}

#[test]
fn test_6() {
    assert_eq!(Solution::merge(vec![1, 3, 5, 7, 9, 11, 13, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 8, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22], 11), None);
}

#[test]
fn test_7() {
    assert_eq!(Solution::merge(vec![1, 3, 5, 7, 9, 11, 13, 0, 0, 0, 0, 0, 0, 0], 7, vec![2, 4, 6, 8, 10, 12, 14], 7), None);
}

#[test]
fn test_8() {
    assert_eq!(Solution::merge(vec![-1, 0, 0, 0, 0], 1, vec![-10, -3, -2, -1], 4), None);
}

#[test]
fn test_9() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10, vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1], 10), None);
}

#[test]
fn test_10() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 0], 0, vec![1, 1, 1, 1, 1, 1], 6), None);
}

#[test]
fn test_11() {
    assert_eq!(Solution::merge(vec![10, 20, 30, 40, 50, 60, 0, 0, 0, 0, 0, 0], 6, vec![5, 15, 25, 35, 45, 55], 6), None);
}

#[test]
fn test_12() {
    assert_eq!(Solution::merge(vec![5, 10, 15, 20, 0, 0, 0, 0], 4, vec![1, 2, 8, 16], 4), None);
}

#[test]
fn test_13() {
    assert_eq!(Solution::merge(vec![100, 200, 300, 400, 0, 0, 0, 0, 0, 0, 0, 0], 4, vec![50, 150, 250, 350, 450, 550], 6), None);
}

#[test]
fn test_14() {
    assert_eq!(Solution::merge(vec![-10, -5, 0, 0, 0], 2, vec![-20, -15, 5], 3), None);
}

#[test]
fn test_15() {
    assert_eq!(Solution::merge(vec![5, 10, 15, 20, 0, 0, 0, 0, 0], 4, vec![1, 6, 11, 16, 21], 5), None);
}

#[test]
fn test_16() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 0, 0, 0], 0, vec![-1, -2, -3, -4, -5, -6, -7, -8], 8), None);
}

#[test]
fn test_17() {
    assert_eq!(Solution::merge(vec![1, 2, 4, 7, 8, 0, 0, 0, 0, 0, 0, 0], 5, vec![3, 5, 6, 9, 10, 11], 6), None);
}

#[test]
fn test_18() {
    assert_eq!(Solution::merge(vec![1, 1, 1, 1, 0, 0, 0], 4, vec![1, 1, 1], 3), None);
}

#[test]
fn test_19() {
    assert_eq!(Solution::merge(vec![3, 6, 9, 12, 0, 0, 0, 0, 0], 4, vec![1, 2, 3, 4, 5], 5), None);
}

#[test]
fn test_20() {
    assert_eq!(Solution::merge(vec![-10, -5, 0, 0, 0, 0, 0], 2, vec![-7, -6, -3, -2, -1], 5), None);
}

#[test]
fn test_21() {
    assert_eq!(Solution::merge(vec![100, 200, 300, 0, 0, 0, 0, 0, 0], 3, vec![50, 150, 250, 350, 450, 550], 6), None);
}

#[test]
fn test_22() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0], 5, vec![-5, -4, -3, -2, -1, 0], 6), None);
}

#[test]
fn test_23() {
    assert_eq!(Solution::merge(vec![-10, -8, -6, 0, 0, 0, 0, 0], 3, vec![-9, -7, -5, -3, -1], 5), None);
}

#[test]
fn test_24() {
    assert_eq!(Solution::merge(vec![1000000000, 1000000000, 1000000000, 0, 0, 0, 0, 0, 0, 0], 3, vec![-1000000000, -500000000, -250000000], 3), None);
}

#[test]
fn test_25() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 0], 19, vec![20], 1), None);
}

#[test]
fn test_26() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10, vec![-9, -8, -7, -6, -5, -4, -3, -2, -1, 0], 10), None);
}

#[test]
fn test_27() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0], 10, vec![0, 1, 2, 3, 4, 5], 6), None);
}

#[test]
fn test_28() {
    assert_eq!(Solution::merge(vec![-10, -5, 0, 0, 0, 0, 0, 0], 3, vec![-20, -15, -10, -5, 5], 5), None);
}

#[test]
fn test_29() {
    assert_eq!(Solution::merge(vec![2, 3, 7, 8, 10, 0, 0, 0, 0, 0, 0, 0], 5, vec![1, 4, 5, 6, 9, 11, 12], 7), None);
}

#[test]
fn test_30() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), None);
}

#[test]
fn test_31() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), None);
}

#[test]
fn test_32() {
    assert_eq!(Solution::merge(vec![10, 20, 30, 0, 0, 0, 0, 0], 3, vec![5, 15, 25, 35, 45], 5), None);
}

#[test]
fn test_33() {
    assert_eq!(Solution::merge(vec![5, 10, 15, 20, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5, vec![0, 5, 10, 15, 20, 25, 30, 35, 40, 45], 10), None);
}

#[test]
fn test_34() {
    assert_eq!(Solution::merge(vec![1, 1, 1, 1, 0, 0, 0, 0], 4, vec![1, 1, 1, 1], 4), None);
}

#[test]
fn test_35() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0, vec![-10, -5, -3, -1, 0, 1, 2, 3, 5, 10], 10), None);
}

#[test]
fn test_36() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 0], 0, vec![-5, -3, -1, 0, 1, 3], 6), None);
}

#[test]
fn test_37() {
    assert_eq!(Solution::merge(vec![100, 200, 300, 400, 500, 600, 0, 0, 0, 0, 0, 0], 6, vec![1, 10, 20, 30, 40, 50], 6), None);
}

#[test]
fn test_38() {
    assert_eq!(Solution::merge(vec![-10, -9, -8, -7, -6, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5, vec![-100, -99, -98, -97, -96, -95, -94, -93, -92], 9), None);
}

#[test]
fn test_39() {
    assert_eq!(Solution::merge(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20], 10), None);
}

#[test]
fn test_40() {
    assert_eq!(Solution::merge(vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10, vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2], 10), None);
}

#[test]
fn test_41() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3, vec![4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 17), None);
}

#[test]
fn test_42() {
    assert_eq!(Solution::merge(vec![10, 20, 30, 0, 0, 0, 0, 0], 3, vec![5, 15, 25, 35, 45], 5), None);
}

#[test]
fn test_43() {
    assert_eq!(Solution::merge(vec![100, 200, 300, 400, 500, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5, vec![50, 150, 250, 350, 450, 550, 650, 750, 850, 950], 10), None);
}

#[test]
fn test_44() {
    assert_eq!(Solution::merge(vec![999999999, 999999999, 999999999, 0, 0, 0, 0, 0, 0, 0], 3, vec![1000000000, 1000000001, 1000000002, 1000000003, 1000000004, 1000000005, 1000000006], 7), None);
}

#[test]
fn test_45() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10), None);
}

#[test]
fn test_46() {
    assert_eq!(Solution::merge(vec![5, 8, 10, 12, 0, 0, 0, 0, 0, 0], 4, vec![6, 7, 9, 11, 13, 14], 6), None);
}

#[test]
fn test_47() {
    assert_eq!(Solution::merge(vec![-10, -8, -6, -4, -2, 0, 0, 0, 0, 0], 5, vec![-9, -7, -5, -3, -1], 5), None);
}

#[test]
fn test_48() {
    assert_eq!(Solution::merge(vec![10, 20, 30, 0, 0, 0, 0, 0, 0, 0], 3, vec![1, 2, 3, 4, 5, 6, 7], 7), None);
}

#[test]
fn test_49() {
    assert_eq!(Solution::merge(vec![10, 20, 30, 40, 50, 0, 0, 0, 0, 0, 0, 0], 5, vec![5, 15, 25, 35, 45, 55], 6), None);
}

#[test]
fn test_50() {
    assert_eq!(Solution::merge(vec![1, 1, 2, 2, 3, 3, 0, 0, 0, 0], 6, vec![1, 1, 2, 2], 4), None);
}

#[test]
fn test_51() {
    assert_eq!(Solution::merge(vec![-1, 0, 1, 2, 3, 0, 0, 0, 0], 5, vec![-4, -3, -2, 4], 4), None);
}

#[test]
fn test_52() {
    assert_eq!(Solution::merge(vec![-1, 0, 1, 0, 0, 0, 0, 0], 3, vec![-2, -1, 0, 2, 3], 5), None);
}

#[test]
fn test_53() {
    assert_eq!(Solution::merge(vec![1, 10, 20, 0, 0, 0, 0, 0, 0, 0], 3, vec![2, 3, 4, 5, 6, 7, 8, 9], 7), None);
}

#[test]
fn test_54() {
    assert_eq!(Solution::merge(vec![3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0], 7, vec![3, 3, 3, 3, 3, 3], 6), None);
}

#[test]
fn test_55() {
    assert_eq!(Solution::merge(vec![1, 1, 1, 1, 1, 0, 0, 0, 0], 5, vec![1, 1, 1, 1], 4), None);
}

#[test]
fn test_56() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 20), None);
}

#[test]
fn test_57() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0], 9, vec![0, 0, 0, 0, 0, 0], 6), None);
}

#[test]
fn test_58() {
    assert_eq!(Solution::merge(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10, vec![5, 15, 25, 35, 45, 55, 65, 75, 85, 95], 10), None);
}

#[test]
fn test_59() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3, vec![4, 5, 6, 7, 8, 9, 10, 11, 12], 9), None);
}

#[test]
fn test_60() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 1, 3, 5, 7, 9], 5, vec![2, 4, 6, 8, 10], 5), None);
}

#[test]
fn test_61() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 0, 0], 0, vec![-10, -8, -6, -4, -2, 0, 2], 7), None);
}

#[test]
fn test_62() {
    assert_eq!(Solution::merge(vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0], 6, vec![2, 2, 2, 2, 2, 2], 6), None);
}

#[test]
fn test_63() {
    assert_eq!(Solution::merge(vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5, vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 10), None);
}

#[test]
fn test_64() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0, vec![-5, -3, -1, 1, 3, 5, 7, 9, 11, 13], 10), None);
}

#[test]
fn test_65() {
    assert_eq!(Solution::merge(vec![50, 100, 150, 200, 250, 300, 350, 400, 0, 0, 0, 0], 8, vec![2, 50, 100, 150], 4), None);
}

#[test]
fn test_66() {
    assert_eq!(Solution::merge(vec![1, 10, 20, 0, 0, 0, 0, 0, 0, 0], 3, vec![2, 3, 5, 7, 8, 11, 15], 7), None);
}

#[test]
fn test_67() {
    assert_eq!(Solution::merge(vec![5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5, vec![1, 2, 3, 4, 10, 11, 12, 13, 14, 15, 16], 11), None);
}

#[test]
fn test_68() {
    assert_eq!(Solution::merge(vec![10, 20, 30, 40, 50, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0], 6, vec![5, 15, 25, 35, 45, 55, 65], 7), None);
}

#[test]
fn test_69() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 10, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10), None);
}

#[test]
fn test_70() {
    assert_eq!(Solution::merge(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0, vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1], 10), None);
}

#[test]
fn test_71() {
    assert_eq!(Solution::merge(vec![-10, -5, -3, 0, 0, 0, 0, 0], 3, vec![-8, -7, -6, -4], 4), None);
}

#[test]
fn test_72() {
    assert_eq!(Solution::merge(vec![1, 4, 7, 10, 13, 0, 0, 0, 0, 0], 5, vec![2, 5, 8, 11, 14], 5), None);
}

#[test]
fn test_73() {
    assert_eq!(Solution::merge(vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0], 10, vec![-2, -2, -2, -2, -2, -2, -2], 7), None);
}

#[test]
fn test_74() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0], 5, vec![0, 6, 7, 8, 9, 10, 11], 7), None);
}

#[test]
fn test_75() {
    assert_eq!(Solution::merge(vec![1, 3, 5, 7, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5, vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20], 10), None);
}

#[test]
fn test_76() {
    assert_eq!(Solution::merge(vec![1000000000, 1000000000, 0, 0, 0, 0, 0, 0, 0, 0], 2, vec![-1000000000, -1000000000, -1000000000, -1000000000, -1000000000, -1000000000], 6), None);
}

#[test]
fn test_77() {
    assert_eq!(Solution::merge(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0], 10, vec![-1, -2, -3, -4, -5, -6], 6), None);
}

#[test]
fn test_78() {
    assert_eq!(Solution::merge(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 20, vec![], 0), None);
}

#[test]
fn test_79() {
    assert_eq!(Solution::merge(vec![1, 2, 2, 3, 3, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 7, vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5], 11), None);
}

#[test]
fn test_80() {
    assert_eq!(Solution::merge(vec![100, 200, 300, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3, vec![50, 150, 250, 350, 450, 550], 6), None);
}

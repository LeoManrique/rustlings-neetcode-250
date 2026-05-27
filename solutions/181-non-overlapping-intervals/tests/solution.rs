include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-2, -1], vec![1, 2], vec![-1, 1], vec![2, 3]]), 0);
}

#[test]
fn test_3() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 5], vec![2, 3], vec![4, 6], vec![7, 8]]), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 5]]), 2);
}

#[test]
fn test_6() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-5, -4], vec![-4, -3], vec![-3, -2], vec![-2, -1]]), 0);
}

#[test]
fn test_7() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-2, -1], vec![0, 1], vec![1, 2], vec![2, 3]]), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9]]), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 100], vec![11, 22], vec![1, 11], vec![2, 12]]), 2);
}

#[test]
fn test_10() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![2, 6], vec![3, 5], vec![7, 9]]), 2);
}

#[test]
fn test_11() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-5, 5], vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6]]), 2);
}

#[test]
fn test_13() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]), 2);
}

#[test]
fn test_14() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![1, 2], vec![2, 3], vec![3, 4]]), 0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![3, 5], vec![5, 7], vec![7, 9], vec![9, 11], vec![11, 13], vec![13, 15], vec![15, 17], vec![17, 19], vec![19, 21]]), 0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9]]), 4);
}

#[test]
fn test_17() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 4], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6]]), 1);
}

#[test]
fn test_18() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]]), 0);
}

#[test]
fn test_19() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2]]), 6);
}

#[test]
fn test_20() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-10, 0], vec![-5, -2], vec![0, 5], vec![5, 10]]), 1);
}

#[test]
fn test_21() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 100], vec![2, 99], vec![3, 98], vec![4, 97], vec![5, 96], vec![6, 95], vec![7, 94], vec![8, 93], vec![9, 92], vec![10, 91]]), 9);
}

#[test]
fn test_22() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-5, 0], vec![0, 5], vec![5, 10], vec![10, 15], vec![-4, 1], vec![2, 8]]), 2);
}

#[test]
fn test_23() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![5, 10], vec![6, 8], vec![8, 10], vec![9, 12], vec![10, 14]]), 2);
}

#[test]
fn test_24() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10]]), 0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9]]), 3);
}

#[test]
fn test_26() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11]]), 0);
}

#[test]
fn test_27() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![0, 2], vec![-2, 0], vec![1, 3]]), 2);
}

#[test]
fn test_28() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![5, 15], vec![10, 20], vec![15, 25]]), 2);
}

#[test]
fn test_29() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10000], vec![5000, 15000], vec![10000, 20000], vec![15000, 25000]]), 2);
}

#[test]
fn test_30() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14]]), 0);
}

#[test]
fn test_31() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![0, 2], vec![2, 3], vec![-3, -2]]), 1);
}

#[test]
fn test_32() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-50000, 50000], vec![-49999, 49999], vec![0, 1], vec![1, 2], vec![2, 3]]), 2);
}

#[test]
fn test_33() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-5, 5], vec![-4, 4], vec![-3, 3], vec![-2, 2], vec![-1, 1], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]), 5);
}

#[test]
fn test_34() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![9, 10]]), 1);
}

#[test]
fn test_35() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![0, 10], vec![10, 20], vec![20, 30], vec![30, 40], vec![0, 40]]), 1);
}

#[test]
fn test_36() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6]]), 4);
}

#[test]
fn test_37() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![3, 5], vec![5, 7], vec![7, 9], vec![9, 11], vec![11, 13], vec![13, 15], vec![15, 17], vec![17, 19], vec![19, 21]]), 0);
}

#[test]
fn test_38() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![-2, -1], vec![1, 2], vec![0, 1], vec![2, 3], vec![3, 4], vec![4, 5]]), 1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 5], vec![2, 3], vec![4, 6], vec![7, 8], vec![9, 10], vec![11, 12]]), 1);
}

#[test]
fn test_40() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-50000, 50000], vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]), 1);
}

#[test]
fn test_41() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 20], vec![3, 5], vec![2, 4], vec![6, 7], vec![8, 10], vec![11, 12]]), 2);
}

#[test]
fn test_42() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6], vec![6, 5], vec![7, 4], vec![8, 3], vec![9, 2], vec![10, 1]]), 4);
}

#[test]
fn test_43() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20], vec![1, 11], vec![11, 21], vec![21, 31], vec![31, 41], vec![41, 51]]), 2);
}

#[test]
fn test_44() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![0, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6]]), 1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10]]), 0);
}

#[test]
fn test_46() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![2, 3], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9]]), 3);
}

#[test]
fn test_47() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![1, 2], vec![2, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7]]), 3);
}

#[test]
fn test_48() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 5], vec![3, 7], vec![6, 10], vec![10, 15], vec![14, 20]]), 2);
}

#[test]
fn test_49() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![5, 10], vec![15, 20], vec![5, 15], vec![10, 25], vec![25, 30]]), 2);
}

#[test]
fn test_50() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 20], vec![5, 10], vec![15, 25], vec![20, 30], vec![25, 35], vec![30, 40], vec![35, 45], vec![40, 50], vec![45, 55], vec![50, 60]]), 5);
}

#[test]
fn test_51() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 4], vec![2, 5], vec![3, 6], vec![4, 7], vec![5, 8]]), 3);
}

#[test]
fn test_52() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-5, 5], vec![0, 10], vec![5, 15], vec![10, 20]]), 2);
}

#[test]
fn test_53() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 4], vec![2, 5], vec![3, 6], vec![4, 7]]), 2);
}

#[test]
fn test_54() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![1, 2], vec![1, 1], vec![1, 0], vec![0, 1], vec![0, 0], vec![-1, 0], vec![-2, -1]]), 3);
}

#[test]
fn test_55() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![-2, -1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9]]), 4);
}

#[test]
fn test_56() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-5, -3], vec![-4, -2], vec![-3, -1], vec![-2, 0], vec![-1, 1], vec![0, 2], vec![2, 4], vec![4, 6]]), 3);
}

#[test]
fn test_57() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![0, 5], vec![1, 6], vec![2, 7], vec![3, 8], vec![4, 9], vec![5, 10], vec![6, 11], vec![7, 12], vec![8, 13], vec![9, 14]]), 8);
}

#[test]
fn test_58() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![10, 20], vec![15, 25], vec![20, 30], vec![25, 35], vec![30, 40], vec![35, 45]]), 3);
}

#[test]
fn test_59() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 100], vec![2, 99], vec![3, 98], vec![4, 97], vec![5, 96]]), 4);
}

#[test]
fn test_60() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2]]), 9);
}

#[test]
fn test_61() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10]]), 0);
}

#[test]
fn test_62() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10]]), 0);
}

#[test]
fn test_63() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 4], vec![2, 5], vec![3, 5], vec![3, 6], vec![4, 6], vec![4, 7], vec![5, 7]]), 7);
}

#[test]
fn test_64() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 100], vec![2, 99], vec![3, 98], vec![4, 97], vec![5, 96], vec![6, 95], vec![7, 94]]), 6);
}

#[test]
fn test_65() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![2, 5], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 11]]), 4);
}

#[test]
fn test_66() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![1, 10], vec![1, 11]]), 9);
}

#[test]
fn test_67() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![5, 15], vec![10, 20], vec![15, 25], vec![20, 30], vec![25, 35], vec![30, 40], vec![35, 45], vec![40, 50], vec![45, 55], vec![50, 60], vec![55, 65], vec![60, 70], vec![65, 75], vec![70, 80]]), 7);
}

#[test]
fn test_68() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16]]), 0);
}

#[test]
fn test_69() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-10000, -9000], vec![-8000, -7000], vec![-6000, -5000], vec![-4000, -3000], vec![-2000, -1000], vec![0, 1000], vec![1000, 2000], vec![2000, 3000], vec![3000, 4000]]), 0);
}

#[test]
fn test_70() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7]]), 5);
}

#[test]
fn test_71() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![1, 3], vec![2, 4], vec![3, 5]]), 1);
}

#[test]
fn test_72() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-50000, 50000], vec![49999, 50001], vec![-50001, -49999], vec![-10000, 10000]]), 1);
}

#[test]
fn test_73() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![2, 5], vec![3, 7], vec![4, 9], vec![5, 11], vec![6, 13], vec![7, 15], vec![8, 17], vec![9, 19], vec![10, 21]]), 7);
}

#[test]
fn test_74() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10], vec![7, 11], vec![8, 12], vec![9, 13], vec![10, 14], vec![11, 15], vec![12, 16], vec![13, 17], vec![14, 18], vec![15, 19]]), 11);
}

#[test]
fn test_75() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![3, 4], vec![2, 3], vec![4, 5], vec![3, 4], vec![5, 6]]), 1);
}

#[test]
fn test_76() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![1, 10], vec![11, 20]]), 1);
}

#[test]
fn test_77() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 4], vec![2, 5], vec![3, 6], vec![4, 7], vec![5, 8], vec![6, 9]]), 4);
}

#[test]
fn test_78() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![10, 11]]), 1);
}

#[test]
fn test_79() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![3, 5], vec![5, 7], vec![7, 9], vec![9, 11], vec![11, 13], vec![13, 15], vec![15, 17], vec![17, 19], vec![19, 21], vec![1, 21]]), 1);
}

#[test]
fn test_80() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 4], vec![2, 8], vec![3, 9], vec![4, 10], vec![5, 11], vec![6, 12]]), 4);
}

#[test]
fn test_81() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![10, 20], vec![20, 30], vec![30, 40], vec![15, 25], vec![25, 35], vec![35, 45]]), 3);
}

#[test]
fn test_82() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 5], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6]]), 1);
}

#[test]
fn test_83() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 0], vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]]), 0);
}

#[test]
fn test_84() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![5, 10], vec![10, 15], vec![15, 20], vec![20, 25], vec![25, 30], vec![30, 35], vec![35, 40], vec![40, 45], vec![45, 50]]), 0);
}

#[test]
fn test_85() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-5, -3], vec![-4, -2], vec![-3, -1], vec![-2, 0], vec![-1, 1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6]]), 5);
}

#[test]
fn test_86() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]), 1);
}

#[test]
fn test_87() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3]]), 3);
}

#[test]
fn test_88() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![0, 2], vec![-2, -1], vec![1, 3], vec![2, 4], vec![-3, -2], vec![4, 5]]), 2);
}

#[test]
fn test_89() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![1, 11]]), 1);
}

#[test]
fn test_90() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-10, -5], vec![-5, 0], vec![0, 5], vec![5, 10], vec![10, 15], vec![15, 20], vec![20, 25], vec![25, 30]]), 0);
}

#[test]
fn test_91() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![3, 7], vec![4, 8], vec![5, 9]]), 5);
}

#[test]
fn test_92() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![2, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10]]), 3);
}

#[test]
fn test_93() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![2, 3], vec![2, 3], vec![2, 3], vec![3, 4], vec![3, 4], vec![3, 4]]), 6);
}

#[test]
fn test_94() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7]]), 2);
}

#[test]
fn test_95() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![10, 20], vec![15, 25], vec![20, 30], vec![25, 35], vec![30, 40], vec![5, 15], vec![25, 35]]), 4);
}

#[test]
fn test_96() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![2, 5], vec![3, 6], vec![4, 7], vec![5, 8], vec![6, 9], vec![7, 10]]), 5);
}

#[test]
fn test_97() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-10, 0], vec![0, 10], vec![10, 20], vec![-20, -10], vec![-10, -5], vec![-5, 0], vec![0, 5], vec![5, 10], vec![10, 15]]), 3);
}

#[test]
fn test_98() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10]]), 0);
}

#[test]
fn test_99() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 5], vec![2, 3], vec![4, 6], vec![7, 8], vec![8, 9], vec![9, 10]]), 1);
}

#[test]
fn test_100() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 1000], vec![2, 999], vec![3, 998], vec![4, 997], vec![5, 996], vec![6, 995], vec![7, 994], vec![8, 993]]), 7);
}

#[test]
fn test_101() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 4], vec![2, 6], vec![8, 10], vec![15, 18]]), 1);
}

#[test]
fn test_102() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![-1, 1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6]]), 3);
}

#[test]
fn test_103() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]]), 0);
}

#[test]
fn test_104() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 5], vec![5, 10], vec![10, 15], vec![15, 20]]), 0);
}

#[test]
fn test_105() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2]]), 5);
}

#[test]
fn test_106() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 5], vec![5, 10], vec![10, 15], vec![15, 20], vec![20, 25], vec![1, 25]]), 1);
}

#[test]
fn test_107() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1, 5], vec![3, 7], vec![4, 10], vec![6, 12]]), 2);
}

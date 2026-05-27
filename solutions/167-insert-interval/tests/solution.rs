include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![2, 3]), vec![vec![1, 5]]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![0, 3]), vec![vec![0, 5]]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 10], vec![12, 16]], vec![10, 11]), vec![vec![1, 2], vec![3, 11], vec![12, 16]]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]], vec![1, 8]), vec![vec![1, 8]]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![13, 14]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7]], vec![4, 4]), vec![vec![1, 3], vec![4, 4], vec![5, 7]]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]], vec![0, 9]), vec![vec![0, 9]]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![10, 12]), vec![vec![1, 3], vec![6, 9], vec![10, 12]]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7]], vec![4, 6]), vec![vec![1, 3], vec![4, 7]]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![0, 0]), vec![vec![0, 0], vec![1, 5]]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![6, 8]), vec![vec![1, 5], vec![6, 8]]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]), vec![vec![1, 5], vec![6, 9]]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![4, 8]), vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![5, 6], vec![9, 10], vec![14, 15], vec![19, 20]], vec![3, 18]), vec![vec![1, 2], vec![3, 18], vec![19, 20]]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![8, 10], vec![15, 17]], vec![4, 6]), vec![vec![1, 3], vec![4, 6], vec![8, 10], vec![15, 17]]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![1, 3]), vec![vec![1, 4], vec![5, 6], vec![7, 8], vec![9, 10]]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::insert(vec![vec![1, 5], vec![10, 15], vec![20, 25], vec![30, 35]], vec![6, 14]), vec![vec![1, 5], vec![6, 15], vec![20, 25], vec![30, 35]]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![7, 8]], vec![3, 3]), vec![vec![1, 2], vec![3, 3], vec![4, 5], vec![7, 8]]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::insert(vec![vec![2, 3], vec![5, 6], vec![8, 9], vec![11, 12], vec![14, 15]], vec![7, 13]), vec![vec![2, 3], vec![5, 6], vec![7, 13], vec![14, 15]]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::insert(vec![], vec![0, 0]), vec![vec![0, 0]]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![14, 16]), vec![vec![1, 3], vec![4, 6], vec![8, 10], vec![12, 18]]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![9, 11], vec![13, 15], vec![17, 19]], vec![2, 16]), vec![vec![1, 16], vec![17, 19]]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]], vec![1, 18]), vec![vec![1, 18], vec![19, 20]]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::insert(vec![vec![1, 4], vec![7, 10], vec![12, 16], vec![20, 24]], vec![5, 18]), vec![vec![1, 4], vec![5, 18], vec![20, 24]]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![6, 7], vec![11, 12]], vec![5, 6]), vec![vec![1, 2], vec![5, 7], vec![11, 12]]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]], vec![0, 20]), vec![vec![0, 20]]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]], vec![0, 20]), vec![vec![0, 20]]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]], vec![4, 11]), vec![vec![1, 3], vec![4, 11], vec![12, 14], vec![16, 18]]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::insert(vec![vec![1, 4], vec![9, 10], vec![11, 12], vec![13, 14]], vec![5, 13]), vec![vec![1, 4], vec![5, 14]]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::insert(vec![vec![0, 2], vec![3, 6], vec![8, 10], vec![12, 14], vec![16, 19]], vec![1, 18]), vec![vec![0, 19]]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::insert(vec![vec![1, 2]], vec![3, 4]), vec![vec![1, 2], vec![3, 4]]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14]], vec![1, 14]), vec![vec![1, 14]]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::insert(vec![vec![5, 10], vec![15, 20], vec![25, 30], vec![35, 40]], vec![22, 28]), vec![vec![5, 10], vec![15, 20], vec![22, 30], vec![35, 40]]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![8, 10], vec![15, 17], vec![20, 22]], vec![4, 9]), vec![vec![1, 3], vec![4, 10], vec![15, 17], vec![20, 22]]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 6], vec![8, 10], vec![12, 15]], vec![7, 9]), vec![vec![1, 2], vec![3, 6], vec![7, 10], vec![12, 15]]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![11, 15]), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 15]]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![9, 11], vec![13, 15], vec![17, 19]], vec![2, 18]), vec![vec![1, 19]]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 7], vec![8, 10], vec![12, 15], vec![17, 20]], vec![3, 18]), vec![vec![1, 2], vec![3, 20]]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::insert(vec![vec![1, 100], vec![200, 300], vec![400, 500]], vec![150, 250]), vec![vec![1, 100], vec![150, 300], vec![400, 500]]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::insert(vec![vec![1, 5], vec![10, 15], vec![20, 25], vec![30, 35]], vec![5, 30]), vec![vec![1, 35]]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 8], vec![10, 15], vec![16, 20]], vec![4, 18]), vec![vec![1, 3], vec![4, 20]]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16], vec![19, 20]], vec![10, 19]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 20]]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![2, 16]), vec![vec![1, 16]]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]], vec![17, 20]), vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 20]]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 9], vec![12, 15], vec![18, 21], vec![24, 27]], vec![5, 20]), vec![vec![1, 3], vec![5, 21], vec![24, 27]]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![9, 11], vec![13, 15], vec![17, 19]], vec![8, 14]), vec![vec![1, 3], vec![5, 7], vec![8, 15], vec![17, 19]]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![5, 6], vec![9, 10], vec![13, 14], vec![17, 18]], vec![3, 15]), vec![vec![1, 2], vec![3, 15], vec![17, 18]]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![17, 20]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16], vec![17, 20]]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![7, 15]), vec![vec![1, 2], vec![3, 5], vec![6, 16]]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::insert(vec![], vec![1, 5]), vec![vec![1, 5]]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![0, 15]), vec![vec![0, 15]]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14]], vec![4, 11]), vec![vec![1, 3], vec![4, 11], vec![12, 14]]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::insert(vec![vec![1, 10], vec![20, 30], vec![40, 50]], vec![15, 25]), vec![vec![1, 10], vec![15, 30], vec![40, 50]]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![3, 12]), vec![vec![1, 2], vec![3, 16]]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::insert(vec![vec![1, 10000]], vec![5000, 15000]), vec![vec![1, 15000]]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![6, 8], vec![9, 10], vec![12, 14], vec![16, 18]], vec![3, 13]), vec![vec![1, 2], vec![3, 14], vec![16, 18]]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::insert(vec![vec![1, 5], vec![10, 15], vec![20, 25], vec![30, 35]], vec![18, 22]), vec![vec![1, 5], vec![10, 15], vec![18, 25], vec![30, 35]]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::insert(vec![vec![5, 7], vec![10, 12], vec![15, 17], vec![20, 22]], vec![8, 19]), vec![vec![5, 7], vec![8, 19], vec![20, 22]]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]], vec![3, 17]), vec![vec![1, 2], vec![3, 18], vec![19, 20]]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::insert(vec![vec![3, 5], vec![10, 12], vec![15, 18]], vec![6, 11]), vec![vec![3, 5], vec![6, 12], vec![15, 18]]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![1, 3]), vec![vec![1, 5], vec![6, 7], vec![8, 10], vec![12, 16]]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![0, 0]), vec![vec![0, 0], vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::insert(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]], vec![0, 6]), vec![vec![0, 6]]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![3, 5]), vec![vec![1, 2], vec![3, 6], vec![8, 10], vec![12, 14], vec![16, 18]]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![10, 11]), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 11]]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![2, 9]), vec![vec![1, 10]]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18], vec![20, 22], vec![24, 26], vec![28, 30]], vec![3, 27]), vec![vec![1, 2], vec![3, 27], vec![28, 30]]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![0, 17]), vec![vec![0, 17]]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![5, 7], vec![9, 11], vec![13, 15], vec![17, 19]], vec![3, 12]), vec![vec![1, 2], vec![3, 12], vec![13, 15], vec![17, 19]]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![5, 10]), vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![6, 12]), vec![vec![1, 2], vec![3, 5], vec![6, 16]]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12]], vec![1, 12]), vec![vec![1, 12]]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::insert(vec![vec![1, 5], vec![6, 10], vec![11, 15], vec![16, 20]], vec![1, 20]), vec![vec![1, 20]]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]], vec![18, 20]), vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 20]]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]], vec![7, 13]), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 14], vec![15, 16], vec![17, 18], vec![19, 20]]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18], vec![20, 22], vec![24, 26], vec![28, 30]], vec![5, 25]), vec![vec![1, 2], vec![4, 26], vec![28, 30]]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![9, 11], vec![13, 15]], vec![2, 14]), vec![vec![1, 15]]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![7, 9], vec![11, 13], vec![15, 17], vec![19, 21]], vec![5, 16]), vec![vec![1, 2], vec![3, 4], vec![5, 17], vec![19, 21]]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![5, 9]), vec![vec![1, 2], vec![4, 10], vec![12, 14], vec![16, 18]]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![9, 11], vec![15, 20]], vec![4, 10]), vec![vec![1, 3], vec![4, 11], vec![15, 20]]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![9, 11], vec![13, 15], vec![17, 19]], vec![6, 9]), vec![vec![1, 3], vec![5, 11], vec![13, 15], vec![17, 19]]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![1, 9]), vec![vec![1, 10], vec![12, 16]]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::insert(vec![vec![1, 4], vec![6, 8], vec![10, 12], vec![14, 16]], vec![5, 15]), vec![vec![1, 4], vec![5, 16]]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18], vec![20, 22], vec![24, 26], vec![28, 30]], vec![1, 30]), vec![vec![1, 30]]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 9], vec![11, 13], vec![15, 17], vec![19, 21]], vec![4, 18]), vec![vec![1, 2], vec![3, 18], vec![19, 21]]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![7, 8], vec![10, 11], vec![13, 14], vec![16, 17], vec![19, 20]], vec![3, 19]), vec![vec![1, 2], vec![3, 20]]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![7, 8], vec![10, 11], vec![13, 14], vec![16, 17], vec![19, 20]], vec![6, 18]), vec![vec![1, 2], vec![4, 5], vec![6, 18], vec![19, 20]]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::insert(vec![vec![1, 5], vec![10, 15], vec![20, 25], vec![30, 35], vec![40, 45]], vec![22, 33]), vec![vec![1, 5], vec![10, 15], vec![20, 35], vec![40, 45]]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![6, 7], vec![9, 10]], vec![3, 8]), vec![vec![1, 2], vec![3, 8], vec![9, 10]]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 6], vec![7, 8], vec![11, 12], vec![13, 14]], vec![2, 10]), vec![vec![1, 10], vec![11, 12], vec![13, 14]]);
}

#[test]
fn test_92() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![14, 16]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]]);
}

#[test]
fn test_93() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![7, 8]], vec![5, 7]), vec![vec![1, 2], vec![4, 8]]);
}

#[test]
fn test_94() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![9, 11], vec![13, 15], vec![17, 19]], vec![4, 10]), vec![vec![1, 3], vec![4, 11], vec![13, 15], vec![17, 19]]);
}

#[test]
fn test_95() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]], vec![15, 17]), vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![15, 18]]);
}

#[test]
fn test_96() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]], vec![2, 19]), vec![vec![1, 20]]);
}

#[test]
fn test_97() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![6, 7], vec![9, 10], vec![12, 13]], vec![3, 8]), vec![vec![1, 2], vec![3, 8], vec![9, 10], vec![12, 13]]);
}

#[test]
fn test_98() {
    assert_eq!(Solution::insert(vec![vec![1, 10], vec![15, 20], vec![25, 30], vec![35, 40]], vec![12, 28]), vec![vec![1, 10], vec![12, 30], vec![35, 40]]);
}

#[test]
fn test_99() {
    assert_eq!(Solution::insert(vec![vec![1, 100]], vec![50, 50]), vec![vec![1, 100]]);
}

#[test]
fn test_100() {
    assert_eq!(Solution::insert(vec![vec![1, 10], vec![15, 20], vec![25, 30]], vec![11, 19]), vec![vec![1, 10], vec![11, 20], vec![25, 30]]);
}

#[test]
fn test_101() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]], vec![0, 1]), vec![vec![0, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]]);
}

#[test]
fn test_102() {
    assert_eq!(Solution::insert(vec![vec![1, 2]], vec![0, 0]), vec![vec![0, 0], vec![1, 2]]);
}

#[test]
fn test_103() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![0, 0]), vec![vec![0, 0], vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]]);
}

#[test]
fn test_104() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![0, 11]), vec![vec![0, 11]]);
}

#[test]
fn test_105() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![5, 13]), vec![vec![1, 2], vec![4, 14], vec![16, 18]]);
}

#[test]
fn test_106() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 9], vec![11, 12], vec![15, 18], vec![20, 22]], vec![13, 16]), vec![vec![1, 3], vec![6, 9], vec![11, 12], vec![13, 18], vec![20, 22]]);
}

#[test]
fn test_107() {
    assert_eq!(Solution::insert(vec![vec![1, 5], vec![10, 15], vec![20, 25]], vec![6, 19]), vec![vec![1, 5], vec![6, 19], vec![20, 25]]);
}

#[test]
fn test_108() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![15, 16]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]]);
}

#[test]
fn test_109() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![7, 8], vec![10, 11], vec![13, 14]], vec![3, 12]), vec![vec![1, 2], vec![3, 12], vec![13, 14]]);
}

#[test]
fn test_110() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![0, 1]), vec![vec![0, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]]);
}

#[test]
fn test_111() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![10, 11]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 11], vec![12, 16]]);
}

#[test]
fn test_112() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![10, 10]), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]);
}

#[test]
fn test_113() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]], vec![11, 12]), vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![11, 14], vec![16, 18]]);
}

#[test]
fn test_114() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![11, 12]), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12]]);
}

#[test]
fn test_115() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![10, 12], vec![15, 18]], vec![6, 11]), vec![vec![1, 3], vec![5, 12], vec![15, 18]]);
}

#[test]
fn test_116() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![6, 7], vec![9, 10], vec![12, 13]], vec![3, 11]), vec![vec![1, 2], vec![3, 11], vec![12, 13]]);
}

#[test]
fn test_117() {
    assert_eq!(Solution::insert(vec![vec![1, 2]], vec![2, 2]), vec![vec![1, 2]]);
}

#[test]
fn test_118() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![15, 18]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 18]]);
}

#[test]
fn test_119() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 12], vec![15, 18]], vec![4, 11]), vec![vec![1, 3], vec![4, 12], vec![15, 18]]);
}

#[test]
fn test_120() {
    assert_eq!(Solution::insert(vec![vec![1, 10], vec![14, 20], vec![22, 30], vec![32, 40]], vec![12, 28]), vec![vec![1, 10], vec![12, 30], vec![32, 40]]);
}

#[test]
fn test_121() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![10, 15], vec![20, 25], vec![30, 35]], vec![5, 19]), vec![vec![1, 2], vec![5, 19], vec![20, 25], vec![30, 35]]);
}

#[test]
fn test_122() {
    assert_eq!(Solution::insert(vec![vec![1, 50]], vec![25, 75]), vec![vec![1, 75]]);
}

#[test]
fn test_123() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 10], vec![15, 20], vec![25, 30]], vec![5, 18]), vec![vec![1, 2], vec![4, 20], vec![25, 30]]);
}

#[test]
fn test_124() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]], vec![19, 20]), vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18], vec![19, 20]]);
}

#[test]
fn test_125() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![8, 9], vec![12, 13], vec![16, 17], vec![20, 21]], vec![6, 14]), vec![vec![1, 2], vec![4, 5], vec![6, 14], vec![16, 17], vec![20, 21]]);
}

#[test]
fn test_126() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![5, 15]), vec![vec![1, 2], vec![3, 16]]);
}

#[test]
fn test_127() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![6, 7], vec![9, 10], vec![12, 13]], vec![5, 9]), vec![vec![1, 2], vec![4, 10], vec![12, 13]]);
}

#[test]
fn test_128() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![11, 11]), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 11]]);
}

#[test]
fn test_129() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![6, 7], vec![8, 9], vec![10, 11]], vec![5, 6]), vec![vec![1, 2], vec![4, 7], vec![8, 9], vec![10, 11]]);
}

#[test]
fn test_130() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![17, 20]), vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 20]]);
}

#[test]
fn test_131() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![12, 13]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]]);
}

#[test]
fn test_132() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![0, 1]), vec![vec![0, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]);
}

#[test]
fn test_133() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]], vec![9, 11]), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]]);
}

#[test]
fn test_134() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![10, 18]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 18]]);
}

#[test]
fn test_135() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![7, 8], vec![11, 12]], vec![3, 10]), vec![vec![1, 2], vec![3, 10], vec![11, 12]]);
}

#[test]
fn test_136() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 14], vec![16, 18]], vec![14, 15]), vec![vec![1, 3], vec![5, 7], vec![8, 10], vec![12, 15], vec![16, 18]]);
}

#[test]
fn test_137() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![6, 7], vec![11, 12]], vec![3, 5]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![11, 12]]);
}

#[test]
fn test_138() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![5, 10], vec![15, 20], vec![25, 30], vec![35, 40]], vec![0, 45]), vec![vec![0, 45]]);
}

#[test]
fn test_139() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![9, 11], vec![13, 15], vec![17, 19]], vec![0, 20]), vec![vec![0, 20]]);
}

#[test]
fn test_140() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 10], vec![14, 16], vec![19, 22], vec![24, 26]], vec![11, 15]), vec![vec![1, 3], vec![6, 10], vec![11, 16], vec![19, 22], vec![24, 26]]);
}

#[test]
fn test_141() {
    assert_eq!(Solution::insert(vec![], vec![1, 2]), vec![vec![1, 2]]);
}

#[test]
fn test_142() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![7, 13]), vec![vec![1, 3], vec![5, 6], vec![7, 14], vec![16, 18]]);
}

#[test]
fn test_143() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![6, 7], vec![11, 12]], vec![1, 17]), vec![vec![1, 17]]);
}

#[test]
fn test_144() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![7, 8], vec![10, 12], vec![14, 16], vec![18, 20]], vec![3, 17]), vec![vec![1, 2], vec![3, 17], vec![18, 20]]);
}

#[test]
fn test_145() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14]], vec![3, 7]), vec![vec![1, 2], vec![3, 7], vec![8, 10], vec![12, 14]]);
}

#[test]
fn test_146() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![8, 10], vec![12, 14], vec![16, 18]], vec![6, 11]), vec![vec![1, 2], vec![4, 5], vec![6, 11], vec![12, 14], vec![16, 18]]);
}

#[test]
fn test_147() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![1, 20]), vec![vec![1, 20]]);
}

#[test]
fn test_148() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![5, 7], vec![9, 11], vec![13, 15], vec![17, 19], vec![21, 23], vec![25, 27], vec![29, 31]], vec![4, 29]), vec![vec![1, 3], vec![4, 31]]);
}

#[test]
fn test_149() {
    assert_eq!(Solution::insert(vec![vec![1, 5], vec![10, 15], vec![20, 25], vec![30, 35], vec![40, 45]], vec![26, 34]), vec![vec![1, 5], vec![10, 15], vec![20, 25], vec![26, 35], vec![40, 45]]);
}

#[test]
fn test_150() {
    assert_eq!(Solution::insert(vec![vec![0, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![3, 15]), vec![vec![0, 2], vec![3, 15], vec![16, 18]]);
}

#[test]
fn test_151() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![9, 15]), vec![vec![1, 2], vec![4, 6], vec![8, 15], vec![16, 18]]);
}

#[test]
fn test_152() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 9], vec![13, 17], vec![20, 24], vec![27, 30]], vec![11, 22]), vec![vec![1, 3], vec![6, 9], vec![11, 24], vec![27, 30]]);
}

#[test]
fn test_153() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![2, 4]), vec![vec![1, 6], vec![8, 10], vec![12, 14], vec![16, 18]]);
}

#[test]
fn test_154() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![7, 8]], vec![0, 1]), vec![vec![0, 2], vec![4, 5], vec![7, 8]]);
}

#[test]
fn test_155() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![5, 8], vec![10, 15], vec![20, 25]], vec![3, 23]), vec![vec![1, 2], vec![3, 25]]);
}

#[test]
fn test_156() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![7, 8], vec![10, 11], vec![13, 14], vec![16, 17]], vec![3, 12]), vec![vec![1, 2], vec![3, 12], vec![13, 14], vec![16, 17]]);
}

#[test]
fn test_157() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 5], vec![6, 10], vec![12, 18], vec![20, 25]], vec![3, 11]), vec![vec![1, 2], vec![3, 11], vec![12, 18], vec![20, 25]]);
}

#[test]
fn test_158() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![16, 16]), vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]]);
}

#[test]
fn test_159() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![16, 18]], vec![15, 17]), vec![vec![1, 2], vec![4, 6], vec![8, 10], vec![12, 14], vec![15, 18]]);
}

#[test]
fn test_160() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 9], vec![12, 15], vec![18, 21], vec![24, 27]], vec![10, 25]), vec![vec![1, 3], vec![6, 9], vec![10, 27]]);
}

#[test]
fn test_161() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], vec![8, 10]), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 10]]);
}

#[test]
fn test_162() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16]], vec![0, 16]), vec![vec![0, 16]]);
}

#[test]
fn test_163() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]], vec![5, 15]), vec![vec![1, 2], vec![3, 4], vec![5, 16], vec![17, 18], vec![19, 20]]);
}

#[test]
fn test_164() {
    assert_eq!(Solution::insert(vec![vec![1, 3], vec![6, 9], vec![13, 17], vec![20, 24], vec![27, 30], vec![33, 37], vec![40, 45]], vec![11, 34]), vec![vec![1, 3], vec![6, 9], vec![11, 37], vec![40, 45]]);
}

#[test]
fn test_165() {
    assert_eq!(Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16]], vec![1, 16]), vec![vec![1, 16]]);
}

#[test]
fn test_166() {
    assert_eq!(Solution::insert(vec![vec![1, 2]], vec![0, 3]), vec![vec![0, 3]]);
}

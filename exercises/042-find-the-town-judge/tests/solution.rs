include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_judge(4, vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_judge(1, vec![]), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1]]), -1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_judge(4, vec![vec![1, 2], vec![1, 3], vec![1, 4]]), -1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_judge(3, vec![vec![1, 2], vec![2, 1], vec![1, 3], vec![3, 1]]), -1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 1]]), -1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 4], vec![2, 4], vec![3, 4], vec![4, 5], vec![5, 1]]), -1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 3], vec![6, 3], vec![7, 3], vec![8, 3]]), 3);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_judge(15, vec![vec![1, 15], vec![2, 15], vec![3, 15], vec![4, 15], vec![5, 15], vec![6, 15], vec![7, 15], vec![8, 15], vec![9, 15], vec![10, 15], vec![11, 15], vec![12, 15], vec![13, 15], vec![14, 15]]), 15);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6], vec![6, 5]]), -1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 3], vec![2, 3], vec![3, 5], vec![4, 5], vec![5, 7], vec![6, 7], vec![7, 9], vec![8, 9], vec![9, 10], vec![10, 1]]), -1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 4], vec![2, 4], vec![3, 4], vec![5, 4]]), 4);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 8], vec![2, 8], vec![3, 8], vec![4, 8], vec![5, 8], vec![6, 8], vec![7, 8]]), 8);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5], vec![6, 5], vec![7, 5], vec![8, 5], vec![9, 5], vec![10, 5]]), 5);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_judge(7, vec![vec![1, 7], vec![2, 7], vec![3, 7], vec![4, 7], vec![5, 7], vec![6, 7]]), 7);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5], vec![3, 4], vec![3, 5], vec![4, 1], vec![4, 2], vec![5, 1], vec![5, 3]]), -1);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_judge(11, vec![vec![1, 11], vec![2, 11], vec![3, 11], vec![4, 11], vec![5, 11], vec![6, 11], vec![7, 11], vec![8, 11], vec![9, 11], vec![10, 11]]), 11);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5]]), 5);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 4], vec![2, 4], vec![3, 4], vec![5, 4], vec![6, 4], vec![4, 6]]), -1);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_judge(9, vec![vec![1, 3], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 1]]), -1);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_judge(13, vec![vec![1, 13], vec![2, 13], vec![3, 13], vec![4, 13], vec![5, 13], vec![6, 13], vec![7, 13], vec![8, 13], vec![9, 13], vec![10, 13], vec![11, 13], vec![12, 13]]), 13);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_judge(9, vec![vec![1, 7], vec![2, 7], vec![3, 7], vec![4, 7], vec![5, 7], vec![6, 7], vec![8, 7], vec![9, 7]]), 7);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6], vec![6, 1]]), -1);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_judge(9, vec![vec![1, 9], vec![2, 9], vec![3, 9], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9], vec![8, 9], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 1]]), 9);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1]]), 6);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 10], vec![2, 10], vec![3, 10], vec![4, 10], vec![5, 10], vec![6, 10], vec![7, 10], vec![8, 10], vec![9, 10], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]]), 10);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]), -1);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 2]]), -1);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_judge(7, vec![vec![1, 4], vec![2, 4], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 1]]), -1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 1]]), -1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 3], vec![2, 3], vec![4, 5], vec![5, 4]]), -1);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_judge(7, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5], vec![6, 5], vec![7, 5], vec![5, 1]]), -1);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 3], vec![1, 5], vec![2, 5], vec![4, 5]]), 3);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_judge(9, vec![vec![1, 9], vec![2, 9], vec![3, 9], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9], vec![8, 9], vec![9, 1]]), -1);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 10], vec![2, 10], vec![3, 10], vec![4, 10], vec![5, 10], vec![6, 10], vec![7, 10], vec![8, 10], vec![9, 10], vec![10, 1]]), -1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 4], vec![2, 4], vec![3, 4], vec![4, 5], vec![5, 4]]), -1);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 8], vec![2, 8], vec![3, 8], vec![4, 8], vec![5, 8], vec![6, 8], vec![7, 8], vec![9, 8], vec![10, 8]]), 8);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_judge(12, vec![vec![1, 12], vec![2, 12], vec![3, 12], vec![4, 12], vec![5, 12], vec![6, 12], vec![7, 12], vec![8, 12], vec![9, 12], vec![10, 12], vec![11, 12], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 1]]), 12);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_judge(12, vec![vec![1, 12], vec![2, 12], vec![3, 12], vec![4, 12], vec![5, 12], vec![6, 12], vec![7, 12], vec![8, 12], vec![9, 12], vec![10, 12], vec![11, 12]]), 12);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 3], vec![2, 3], vec![3, 1], vec![4, 5], vec![5, 4], vec![1, 5], vec![2, 6], vec![6, 2], vec![3, 4]]), -1);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5], vec![6, 5], vec![7, 5], vec![8, 5], vec![5, 8]]), -1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10]]), -1);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![1, 9], vec![1, 10]]), -1);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 9], vec![2, 9], vec![3, 9], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9], vec![8, 9], vec![10, 1]]), -1);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5], vec![6, 5], vec![7, 5], vec![8, 5]]), 5);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_judge(9, vec![vec![1, 2], vec![1, 3], vec![2, 3], vec![3, 1], vec![4, 5], vec![4, 6], vec![5, 6], vec![6, 4], vec![7, 8], vec![7, 9], vec![8, 9], vec![9, 7]]), -1);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_judge(7, vec![vec![1, 4], vec![2, 4], vec![3, 4], vec![5, 4], vec![6, 4], vec![7, 4]]), 4);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 8], vec![2, 8], vec![3, 8], vec![4, 8], vec![5, 8], vec![6, 8], vec![7, 8], vec![8, 7]]), -1);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_judge(11, vec![vec![1, 11], vec![2, 11], vec![3, 11], vec![4, 11], vec![5, 11], vec![6, 11], vec![7, 11], vec![8, 11], vec![9, 11], vec![10, 11], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 1]]), 11);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_judge(12, vec![vec![1, 12], vec![2, 12], vec![3, 12], vec![4, 12], vec![5, 12], vec![6, 12], vec![7, 12], vec![8, 12], vec![9, 12], vec![10, 12], vec![11, 12], vec![12, 10]]), -1);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 10], vec![2, 10], vec![3, 10], vec![4, 10], vec![5, 10], vec![6, 10], vec![7, 10], vec![8, 10], vec![9, 10], vec![10, 1], vec![10, 2], vec![10, 3], vec![10, 4], vec![10, 5], vec![10, 6], vec![10, 7], vec![10, 8], vec![10, 9]]), -1);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![4, 3]]), -1);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_judge(10, vec![vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1], vec![6, 1], vec![7, 1], vec![8, 1], vec![9, 1], vec![10, 1]]), 1);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 8], vec![2, 8], vec![3, 8], vec![4, 8], vec![5, 8], vec![6, 8], vec![7, 8], vec![8, 7], vec![8, 1]]), -1);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 1], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 2], vec![8, 3]]), -1);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 1], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 1], vec![5, 2]]), -1);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6]]), -1);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_judge(9, vec![vec![1, 9], vec![2, 9], vec![3, 9], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9], vec![8, 9], vec![1, 2], vec![2, 1], vec![1, 3], vec![3, 1], vec![1, 4], vec![4, 1], vec![1, 5], vec![5, 1], vec![1, 6], vec![6, 1], vec![1, 7], vec![7, 1], vec![1, 8], vec![8, 1]]), 9);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 3]]), 3);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_judge(20, vec![vec![1, 20], vec![2, 20], vec![3, 20], vec![4, 20], vec![5, 20], vec![6, 20], vec![7, 20], vec![8, 20], vec![9, 20], vec![10, 20], vec![11, 20], vec![12, 20], vec![13, 20], vec![14, 20], vec![15, 20], vec![16, 20], vec![17, 20], vec![18, 20], vec![19, 20], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 1]]), 20);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![1, 10]]), -1);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 4], vec![3, 5], vec![4, 5]]), 5);
}

#[test]
fn test_66() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6]]), 6);
}

#[test]
fn test_67() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 8], vec![2, 8], vec![3, 8], vec![4, 8], vec![5, 8], vec![6, 8], vec![7, 8], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 1]]), 8);
}

#[test]
fn test_68() {
    assert_eq!(Solution::find_judge(7, vec![vec![1, 4], vec![2, 4], vec![3, 4], vec![5, 4], vec![6, 4], vec![1, 5], vec![2, 5], vec![3, 5], vec![5, 6], vec![6, 7]]), -1);
}

#[test]
fn test_69() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 3], vec![2, 3], vec![3, 1], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 4]]), -1);
}

#[test]
fn test_70() {
    assert_eq!(Solution::find_judge(9, vec![vec![1, 9], vec![2, 9], vec![3, 9], vec![4, 9], vec![5, 9], vec![6, 9], vec![7, 9], vec![8, 9]]), 9);
}

#[test]
fn test_71() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 2], vec![2, 3], vec![3, 1], vec![4, 5], vec![5, 4]]), -1);
}

#[test]
fn test_72() {
    assert_eq!(Solution::find_judge(7, vec![vec![1, 7], vec![2, 7], vec![3, 7], vec![4, 7], vec![5, 7], vec![6, 7], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 1]]), 7);
}

#[test]
fn test_73() {
    assert_eq!(Solution::find_judge(7, vec![vec![1, 7], vec![2, 7], vec![3, 7], vec![4, 7], vec![5, 7], vec![6, 7], vec![7, 1]]), -1);
}

#[test]
fn test_74() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 2], vec![2, 3], vec![3, 1], vec![4, 5], vec![5, 6]]), -1);
}

#[test]
fn test_75() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 3], vec![2, 3], vec![3, 1], vec![4, 5], vec![5, 4]]), -1);
}

#[test]
fn test_76() {
    assert_eq!(Solution::find_judge(8, vec![vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6], vec![7, 6], vec![8, 6]]), 6);
}

#[test]
fn test_77() {
    assert_eq!(Solution::find_judge(12, vec![vec![1, 12], vec![2, 12], vec![3, 12], vec![4, 12], vec![5, 12], vec![6, 12], vec![7, 12], vec![8, 12], vec![9, 12], vec![10, 12], vec![11, 12], vec![12, 1]]), -1);
}

#[test]
fn test_78() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 6], vec![2, 6], vec![3, 6], vec![4, 6], vec![5, 6], vec![1, 2], vec![2, 1], vec![3, 4], vec![4, 3]]), 6);
}

#[test]
fn test_79() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 10], vec![2, 10], vec![3, 10], vec![4, 10], vec![5, 10], vec![6, 10], vec![7, 10], vec![8, 10], vec![9, 10]]), 10);
}

#[test]
fn test_80() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]), -1);
}

#[test]
fn test_81() {
    assert_eq!(Solution::find_judge(5, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5], vec![5, 1]]), -1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::find_judge(100, vec![vec![1, 100], vec![2, 100], vec![3, 100], vec![4, 100], vec![5, 100], vec![6, 100], vec![7, 100], vec![8, 100], vec![9, 100], vec![10, 100], vec![11, 100], vec![12, 100], vec![13, 100], vec![14, 100], vec![15, 100], vec![16, 100], vec![17, 100], vec![18, 100], vec![19, 100], vec![20, 100], vec![21, 100], vec![22, 100], vec![23, 100], vec![24, 100], vec![25, 100], vec![26, 100], vec![27, 100], vec![28, 100], vec![29, 100], vec![30, 100], vec![31, 100], vec![32, 100], vec![33, 100], vec![34, 100], vec![35, 100], vec![36, 100], vec![37, 100], vec![38, 100], vec![39, 100], vec![40, 100], vec![41, 100], vec![42, 100], vec![43, 100], vec![44, 100], vec![45, 100], vec![46, 100], vec![47, 100], vec![48, 100], vec![49, 100], vec![50, 100], vec![51, 100], vec![52, 100], vec![53, 100], vec![54, 100], vec![55, 100], vec![56, 100], vec![57, 100], vec![58, 100], vec![59, 100], vec![60, 100], vec![61, 100], vec![62, 100], vec![63, 100], vec![64, 100], vec![65, 100], vec![66, 100], vec![67, 100], vec![68, 100], vec![69, 100], vec![70, 100], vec![71, 100], vec![72, 100], vec![73, 100], vec![74, 100], vec![75, 100], vec![76, 100], vec![77, 100], vec![78, 100], vec![79, 100], vec![80, 100], vec![81, 100], vec![82, 100], vec![83, 100], vec![84, 100], vec![85, 100], vec![86, 100], vec![87, 100], vec![88, 100], vec![89, 100], vec![90, 100], vec![91, 100], vec![92, 100], vec![93, 100], vec![94, 100], vec![95, 100], vec![96, 100], vec![97, 100], vec![98, 100], vec![99, 100]]), 100);
}

#[test]
fn test_83() {
    assert_eq!(Solution::find_judge(6, vec![vec![1, 4], vec![2, 4], vec![3, 4], vec![5, 4], vec![6, 4]]), 4);
}

#[test]
fn test_84() {
    assert_eq!(Solution::find_judge(7, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5], vec![6, 5], vec![7, 5]]), 5);
}

#[test]
fn test_85() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![4, 5], vec![6, 5], vec![7, 5], vec![8, 5], vec![9, 5], vec![1, 10], vec![2, 10], vec![3, 10], vec![4, 10]]), -1);
}

#[test]
fn test_86() {
    assert_eq!(Solution::find_judge(10, vec![vec![1, 3], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10]]), -1);
}

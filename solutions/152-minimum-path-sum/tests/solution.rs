include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20]]), 60);
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_path_sum(vec![vec![10, 15, 20], vec![25, 30, 35], vec![40, 45, 50]]), 130);
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_path_sum(vec![vec![1]]), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_path_sum(vec![vec![7]]), 7);
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2], vec![3, 4]]), 7);
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2], vec![3, 4], vec![5, 6]]), 13);
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]), 7);
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]), 30);
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10], vec![1, 1, 1, 1, 1], vec![9, 7, 5, 3, 1]]), 9);
}

#[test]
fn test_11() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1], vec![1, 3, 1, 5, 1], vec![2, 1, 2, 3, 4]]), 17);
}

#[test]
fn test_12() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12], vec![13, 14, 15]]), 48);
}

#[test]
fn test_13() {
    assert_eq!(Solution::min_path_sum(vec![vec![9, 1, 4, 3, 2], vec![1, 2, 5, 6, 7], vec![3, 6, 8, 5, 4], vec![2, 5, 7, 8, 1], vec![6, 7, 8, 9, 10]]), 41);
}

#[test]
fn test_14() {
    assert_eq!(Solution::min_path_sum(vec![vec![5, 9, 6], vec![11, 5, 2], vec![3, 12, 4], vec![15, 7, 8]]), 33);
}

#[test]
fn test_15() {
    assert_eq!(Solution::min_path_sum(vec![vec![7, 1, 3, 4, 2], vec![5, 6, 7, 2, 1], vec![3, 2, 1, 5, 6], vec![4, 3, 2, 1, 5], vec![6, 7, 8, 9, 1]]), 26);
}

#[test]
fn test_16() {
    assert_eq!(Solution::min_path_sum(vec![vec![10, 20, 30, 40], vec![20, 30, 40, 50], vec![30, 40, 50, 60], vec![40, 50, 60, 70]]), 280);
}

#[test]
fn test_17() {
    assert_eq!(Solution::min_path_sum(vec![vec![50, 40, 30, 20, 10], vec![45, 41, 36, 31, 21], vec![40, 35, 30, 25, 15], vec![35, 30, 25, 20, 10], vec![30, 25, 20, 15, 5]]), 201);
}

#[test]
fn test_18() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]]), 46);
}

#[test]
fn test_19() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 1, 1, 1, 1, 1], vec![1, 2, 3, 4, 5, 1], vec![1, 3, 5, 7, 9, 1], vec![1, 5, 9, 13, 17, 1], vec![1, 7, 15, 21, 28, 1], vec![1, 1, 1, 1, 1, 1]]), 11);
}

#[test]
fn test_20() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]]), 14);
}

#[test]
fn test_21() {
    assert_eq!(Solution::min_path_sum(vec![vec![5, 3, 2, 1], vec![1, 2, 10, 1], vec![4, 3, 2, 20], vec![7, 1, 6, 5]]), 23);
}

#[test]
fn test_22() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 5, 7, 9, 11], vec![2, 4, 6, 8, 10], vec![10, 8, 6, 4, 2], vec![11, 13, 15, 17, 19], vec![12, 14, 16, 18, 20]]), 64);
}

#[test]
fn test_23() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 0, 0, 0, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 1, 0], vec![0, 0, 0, 0, 1]]), 2);
}

#[test]
fn test_24() {
    assert_eq!(Solution::min_path_sum(vec![vec![9, 5, 4, 9, 6, 5, 6, 9, 8], vec![2, 9, 4, 7, 6, 7, 5, 7, 5], vec![5, 9, 9, 3, 8, 4, 7, 8, 5], vec![8, 4, 5, 9, 6, 7, 4, 9, 5], vec![6, 4, 3, 2, 7, 1, 8, 9, 5], vec![2, 9, 1, 5, 4, 8, 9, 1, 5], vec![9, 5, 5, 1, 3, 5, 1, 3, 5], vec![2, 9, 3, 8, 7, 5, 9, 2, 1], vec![9, 5, 1, 5, 3, 5, 6, 9, 5]]), 62);
}

#[test]
fn test_25() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 9, 9, 9, 9, 9, 9], vec![1, 1, 9, 9, 9, 9, 9], vec![1, 1, 1, 9, 9, 9, 9], vec![1, 1, 1, 1, 9, 9, 9], vec![1, 1, 1, 1, 1, 9, 9], vec![1, 1, 1, 1, 1, 1, 9], vec![1, 1, 1, 1, 1, 1, 1]]), 13);
}

#[test]
fn test_26() {
    assert_eq!(Solution::min_path_sum(vec![vec![9, 8, 7, 6, 5], vec![4, 3, 2, 1, 0], vec![9, 8, 7, 6, 5], vec![4, 3, 2, 1, 0], vec![9, 8, 7, 6, 5]]), 29);
}

#[test]
fn test_27() {
    assert_eq!(Solution::min_path_sum(vec![vec![0, 1, 2, 3, 4], vec![4, 3, 2, 1, 0], vec![0, 1, 2, 3, 4], vec![4, 3, 2, 1, 0], vec![0, 1, 2, 3, 4]]), 14);
}

#[test]
fn test_28() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3], vec![6, 5, 4], vec![7, 8, 9]]), 19);
}

#[test]
fn test_29() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]]), 33);
}

#[test]
fn test_30() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 5, 7, 9, 11], vec![2, 4, 6, 8, 10, 12], vec![13, 15, 17, 19, 21, 23], vec![24, 22, 20, 18, 16, 14]]), 80);
}

#[test]
fn test_31() {
    assert_eq!(Solution::min_path_sum(vec![vec![9, 1, 4, 8, 7], vec![6, 5, 3, 2, 1], vec![9, 1, 4, 8, 7], vec![6, 5, 3, 2, 1], vec![9, 1, 4, 8, 7]]), 33);
}

#[test]
fn test_32() {
    assert_eq!(Solution::min_path_sum(vec![vec![20, 30, 40], vec![5, 15, 25], vec![10, 20, 30], vec![1, 2, 3]]), 41);
}

#[test]
fn test_33() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]]), 75);
}

#[test]
fn test_34() {
    assert_eq!(Solution::min_path_sum(vec![vec![0, 1, 2, 3, 4, 5, 6], vec![6, 5, 4, 3, 2, 1, 0], vec![1, 3, 5, 7, 9, 11, 13], vec![13, 11, 9, 7, 5, 3, 1], vec![0, 2, 4, 6, 8, 10, 12]]), 38);
}

#[test]
fn test_35() {
    assert_eq!(Solution::min_path_sum(vec![vec![9, 8, 7, 6, 5], vec![4, 3, 2, 1, 0], vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9], vec![9, 8, 7, 6, 5]]), 37);
}

#[test]
fn test_36() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]]), 9);
}

#[test]
fn test_37() {
    assert_eq!(Solution::min_path_sum(vec![vec![10, 15, 20, 25], vec![30, 35, 40, 45], vec![50, 55, 60, 65], vec![70, 75, 80, 85], vec![90, 95, 100, 105]]), 370);
}

#[test]
fn test_38() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 20, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1]]), 11);
}

#[test]
fn test_39() {
    assert_eq!(Solution::min_path_sum(vec![vec![7, 5, 12, 6, 8], vec![9, 6, 10, 3, 7], vec![8, 15, 4, 2, 9], vec![7, 11, 12, 5, 3], vec![6, 10, 7, 4, 8]]), 49);
}

#[test]
fn test_40() {
    assert_eq!(Solution::min_path_sum(vec![vec![7, 1, 3, 4, 1], vec![2, 1, 3, 1, 1], vec![1, 5, 1, 2, 2], vec![3, 2, 4, 3, 2], vec![4, 1, 5, 2, 3]]), 21);
}

#[test]
fn test_41() {
    assert_eq!(Solution::min_path_sum(vec![vec![100, 200, 150], vec![50, 100, 125], vec![175, 200, 250], vec![225, 150, 300]]), 900);
}

#[test]
fn test_42() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 4, 5, 2, 3], vec![1, 2, 8, 9, 3], vec![3, 7, 6, 2, 1], vec![5, 5, 4, 2, 3]]), 22);
}

#[test]
fn test_43() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 1, 1, 1, 1, 1, 1], vec![1, 2, 2, 2, 2, 2, 1], vec![1, 2, 3, 3, 3, 2, 1], vec![1, 2, 3, 4, 3, 2, 1], vec![1, 2, 3, 3, 3, 2, 1], vec![1, 2, 2, 2, 2, 2, 1], vec![1, 1, 1, 1, 1, 1, 1]]), 13);
}

#[test]
fn test_44() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 5, 7, 9, 11, 13], vec![2, 4, 6, 8, 10, 12, 14], vec![13, 11, 9, 7, 5, 3, 1], vec![14, 12, 10, 8, 6, 4, 2], vec![15, 17, 19, 21, 23, 25, 27]]), 66);
}

#[test]
fn test_45() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6]]), 26);
}

#[test]
fn test_46() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 5, 3], vec![2, 9, 4], vec![5, 6, 7], vec![8, 9, 10]]), 30);
}

#[test]
fn test_47() {
    assert_eq!(Solution::min_path_sum(vec![vec![5, 4, 3, 2, 1], vec![4, 3, 2, 1, 5], vec![3, 2, 1, 5, 4], vec![2, 1, 5, 4, 3], vec![1, 5, 4, 3, 2]]), 29);
}

#[test]
fn test_48() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 9, 1, 9, 1, 9], vec![9, 1, 9, 1, 9, 1], vec![1, 9, 1, 9, 1, 9], vec![9, 1, 9, 1, 9, 1], vec![1, 9, 1, 9, 1, 9]]), 50);
}

#[test]
fn test_49() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 1, 5], vec![2, 1, 2, 1], vec![5, 3, 1, 1], vec![4, 2, 1, 1]]), 9);
}

#[test]
fn test_50() {
    assert_eq!(Solution::min_path_sum(vec![vec![0, 3, 1, 2, 9], vec![3, 4, 1, 3, 8], vec![5, 6, 7, 8, 9], vec![1, 9, 8, 7, 6], vec![9, 8, 7, 6, 5]]), 34);
}

#[test]
fn test_51() {
    assert_eq!(Solution::min_path_sum(vec![vec![200, 0, 0, 0], vec![0, 200, 0, 0], vec![0, 0, 200, 0], vec![0, 0, 0, 200]]), 400);
}

#[test]
fn test_52() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 1, 1, 1, 1], vec![1, 2, 2, 2, 1], vec![1, 2, 3, 2, 1], vec![1, 2, 2, 2, 1], vec![1, 1, 1, 1, 1]]), 9);
}

#[test]
fn test_53() {
    assert_eq!(Solution::min_path_sum(vec![vec![1], vec![1], vec![1], vec![1], vec![1]]), 5);
}

#[test]
fn test_54() {
    assert_eq!(Solution::min_path_sum(vec![vec![10, 3, 12, 5, 8], vec![7, 4, 6, 9, 2], vec![1, 14, 11, 13, 7], vec![15, 8, 10, 4, 12], vec![9, 6, 3, 16, 5]]), 58);
}

#[test]
fn test_55() {
    assert_eq!(Solution::min_path_sum(vec![vec![9, 1, 4, 8, 7], vec![6, 3, 5, 9, 2], vec![2, 8, 0, 1, 5], vec![3, 6, 9, 4, 6], vec![6, 8, 2, 4, 0]]), 27);
}

#[test]
fn test_56() {
    assert_eq!(Solution::min_path_sum(vec![vec![2, 1, 2, 5, 1, 2], vec![5, 2, 3, 1, 3, 5], vec![1, 1, 2, 1, 2, 1], vec![2, 1, 1, 1, 1, 1], vec![1, 1, 2, 3, 1, 1]]), 12);
}

#[test]
fn test_57() {
    assert_eq!(Solution::min_path_sum(vec![vec![5, 3, 2, 1], vec![6, 7, 8, 9], vec![1, 2, 3, 4], vec![5, 4, 3, 2]]), 22);
}

#[test]
fn test_58() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 5], vec![3, 1, 1], vec![1, 5, 1]]), 6);
}

#[test]
fn test_59() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 9, 9, 9, 9], vec![1, 1, 9, 9, 9], vec![1, 1, 1, 9, 9], vec![1, 1, 1, 1, 9], vec![1, 1, 1, 1, 1]]), 9);
}

#[test]
fn test_60() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1], vec![1, 3, 5, 7, 9], vec![9, 7, 5, 3, 1], vec![2, 4, 6, 8, 10]]), 32);
}

#[test]
fn test_61() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 100, 200, 300, 400], vec![1, 1, 1, 1, 1], vec![100, 100, 100, 100, 100], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]]), 108);
}

#[test]
fn test_62() {
    assert_eq!(Solution::min_path_sum(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]), 0);
}

#[test]
fn test_63() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1]]), 11);
}

#[test]
fn test_64() {
    assert_eq!(Solution::min_path_sum(vec![vec![10, 33, 14, 7, 24], vec![13, 4, 7, 6, 23], vec![6, 15, 66, 3, 18], vec![8, 41, 9, 12, 32], vec![1, 5, 3, 9, 20]]), 75);
}

#[test]
fn test_65() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]]), 85);
}

#[test]
fn test_66() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]]), 12);
}

#[test]
fn test_67() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 8, 7, 3, 6], vec![1, 1, 7, 1, 2, 8], vec![4, 9, 10, 5, 1, 6], vec![6, 7, 2, 8, 2, 5], vec![3, 4, 9, 1, 8, 4]]), 25);
}

#[test]
fn test_68() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 1, 1], vec![1, 5, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]]), 7);
}

#[test]
fn test_69() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 9, 1, 9, 1, 9, 1], vec![9, 1, 9, 1, 9, 1, 9], vec![1, 9, 1, 9, 1, 9, 1], vec![9, 1, 9, 1, 9, 1, 9], vec![1, 9, 1, 9, 1, 9, 1], vec![9, 1, 9, 1, 9, 1, 9], vec![1, 9, 1, 9, 1, 9, 1]]), 61);
}

#[test]
fn test_70() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10], vec![3, 5, 7, 9, 11], vec![4, 6, 8, 10, 12], vec![5, 7, 9, 11, 13]]), 55);
}

#[test]
fn test_71() {
    assert_eq!(Solution::min_path_sum(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 0);
}

#[test]
fn test_72() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1]]), 10);
}

#[test]
fn test_73() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]]), 7);
}

#[test]
fn test_74() {
    assert_eq!(Solution::min_path_sum(vec![vec![10, 11, 12, 13], vec![14, 15, 16, 17], vec![18, 19, 20, 21], vec![22, 23, 24, 25]]), 109);
}

#[test]
fn test_75() {
    assert_eq!(Solution::min_path_sum(vec![vec![7, 8, 9, 10, 11, 12], vec![6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 10, 11, 12]]), 46);
}

#[test]
fn test_76() {
    assert_eq!(Solution::min_path_sum(vec![vec![100, 200, 300], vec![400, 500, 600], vec![700, 800, 900]]), 2100);
}

#[test]
fn test_77() {
    assert_eq!(Solution::min_path_sum(vec![vec![5, 3, 2, 7], vec![8, 6, 4, 3], vec![1, 2, 6, 5], vec![9, 7, 4, 8]]), 30);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 2, 8], vec![4, 4, 6], vec![10, 8, 10]], 12), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 2, 7], vec![2, 7, 9], vec![4, 1, 3]], 6), false);
}

#[test]
fn test_3() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 5), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 6], vec![1, 3, 5], vec![4, 4, 8]], 9), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 4, 5], vec![3, 1, 3], vec![3, 3, 7]], 5), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 3, 6], vec![2, 3, 7], vec![1, 8, 9]], 11), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7], vec![2, 5, 8]], 6), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4), false);
}

#[test]
fn test_11() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 2, 4], vec![3, 1, 3], vec![8, 6, 9], vec![2, 4, 8]], 15), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::car_pooling(vec![vec![100, 0, 1], vec![1, 1, 2], vec![1, 2, 3], vec![1, 3, 4], vec![1, 4, 5], vec![1, 5, 6], vec![1, 6, 7], vec![1, 7, 8], vec![1, 8, 9]], 105), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1], vec![1, 0, 1]], 8), false);
}

#[test]
fn test_14() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 3], vec![1, 1, 4], vec![1, 1, 5], vec![1, 1, 6], vec![1, 1, 7], vec![1, 1, 8], vec![1, 1, 9], vec![1, 1, 10]], 7), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::car_pooling(vec![vec![100, 0, 100], vec![50, 50, 150], vec![25, 75, 200]], 150), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 0, 100], vec![3, 10, 50], vec![4, 50, 90], vec![2, 60, 80]], 20), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 20], vec![5, 5, 15], vec![3, 10, 20], vec![2, 15, 20]], 20), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 3], vec![10, 2, 6], vec![1, 2, 10], vec![2, 5, 7]], 17), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 1], vec![1, 1, 2], vec![1, 2, 3], vec![1, 3, 4], vec![1, 4, 5], vec![1, 5, 6]], 2), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 20], vec![2, 0, 15], vec![3, 0, 10], vec![4, 0, 5]], 20), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 10], vec![1, 10, 20], vec![1, 20, 30], vec![1, 30, 40], vec![1, 40, 50]], 10), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 1, 10], vec![2, 2, 8], vec![1, 5, 7], vec![4, 6, 9]], 10), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::car_pooling(vec![vec![30, 0, 2], vec![50, 1, 3], vec![40, 2, 4], vec![20, 3, 5], vec![10, 4, 6]], 80), false);
}

#[test]
fn test_24() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 5], vec![10, 5, 10], vec![10, 10, 15]], 25), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 6], vec![5, 3, 8], vec![5, 5, 10], vec![5, 7, 12]], 15), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 500], vec![2, 100, 400], vec![3, 200, 300], vec![4, 300, 400], vec![5, 400, 500]], 15), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 4], vec![3, 2, 5], vec![4, 3, 6], vec![1, 4, 7], vec![1, 5, 8], vec![1, 6, 9]], 10), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 5], vec![2, 1, 6], vec![1, 2, 7], vec![1, 3, 8], vec![1, 4, 9], vec![1, 5, 10]], 5), false);
}

#[test]
fn test_29() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 2, 3], vec![2, 3, 5], vec![2, 5, 7], vec![2, 7, 9]], 3), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 10], vec![10, 10, 20], vec![10, 20, 30], vec![10, 30, 40], vec![10, 40, 50]], 40), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 1, 2], vec![4, 2, 3], vec![5, 3, 4], vec![4, 4, 5], vec![3, 5, 6], vec![2, 6, 7]], 15), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 2], vec![2, 2, 3], vec![3, 3, 4], vec![4, 4, 5], vec![5, 5, 6], vec![6, 6, 7], vec![7, 7, 8], vec![8, 8, 9]], 7), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 1], vec![10, 1, 2], vec![10, 2, 3], vec![10, 3, 4], vec![10, 4, 5], vec![10, 5, 6]], 10), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 3], vec![2, 2, 5], vec![3, 3, 7], vec![4, 4, 8], vec![5, 5, 9]], 10), false);
}

#[test]
fn test_35() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 2], vec![2, 2, 3], vec![3, 3, 4], vec![4, 4, 5], vec![5, 5, 6], vec![6, 6, 7], vec![7, 7, 8], vec![8, 8, 9]], 28), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::car_pooling(vec![vec![100, 0, 1000], vec![100, 100, 900], vec![100, 200, 800], vec![100, 300, 700], vec![100, 400, 600]], 300), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 2], vec![2, 2, 3], vec![3, 3, 4], vec![4, 4, 5], vec![5, 5, 6], vec![6, 6, 7]], 6), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 3], vec![5, 3, 5], vec![5, 5, 7], vec![5, 7, 9], vec![5, 9, 11]], 20), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 1, 2], vec![10, 2, 3], vec![10, 3, 4], vec![10, 4, 5], vec![10, 5, 6], vec![10, 6, 7]], 25), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 10], vec![5, 0, 5], vec![1, 6, 10], vec![2, 5, 8], vec![3, 7, 12]], 20), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::car_pooling(vec![vec![20, 0, 30], vec![10, 10, 20], vec![5, 20, 30]], 25), false);
}

#[test]
fn test_42() {
    assert_eq!(Solution::car_pooling(vec![vec![30, 0, 10], vec![20, 10, 20], vec![10, 20, 30], vec![5, 30, 40], vec![5, 40, 50]], 55), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 3], vec![2, 3, 6], vec![3, 6, 9], vec![4, 9, 12], vec![5, 12, 15], vec![6, 15, 18], vec![7, 18, 21], vec![8, 21, 24]], 20), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::car_pooling(vec![vec![20, 0, 1000], vec![20, 1000, 2000], vec![20, 2000, 3000], vec![20, 3000, 4000], vec![20, 4000, 5000]], 60), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 3], vec![2, 2, 5], vec![4, 4, 8], vec![3, 5, 9]], 12), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 2], vec![1, 2, 3], vec![1, 3, 4], vec![1, 4, 5], vec![1, 5, 6], vec![1, 6, 7]], 3), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 100], vec![2, 50, 100], vec![3, 75, 100]], 6), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 3], vec![5, 3, 6], vec![10, 6, 9], vec![5, 9, 12]], 20), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 0, 1], vec![3, 1, 2], vec![3, 2, 3], vec![3, 3, 4], vec![3, 4, 5], vec![3, 5, 6], vec![3, 6, 7], vec![3, 7, 8], vec![3, 8, 9]], 9), true);
}

#[test]
fn test_50() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 0, 3], vec![2, 3, 5], vec![7, 5, 10], vec![1, 10, 15]], 12), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 1, 4], vec![5, 1, 5], vec![2, 3, 7], vec![2, 4, 8]], 14), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 1], vec![10, 0, 2], vec![10, 0, 3], vec![10, 0, 4], vec![10, 0, 5]], 50), true);
}

#[test]
fn test_53() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 1, 3], vec![2, 2, 4], vec![1, 3, 5], vec![4, 4, 6], vec![2, 5, 7], vec![1, 6, 8]], 10), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::car_pooling(vec![vec![4, 1, 10], vec![2, 2, 5], vec![3, 3, 6], vec![1, 5, 12]], 15), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 0, 5], vec![4, 5, 10], vec![3, 10, 15], vec![2, 15, 20], vec![1, 20, 25]], 15), true);
}

#[test]
fn test_56() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 10], vec![10, 10, 20], vec![10, 20, 30], vec![10, 30, 40], vec![10, 40, 50]], 20), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 4], vec![3, 2, 5], vec![2, 3, 6], vec![1, 4, 7], vec![4, 5, 8], vec![3, 6, 9]], 8), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 0, 1], vec![3, 1, 2], vec![3, 2, 3], vec![3, 3, 4], vec![3, 4, 5], vec![3, 5, 6]], 10), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 10], vec![20, 5, 15], vec![30, 10, 20], vec![40, 15, 25], vec![50, 20, 30]], 100), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 1, 10], vec![5, 2, 5], vec![3, 5, 7], vec![2, 7, 8]], 20), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 0, 1], vec![3, 1, 2], vec![3, 2, 3], vec![3, 3, 4], vec![3, 4, 5]], 12), true);
}

#[test]
fn test_62() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7], vec![1, 1, 3], vec![4, 5, 9]], 7), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 1000], vec![2, 2, 999], vec![3, 3, 998], vec![4, 4, 997], vec![5, 5, 996]], 15), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 5], vec![3, 5, 9], vec![2, 9, 12], vec![1, 12, 15]], 15), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 10], vec![20, 10, 20], vec![10, 15, 25], vec![20, 20, 30], vec![30, 25, 35]], 40), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 3], vec![3, 2, 4], vec![4, 3, 5], vec![5, 4, 6], vec![6, 5, 7]], 15), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 2], vec![2, 2, 3], vec![2, 3, 4], vec![2, 4, 5], vec![2, 5, 6]], 4), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 2, 7], vec![3, 2, 7], vec![3, 2, 7]], 9), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::car_pooling(vec![vec![50, 0, 500], vec![50, 100, 600], vec![50, 200, 700], vec![50, 300, 800], vec![50, 400, 900]], 150), false);
}

#[test]
fn test_70() {
    assert_eq!(Solution::car_pooling(vec![vec![100, 0, 1], vec![100, 1, 2], vec![100, 2, 3], vec![100, 3, 4], vec![100, 4, 5]], 100), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::car_pooling(vec![vec![50, 10, 20], vec![30, 20, 30], vec![20, 30, 40], vec![10, 40, 50], vec![5, 50, 60]], 100), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::car_pooling(vec![vec![50, 0, 500], vec![50, 500, 1000]], 100), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 2, 5], vec![3, 2, 7], vec![3, 3, 6], vec![3, 4, 8]], 9), false);
}

#[test]
fn test_74() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 3], vec![2, 3, 8], vec![3, 5, 9], vec![1, 6, 10]], 15), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 0, 3], vec![4, 2, 5], vec![3, 4, 8], vec![2, 6, 10]], 14), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 0, 2], vec![2, 2, 4], vec![2, 4, 6], vec![2, 6, 8], vec![2, 8, 10], vec![2, 10, 12]], 12), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 2], vec![1, 1, 2], vec![1, 1, 2], vec![1, 1, 2], vec![1, 1, 2]], 4), false);
}

#[test]
fn test_78() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 1], vec![1, 1, 2], vec![1, 2, 3], vec![1, 3, 4], vec![1, 4, 5], vec![1, 5, 6], vec![1, 6, 7], vec![1, 7, 8], vec![1, 8, 9], vec![1, 9, 10]], 10), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 3], vec![2, 1, 5], vec![3, 2, 7], vec![4, 3, 9], vec![5, 4, 10]], 15), true);
}

#[test]
fn test_80() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 100], vec![10, 20, 120], vec![10, 40, 140], vec![10, 60, 160], vec![10, 80, 180]], 30), false);
}

#[test]
fn test_81() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 5], vec![3, 5, 10], vec![4, 10, 15], vec![2, 15, 20]], 12), true);
}

#[test]
fn test_82() {
    assert_eq!(Solution::car_pooling(vec![vec![20, 0, 1000], vec![10, 100, 900], vec![5, 200, 800], vec![1, 300, 700]], 50), true);
}

#[test]
fn test_83() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 3], vec![2, 2, 5], vec![4, 3, 8], vec![3, 5, 7]], 10), true);
}

#[test]
fn test_84() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 2], vec![1, 2, 4], vec![1, 4, 6], vec![1, 6, 8], vec![1, 8, 10], vec![1, 10, 12]], 5), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 5], vec![3, 1, 4], vec![2, 3, 6], vec![1, 4, 7]], 10), true);
}

#[test]
fn test_86() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 5], vec![2, 5, 10], vec![1, 10, 15], vec![10, 15, 20]], 12), true);
}

#[test]
fn test_87() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 1, 5], vec![2, 2, 6], vec![4, 3, 8], vec![1, 4, 9], vec![5, 5, 10]], 10), false);
}

#[test]
fn test_88() {
    assert_eq!(Solution::car_pooling(vec![vec![6, 0, 20], vec![2, 10, 30], vec![4, 20, 40], vec![1, 30, 50], vec![3, 40, 60]], 15), true);
}

#[test]
fn test_89() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 1], vec![9, 1, 2], vec![8, 2, 3], vec![7, 3, 4], vec![6, 4, 5], vec![5, 5, 6]], 10), true);
}

#[test]
fn test_90() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 1], vec![10, 1, 2], vec![10, 2, 3], vec![10, 3, 4], vec![10, 4, 5]], 10), true);
}

#[test]
fn test_91() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 2], vec![2, 2, 3], vec![3, 3, 4], vec![4, 4, 5], vec![5, 5, 6], vec![6, 6, 7], vec![7, 7, 8], vec![8, 8, 9], vec![9, 9, 10]], 45), true);
}

#[test]
fn test_92() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 0, 1], vec![3, 1, 2], vec![3, 2, 3], vec![3, 3, 4], vec![3, 4, 5], vec![3, 5, 6], vec![3, 6, 7]], 9), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 100], vec![2, 2, 99], vec![3, 3, 98], vec![4, 4, 97]], 10), true);
}

#[test]
fn test_94() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 1], vec![1, 1, 2], vec![1, 2, 3], vec![1, 3, 4], vec![1, 4, 5], vec![1, 5, 6], vec![1, 6, 7]], 1), true);
}

#[test]
fn test_95() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 1, 3], vec![3, 2, 5], vec![3, 3, 7], vec![3, 4, 8], vec![3, 5, 9], vec![3, 6, 10]], 9), false);
}

#[test]
fn test_96() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 3], vec![5, 3, 5], vec![5, 5, 7], vec![5, 7, 9]], 10), true);
}

#[test]
fn test_97() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 5], vec![2, 3, 8], vec![4, 5, 9], vec![3, 6, 10]], 12), true);
}

#[test]
fn test_98() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 0, 2], vec![5, 2, 5], vec![3, 5, 10], vec![7, 7, 12]], 15), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 0, 5], vec![2, 0, 10], vec![1, 5, 7], vec![4, 8, 10]], 10), true);
}

#[test]
fn test_100() {
    assert_eq!(Solution::car_pooling(vec![vec![50, 0, 100], vec![20, 100, 200], vec![30, 200, 300], vec![40, 300, 400]], 150), true);
}

#[test]
fn test_101() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 3], vec![2, 2, 4], vec![3, 3, 5], vec![4, 4, 6], vec![5, 5, 7], vec![6, 6, 8], vec![7, 7, 9]], 15), true);
}

#[test]
fn test_102() {
    assert_eq!(Solution::car_pooling(vec![vec![50, 0, 10], vec![30, 5, 15], vec![20, 10, 20], vec![10, 15, 25]], 110), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 1, 1000], vec![1, 1000, 1000], vec![1, 500, 501], vec![1, 501, 502]], 3), true);
}

#[test]
fn test_104() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 8], vec![1, 8, 12], vec![4, 10, 15]], 10), true);
}

#[test]
fn test_105() {
    assert_eq!(Solution::car_pooling(vec![vec![1, 0, 10], vec![2, 1, 9], vec![3, 2, 8], vec![4, 3, 7], vec![5, 4, 6]], 15), true);
}

#[test]
fn test_106() {
    assert_eq!(Solution::car_pooling(vec![vec![10, 1, 4], vec![5, 5, 7], vec![2, 2, 6], vec![4, 6, 8]], 15), true);
}

#[test]
fn test_107() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 2], vec![2, 2, 3], vec![2, 3, 4], vec![2, 4, 5], vec![2, 5, 6], vec![2, 6, 7], vec![2, 7, 8]], 3), true);
}

#[test]
fn test_108() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 1, 4], vec![5, 2, 5], vec![1, 2, 6], vec![2, 3, 7], vec![3, 4, 8]], 14), true);
}

#[test]
fn test_109() {
    assert_eq!(Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7], vec![5, 5, 10], vec![1, 7, 12], vec![4, 8, 13]], 15), true);
}

#[test]
fn test_110() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 5], vec![6, 4, 9], vec![4, 9, 14], vec![3, 14, 19]], 15), true);
}

#[test]
fn test_111() {
    assert_eq!(Solution::car_pooling(vec![vec![5, 1, 5], vec![2, 2, 8], vec![3, 4, 6], vec![1, 3, 10]], 12), true);
}

#[test]
fn test_112() {
    assert_eq!(Solution::car_pooling(vec![vec![3, 1, 3], vec![2, 1, 4], vec![1, 2, 5], vec![4, 3, 6], vec![2, 4, 7]], 10), true);
}

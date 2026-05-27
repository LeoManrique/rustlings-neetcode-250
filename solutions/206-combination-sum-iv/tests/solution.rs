include!("../src/lib.rs");

#[test]
fn test_2() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30], 100), 274);
}

#[test]
fn test_3() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
}

#[test]
fn test_4() {
    assert_eq!(Solution::combination_sum4(vec![5, 50, 75], 95), 16);
}

#[test]
fn test_5() {
    assert_eq!(Solution::combination_sum4(vec![5, 1, 3], 8), 19);
}

#[test]
fn test_6() {
    assert_eq!(Solution::combination_sum4(vec![5, 10, 20], 100), 46754);
}

#[test]
fn test_7() {
    assert_eq!(Solution::combination_sum4(vec![2, 1, 5], 10), 128);
}

#[test]
fn test_8() {
    assert_eq!(Solution::combination_sum4(vec![4, 2, 1], 32), 39882198);
}

#[test]
fn test_9() {
    assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::combination_sum4(vec![1], 100), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::combination_sum4(vec![2, 5, 10, 20], 25), 119);
}

#[test]
fn test_14() {
    assert_eq!(Solution::combination_sum4(vec![2, 1, 5], 8), 44);
}

#[test]
fn test_15() {
    assert_eq!(Solution::combination_sum4(vec![15, 25, 5, 50, 100], 100), 4600);
}

#[test]
fn test_16() {
    assert_eq!(Solution::combination_sum4(vec![7, 14], 300), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5], 10), 464);
}

#[test]
fn test_20() {
    assert_eq!(Solution::combination_sum4(vec![4, 11, 3, 4, 1], 21), 46333);
}

#[test]
fn test_22() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30], 50), 13);
}

#[test]
fn test_23() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 5], 25), 382396);
}

#[test]
fn test_24() {
    assert_eq!(Solution::combination_sum4(vec![18, 23, 50, 51], 200), 493);
}

#[test]
fn test_28() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 21, 28], 100), 0);
}

#[test]
fn test_31() {
    assert_eq!(Solution::combination_sum4(vec![999, 1000], 1999), 2);
}

#[test]
fn test_32() {
    assert_eq!(Solution::combination_sum4(vec![3, 6, 9, 12], 100), 0);
}

#[test]
fn test_33() {
    assert_eq!(Solution::combination_sum4(vec![2, 5, 10, 20], 30), 417);
}

#[test]
fn test_34() {
    assert_eq!(Solution::combination_sum4(vec![5, 10, 25, 50], 100), 27517);
}

#[test]
fn test_35() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 4, 8, 16], 32), 47350055);
}

#[test]
fn test_36() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30, 40], 100), 401);
}

#[test]
fn test_37() {
    assert_eq!(Solution::combination_sum4(vec![1, 10, 25, 50], 100), 37971048);
}

#[test]
fn test_38() {
    assert_eq!(Solution::combination_sum4(vec![4, 8, 15, 16, 23, 42], 100), 878907);
}

#[test]
fn test_40() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 5], 30), 15778);
}

#[test]
fn test_44() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 4, 5], 15), 4185);
}

#[test]
fn test_46() {
    assert_eq!(Solution::combination_sum4(vec![3, 7, 15], 120), 92796380);
}

#[test]
fn test_49() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 21, 28, 35], 100), 0);
}

#[test]
fn test_50() {
    assert_eq!(Solution::combination_sum4(vec![4, 10, 40, 100], 150), 3504031);
}

#[test]
fn test_51() {
    assert_eq!(Solution::combination_sum4(vec![3, 7, 8], 11), 2);
}

#[test]
fn test_52() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7, 9], 20), 576);
}

#[test]
fn test_54() {
    assert_eq!(Solution::combination_sum4(vec![50, 25, 75, 20, 10], 200), 197308);
}

#[test]
fn test_56() {
    assert_eq!(Solution::combination_sum4(vec![3, 33, 333], 1000), 0);
}

#[test]
fn test_58() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7], 12), 26);
}

#[test]
fn test_59() {
    assert_eq!(Solution::combination_sum4(vec![3, 6, 9, 12], 30), 401);
}

#[test]
fn test_60() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10], 25), 915);
}

#[test]
fn test_62() {
    assert_eq!(Solution::combination_sum4(vec![100, 200, 300, 400, 500], 1000), 464);
}

#[test]
fn test_67() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30, 40, 50], 150), 13624);
}

#[test]
fn test_69() {
    assert_eq!(Solution::combination_sum4(vec![3, 8, 10], 120), 317567468);
}

#[test]
fn test_70() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5], 15), 13624);
}

#[test]
fn test_71() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 5, 7, 9], 25), 70464);
}

#[test]
fn test_72() {
    assert_eq!(Solution::combination_sum4(vec![33, 39, 45, 51, 60], 150), 29);
}

#[test]
fn test_73() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 7, 8, 9], 50), 252672);
}

#[test]
fn test_74() {
    assert_eq!(Solution::combination_sum4(vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30], 100), 0);
}

#[test]
fn test_76() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5], 10), 464);
}

#[test]
fn test_78() {
    assert_eq!(Solution::combination_sum4(vec![15, 25, 35, 45], 200), 6259);
}

#[test]
fn test_79() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10], 50), 1919938);
}

#[test]
fn test_80() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30], 100), 274);
}

#[test]
fn test_83() {
    assert_eq!(Solution::combination_sum4(vec![50, 150, 250, 350, 450], 1000), 6475);
}

#[test]
fn test_84() {
    assert_eq!(Solution::combination_sum4(vec![50, 25, 75, 125], 250), 331);
}

#[test]
fn test_85() {
    assert_eq!(Solution::combination_sum4(vec![5, 10, 15, 20, 25, 30], 100), 463968);
}

#[test]
fn test_87() {
    assert_eq!(Solution::combination_sum4(vec![15, 25, 35], 100), 25);
}

#[test]
fn test_89() {
    assert_eq!(Solution::combination_sum4(vec![2, 4, 6, 8, 10], 20), 464);
}

#[test]
fn test_90() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 4, 5, 6], 20), 22750);
}

#[test]
fn test_92() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 4], 10), 64);
}

#[test]
fn test_94() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10, 25], 50), 1931845);
}

#[test]
fn test_95() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7], 15), 78);
}

#[test]
fn test_96() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 4, 5], 13), 424);
}

#[test]
fn test_97() {
    assert_eq!(Solution::combination_sum4(vec![13, 17, 19, 23, 29], 100), 256);
}

#[test]
fn test_98() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 7, 10], 20), 35);
}

#[test]
fn test_99() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7], 30), 19096);
}

#[test]
fn test_100() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 28], 1000), 0);
}

#[test]
fn test_102() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 21, 28, 35], 105), 13624);
}

#[test]
fn test_103() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 28, 35, 56], 100), 0);
}

#[test]
fn test_104() {
    assert_eq!(Solution::combination_sum4(vec![4, 7, 9], 20), 7);
}

#[test]
fn test_105() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 10, 15], 50), 12043);
}

#[test]
fn test_108() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 7], 15), 8);
}

#[test]
fn test_113() {
    assert_eq!(Solution::combination_sum4(vec![4, 10, 40, 25], 100), 18984);
}

#[test]
fn test_114() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 4, 8, 16], 31), 26805983);
}

#[test]
fn test_115() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30, 40, 50], 200), 400096);
}

#[test]
fn test_116() {
    assert_eq!(Solution::combination_sum4(vec![1, 100, 101, 102], 300), 45158);
}

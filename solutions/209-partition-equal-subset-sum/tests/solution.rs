include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 5, 5]), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::can_partition(vec![1, 2, 2, 3]), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4]), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::can_partition(vec![2, 2, 3, 3]), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::can_partition(vec![1, 1, 1, 1, 1, 1, 1, 1]), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 6, 7]), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::can_partition(vec![2, 2, 3, 5]), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::can_partition(vec![2, 2, 3, 6]), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
}

#[test]
fn test_11() {
    assert_eq!(Solution::can_partition(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::can_partition(vec![99, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), false);
}

#[test]
fn test_13() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 6, 12, 24, 48, 96, 192, 384, 768, 1536, 3072, 6144, 12288, 24576, 49152, 98304, 196608]), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::can_partition(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::can_partition(vec![1, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190]), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::can_partition(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), false);
}

#[test]
fn test_17() {
    assert_eq!(Solution::can_partition(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]), false);
}

#[test]
fn test_18() {
    assert_eq!(Solution::can_partition(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), false);
}

#[test]
fn test_20() {
    assert_eq!(Solution::can_partition(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288]), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::can_partition(vec![100, 200, 300, 400, 500, 500]), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::can_partition(vec![1, 2, 5, 6, 8, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54]), false);
}

#[test]
fn test_23() {
    assert_eq!(Solution::can_partition(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::can_partition(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::can_partition(vec![7, 14, 3, 9, 1, 4, 13, 2, 3, 10, 5, 3, 5, 8, 6, 9, 6, 5, 14, 4, 5, 8, 6, 4, 10, 1, 4, 9, 1, 3, 1, 5, 4, 3, 9, 1, 5, 9, 1, 2, 9, 1, 8, 1, 14, 5, 8, 3, 13, 14, 1, 10, 1, 1, 1, 5, 14, 1, 14, 5, 14, 1, 8, 10, 9, 5, 14, 10, 13]), false);
}

#[test]
fn test_26() {
    assert_eq!(Solution::can_partition(vec![5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::can_partition(vec![10, 20, 15, 5, 5]), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::can_partition(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::can_partition(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::can_partition(vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4]), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), false);
}

#[test]
fn test_32() {
    assert_eq!(Solution::can_partition(vec![10, 15, 20, 25, 30, 35, 40]), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::can_partition(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]), false);
}

#[test]
fn test_34() {
    assert_eq!(Solution::can_partition(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::can_partition(vec![10, 20, 15, 5, 5]), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::can_partition(vec![15, 10, 20, 30, 50, 50]), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::can_partition(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000]), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::can_partition(vec![8, 15, 3, 7, 15, 16, 9, 16, 8, 15, 7, 13, 16, 3, 14, 1, 1, 1, 1, 1]), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::can_partition(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::can_partition(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::can_partition(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::can_partition(vec![3, 3, 3, 3, 10, 5, 5, 5]), false);
}

#[test]
fn test_43() {
    assert_eq!(Solution::can_partition(vec![1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000, 50000, 100000, 200000, 500000, 1000000, 2000000]), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::can_partition(vec![23, 13, 2, 16, 21, 5, 14, 11, 7, 9, 4, 18, 3, 6, 8, 10, 12, 15, 17, 20]), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::can_partition(vec![15, 20, 25, 30, 35, 40, 45, 50, 55, 60]), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::can_partition(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::can_partition(vec![33, 14, 60, 22, 5, 9, 38, 35, 7, 3, 19, 25, 36, 29, 28, 17, 41, 21, 18, 19]), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::can_partition(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300]), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 6, 7]), true);
}

#[test]
fn test_50() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::can_partition(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384]), false);
}

#[test]
fn test_52() {
    assert_eq!(Solution::can_partition(vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90]), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::can_partition(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 16]), false);
}

#[test]
fn test_54() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), false);
}

#[test]
fn test_55() {
    assert_eq!(Solution::can_partition(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), true);
}

#[test]
fn test_56() {
    assert_eq!(Solution::can_partition(vec![10, 2, 30, 12, 3, 1]), false);
}

#[test]
fn test_57() {
    assert_eq!(Solution::can_partition(vec![100, 200, 300, 400, 500, 100, 200, 300, 400, 500]), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::can_partition(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::can_partition(vec![99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99]), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::can_partition(vec![7, 15, 6, 5, 9, 8, 3, 5, 4, 4, 5, 7, 9, 6, 5, 4, 5, 9, 10, 5]), false);
}

#[test]
fn test_61() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::can_partition(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::can_partition(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 100]), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::can_partition(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250]), false);
}

#[test]
fn test_65() {
    assert_eq!(Solution::can_partition(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288]), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::can_partition(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536]), false);
}

#[test]
fn test_67() {
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::can_partition(vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4]), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::can_partition(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 10]), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::can_partition(vec![15, 20, 25, 30, 35, 40, 45, 50]), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::can_partition(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100]), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::can_partition(vec![10, 20, 30, 40, 50, 60, 70]), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::can_partition(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8]), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::can_partition(vec![99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118]), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::can_partition(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::can_partition(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200]), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::can_partition(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 55]), false);
}

#[test]
fn test_78() {
    assert_eq!(Solution::can_partition(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 10]), false);
}

#[test]
fn test_79() {
    assert_eq!(Solution::can_partition(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), false);
}

#[test]
fn test_80() {
    assert_eq!(Solution::can_partition(vec![50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300, 325, 350]), true);
}

#[test]
fn test_81() {
    assert_eq!(Solution::can_partition(vec![42, 39, 27, 27, 38, 29, 37, 29, 43, 42, 27, 26, 28, 38, 37, 29, 30, 29, 39, 26]), true);
}

#[test]
fn test_82() {
    assert_eq!(Solution::can_partition(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), true);
}

#[test]
fn test_83() {
    assert_eq!(Solution::can_partition(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), true);
}

#[test]
fn test_84() {
    assert_eq!(Solution::can_partition(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50]), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::can_partition(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), false);
}

#[test]
fn test_86() {
    assert_eq!(Solution::can_partition(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), true);
}

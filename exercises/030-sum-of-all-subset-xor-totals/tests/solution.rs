include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::subset_xor_sum(vec![5, 1, 6]), 28);
}

#[test]
fn test_2() {
    assert_eq!(Solution::subset_xor_sum(vec![10, 20, 30]), 120);
}

#[test]
fn test_3() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 1, 1, 1]), 8);
}

#[test]
fn test_4() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 4, 8, 16]), 496);
}

#[test]
fn test_5() {
    assert_eq!(Solution::subset_xor_sum(vec![10, 11, 12]), 60);
}

#[test]
fn test_6() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 4, 6, 8, 10, 12, 14]), 896);
}

#[test]
fn test_7() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 3, 5]), 28);
}

#[test]
fn test_8() {
    assert_eq!(Solution::subset_xor_sum(vec![11, 2, 3, 14, 7]), 240);
}

#[test]
fn test_9() {
    assert_eq!(Solution::subset_xor_sum(vec![1]), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::subset_xor_sum(vec![7, 8, 9, 10, 11]), 240);
}

#[test]
fn test_11() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 3, 4]), 56);
}

#[test]
fn test_12() {
    assert_eq!(Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
}

#[test]
fn test_13() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 30720);
}

#[test]
fn test_14() {
    assert_eq!(Solution::subset_xor_sum(vec![12, 34, 56]), 248);
}

#[test]
fn test_15() {
    assert_eq!(Solution::subset_xor_sum(vec![10, 15, 20, 25]), 248);
}

#[test]
fn test_16() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 3, 5, 7]), 56);
}

#[test]
fn test_17() {
    assert_eq!(Solution::subset_xor_sum(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9]), 63488);
}

#[test]
fn test_18() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
}

#[test]
fn test_19() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 2, 2, 2]), 16);
}

#[test]
fn test_20() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 4, 8, 16, 32]), 2016);
}

#[test]
fn test_21() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 15360);
}

#[test]
fn test_22() {
    assert_eq!(Solution::subset_xor_sum(vec![15, 16, 17, 18, 19, 20]), 992);
}

#[test]
fn test_23() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 3]), 12);
}

#[test]
fn test_24() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 2048);
}

#[test]
fn test_25() {
    assert_eq!(Solution::subset_xor_sum(vec![6, 3, 5, 2, 1, 9]), 480);
}

#[test]
fn test_26() {
    assert_eq!(Solution::subset_xor_sum(vec![31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2, 1]), 63488);
}

#[test]
fn test_27() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 4, 8, 16, 32]), 2016);
}

#[test]
fn test_28() {
    assert_eq!(Solution::subset_xor_sum(vec![12, 24, 36, 48, 60, 72, 84, 96, 108, 120, 132, 144]), 516096);
}

#[test]
fn test_29() {
    assert_eq!(Solution::subset_xor_sum(vec![8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384]), 67092480);
}

#[test]
fn test_30() {
    assert_eq!(Solution::subset_xor_sum(vec![10, 20, 30, 40, 50, 60]), 1984);
}

#[test]
fn test_31() {
    assert_eq!(Solution::subset_xor_sum(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 16252928);
}

#[test]
fn test_32() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095]), 8386560);
}

#[test]
fn test_33() {
    assert_eq!(Solution::subset_xor_sum(vec![15, 20, 25, 30]), 248);
}

#[test]
fn test_34() {
    assert_eq!(Solution::subset_xor_sum(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60]), 129024);
}

#[test]
fn test_35() {
    assert_eq!(Solution::subset_xor_sum(vec![128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144]), 1073479680);
}

#[test]
fn test_36() {
    assert_eq!(Solution::subset_xor_sum(vec![3, 3, 3, 5, 5, 5, 7, 7, 7, 11, 11, 11]), 30720);
}

#[test]
fn test_37() {
    assert_eq!(Solution::subset_xor_sum(vec![15, 30, 45, 60, 75, 90, 105, 120, 135, 150]), 130560);
}

#[test]
fn test_38() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 4, 8, 16, 32, 64]), 8128);
}

#[test]
fn test_39() {
    assert_eq!(Solution::subset_xor_sum(vec![15, 25, 35, 45, 55, 65, 75]), 8128);
}

#[test]
fn test_40() {
    assert_eq!(Solution::subset_xor_sum(vec![19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8]), 63488);
}

#[test]
fn test_41() {
    assert_eq!(Solution::subset_xor_sum(vec![7, 14, 21, 28, 35, 42, 49, 56, 63, 70, 77, 84]), 260096);
}

#[test]
fn test_42() {
    assert_eq!(Solution::subset_xor_sum(vec![15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70]), 260096);
}

#[test]
fn test_43() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 245760);
}

#[test]
fn test_44() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096]), 16773120);
}

#[test]
fn test_45() {
    assert_eq!(Solution::subset_xor_sum(vec![15, 25, 35, 45, 55, 65, 75, 85, 95, 105, 115, 125]), 260096);
}

#[test]
fn test_46() {
    assert_eq!(Solution::subset_xor_sum(vec![31, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]), 1015808);
}

#[test]
fn test_47() {
    assert_eq!(Solution::subset_xor_sum(vec![31, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11, 9]), 63488);
}

#[test]
fn test_48() {
    assert_eq!(Solution::subset_xor_sum(vec![12, 24, 36, 48, 60, 72, 84, 96, 108, 120, 132, 144]), 516096);
}

#[test]
fn test_49() {
    assert_eq!(Solution::subset_xor_sum(vec![13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]), 63488);
}

#[test]
fn test_50() {
    assert_eq!(Solution::subset_xor_sum(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200]), 4186112);
}

#[test]
fn test_51() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096]), 16773120);
}

#[test]
fn test_52() {
    assert_eq!(Solution::subset_xor_sum(vec![13, 17, 19, 23, 29, 31, 37, 41]), 8064);
}

#[test]
fn test_53() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]), 129024);
}

#[test]
fn test_54() {
    assert_eq!(Solution::subset_xor_sum(vec![13, 23, 33, 43, 53, 63, 73, 83, 93, 103, 113, 123]), 260096);
}

#[test]
fn test_55() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23]), 63488);
}

#[test]
fn test_56() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 30720);
}

#[test]
fn test_57() {
    assert_eq!(Solution::subset_xor_sum(vec![14, 28, 42, 56, 70, 84, 98, 112, 126, 140, 154, 168]), 520192);
}

#[test]
fn test_58() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6]), 14336);
}

#[test]
fn test_59() {
    assert_eq!(Solution::subset_xor_sum(vec![19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8]), 63488);
}

#[test]
fn test_60() {
    assert_eq!(Solution::subset_xor_sum(vec![14, 21, 28, 35, 42, 49, 56, 63, 70, 77, 84, 91]), 260096);
}

#[test]
fn test_61() {
    assert_eq!(Solution::subset_xor_sum(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200]), 4186112);
}

#[test]
fn test_62() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 3, 4, 5, 6]), 112);
}

#[test]
fn test_63() {
    assert_eq!(Solution::subset_xor_sum(vec![12, 14, 18, 22, 26, 30]), 960);
}

#[test]
fn test_64() {
    assert_eq!(Solution::subset_xor_sum(vec![10, 15, 20, 25]), 248);
}

#[test]
fn test_65() {
    assert_eq!(Solution::subset_xor_sum(vec![17, 19, 21, 23, 25, 27, 29, 31]), 3968);
}

#[test]
fn test_66() {
    assert_eq!(Solution::subset_xor_sum(vec![3, 5, 7, 9, 11, 13]), 480);
}

#[test]
fn test_67() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]), 8386560);
}

#[test]
fn test_68() {
    assert_eq!(Solution::subset_xor_sum(vec![5, 10, 15, 20, 25, 30, 35, 40, 45]), 16128);
}

#[test]
fn test_69() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]), 8386560);
}

#[test]
fn test_70() {
    assert_eq!(Solution::subset_xor_sum(vec![15, 25, 35, 45, 55]), 1008);
}

#[test]
fn test_71() {
    assert_eq!(Solution::subset_xor_sum(vec![10, 20, 30, 40, 50]), 992);
}

#[test]
fn test_72() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]), 15360);
}

#[test]
fn test_73() {
    assert_eq!(Solution::subset_xor_sum(vec![13, 17, 19, 23, 29, 31, 37, 41]), 8064);
}

#[test]
fn test_74() {
    assert_eq!(Solution::subset_xor_sum(vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42]), 129024);
}

#[test]
fn test_75() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]), 129024);
}

#[test]
fn test_76() {
    assert_eq!(Solution::subset_xor_sum(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120]), 258048);
}

#[test]
fn test_77() {
    assert_eq!(Solution::subset_xor_sum(vec![3, 11, 19, 27, 35, 43, 51, 59, 67, 75, 83, 91]), 251904);
}

#[test]
fn test_78() {
    assert_eq!(Solution::subset_xor_sum(vec![13, 26, 39, 52, 65, 78, 91, 104]), 16256);
}

#[test]
fn test_79() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 4, 6, 8, 10, 12, 14]), 896);
}

#[test]
fn test_80() {
    assert_eq!(Solution::subset_xor_sum(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120]), 258048);
}

#[test]
fn test_81() {
    assert_eq!(Solution::subset_xor_sum(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9]), 63488);
}

#[test]
fn test_82() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4]), 14336);
}

#[test]
fn test_83() {
    assert_eq!(Solution::subset_xor_sum(vec![31, 33, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73]), 260096);
}

#[test]
fn test_84() {
    assert_eq!(Solution::subset_xor_sum(vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]), 129024);
}

#[test]
fn test_85() {
    assert_eq!(Solution::subset_xor_sum(vec![17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61]), 129024);
}

#[test]
fn test_86() {
    assert_eq!(Solution::subset_xor_sum(vec![8, 16, 32, 64, 128, 256, 512]), 65024);
}

#[test]
fn test_87() {
    assert_eq!(Solution::subset_xor_sum(vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 110, 121, 132]), 522240);
}

#[test]
fn test_88() {
    assert_eq!(Solution::subset_xor_sum(vec![19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67]), 260096);
}

#[test]
fn test_89() {
    assert_eq!(Solution::subset_xor_sum(vec![15, 20, 25, 30, 35, 40, 45]), 4032);
}

#[test]
fn test_90() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]), 15360);
}

#[test]
fn test_91() {
    assert_eq!(Solution::subset_xor_sum(vec![19, 17, 15, 13, 11, 9, 7]), 1984);
}

#[test]
fn test_92() {
    assert_eq!(Solution::subset_xor_sum(vec![8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96]), 245760);
}

#[test]
fn test_93() {
    assert_eq!(Solution::subset_xor_sum(vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36]), 129024);
}

#[test]
fn test_94() {
    assert_eq!(Solution::subset_xor_sum(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4]), 30720);
}

#[test]
fn test_95() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23]), 63488);
}

#[test]
fn test_96() {
    assert_eq!(Solution::subset_xor_sum(vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 30720);
}

#[test]
fn test_97() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1]), 30720);
}

#[test]
fn test_98() {
    assert_eq!(Solution::subset_xor_sum(vec![7, 14, 21, 28, 35, 42, 49, 56, 63]), 16128);
}

#[test]
fn test_99() {
    assert_eq!(Solution::subset_xor_sum(vec![2, 3, 5, 7, 11, 13, 17, 19]), 3968);
}

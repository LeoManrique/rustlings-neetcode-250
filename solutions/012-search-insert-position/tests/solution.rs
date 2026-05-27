include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::search_insert(vec![-10, 0, 5, 10], 0), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::search_insert(vec![1, 2, 4, 6, 7, 9], 3), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5], 4), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::search_insert(vec![-10, -5, -3, 2, 3, 4, 5], -4), 2);
}

#[test]
fn test_6() {
    assert_eq!(Solution::search_insert(vec![-10, 0, 10, 20, 30, 40, 50], -5), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::search_insert(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11], 10), 5);
}

#[test]
fn test_9() {
    assert_eq!(Solution::search_insert(vec![1], 1), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::search_insert(vec![-10, 0, 10, 20, 30, 40, 50], 25), 4);
}

#[test]
fn test_11() {
    assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::search_insert(vec![-10, 0, 5, 10], 15), 4);
}

#[test]
fn test_13() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
}

#[test]
fn test_14() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
}

#[test]
fn test_15() {
    assert_eq!(Solution::search_insert(vec![1, 3], 4), 2);
}

#[test]
fn test_16() {
    assert_eq!(Solution::search_insert(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 4);
}

#[test]
fn test_17() {
    assert_eq!(Solution::search_insert(vec![1, 3, 4, 5, 6, 8, 9, 11, 13, 15], 7), 5);
}

#[test]
fn test_18() {
    assert_eq!(Solution::search_insert(vec![1, 2, 4, 6, 8, 10], 5), 3);
}

#[test]
fn test_19() {
    assert_eq!(Solution::search_insert(vec![1], 0), 0);
}

#[test]
fn test_20() {
    assert_eq!(Solution::search_insert(vec![1], 2), 1);
}

#[test]
fn test_21() {
    assert_eq!(Solution::search_insert(vec![-10, -5, -3, 2, 3, 4, 5], 3), 4);
}

#[test]
fn test_22() {
    assert_eq!(Solution::search_insert(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 11), 10);
}

#[test]
fn test_23() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
}

#[test]
fn test_24() {
    assert_eq!(Solution::search_insert(vec![-10, 0, 5, 10], -5), 1);
}

#[test]
fn test_25() {
    assert_eq!(Solution::search_insert(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024], 256), 8);
}

#[test]
fn test_26() {
    assert_eq!(Solution::search_insert(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 5), 0);
}

#[test]
fn test_27() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25], 14), 7);
}

#[test]
fn test_28() {
    assert_eq!(Solution::search_insert(vec![10000], 9999), 0);
}

#[test]
fn test_29() {
    assert_eq!(Solution::search_insert(vec![-1000, -900, -800, -700, -600, -500, -400, -300, -200, -100], -550), 5);
}

#[test]
fn test_30() {
    assert_eq!(Solution::search_insert(vec![1, 4, 6, 8, 10, 12, 14, 16, 18, 20], 15), 7);
}

#[test]
fn test_31() {
    assert_eq!(Solution::search_insert(vec![-100, -90, -80, -70, -60, -50, -40, -30, -20, -10], -85), 2);
}

#[test]
fn test_32() {
    assert_eq!(Solution::search_insert(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 12), 11);
}

#[test]
fn test_33() {
    assert_eq!(Solution::search_insert(vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20], 20), 10);
}

#[test]
fn test_34() {
    assert_eq!(Solution::search_insert(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 29), 29);
}

#[test]
fn test_35() {
    assert_eq!(Solution::search_insert(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200], 155), 15);
}

#[test]
fn test_36() {
    assert_eq!(Solution::search_insert(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20], 15), 7);
}

#[test]
fn test_37() {
    assert_eq!(Solution::search_insert(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50], 27), 5);
}

#[test]
fn test_38() {
    assert_eq!(Solution::search_insert(vec![2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35, 38, 41], 39), 13);
}

#[test]
fn test_39() {
    assert_eq!(Solution::search_insert(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 5);
}

#[test]
fn test_40() {
    assert_eq!(Solution::search_insert(vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30], 27), 8);
}

#[test]
fn test_41() {
    assert_eq!(Solution::search_insert(vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000], 500000), 6);
}

#[test]
fn test_42() {
    assert_eq!(Solution::search_insert(vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576, 625, 676, 729, 784, 841, 900, 961, 1024], 200), 14);
}

#[test]
fn test_43() {
    assert_eq!(Solution::search_insert(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512], 33), 6);
}

#[test]
fn test_44() {
    assert_eq!(Solution::search_insert(vec![-10000, 10000], 10000), 1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::search_insert(vec![3, 6, 9, 12, 15, 18, 21, 24], 11), 3);
}

#[test]
fn test_46() {
    assert_eq!(Solution::search_insert(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 19), 18);
}

#[test]
fn test_47() {
    assert_eq!(Solution::search_insert(vec![-10000, 10000], 0), 1);
}

#[test]
fn test_48() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 10), 5);
}

#[test]
fn test_49() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 0), 0);
}

#[test]
fn test_50() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39], 20), 10);
}

#[test]
fn test_51() {
    assert_eq!(Solution::search_insert(vec![-10, -5, -3, -1, 0, 2, 4, 6, 8, 10], -6), 1);
}

#[test]
fn test_52() {
    assert_eq!(Solution::search_insert(vec![1, 4, 7, 10, 13, 16, 19, 22, 25, 28], 26), 9);
}

#[test]
fn test_53() {
    assert_eq!(Solution::search_insert(vec![50, 60, 70, 80, 90, 100, 110, 120, 130, 140], 145), 10);
}

#[test]
fn test_54() {
    assert_eq!(Solution::search_insert(vec![-10, -5, -3, 0, 4, 8, 12], 3), 4);
}

#[test]
fn test_55() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25], 12), 6);
}

#[test]
fn test_56() {
    assert_eq!(Solution::search_insert(vec![10000], 10001), 1);
}

#[test]
fn test_57() {
    assert_eq!(Solution::search_insert(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024], 512), 9);
}

#[test]
fn test_58() {
    assert_eq!(Solution::search_insert(vec![-10, -5, 0, 3, 9, 12, 15], 1), 3);
}

#[test]
fn test_59() {
    assert_eq!(Solution::search_insert(vec![-500, -400, -300, -200, -100, 0, 100, 200, 300, 400, 500], -350), 2);
}

#[test]
fn test_60() {
    assert_eq!(Solution::search_insert(vec![1, 2, 4, 8, 16, 32, 64, 128, 256], 64), 6);
}

#[test]
fn test_61() {
    assert_eq!(Solution::search_insert(vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100], 50), 7);
}

#[test]
fn test_62() {
    assert_eq!(Solution::search_insert(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80], 67), 13);
}

#[test]
fn test_63() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21], 10), 5);
}

#[test]
fn test_64() {
    assert_eq!(Solution::search_insert(vec![1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34, 37, 40, 43, 46, 49, 52, 55, 58], 59), 20);
}

#[test]
fn test_65() {
    assert_eq!(Solution::search_insert(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20], 1), 0);
}

#[test]
fn test_66() {
    assert_eq!(Solution::search_insert(vec![5, 15, 25, 35, 45, 55, 65, 75, 85, 95], 100), 10);
}

#[test]
fn test_67() {
    assert_eq!(Solution::search_insert(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 500), 4);
}

#[test]
fn test_68() {
    assert_eq!(Solution::search_insert(vec![-500, -400, -300, -200, -100, 0, 100, 200, 300, 400, 500], -250), 3);
}

#[test]
fn test_69() {
    assert_eq!(Solution::search_insert(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 6);
}

#[test]
fn test_70() {
    assert_eq!(Solution::search_insert(vec![-10, -5, 0, 5, 10, 15, 20, 25, 30], 12), 5);
}

#[test]
fn test_71() {
    assert_eq!(Solution::search_insert(vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000], 5500), 5);
}

#[test]
fn test_72() {
    assert_eq!(Solution::search_insert(vec![-1000, -500, -250, -100, -50, -25, -10, -5, -2, -1, 0, 1, 2, 5, 10, 25, 50, 100, 250, 500, 1000], -750), 1);
}

#[test]
fn test_73() {
    assert_eq!(Solution::search_insert(vec![-10, -5, 0, 5, 10, 15, 20], -3), 2);
}

#[test]
fn test_74() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9], 6), 3);
}

#[test]
fn test_75() {
    assert_eq!(Solution::search_insert(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 100), 0);
}

#[test]
fn test_76() {
    assert_eq!(Solution::search_insert(vec![1, 10, 100, 1000, 10000], 500), 3);
}

#[test]
fn test_77() {
    assert_eq!(Solution::search_insert(vec![-50, -40, -30, -20, -10, 0, 10, 20, 30, 40, 50], -35), 2);
}

#[test]
fn test_78() {
    assert_eq!(Solution::search_insert(vec![2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35, 38, 41, 44, 47, 50, 53, 56, 59], 3), 1);
}

#[test]
fn test_79() {
    assert_eq!(Solution::search_insert(vec![-10, -5, 0, 5, 10], -7), 1);
}

#[test]
fn test_80() {
    assert_eq!(Solution::search_insert(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 8), 7);
}

#[test]
fn test_81() {
    assert_eq!(Solution::search_insert(vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000], 1500), 1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::search_insert(vec![1, 4, 7, 10, 13, 16, 19], 18), 6);
}

#[test]
fn test_83() {
    assert_eq!(Solution::search_insert(vec![-10000, 10000], -10000), 0);
}

#[test]
fn test_84() {
    assert_eq!(Solution::search_insert(vec![-999, -998, -997, -996, -995, -994, -993, -992, -991, -990], -993), 6);
}

#[test]
fn test_85() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21], 20), 10);
}

#[test]
fn test_86() {
    assert_eq!(Solution::search_insert(vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576, 625, 676, 729, 784, 841, 900, 961, 1024], 1024), 32);
}

#[test]
fn test_87() {
    assert_eq!(Solution::search_insert(vec![-1000, -500, -250, -125, -62, -31, -15, -7, -3, -1], -10), 7);
}

#[test]
fn test_88() {
    assert_eq!(Solution::search_insert(vec![-10000, -9000, -8000, -7000, -6000, -5000, -4000, -3000, -2000, -1000], -7500), 3);
}

#[test]
fn test_89() {
    assert_eq!(Solution::search_insert(vec![2, 4, 6, 8, 10], 5), 2);
}

#[test]
fn test_90() {
    assert_eq!(Solution::search_insert(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 11), 11);
}

#[test]
fn test_91() {
    assert_eq!(Solution::search_insert(vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30], 29), 9);
}

#[test]
fn test_92() {
    assert_eq!(Solution::search_insert(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100], 93), 18);
}

#[test]
fn test_93() {
    assert_eq!(Solution::search_insert(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 550), 5);
}

#[test]
fn test_94() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 20), 10);
}

#[test]
fn test_95() {
    assert_eq!(Solution::search_insert(vec![1, 10, 100, 1000, 10000], 5000), 4);
}

#[test]
fn test_96() {
    assert_eq!(Solution::search_insert(vec![-10000, -9000, -8000, -7000, -6000, -5000, -4000, -3000, -2000, -1000, 0, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000], 5000), 15);
}

#[test]
fn test_97() {
    assert_eq!(Solution::search_insert(vec![-10, -5, -3, -1, 0, 2, 4, 6, 8, 10], -4), 2);
}

#[test]
fn test_98() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], 14), 7);
}

#[test]
fn test_99() {
    assert_eq!(Solution::search_insert(vec![-5000, -4000, -3000, -2000, -1000, 0, 1000, 2000, 3000, 4000, 5000], -2500), 3);
}

#[test]
fn test_100() {
    assert_eq!(Solution::search_insert(vec![-10, -5, 0, 2, 5, 9, 11, 20], 3), 4);
}

#[test]
fn test_101() {
    assert_eq!(Solution::search_insert(vec![1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34, 37, 40, 43, 46, 49, 52, 55, 58], 26), 9);
}

#[test]
fn test_102() {
    assert_eq!(Solution::search_insert(vec![10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000, 90000, 100000], 55000), 5);
}

#[test]
fn test_103() {
    assert_eq!(Solution::search_insert(vec![-500, -250, -100, -50, -25, -10, -5, -2, -1, 0, 1, 2, 5, 10, 25, 50, 100, 250, 500], -75), 3);
}

#[test]
fn test_104() {
    assert_eq!(Solution::search_insert(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30], 31), 15);
}

#[test]
fn test_105() {
    assert_eq!(Solution::search_insert(vec![-100, -90, -80, -70, -60, -50, -40, -30, -20, -10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100], -45), 6);
}

#[test]
fn test_106() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59], 44), 22);
}

#[test]
fn test_107() {
    assert_eq!(Solution::search_insert(vec![1, 4, 7, 10, 13, 16, 19, 22, 25], 15), 5);
}

#[test]
fn test_108() {
    assert_eq!(Solution::search_insert(vec![1, 3, 7, 15, 31, 63, 127, 255, 511, 1023], 1024), 10);
}

#[test]
fn test_109() {
    assert_eq!(Solution::search_insert(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500], 1150), 11);
}

#[test]
fn test_110() {
    assert_eq!(Solution::search_insert(vec![1, 4, 7, 10, 13, 16, 19, 22, 25, 28], 15), 5);
}

#[test]
fn test_111() {
    assert_eq!(Solution::search_insert(vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024], 100), 6);
}

#[test]
fn test_112() {
    assert_eq!(Solution::search_insert(vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000], 10000000), 7);
}

#[test]
fn test_113() {
    assert_eq!(Solution::search_insert(vec![100, 200, 300, 400, 500], 250), 2);
}

#[test]
fn test_114() {
    assert_eq!(Solution::search_insert(vec![-100, -50, 0, 50, 100], -75), 1);
}

#[test]
fn test_115() {
    assert_eq!(Solution::search_insert(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 15), 14);
}

#[test]
fn test_116() {
    assert_eq!(Solution::search_insert(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20], 19), 9);
}

#[test]
fn test_117() {
    assert_eq!(Solution::search_insert(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512], 257), 9);
}

#[test]
fn test_118() {
    assert_eq!(Solution::search_insert(vec![10, 20, 30, 40, 50, 60, 70, 80, 90], 5), 0);
}

#[test]
fn test_119() {
    assert_eq!(Solution::search_insert(vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576, 625, 676, 729, 784, 841, 900, 961, 1024], -1), 0);
}

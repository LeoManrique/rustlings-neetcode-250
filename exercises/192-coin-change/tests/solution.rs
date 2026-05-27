include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
}

#[test]
fn test_2() {
    assert_eq!(Solution::coin_change(vec![3, 7, 405, 436], 8839), 25);
}

#[test]
fn test_3() {
    assert_eq!(Solution::coin_change(vec![3, 7, 405], 8839), 71);
}

#[test]
fn test_4() {
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 1], 27), 4);
}

#[test]
fn test_5() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 1), 1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::coin_change(vec![1, 3, 4, 5], 7), 2);
}

#[test]
fn test_7() {
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::coin_change(vec![5, 7, 8], 100), 13);
}

#[test]
fn test_9() {
    assert_eq!(Solution::coin_change(vec![1, 3, 4], 6), 2);
}

#[test]
fn test_10() {
    assert_eq!(Solution::coin_change(vec![4, 2, 1], 11), 4);
}

#[test]
fn test_11() {
    assert_eq!(Solution::coin_change(vec![1, 3, 4], 6), 2);
}

#[test]
fn test_12() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
}

#[test]
fn test_13() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 0), 0);
}

#[test]
fn test_14() {
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::coin_change(vec![70, 171, 358, 439, 286], 9963), 27);
}

#[test]
fn test_16() {
    assert_eq!(Solution::coin_change(vec![3, 7, 405, 88, 43], 6803), 24);
}

#[test]
fn test_17() {
    assert_eq!(Solution::coin_change(vec![5, 7], 15), 3);
}

#[test]
fn test_18() {
    assert_eq!(Solution::coin_change(vec![18, 27, 41], 987), 26);
}

#[test]
fn test_19() {
    assert_eq!(Solution::coin_change(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048], 4095), 12);
}

#[test]
fn test_20() {
    assert_eq!(Solution::coin_change(vec![5, 15, 25, 50], 3000), 60);
}

#[test]
fn test_21() {
    assert_eq!(Solution::coin_change(vec![1, 2, 4, 8, 16], 2048), 128);
}

#[test]
fn test_22() {
    assert_eq!(Solution::coin_change(vec![10, 20, 30, 40, 50, 60], 1234), -1);
}

#[test]
fn test_23() {
    assert_eq!(Solution::coin_change(vec![3, 5, 7, 9, 11], 9876), 898);
}

#[test]
fn test_24() {
    assert_eq!(Solution::coin_change(vec![18, 24, 28], 100), 4);
}

#[test]
fn test_25() {
    assert_eq!(Solution::coin_change(vec![17, 29, 31, 37, 41, 43], 5000), 118);
}

#[test]
fn test_26() {
    assert_eq!(Solution::coin_change(vec![3, 5], 11), 3);
}

#[test]
fn test_27() {
    assert_eq!(Solution::coin_change(vec![17, 29, 41, 53, 65], 8300), 136);
}

#[test]
fn test_28() {
    assert_eq!(Solution::coin_change(vec![11, 22, 33, 44, 55], 6600), 120);
}

#[test]
fn test_29() {
    assert_eq!(Solution::coin_change(vec![34, 7, 23, 32, 5, 62], 9999), 164);
}

#[test]
fn test_30() {
    assert_eq!(Solution::coin_change(vec![1, 5, 25, 50], 9999), 208);
}

#[test]
fn test_31() {
    assert_eq!(Solution::coin_change(vec![2, 3, 7, 10], 500), 50);
}

#[test]
fn test_32() {
    assert_eq!(Solution::coin_change(vec![7, 14, 21, 28, 35, 42, 49, 56], 1000), -1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::coin_change(vec![3, 6, 9, 12, 15], 1000), -1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::coin_change(vec![1, 5, 10, 25, 50], 78), 5);
}

#[test]
fn test_35() {
    assert_eq!(Solution::coin_change(vec![10, 20, 30, 40, 50], 9999), -1);
}

#[test]
fn test_36() {
    assert_eq!(Solution::coin_change(vec![1, 7, 11], 100), 10);
}

#[test]
fn test_37() {
    assert_eq!(Solution::coin_change(vec![2, 3, 6, 12, 24, 48], 500), 13);
}

#[test]
fn test_38() {
    assert_eq!(Solution::coin_change(vec![1, 2, 4, 8, 16, 32, 64], 1023), 21);
}

#[test]
fn test_39() {
    assert_eq!(Solution::coin_change(vec![1], 10000), 10000);
}

#[test]
fn test_40() {
    assert_eq!(Solution::coin_change(vec![7, 14, 30, 50, 80], 287), 6);
}

#[test]
fn test_41() {
    assert_eq!(Solution::coin_change(vec![2, 4, 8, 16, 32, 64, 128], 1023), -1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5, 10, 20, 50, 100, 200], 10002), 51);
}

#[test]
fn test_43() {
    assert_eq!(Solution::coin_change(vec![2, 3, 5, 7, 11, 13], 5000), 386);
}

#[test]
fn test_44() {
    assert_eq!(Solution::coin_change(vec![3, 5, 7, 9, 11], 10000), 910);
}

#[test]
fn test_45() {
    assert_eq!(Solution::coin_change(vec![7], 100), -1);
}

#[test]
fn test_46() {
    assert_eq!(Solution::coin_change(vec![1, 5, 10, 25, 50, 100], 99999), 1007);
}

#[test]
fn test_47() {
    assert_eq!(Solution::coin_change(vec![1, 2, 3, 5, 10], 5000), 500);
}

#[test]
fn test_48() {
    assert_eq!(Solution::coin_change(vec![3, 6, 9, 12, 15], 444), 30);
}

#[test]
fn test_49() {
    assert_eq!(Solution::coin_change(vec![3, 5, 7, 11, 13], 997), 77);
}

#[test]
fn test_50() {
    assert_eq!(Solution::coin_change(vec![1, 3, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31], 100), 4);
}

#[test]
fn test_51() {
    assert_eq!(Solution::coin_change(vec![7, 15, 23, 31], 750), 26);
}

#[test]
fn test_52() {
    assert_eq!(Solution::coin_change(vec![23, 14, 5, 12], 5000), 220);
}

#[test]
fn test_53() {
    assert_eq!(Solution::coin_change(vec![11, 23, 37, 41, 43, 47, 53], 12345), 235);
}

#[test]
fn test_54() {
    assert_eq!(Solution::coin_change(vec![1, 5, 10, 25, 50, 100], 357), 7);
}

#[test]
fn test_55() {
    assert_eq!(Solution::coin_change(vec![18, 24, 27, 36, 50], 999), 21);
}

#[test]
fn test_56() {
    assert_eq!(Solution::coin_change(vec![1, 5, 10, 25, 50], 9999), 206);
}

#[test]
fn test_57() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5, 10, 20, 50, 100, 200], 9999), 56);
}

#[test]
fn test_58() {
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 25], 100), 4);
}

#[test]
fn test_59() {
    assert_eq!(Solution::coin_change(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048], 1048575), 522);
}

#[test]
fn test_60() {
    assert_eq!(Solution::coin_change(vec![7, 15, 23, 42], 999), 27);
}

#[test]
fn test_61() {
    assert_eq!(Solution::coin_change(vec![7, 17, 23, 29, 31], 1000), 34);
}

#[test]
fn test_62() {
    assert_eq!(Solution::coin_change(vec![1, 3, 4, 6, 8, 10], 450), 45);
}

#[test]
fn test_63() {
    assert_eq!(Solution::coin_change(vec![1, 5, 10, 20, 50, 100], 10001), 101);
}

#[test]
fn test_64() {
    assert_eq!(Solution::coin_change(vec![2, 3, 7], 100), 15);
}

#[test]
fn test_65() {
    assert_eq!(Solution::coin_change(vec![5, 11, 23, 37, 41, 43, 61, 71, 73, 79, 83, 89], 999), 13);
}

#[test]
fn test_66() {
    assert_eq!(Solution::coin_change(vec![1, 10, 100, 1000], 9999), 36);
}

#[test]
fn test_67() {
    assert_eq!(Solution::coin_change(vec![1, 10, 25, 50, 100], 12345), 126);
}

#[test]
fn test_68() {
    assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
}

#[test]
fn test_69() {
    assert_eq!(Solution::coin_change(vec![2, 3, 7], 27), 5);
}

#[test]
fn test_70() {
    assert_eq!(Solution::coin_change(vec![3, 6, 9, 12, 15], 9000), 600);
}

#[test]
fn test_71() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5, 10, 20, 50, 100], 9999), 105);
}

#[test]
fn test_72() {
    assert_eq!(Solution::coin_change(vec![5, 9, 12, 27, 31], 1276), 42);
}

#[test]
fn test_73() {
    assert_eq!(Solution::coin_change(vec![2, 3, 7, 11, 19, 23], 987), 44);
}

#[test]
fn test_74() {
    assert_eq!(Solution::coin_change(vec![10, 25, 50, 100], 9876), -1);
}

#[test]
fn test_75() {
    assert_eq!(Solution::coin_change(vec![2, 6, 10, 14], 9999), -1);
}

#[test]
fn test_76() {
    assert_eq!(Solution::coin_change(vec![10, 22, 35], 1000), 30);
}

#[test]
fn test_77() {
    assert_eq!(Solution::coin_change(vec![2, 5, 11, 17, 23], 1000), 44);
}

#[test]
fn test_78() {
    assert_eq!(Solution::coin_change(vec![23, 37, 41, 53, 67, 71], 8675309), 122189);
}

#[test]
fn test_79() {
    assert_eq!(Solution::coin_change(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 12), 12);
}

#[test]
fn test_80() {
    assert_eq!(Solution::coin_change(vec![1, 6, 10], 111), 12);
}

#[test]
fn test_81() {
    assert_eq!(Solution::coin_change(vec![7, 14, 33, 19, 100], 12345), 126);
}

#[test]
fn test_82() {
    assert_eq!(Solution::coin_change(vec![13, 21, 34, 55, 89, 144], 6765), 51);
}

#[test]
fn test_83() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5, 10], 27), 4);
}

#[test]
fn test_84() {
    assert_eq!(Solution::coin_change(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29], 8000), 277);
}

#[test]
fn test_85() {
    assert_eq!(Solution::coin_change(vec![1, 5, 10, 25], 99), 9);
}

#[test]
fn test_86() {
    assert_eq!(Solution::coin_change(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048], 1023), 10);
}

#[test]
fn test_87() {
    assert_eq!(Solution::coin_change(vec![10, 15, 20], 120), 6);
}

#[test]
fn test_88() {
    assert_eq!(Solution::coin_change(vec![1, 2, 3, 4, 5], 10000), 2000);
}

#[test]
fn test_89() {
    assert_eq!(Solution::coin_change(vec![1, 11, 21, 31], 10000), 330);
}

#[test]
fn test_90() {
    assert_eq!(Solution::coin_change(vec![1, 3, 5, 7, 9], 10000), 1112);
}

#[test]
fn test_91() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5, 10, 20, 50, 100, 200], 399), 8);
}

#[test]
fn test_92() {
    assert_eq!(Solution::coin_change(vec![13, 19, 23, 29, 31, 37, 41], 9998), 246);
}

#[test]
fn test_93() {
    assert_eq!(Solution::coin_change(vec![1, 6, 13, 37, 150], 9999), 71);
}

#[test]
fn test_94() {
    assert_eq!(Solution::coin_change(vec![10, 20, 50, 100, 200], 10000), 50);
}

#[test]
fn test_95() {
    assert_eq!(Solution::coin_change(vec![1, 5, 10, 25, 50], 87), 5);
}

#[test]
fn test_96() {
    assert_eq!(Solution::coin_change(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10000), 1000);
}

#[test]
fn test_97() {
    assert_eq!(Solution::coin_change(vec![100, 200, 300, 400, 500], 9999), -1);
}

#[test]
fn test_98() {
    assert_eq!(Solution::coin_change(vec![1, 3, 4, 5], 150), 30);
}

#[test]
fn test_99() {
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 20], 98), 9);
}

#[test]
fn test_100() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5, 10, 25], 9999), 403);
}

#[test]
fn test_101() {
    assert_eq!(Solution::coin_change(vec![10, 25, 50], 9999), -1);
}

#[test]
fn test_102() {
    assert_eq!(Solution::coin_change(vec![11, 17, 29, 31], 10000), 324);
}

#[test]
fn test_103() {
    assert_eq!(Solution::coin_change(vec![7, 14, 28, 29], 10000), 345);
}

#[test]
fn test_104() {
    assert_eq!(Solution::coin_change(vec![10, 25, 50], 99), -1);
}

#[test]
fn test_105() {
    assert_eq!(Solution::coin_change(vec![1, 3, 4, 5, 7], 10000), 1429);
}

#[test]
fn test_106() {
    assert_eq!(Solution::coin_change(vec![100, 50, 20, 10, 5, 1], 19876), 202);
}

#[test]
fn test_107() {
    assert_eq!(Solution::coin_change(vec![1, 3, 4, 5], 15), 3);
}

#[test]
fn test_108() {
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 20, 50, 100], 9999), 105);
}

#[test]
fn test_109() {
    assert_eq!(Solution::coin_change(vec![13, 17, 19], 1234), 66);
}

#[test]
fn test_110() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5, 10, 20, 50, 100], 3689), 42);
}

#[test]
fn test_111() {
    assert_eq!(Solution::coin_change(vec![335, 23, 102, 75, 402], 9783), 30);
}

#[test]
fn test_112() {
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 20, 50], 399), 12);
}

#[test]
fn test_113() {
    assert_eq!(Solution::coin_change(vec![17, 29, 31, 37, 41, 43], 8942), 208);
}

#[test]
fn test_114() {
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 20, 50, 100], 363), 9);
}

#[test]
fn test_115() {
    assert_eq!(Solution::coin_change(vec![12, 25, 50, 100, 200], 3678), 37);
}

#[test]
fn test_116() {
    assert_eq!(Solution::coin_change(vec![13, 23, 33, 43, 53], 888), 26);
}

#[test]
fn test_117() {
    assert_eq!(Solution::coin_change(vec![3, 6, 9, 12, 15, 18], 100), -1);
}

#[test]
fn test_118() {
    assert_eq!(Solution::coin_change(vec![11, 17, 23], 457), 23);
}

#[test]
fn test_119() {
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 25], 9999), 403);
}

#[test]
fn test_120() {
    assert_eq!(Solution::coin_change(vec![29, 81, 135, 50, 1], 2101), 19);
}

#[test]
fn test_121() {
    assert_eq!(Solution::coin_change(vec![33, 37, 41, 43, 47, 53, 59], 10000), 170);
}

#[test]
fn test_122() {
    assert_eq!(Solution::coin_change(vec![5, 11, 13], 10000), 770);
}

#[test]
fn test_123() {
    assert_eq!(Solution::coin_change(vec![10, 20, 50, 100], 345), -1);
}

#[test]
fn test_124() {
    assert_eq!(Solution::coin_change(vec![5, 10, 20, 50, 100], 4321), -1);
}

#[test]
fn test_125() {
    assert_eq!(Solution::coin_change(vec![13, 112, 197, 84, 205], 4873), 26);
}

#[test]
fn test_126() {
    assert_eq!(Solution::coin_change(vec![33, 77, 111, 155], 8500), 60);
}

#[test]
fn test_127() {
    assert_eq!(Solution::coin_change(vec![7, 15, 23], 1000), 48);
}

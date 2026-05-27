include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::rob(vec![1]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

#[test]
fn test_3() {
    assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
}

#[test]
fn test_4() {
    assert_eq!(Solution::rob(vec![5, 2, 6, 3, 4, 1]), 15);
}

#[test]
fn test_5() {
    assert_eq!(Solution::rob(vec![5, 2, 6, 3, 1, 8, 9, 4, 7]), 28);
}

#[test]
fn test_6() {
    assert_eq!(Solution::rob(vec![1, 2]), 2);
}

#[test]
fn test_7() {
    assert_eq!(Solution::rob(vec![10, 2, 3, 8, 10, 1]), 23);
}

#[test]
fn test_8() {
    assert_eq!(Solution::rob(vec![0]), 0);
}

#[test]
fn test_9() {
    assert_eq!(Solution::rob(vec![3, 2, 5, 10, 7]), 15);
}

#[test]
fn test_10() {
    assert_eq!(Solution::rob(vec![5, 10, 5, 10]), 20);
}

#[test]
fn test_11() {
    assert_eq!(Solution::rob(vec![1, 3, 1, 3, 100]), 103);
}

#[test]
fn test_12() {
    assert_eq!(Solution::rob(vec![10, 20, 30, 40, 50]), 90);
}

#[test]
fn test_13() {
    assert_eq!(Solution::rob(vec![0, 0, 0, 0]), 0);
}

#[test]
fn test_14() {
    assert_eq!(Solution::rob(vec![50, 10, 100, 10, 5]), 155);
}

#[test]
fn test_15() {
    assert_eq!(Solution::rob(vec![3, 5, 100, 1, 1, 1, 1, 1, 100, 1, 1, 1, 1]), 207);
}

#[test]
fn test_16() {
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}

#[test]
fn test_17() {
    assert_eq!(Solution::rob(vec![3, 10, 3, 1, 2, 1]), 12);
}

#[test]
fn test_18() {
    assert_eq!(Solution::rob(vec![300, 400, 500, 100, 200, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500]), 6500);
}

#[test]
fn test_19() {
    assert_eq!(Solution::rob(vec![299, 399, 199, 299, 399, 199, 299, 399, 199, 299, 399, 199, 299, 399, 199, 299, 399, 199, 299, 399]), 3090);
}

#[test]
fn test_20() {
    assert_eq!(Solution::rob(vec![10, 5, 15, 20, 25, 30, 35, 40]), 100);
}

#[test]
fn test_21() {
    assert_eq!(Solution::rob(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]), 136);
}

#[test]
fn test_22() {
    assert_eq!(Solution::rob(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), 3000);
}

#[test]
fn test_23() {
    assert_eq!(Solution::rob(vec![400, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 400);
}

#[test]
fn test_24() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 30);
}

#[test]
fn test_25() {
    assert_eq!(Solution::rob(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]), 27);
}

#[test]
fn test_26() {
    assert_eq!(Solution::rob(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000]), 11000);
}

#[test]
fn test_27() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 42);
}

#[test]
fn test_28() {
    assert_eq!(Solution::rob(vec![399, 0, 399, 0, 399, 0, 399, 0, 399, 0, 399, 0, 399, 0, 399, 0, 399]), 3591);
}

#[test]
fn test_29() {
    assert_eq!(Solution::rob(vec![100, 0, 100, 0, 100, 0, 100, 0, 100]), 500);
}

#[test]
fn test_30() {
    assert_eq!(Solution::rob(vec![4, 1, 2, 7, 5, 3, 9, 2, 8, 6, 4, 10, 1, 2, 5, 3, 7, 8, 6, 9]), 60);
}

#[test]
fn test_31() {
    assert_eq!(Solution::rob(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10);
}

#[test]
fn test_32() {
    assert_eq!(Solution::rob(vec![200, 100, 150, 50, 300, 100, 400, 200, 350, 150]), 1400);
}

#[test]
fn test_33() {
    assert_eq!(Solution::rob(vec![300, 200, 100, 50, 25, 10, 5, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 458);
}

#[test]
fn test_34() {
    assert_eq!(Solution::rob(vec![400, 399, 398, 397, 396, 395, 394, 393, 392, 391]), 1980);
}

#[test]
fn test_35() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 240);
}

#[test]
fn test_36() {
    assert_eq!(Solution::rob(vec![400, 0, 400, 0, 400, 0, 400, 0, 400, 0]), 2000);
}

#[test]
fn test_37() {
    assert_eq!(Solution::rob(vec![10, 20, 10, 20, 10, 20, 10, 20, 10, 20]), 100);
}

#[test]
fn test_38() {
    assert_eq!(Solution::rob(vec![10, 1, 20, 3, 40, 5, 60, 7, 80]), 210);
}

#[test]
fn test_39() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 64);
}

#[test]
fn test_40() {
    assert_eq!(Solution::rob(vec![10, 5, 1, 2, 12, 4, 5, 6, 1, 2]), 31);
}

#[test]
fn test_41() {
    assert_eq!(Solution::rob(vec![100, 1, 1, 100, 1, 1, 100, 1, 1, 100]), 400);
}

#[test]
fn test_42() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 420);
}

#[test]
fn test_43() {
    assert_eq!(Solution::rob(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_44() {
    assert_eq!(Solution::rob(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 400]), 400);
}

#[test]
fn test_45() {
    assert_eq!(Solution::rob(vec![1, 3, 1, 1, 3, 1, 1, 3, 1, 1]), 10);
}

#[test]
fn test_46() {
    assert_eq!(Solution::rob(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 16);
}

#[test]
fn test_47() {
    assert_eq!(Solution::rob(vec![400, 300, 200, 100, 0, 0, 0, 0, 0, 0]), 600);
}

#[test]
fn test_48() {
    assert_eq!(Solution::rob(vec![100, 0, 50, 0, 200, 0, 300, 0, 400, 0]), 1050);
}

#[test]
fn test_49() {
    assert_eq!(Solution::rob(vec![300, 200, 100, 400, 300, 500, 400, 600, 500, 700]), 2500);
}

#[test]
fn test_50() {
    assert_eq!(Solution::rob(vec![150, 100, 120, 200, 180, 170, 160, 250, 300, 290]), 1060);
}

#[test]
fn test_51() {
    assert_eq!(Solution::rob(vec![300, 0, 400, 0, 300, 0]), 1000);
}

#[test]
fn test_52() {
    assert_eq!(Solution::rob(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 400]), 435);
}

#[test]
fn test_53() {
    assert_eq!(Solution::rob(vec![400, 300, 200, 100, 50, 40]), 650);
}

#[test]
fn test_54() {
    assert_eq!(Solution::rob(vec![300, 0, 0, 300, 0, 0, 300, 0, 0, 300]), 1200);
}

#[test]
fn test_55() {
    assert_eq!(Solution::rob(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), 55);
}

#[test]
fn test_56() {
    assert_eq!(Solution::rob(vec![2, 1, 1, 2, 1, 2, 1, 1, 2, 1, 2, 1, 1, 2, 1, 2, 1]), 14);
}

#[test]
fn test_57() {
    assert_eq!(Solution::rob(vec![1, 3, 1, 5, 2, 2, 1, 3, 4]), 14);
}

#[test]
fn test_58() {
    assert_eq!(Solution::rob(vec![3, 0, 3, 0, 3, 0, 3, 0, 3, 0, 3, 0, 3]), 21);
}

#[test]
fn test_59() {
    assert_eq!(Solution::rob(vec![100, 200, 300, 400, 500, 600]), 1200);
}

#[test]
fn test_60() {
    assert_eq!(Solution::rob(vec![1, 3, 1, 3, 100, 1, 1, 1, 1, 1]), 105);
}

#[test]
fn test_61() {
    assert_eq!(Solution::rob(vec![400, 0, 400, 0, 400, 0, 400, 0, 400, 0, 400, 0, 400, 0, 400, 0, 400, 0, 400, 0]), 4000);
}

#[test]
fn test_62() {
    assert_eq!(Solution::rob(vec![5, 4, 3, 2, 1, 0, 9, 8, 7, 6]), 25);
}

#[test]
fn test_63() {
    assert_eq!(Solution::rob(vec![4, 2, 3, 6, 3, 8, 5, 10, 7]), 28);
}

#[test]
fn test_64() {
    assert_eq!(Solution::rob(vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55]), 175);
}

#[test]
fn test_65() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 110);
}

#[test]
fn test_66() {
    assert_eq!(Solution::rob(vec![399, 1, 399, 1, 399, 1, 399, 1, 399, 1]), 1995);
}

#[test]
fn test_67() {
    assert_eq!(Solution::rob(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_68() {
    assert_eq!(Solution::rob(vec![3, 10, 3, 1, 2, 9, 5, 4, 6, 8, 1, 7]), 39);
}

#[test]
fn test_69() {
    assert_eq!(Solution::rob(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 30);
}

#[test]
fn test_70() {
    assert_eq!(Solution::rob(vec![10, 5, 15, 20, 25, 30, 35, 40, 45, 50]), 150);
}

#[test]
fn test_71() {
    assert_eq!(Solution::rob(vec![100, 0, 100, 0, 100, 0, 100]), 400);
}

#[test]
fn test_72() {
    assert_eq!(Solution::rob(vec![3, 2, 5, 10, 7, 8, 5, 1, 100, 2, 5, 8, 5, 1, 100]), 231);
}

#[test]
fn test_73() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1, 1, 2, 3, 1, 1, 2]), 10);
}

#[test]
fn test_74() {
    assert_eq!(Solution::rob(vec![10, 5, 10, 5, 10, 5, 10, 5, 10, 5]), 50);
}

#[test]
fn test_75() {
    assert_eq!(Solution::rob(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]), 128);
}

#[test]
fn test_76() {
    assert_eq!(Solution::rob(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 5);
}

#[test]
fn test_77() {
    assert_eq!(Solution::rob(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 300);
}

#[test]
fn test_78() {
    assert_eq!(Solution::rob(vec![1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3]), 30);
}

#[test]
fn test_79() {
    assert_eq!(Solution::rob(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 60);
}

#[test]
fn test_80() {
    assert_eq!(Solution::rob(vec![200, 100, 200, 200, 100, 300, 150, 250, 200, 100]), 1050);
}

#[test]
fn test_81() {
    assert_eq!(Solution::rob(vec![300, 1, 1, 300, 1, 1, 300, 1, 1, 300, 1, 1, 300, 1, 1, 300, 1, 1, 300, 1]), 2100);
}

#[test]
fn test_82() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 30);
}

#[test]
fn test_83() {
    assert_eq!(Solution::rob(vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0]), 7);
}

#[test]
fn test_84() {
    assert_eq!(Solution::rob(vec![1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3]), 18);
}

#[test]
fn test_85() {
    assert_eq!(Solution::rob(vec![300, 100, 200, 150, 250, 125, 350]), 1100);
}

#[test]
fn test_86() {
    assert_eq!(Solution::rob(vec![100, 1, 200, 3, 400, 5]), 700);
}

#[test]
fn test_87() {
    assert_eq!(Solution::rob(vec![0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_88() {
    assert_eq!(Solution::rob(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_89() {
    assert_eq!(Solution::rob(vec![150, 200, 50, 100, 100, 50, 200, 150, 100, 100, 50, 200, 150]), 800);
}

#[test]
fn test_90() {
    assert_eq!(Solution::rob(vec![100, 200, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 260);
}

#[test]
fn test_91() {
    assert_eq!(Solution::rob(vec![300, 200, 100, 0, 50, 10, 20, 30, 40, 5]), 510);
}

#[test]
fn test_92() {
    assert_eq!(Solution::rob(vec![10, 22, 9, 33, 21, 50, 41, 60, 80, 70]), 235);
}

#[test]
fn test_93() {
    assert_eq!(Solution::rob(vec![50, 1, 50, 1, 50, 1, 50, 1, 50, 1]), 250);
}

#[test]
fn test_94() {
    assert_eq!(Solution::rob(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_95() {
    assert_eq!(Solution::rob(vec![300, 200, 100, 400, 300, 500, 200]), 1200);
}

#[test]
fn test_96() {
    assert_eq!(Solution::rob(vec![300, 200, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 400);
}

#[test]
fn test_97() {
    assert_eq!(Solution::rob(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_98() {
    assert_eq!(Solution::rob(vec![50, 100, 200, 50, 100, 200, 50, 100, 200, 50]), 650);
}

#[test]
fn test_99() {
    assert_eq!(Solution::rob(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 50);
}

#[test]
fn test_100() {
    assert_eq!(Solution::rob(vec![1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1]), 18);
}

#[test]
fn test_101() {
    assert_eq!(Solution::rob(vec![50, 10, 40, 70, 30, 60, 20, 80, 100, 90]), 350);
}

#[test]
fn test_102() {
    assert_eq!(Solution::rob(vec![31, 41, 59, 26, 53, 58, 97, 93, 23, 84, 62, 64, 33, 83, 27, 95, 8, 41, 32, 9]), 617);
}

#[test]
fn test_103() {
    assert_eq!(Solution::rob(vec![400, 300, 200, 100, 0, 100, 200, 300, 400]), 1200);
}

#[test]
fn test_104() {
    assert_eq!(Solution::rob(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 90);
}

#[test]
fn test_105() {
    assert_eq!(Solution::rob(vec![4, 1, 2, 7, 5, 3, 1, 1, 1, 2, 3, 4, 5]), 23);
}

#[test]
fn test_106() {
    assert_eq!(Solution::rob(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 20);
}

#[test]
fn test_107() {
    assert_eq!(Solution::rob(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21]), 66);
}

#[test]
fn test_108() {
    assert_eq!(Solution::rob(vec![3, 7, 5, 1, 9, 2, 6, 4]), 23);
}

#[test]
fn test_109() {
    assert_eq!(Solution::rob(vec![400, 399, 398, 397, 396, 395, 394, 393, 392, 391]), 1980);
}

#[test]
fn test_110() {
    assert_eq!(Solution::rob(vec![4, 1, 2, 7, 5, 3, 1, 1, 1, 1]), 16);
}

#[test]
fn test_111() {
    assert_eq!(Solution::rob(vec![10, 15, 10, 5, 10, 15, 10, 5, 10, 15]), 55);
}

#[test]
fn test_112() {
    assert_eq!(Solution::rob(vec![10, 5, 10, 5, 15, 20]), 40);
}

#[test]
fn test_113() {
    assert_eq!(Solution::rob(vec![2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2]), 10);
}

#[test]
fn test_114() {
    assert_eq!(Solution::rob(vec![10, 0, 0, 0, 0, 10, 0, 0, 0, 0, 10]), 30);
}

#[test]
fn test_115() {
    assert_eq!(Solution::rob(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), 3000);
}

#[test]
fn test_116() {
    assert_eq!(Solution::rob(vec![3, 5, 100, 1, 1, 1, 1, 1, 100, 1, 1, 5, 3]), 210);
}

#[test]
fn test_117() {
    assert_eq!(Solution::rob(vec![8, 2, 8, 10, 6, 5, 1, 9, 3, 7, 4, 2, 5]), 44);
}

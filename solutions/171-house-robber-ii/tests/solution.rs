include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

#[test]
fn test_2() {
    assert_eq!(Solution::rob(vec![1, 0, 1, 0, 1]), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::rob(vec![5, 1, 2, 4, 7, 8]), 14);
}

#[test]
fn test_4() {
    assert_eq!(Solution::rob(vec![5, 3, 1, 1, 1]), 6);
}

#[test]
fn test_5() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 30);
}

#[test]
fn test_6() {
    assert_eq!(Solution::rob(vec![10, 20, 30, 40, 50]), 80);
}

#[test]
fn test_7() {
    assert_eq!(Solution::rob(vec![300, 3, 140, 20, 10, 5, 5, 200]), 455);
}

#[test]
fn test_8() {
    assert_eq!(Solution::rob(vec![5, 5, 5, 5, 5]), 10);
}

#[test]
fn test_9() {
    assert_eq!(Solution::rob(vec![4, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 25);
}

#[test]
fn test_10() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5]), 8);
}

#[test]
fn test_11() {
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 11);
}

#[test]
fn test_12() {
    assert_eq!(Solution::rob(vec![1000]), 1000);
}

#[test]
fn test_13() {
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
}

#[test]
fn test_14() {
    assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 3);
}

#[test]
fn test_15() {
    assert_eq!(Solution::rob(vec![3, 6, 4, 2, 5, 7, 8]), 19);
}

#[test]
fn test_16() {
    assert_eq!(Solution::rob(vec![1, 1, 1, 1, 1]), 2);
}

#[test]
fn test_17() {
    assert_eq!(Solution::rob(vec![0]), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::rob(vec![200, 3, 140, 20, 10]), 340);
}

#[test]
fn test_19() {
    assert_eq!(Solution::rob(vec![5, 5, 10, 100, 10, 5, 5]), 110);
}

#[test]
fn test_20() {
    assert_eq!(Solution::rob(vec![300, 3, 100, 4]), 400);
}

#[test]
fn test_21() {
    assert_eq!(Solution::rob(vec![5, 1, 1, 5]), 6);
}

#[test]
fn test_22() {
    assert_eq!(Solution::rob(vec![1, 2]), 2);
}

#[test]
fn test_23() {
    assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
}

#[test]
fn test_24() {
    assert_eq!(Solution::rob(vec![10, 2, 3, 4, 5, 6]), 18);
}

#[test]
fn test_25() {
    assert_eq!(Solution::rob(vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000]), 7000);
}

#[test]
fn test_26() {
    assert_eq!(Solution::rob(vec![200, 300, 150, 100, 250, 300, 200, 100, 50, 400, 350, 250]), 1450);
}

#[test]
fn test_27() {
    assert_eq!(Solution::rob(vec![100, 0, 100, 0, 100, 0, 100, 0, 100, 0]), 500);
}

#[test]
fn test_28() {
    assert_eq!(Solution::rob(vec![50, 100, 150, 200, 250, 300]), 600);
}

#[test]
fn test_29() {
    assert_eq!(Solution::rob(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 110);
}

#[test]
fn test_30() {
    assert_eq!(Solution::rob(vec![500, 400, 300, 200, 100, 0, 100, 200, 300, 400, 500, 0, 500, 400, 300, 200, 100, 0, 100, 200, 300, 400, 500]), 3300);
}

#[test]
fn test_31() {
    assert_eq!(Solution::rob(vec![200, 100, 50, 150, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 100, 50, 150, 200, 300, 400, 500]), 4050);
}

#[test]
fn test_32() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100]), 2550);
}

#[test]
fn test_33() {
    assert_eq!(Solution::rob(vec![10, 1, 1, 10, 1, 1, 10, 1, 1, 10, 1, 1, 10, 1, 1, 10, 1, 1, 10, 1]), 70);
}

#[test]
fn test_34() {
    assert_eq!(Solution::rob(vec![1, 10, 100, 1000, 900, 800, 700, 600, 500, 400, 300, 200, 100, 10, 1]), 3020);
}

#[test]
fn test_35() {
    assert_eq!(Solution::rob(vec![90, 200, 200, 100, 400, 90, 200, 200, 100, 400, 90, 200, 200, 100, 400]), 1800);
}

#[test]
fn test_36() {
    assert_eq!(Solution::rob(vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3]), 24);
}

#[test]
fn test_37() {
    assert_eq!(Solution::rob(vec![1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1]), 5000);
}

#[test]
fn test_38() {
    assert_eq!(Solution::rob(vec![5, 10, 3, 12, 8, 6, 11, 7, 14, 9]), 47);
}

#[test]
fn test_39() {
    assert_eq!(Solution::rob(vec![5, 2, 6, 2, 5, 2, 6, 2, 5, 2, 6, 2, 5, 2, 6]), 39);
}

#[test]
fn test_40() {
    assert_eq!(Solution::rob(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 63);
}

#[test]
fn test_41() {
    assert_eq!(Solution::rob(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10);
}

#[test]
fn test_42() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]), 12);
}

#[test]
fn test_43() {
    assert_eq!(Solution::rob(vec![100, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 100, 100]), 208);
}

#[test]
fn test_44() {
    assert_eq!(Solution::rob(vec![1000, 900, 800, 700, 600, 500, 400, 300, 200, 100]), 3000);
}

#[test]
fn test_45() {
    assert_eq!(Solution::rob(vec![25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 168);
}

#[test]
fn test_46() {
    assert_eq!(Solution::rob(vec![100, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 100]), 109);
}

#[test]
fn test_47() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 240);
}

#[test]
fn test_48() {
    assert_eq!(Solution::rob(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 35);
}

#[test]
fn test_49() {
    assert_eq!(Solution::rob(vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3]), 84);
}

#[test]
fn test_50() {
    assert_eq!(Solution::rob(vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 100, 100, 0, 0, 0, 0, 0, 0, 0, 0, 100]), 200);
}

#[test]
fn test_51() {
    assert_eq!(Solution::rob(vec![500, 100, 500, 100, 500, 100, 500, 100, 500, 100, 500, 100, 500]), 3000);
}

#[test]
fn test_52() {
    assert_eq!(Solution::rob(vec![123, 456, 789, 101, 202, 303, 404, 505, 606, 707, 808, 909]), 3213);
}

#[test]
fn test_53() {
    assert_eq!(Solution::rob(vec![200, 1, 100, 1, 100, 1, 200, 1, 100, 1, 100, 1]), 800);
}

#[test]
fn test_54() {
    assert_eq!(Solution::rob(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90]), 500);
}

#[test]
fn test_55() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 63);
}

#[test]
fn test_56() {
    assert_eq!(Solution::rob(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_57() {
    assert_eq!(Solution::rob(vec![1000, 0, 0, 0, 0, 0, 0, 0, 0, 1000]), 1000);
}

#[test]
fn test_58() {
    assert_eq!(Solution::rob(vec![10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 10]), 19);
}

#[test]
fn test_59() {
    assert_eq!(Solution::rob(vec![800, 200, 100, 300, 600, 400, 500, 700, 50, 90]), 2200);
}

#[test]
fn test_60() {
    assert_eq!(Solution::rob(vec![999, 0, 999, 0, 999, 0, 999, 0, 999, 0]), 4995);
}

#[test]
fn test_61() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 50);
}

#[test]
fn test_62() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 200);
}

#[test]
fn test_63() {
    assert_eq!(Solution::rob(vec![50, 1, 1, 50, 1, 1, 50, 1, 1, 50, 1, 1]), 200);
}

#[test]
fn test_64() {
    assert_eq!(Solution::rob(vec![5, 100, 5, 100, 5, 100, 5, 100, 5, 100, 5, 100, 5, 100, 5, 100]), 800);
}

#[test]
fn test_65() {
    assert_eq!(Solution::rob(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 100]), 2500);
}

#[test]
fn test_66() {
    assert_eq!(Solution::rob(vec![200, 3, 140, 20, 10, 5, 5, 200, 150, 250, 350, 450]), 1050);
}

#[test]
fn test_67() {
    assert_eq!(Solution::rob(vec![1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0]), 10000);
}

#[test]
fn test_68() {
    assert_eq!(Solution::rob(vec![5, 10, 5, 10, 5, 10, 5]), 30);
}

#[test]
fn test_69() {
    assert_eq!(Solution::rob(vec![5, 1, 2, 9, 1, 5, 1, 2, 9, 1, 5, 1, 2, 9, 1, 5, 1, 2, 9, 1]), 56);
}

#[test]
fn test_70() {
    assert_eq!(Solution::rob(vec![10, 2, 3, 4, 1, 5, 6, 7, 8, 9, 10]), 35);
}

#[test]
fn test_71() {
    assert_eq!(Solution::rob(vec![9, 4, 1, 7, 3, 2, 5, 6, 8, 0, 12, 15]), 41);
}

#[test]
fn test_72() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 110);
}

#[test]
fn test_73() {
    assert_eq!(Solution::rob(vec![2, 3, 5, 6, 7, 8, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 44);
}

#[test]
fn test_74() {
    assert_eq!(Solution::rob(vec![1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100]), 1000);
}

#[test]
fn test_75() {
    assert_eq!(Solution::rob(vec![10, 1, 1, 1, 10, 1, 1, 1, 10, 1, 1, 1, 10, 1, 1, 1]), 44);
}

#[test]
fn test_76() {
    assert_eq!(Solution::rob(vec![999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985]), 6951);
}

#[test]
fn test_77() {
    assert_eq!(Solution::rob(vec![10, 15, 20, 25, 30, 35, 40, 45, 50]), 140);
}

#[test]
fn test_78() {
    assert_eq!(Solution::rob(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 50);
}

#[test]
fn test_79() {
    assert_eq!(Solution::rob(vec![100, 0, 100, 0, 100, 0, 100]), 300);
}

#[test]
fn test_80() {
    assert_eq!(Solution::rob(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 190);
}

#[test]
fn test_81() {
    assert_eq!(Solution::rob(vec![50, 1, 1, 50, 1, 1, 50, 1, 1, 50]), 151);
}

#[test]
fn test_82() {
    assert_eq!(Solution::rob(vec![50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1, 50, 1]), 500);
}

#[test]
fn test_83() {
    assert_eq!(Solution::rob(vec![50, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), 640);
}

#[test]
fn test_84() {
    assert_eq!(Solution::rob(vec![1000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 1000);
}

#[test]
fn test_85() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 168);
}

#[test]
fn test_86() {
    assert_eq!(Solution::rob(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 300);
}

#[test]
fn test_87() {
    assert_eq!(Solution::rob(vec![1, 3, 1, 3, 100, 1, 3, 1, 3, 1, 3, 100]), 209);
}

#[test]
fn test_88() {
    assert_eq!(Solution::rob(vec![10, 2, 3, 4, 5, 1, 2, 3, 4, 5]), 24);
}

#[test]
fn test_89() {
    assert_eq!(Solution::rob(vec![1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3]), 30);
}

#[test]
fn test_90() {
    assert_eq!(Solution::rob(vec![10, 2, 5, 3, 7, 9, 8, 10, 2, 5]), 34);
}

#[test]
fn test_91() {
    assert_eq!(Solution::rob(vec![200, 300, 200, 300, 200, 300, 200, 300, 200, 300, 200, 300, 200, 300, 200, 300, 200, 300, 200, 300]), 3000);
}

#[test]
fn test_92() {
    assert_eq!(Solution::rob(vec![50, 100, 50, 100, 50, 100, 50]), 300);
}

#[test]
fn test_93() {
    assert_eq!(Solution::rob(vec![100, 100, 0, 100, 0, 100, 0, 100, 0, 100]), 500);
}

#[test]
fn test_94() {
    assert_eq!(Solution::rob(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 900, 800, 700, 600, 500, 400, 300, 200, 100, 50]), 5050);
}

#[test]
fn test_95() {
    assert_eq!(Solution::rob(vec![250, 200, 150, 100, 50, 0, 50, 100, 150, 200, 250, 0, 250, 200, 150, 100, 50, 0, 50, 100, 150, 200, 250]), 1650);
}

#[test]
fn test_96() {
    assert_eq!(Solution::rob(vec![1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0, 1000, 0]), 8000);
}

#[test]
fn test_97() {
    assert_eq!(Solution::rob(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 110);
}

#[test]
fn test_98() {
    assert_eq!(Solution::rob(vec![20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20]), 200);
}

#[test]
fn test_99() {
    assert_eq!(Solution::rob(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), 630);
}

#[test]
fn test_100() {
    assert_eq!(Solution::rob(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50]), 500);
}

#[test]
fn test_101() {
    assert_eq!(Solution::rob(vec![50, 1, 50, 1, 50, 1, 50, 1, 50, 1]), 250);
}

#[test]
fn test_102() {
    assert_eq!(Solution::rob(vec![1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000]), 10000);
}

#[test]
fn test_103() {
    assert_eq!(Solution::rob(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 300);
}

#[test]
fn test_104() {
    assert_eq!(Solution::rob(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0]), 800);
}

#[test]
fn test_105() {
    assert_eq!(Solution::rob(vec![9, 3, 5, 7, 1, 3, 5, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 47);
}

#[test]
fn test_106() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 60);
}

#[test]
fn test_107() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 650);
}

#[test]
fn test_108() {
    assert_eq!(Solution::rob(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_109() {
    assert_eq!(Solution::rob(vec![50, 100, 200, 300, 100, 50]), 450);
}

#[test]
fn test_110() {
    assert_eq!(Solution::rob(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250]), 1680);
}

#[test]
fn test_111() {
    assert_eq!(Solution::rob(vec![999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984, 983, 982, 981, 980, 979, 978, 977, 976, 975]), 11856);
}

#[test]
fn test_112() {
    assert_eq!(Solution::rob(vec![100, 0, 0, 100, 0, 0, 100, 0, 0, 100]), 300);
}

#[test]
fn test_113() {
    assert_eq!(Solution::rob(vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3]), 30);
}

#[test]
fn test_114() {
    assert_eq!(Solution::rob(vec![5, 2, 3, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5]), 52);
}

#[test]
fn test_115() {
    assert_eq!(Solution::rob(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 300);
}

#[test]
fn test_116() {
    assert_eq!(Solution::rob(vec![999, 1, 999, 1, 999, 1, 999, 1]), 3996);
}

#[test]
fn test_117() {
    assert_eq!(Solution::rob(vec![200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1]), 2000);
}

#[test]
fn test_118() {
    assert_eq!(Solution::rob(vec![999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1]), 9990);
}

#[test]
fn test_119() {
    assert_eq!(Solution::rob(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 90);
}

#[test]
fn test_120() {
    assert_eq!(Solution::rob(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 85);
}

#[test]
fn test_121() {
    assert_eq!(Solution::rob(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), 3000);
}

#[test]
fn test_122() {
    assert_eq!(Solution::rob(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 100);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::stone_game(vec![15, 30, 5, 10, 20, 25]), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::stone_game(vec![8, 15, 3, 7]), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::stone_game(vec![7, 3, 8, 5, 12, 10]), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 4, 5, 6, 7, 8]), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::stone_game(vec![1, 100, 1, 100, 1, 100, 1, 100]), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::stone_game(vec![2, 2, 2, 2, 2, 2]), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::stone_game(vec![10, 20, 30, 40, 50, 60]), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::stone_game(vec![8, 6, 5, 1, 7, 9]), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::stone_game(vec![10, 20, 30, 40]), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::stone_game(vec![1, 5, 2, 4, 6, 3]), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 4]), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::stone_game(vec![2, 2, 2, 2, 2, 2, 2, 2]), false);
}

#[test]
fn test_13() {
    assert_eq!(Solution::stone_game(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::stone_game(vec![3, 6, 9, 12, 15, 18]), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::stone_game(vec![8, 9, 7, 6, 5, 4]), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::stone_game(vec![1, 3, 5, 7, 9, 11]), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::stone_game(vec![10, 5, 1, 2, 3, 7, 4, 8]), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 4, 5, 6]), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::stone_game(vec![1, 100, 1, 100, 1, 100]), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::stone_game(vec![4, 3, 2, 1, 6, 5, 8, 7]), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::stone_game(vec![7, 2, 5, 10, 14, 3, 1, 2]), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::stone_game(vec![8, 9, 7, 6]), false);
}

#[test]
fn test_23() {
    assert_eq!(Solution::stone_game(vec![1, 2, 100, 3]), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::stone_game(vec![4, 1, 5, 2, 6, 3]), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 5, 4, 6]), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::stone_game(vec![5, 3, 4, 5]), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::stone_game(vec![100, 100, 100, 100, 100, 100]), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::stone_game(vec![2, 4, 6, 8, 10, 12]), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::stone_game(vec![3, 3, 3, 3, 3, 3, 3, 3]), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::stone_game(vec![3, 7, 2, 3]), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::stone_game(vec![100, 1, 100, 1, 100, 1]), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::stone_game(vec![7, 9, 8, 6]), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::stone_game(vec![29, 18, 17, 26, 34, 15, 45, 13, 50, 25, 30, 10, 35, 40, 5, 20]), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::stone_game(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::stone_game(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160]), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::stone_game(vec![5, 1, 100, 4, 10, 8, 6, 2]), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::stone_game(vec![10, 23, 5, 2, 7, 8, 3, 12, 15, 6]), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::stone_game(vec![1, 100, 2, 99, 3, 98, 4, 97]), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::stone_game(vec![8, 15, 3, 7, 6, 9, 5, 10, 4, 12, 11, 14, 2, 13]), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::stone_game(vec![100, 50, 200, 150, 300, 250, 400, 350, 500, 450]), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::stone_game(vec![150, 120, 180, 160, 200, 140, 220, 190, 210, 170]), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::stone_game(vec![50, 100, 75, 25, 120, 150, 80, 60]), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::stone_game(vec![200, 100, 150, 50, 250, 125, 300, 150, 350, 175, 400, 200, 450, 225, 500, 250]), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::stone_game(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::stone_game(vec![1, 100, 2, 99, 3, 98, 4, 97, 5, 96]), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::stone_game(vec![123, 456, 789, 101, 202, 303, 404, 505]), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::stone_game(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32]), true);
}

#[test]
fn test_50() {
    assert_eq!(Solution::stone_game(vec![12, 34, 56, 78, 90, 23, 45, 67, 89, 101, 123, 145]), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::stone_game(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::stone_game(vec![15, 23, 12, 18, 35, 10, 42, 8, 20, 14]), true);
}

#[test]
fn test_53() {
    assert_eq!(Solution::stone_game(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::stone_game(vec![10, 15, 3, 7, 8, 2, 1, 5]), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::stone_game(vec![8, 9, 5, 7, 2, 3, 4, 6, 1, 10]), true);
}

#[test]
fn test_56() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::stone_game(vec![25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14]), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::stone_game(vec![5, 4, 3, 2, 1, 6, 7, 8, 9, 10, 15, 14, 13, 12, 11, 20, 19, 18, 17, 16]), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::stone_game(vec![100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1]), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::stone_game(vec![2, 8, 4, 6, 10, 14, 12, 16, 18, 20]), true);
}

#[test]
fn test_62() {
    assert_eq!(Solution::stone_game(vec![1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000]), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::stone_game(vec![999, 1, 998, 2, 997, 3, 996, 4, 995, 5, 994, 6]), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::stone_game(vec![17, 23, 42, 35, 29, 49, 31, 47, 53, 41]), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::stone_game(vec![23, 45, 12, 34, 56, 78, 90, 12, 34, 56]), true);
}

#[test]
fn test_66() {
    assert_eq!(Solution::stone_game(vec![200, 300, 100, 400, 50, 600, 150, 700, 200, 800, 250, 900, 300, 1000, 350, 1100, 400, 1200, 450, 1300]), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::stone_game(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23]), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::stone_game(vec![8, 6, 4, 2, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23]), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::stone_game(vec![3, 1, 2, 5, 4, 6, 9, 7, 8, 10]), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::stone_game(vec![55, 65, 25, 30, 40, 50, 15, 20, 35, 45]), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::stone_game(vec![45, 55, 65, 75, 85, 95, 105, 115, 125, 135]), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::stone_game(vec![500, 1, 500, 1, 500, 1, 500, 1, 500, 1, 500, 1]), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::stone_game(vec![3, 2, 1, 6, 5, 4, 9, 8, 7, 12, 11, 10, 15, 14, 13, 18, 17, 16, 21, 20]), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::stone_game(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::stone_game(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6]), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::stone_game(vec![300, 200, 100, 50, 400, 350, 150, 250, 600, 550, 450, 100, 300, 200, 700, 650]), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::stone_game(vec![400, 10, 300, 20, 200, 30, 100, 40, 150, 50, 250, 60, 500, 5, 1, 800, 350, 450, 15, 25]), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::stone_game(vec![45, 22, 33, 11, 55, 66, 77, 88, 99, 111, 222, 333]), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::stone_game(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000]), true);
}

#[test]
fn test_80() {
    assert_eq!(Solution::stone_game(vec![250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250]), false);
}

#[test]
fn test_81() {
    assert_eq!(Solution::stone_game(vec![48, 32, 15, 22, 39, 28, 33, 27, 19, 25]), true);
}

#[test]
fn test_82() {
    assert_eq!(Solution::stone_game(vec![150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150]), false);
}

#[test]
fn test_83() {
    assert_eq!(Solution::stone_game(vec![10, 23, 15, 70, 6, 18, 40, 5]), true);
}

#[test]
fn test_84() {
    assert_eq!(Solution::stone_game(vec![150, 200, 50, 100, 350, 400, 150, 200, 50, 100, 350, 400, 150, 200, 50, 100]), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::stone_game(vec![500, 1, 500, 1, 500, 1, 500, 1, 500, 1]), true);
}

#[test]
fn test_86() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]), true);
}

#[test]
fn test_87() {
    assert_eq!(Solution::stone_game(vec![500, 1, 500, 2, 500, 3, 500, 4, 500, 5]), true);
}

#[test]
fn test_88() {
    assert_eq!(Solution::stone_game(vec![9, 3, 15, 2, 11, 8, 6, 10, 5, 4]), true);
}

#[test]
fn test_89() {
    assert_eq!(Solution::stone_game(vec![500, 1, 500, 1, 500, 1, 500, 1, 500, 1, 500, 1, 500, 1, 500, 1]), true);
}

#[test]
fn test_90() {
    assert_eq!(Solution::stone_game(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500]), true);
}

#[test]
fn test_91() {
    assert_eq!(Solution::stone_game(vec![10, 100, 10, 100, 10, 100, 10, 100, 10, 100, 10, 100, 10, 100, 10, 100]), true);
}

#[test]
fn test_92() {
    assert_eq!(Solution::stone_game(vec![101, 102, 103, 104, 105, 106, 107, 108, 109, 110]), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::stone_game(vec![12, 24, 36, 48, 60, 72, 84, 96, 108, 120]), true);
}

#[test]
fn test_94() {
    assert_eq!(Solution::stone_game(vec![3, 2, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15]), true);
}

#[test]
fn test_95() {
    assert_eq!(Solution::stone_game(vec![23, 34, 56, 45, 78, 89, 12, 11, 22, 33, 44, 55]), true);
}

#[test]
fn test_96() {
    assert_eq!(Solution::stone_game(vec![250, 100, 300, 200, 400, 150, 350, 25]), true);
}

#[test]
fn test_97() {
    assert_eq!(Solution::stone_game(vec![5, 1, 9, 4, 8, 2, 7, 3, 6, 11]), true);
}

#[test]
fn test_98() {
    assert_eq!(Solution::stone_game(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::stone_game(vec![10, 20, 30, 40, 50, 60, 70, 80]), true);
}

#[test]
fn test_100() {
    assert_eq!(Solution::stone_game(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 12, 11, 20, 19, 18, 17, 16, 15, 14, 13]), true);
}

#[test]
fn test_101() {
    assert_eq!(Solution::stone_game(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), true);
}

#[test]
fn test_102() {
    assert_eq!(Solution::stone_game(vec![150, 100, 50, 200, 25, 300, 75, 175, 125, 120, 60, 180, 220, 30, 240, 80]), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::stone_game(vec![100, 200, 101, 201, 102, 202, 103, 203, 104, 204]), true);
}

#[test]
fn test_104() {
    assert_eq!(Solution::stone_game(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 12, 13, 14, 15, 16]), true);
}

#[test]
fn test_105() {
    assert_eq!(Solution::stone_game(vec![5, 8, 6, 3, 4, 2, 7, 9, 1, 10]), true);
}

#[test]
fn test_106() {
    assert_eq!(Solution::stone_game(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200]), true);
}

#[test]
fn test_107() {
    assert_eq!(Solution::stone_game(vec![1, 1000, 2, 999, 3, 998, 4, 997, 5, 996]), true);
}

#[test]
fn test_108() {
    assert_eq!(Solution::stone_game(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), true);
}

#[test]
fn test_109() {
    assert_eq!(Solution::stone_game(vec![20, 40, 30, 60, 50, 70, 80, 100, 90, 110]), true);
}

#[test]
fn test_110() {
    assert_eq!(Solution::stone_game(vec![8, 15, 3, 7, 12, 9, 6, 11, 2, 10, 5, 14]), true);
}

#[test]
fn test_111() {
    assert_eq!(Solution::stone_game(vec![250, 100, 300, 50, 400, 150, 600, 200, 700, 50, 800, 25]), true);
}

#[test]
fn test_112() {
    assert_eq!(Solution::stone_game(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12]), false);
}

#[test]
fn test_113() {
    assert_eq!(Solution::stone_game(vec![1, 3, 2, 4, 5, 7, 6, 8, 9, 11, 10, 12, 13, 15, 14, 16, 17, 19, 18, 20]), true);
}

#[test]
fn test_114() {
    assert_eq!(Solution::stone_game(vec![5, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]), true);
}

#[test]
fn test_115() {
    assert_eq!(Solution::stone_game(vec![12, 14, 16, 18, 20, 22, 24, 26, 28, 30]), true);
}

#[test]
fn test_116() {
    assert_eq!(Solution::stone_game(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 11, 12, 13, 14]), true);
}

#[test]
fn test_117() {
    assert_eq!(Solution::stone_game(vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 9]), true);
}

#[test]
fn test_118() {
    assert_eq!(Solution::stone_game(vec![42, 33, 24, 15, 6, 17, 28, 39, 50, 61, 72, 83]), true);
}

#[test]
fn test_119() {
    assert_eq!(Solution::stone_game(vec![12, 22, 32, 42, 52, 62, 72, 82, 92, 102]), true);
}

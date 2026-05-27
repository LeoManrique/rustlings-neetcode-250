include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
}

#[test]
fn test_2() {
    assert_eq!(Solution::stone_game_ii(vec![10]), 10);
}

#[test]
fn test_3() {
    assert_eq!(Solution::stone_game_ii(vec![100, 100, 100, 100]), 200);
}

#[test]
fn test_4() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3]), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::stone_game_ii(vec![1, 100, 1, 100, 1, 100]), 102);
}

#[test]
fn test_6() {
    assert_eq!(Solution::stone_game_ii(vec![3, 6, 9, 12]), 15);
}

#[test]
fn test_7() {
    assert_eq!(Solution::stone_game_ii(vec![3, 3, 3, 3, 3, 3, 3, 3]), 12);
}

#[test]
fn test_8() {
    assert_eq!(Solution::stone_game_ii(vec![8, 5, 7, 3, 8, 9]), 24);
}

#[test]
fn test_9() {
    assert_eq!(Solution::stone_game_ii(vec![3, 3, 3, 3, 3, 3, 3]), 12);
}

#[test]
fn test_10() {
    assert_eq!(Solution::stone_game_ii(vec![10, 20, 30, 40, 50]), 80);
}

#[test]
fn test_11() {
    assert_eq!(Solution::stone_game_ii(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8]), 48);
}

#[test]
fn test_12() {
    assert_eq!(Solution::stone_game_ii(vec![5, 4, 3, 2, 1]), 9);
}

#[test]
fn test_13() {
    assert_eq!(Solution::stone_game_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 6);
}

#[test]
fn test_14() {
    assert_eq!(Solution::stone_game_ii(vec![3, 3, 3, 3, 3, 3]), 9);
}

#[test]
fn test_15() {
    assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
}

#[test]
fn test_16() {
    assert_eq!(Solution::stone_game_ii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 260);
}

#[test]
fn test_17() {
    assert_eq!(Solution::stone_game_ii(vec![1, 100, 100, 1]), 101);
}

#[test]
fn test_18() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 15);
}

#[test]
fn test_19() {
    assert_eq!(Solution::stone_game_ii(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 57);
}

#[test]
fn test_20() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100, 1, 2, 3, 4, 5, 100, 1, 2, 3, 4, 5, 100, 1, 2, 3, 4, 5, 100]), 237);
}

#[test]
fn test_21() {
    assert_eq!(Solution::stone_game_ii(vec![1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984, 983, 982, 981, 980, 979, 978, 977, 976, 975, 974, 973, 972, 971, 970, 969, 968, 967, 966, 965, 964, 963, 962, 961, 960, 959, 958, 957, 956, 955, 954, 953, 952, 951, 950]), 25360);
}

#[test]
fn test_22() {
    assert_eq!(Solution::stone_game_ii(vec![3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]), 128);
}

#[test]
fn test_23() {
    assert_eq!(Solution::stone_game_ii(vec![7, 14, 28, 56, 112, 224, 448, 896]), 861);
}

#[test]
fn test_24() {
    assert_eq!(Solution::stone_game_ii(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), 47);
}

#[test]
fn test_25() {
    assert_eq!(Solution::stone_game_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 32);
}

#[test]
fn test_26() {
    assert_eq!(Solution::stone_game_ii(vec![5, 3, 9, 1, 10, 12, 8, 7, 6, 11, 4, 2]), 39);
}

#[test]
fn test_27() {
    assert_eq!(Solution::stone_game_ii(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100]), 540);
}

#[test]
fn test_28() {
    assert_eq!(Solution::stone_game_ii(vec![25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 172);
}

#[test]
fn test_29() {
    assert_eq!(Solution::stone_game_ii(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60]), 466);
}

#[test]
fn test_30() {
    assert_eq!(Solution::stone_game_ii(vec![3, 8, 4, 5, 12, 10]), 17);
}

#[test]
fn test_31() {
    assert_eq!(Solution::stone_game_ii(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 30);
}

#[test]
fn test_32() {
    assert_eq!(Solution::stone_game_ii(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10]), 59);
}

#[test]
fn test_33() {
    assert_eq!(Solution::stone_game_ii(vec![5, 15, 20, 10, 35, 25, 40]), 85);
}

#[test]
fn test_34() {
    assert_eq!(Solution::stone_game_ii(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 50);
}

#[test]
fn test_35() {
    assert_eq!(Solution::stone_game_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 34);
}

#[test]
fn test_36() {
    assert_eq!(Solution::stone_game_ii(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 300);
}

#[test]
fn test_37() {
    assert_eq!(Solution::stone_game_ii(vec![5, 3, 7, 2, 9, 11, 4, 6, 8, 10, 1, 12, 14, 13]), 54);
}

#[test]
fn test_38() {
    assert_eq!(Solution::stone_game_ii(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]), 210);
}

#[test]
fn test_39() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 108);
}

#[test]
fn test_40() {
    assert_eq!(Solution::stone_game_ii(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 321);
}

#[test]
fn test_41() {
    assert_eq!(Solution::stone_game_ii(vec![3, 6, 7, 10, 20, 5, 15, 25, 30, 1]), 59);
}

#[test]
fn test_42() {
    assert_eq!(Solution::stone_game_ii(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 114);
}

#[test]
fn test_43() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 108);
}

#[test]
fn test_44() {
    assert_eq!(Solution::stone_game_ii(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), 87);
}

#[test]
fn test_45() {
    assert_eq!(Solution::stone_game_ii(vec![10000, 5000, 2500, 1250, 625, 312, 156, 78, 39, 19, 9, 4, 2, 1]), 15308);
}

#[test]
fn test_46() {
    assert_eq!(Solution::stone_game_ii(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 300);
}

#[test]
fn test_47() {
    assert_eq!(Solution::stone_game_ii(vec![1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100]), 505);
}

#[test]
fn test_48() {
    assert_eq!(Solution::stone_game_ii(vec![1000, 900, 800, 700, 600, 500, 400, 300, 200, 100]), 3000);
}

#[test]
fn test_49() {
    assert_eq!(Solution::stone_game_ii(vec![1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3]), 32);
}

#[test]
fn test_50() {
    assert_eq!(Solution::stone_game_ii(vec![3, 2, 10, 7, 8, 9, 1, 2, 10, 5]), 28);
}

#[test]
fn test_51() {
    assert_eq!(Solution::stone_game_ii(vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80]), 913);
}

#[test]
fn test_52() {
    assert_eq!(Solution::stone_game_ii(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), 205);
}

#[test]
fn test_53() {
    assert_eq!(Solution::stone_game_ii(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 140);
}

#[test]
fn test_54() {
    assert_eq!(Solution::stone_game_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 30);
}

#[test]
fn test_55() {
    assert_eq!(Solution::stone_game_ii(vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 537);
}

#[test]
fn test_56() {
    assert_eq!(Solution::stone_game_ii(vec![1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000]), 5005);
}

#[test]
fn test_57() {
    assert_eq!(Solution::stone_game_ii(vec![5, 3, 8, 9, 1, 2, 4, 6, 7, 10, 11, 12]), 42);
}

#[test]
fn test_58() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 163);
}

#[test]
fn test_59() {
    assert_eq!(Solution::stone_game_ii(vec![5, 15, 25, 35, 45, 55, 65, 75, 85, 95]), 235);
}

#[test]
fn test_60() {
    assert_eq!(Solution::stone_game_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10);
}

#[test]
fn test_61() {
    assert_eq!(Solution::stone_game_ii(vec![3, 1, 5, 6, 8, 9, 2, 4, 7, 10]), 28);
}

#[test]
fn test_62() {
    assert_eq!(Solution::stone_game_ii(vec![3, 8, 7, 2, 10, 5, 6, 4, 9, 1]), 31);
}

#[test]
fn test_63() {
    assert_eq!(Solution::stone_game_ii(vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36]), 123);
}

#[test]
fn test_64() {
    assert_eq!(Solution::stone_game_ii(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 20);
}

#[test]
fn test_65() {
    assert_eq!(Solution::stone_game_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 34);
}

#[test]
fn test_66() {
    assert_eq!(Solution::stone_game_ii(vec![300, 290, 280, 270, 260, 250, 240, 230, 220, 210, 200, 190, 180, 170, 160, 150, 140, 130, 120, 110, 100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 2410);
}

#[test]
fn test_67() {
    assert_eq!(Solution::stone_game_ii(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000]), 10800);
}

#[test]
fn test_68() {
    assert_eq!(Solution::stone_game_ii(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29]), 113);
}

#[test]
fn test_69() {
    assert_eq!(Solution::stone_game_ii(vec![100, 1, 1, 1, 100, 1, 1, 1, 100, 1]), 204);
}

#[test]
fn test_70() {
    assert_eq!(Solution::stone_game_ii(vec![1000, 2000, 1000, 2000, 1000, 2000, 1000, 2000, 1000, 2000, 1000, 2000, 1000, 2000, 1000, 2000, 1000, 2000, 1000, 2000]), 15000);
}

#[test]
fn test_71() {
    assert_eq!(Solution::stone_game_ii(vec![100, 200, 300, 400, 500, 100, 200, 300, 400, 500, 100, 200, 300, 400, 500]), 2300);
}

#[test]
fn test_72() {
    assert_eq!(Solution::stone_game_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 16);
}

#[test]
fn test_73() {
    assert_eq!(Solution::stone_game_ii(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), 2600);
}

#[test]
fn test_74() {
    assert_eq!(Solution::stone_game_ii(vec![50, 25, 100, 75, 200, 150, 300, 125, 350, 225, 400, 275, 450, 325, 500, 375, 550, 425, 600, 475]), 3025);
}

#[test]
fn test_75() {
    assert_eq!(Solution::stone_game_ii(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 20);
}

#[test]
fn test_76() {
    assert_eq!(Solution::stone_game_ii(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 60);
}

#[test]
fn test_77() {
    assert_eq!(Solution::stone_game_ii(vec![10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20]), 150);
}

#[test]
fn test_78() {
    assert_eq!(Solution::stone_game_ii(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 86);
}

#[test]
fn test_79() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]), 410);
}

#[test]
fn test_80() {
    assert_eq!(Solution::stone_game_ii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), 600);
}

#[test]
fn test_81() {
    assert_eq!(Solution::stone_game_ii(vec![34, 54, 74, 94, 114, 134, 154, 174, 194, 214, 234, 254, 274, 294, 314, 334, 354, 374, 394, 414, 434, 454, 474, 494, 514, 534, 554, 574, 594, 614, 634, 654, 674, 694, 714, 734, 754, 774, 794, 814, 834, 854, 874, 894, 914, 934, 954, 974, 994]), 12668);
}

#[test]
fn test_82() {
    assert_eq!(Solution::stone_game_ii(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 327);
}

#[test]
fn test_83() {
    assert_eq!(Solution::stone_game_ii(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 694);
}

#[test]
fn test_84() {
    assert_eq!(Solution::stone_game_ii(vec![5, 3, 7, 8, 4, 2, 6, 1, 9, 10, 11, 12]), 43);
}

#[test]
fn test_85() {
    assert_eq!(Solution::stone_game_ii(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]), 288);
}

#[test]
fn test_86() {
    assert_eq!(Solution::stone_game_ii(vec![5, 1, 4, 2, 3, 6, 9, 8, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 235);
}

#[test]
fn test_87() {
    assert_eq!(Solution::stone_game_ii(vec![4, 6, 2, 8, 10, 5, 7, 3, 9, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60]), 916);
}

#[test]
fn test_88() {
    assert_eq!(Solution::stone_game_ii(vec![100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119]), 1103);
}

#[test]
fn test_89() {
    assert_eq!(Solution::stone_game_ii(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 109);
}

#[test]
fn test_90() {
    assert_eq!(Solution::stone_game_ii(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21]), 64);
}

#[test]
fn test_91() {
    assert_eq!(Solution::stone_game_ii(vec![10000, 1, 9999, 2, 9998, 3, 9997, 4, 9996, 5, 9995, 6, 9994, 7, 9993, 8, 9992, 9, 9991, 10, 9990, 11, 9989, 12, 9988, 13, 9987, 14, 9986, 15, 9985, 16, 9984, 17, 9983, 18, 9982, 19, 9981, 20, 9980, 21, 9979, 22, 9978, 23, 9977, 24, 9976, 25, 9975, 26, 9974, 27, 9973, 28, 9972, 29, 9971, 30, 9970, 31, 9969, 32, 9968, 33, 9967, 34, 9966, 35, 9965, 36, 9964, 37, 9963, 38, 9962, 39, 9961, 40, 9960, 41, 9959, 42, 9958, 43, 9957, 44, 9956, 45, 9955, 46, 9954, 47, 9953, 48, 9952, 49, 9951, 50]), 250050);
}

#[test]
fn test_92() {
    assert_eq!(Solution::stone_game_ii(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 114);
}

#[test]
fn test_93() {
    assert_eq!(Solution::stone_game_ii(vec![23, 35, 18, 41, 7, 25, 38, 13, 29, 22, 33, 3, 19, 27, 15, 32, 6, 24, 20, 21, 8, 14, 16, 30, 34, 9, 26, 17, 31, 12, 28, 10, 36, 37, 4, 5, 11, 39, 40, 2, 42]), 470);
}

#[test]
fn test_94() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100]), 2526);
}

#[test]
fn test_95() {
    assert_eq!(Solution::stone_game_ii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300]), 2330);
}

#[test]
fn test_96() {
    assert_eq!(Solution::stone_game_ii(vec![5, 8, 12, 21, 14, 9, 10, 4, 15, 6, 11, 3, 17, 7, 2, 13, 18, 20, 1, 16]), 112);
}

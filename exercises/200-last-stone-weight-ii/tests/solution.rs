include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_2() {
    assert_eq!(Solution::last_stone_weight_ii(vec![3, 9, 7, 3]), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 2, 3, 4, 5]), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::last_stone_weight_ii(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::last_stone_weight_ii(vec![30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 1]), 99);
}

#[test]
fn test_8() {
    assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
}

#[test]
fn test_9() {
    assert_eq!(Solution::last_stone_weight_ii(vec![15, 10, 5, 10, 10]), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 100, 100, 1]), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::last_stone_weight_ii(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 10, 10, 10]), 0);
}

#[test]
fn test_14() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1]), 1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
}

#[test]
fn test_16() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 0);
}

#[test]
fn test_17() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90]), 65);
}

#[test]
fn test_18() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), 0);
}

#[test]
fn test_19() {
    assert_eq!(Solution::last_stone_weight_ii(vec![9, 3, 9, 8, 7, 8, 5, 4]), 1);
}

#[test]
fn test_20() {
    assert_eq!(Solution::last_stone_weight_ii(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100, 105, 110, 115, 120, 125, 130, 135, 140, 145]), 5);
}

#[test]
fn test_21() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 1);
}

#[test]
fn test_22() {
    assert_eq!(Solution::last_stone_weight_ii(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]), 9);
}

#[test]
fn test_23() {
    assert_eq!(Solution::last_stone_weight_ii(vec![50, 25, 25, 75, 100, 50]), 25);
}

#[test]
fn test_24() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59]), 0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25]), 1);
}

#[test]
fn test_26() {
    assert_eq!(Solution::last_stone_weight_ii(vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70]), 1);
}

#[test]
fn test_27() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120]), 0);
}

#[test]
fn test_28() {
    assert_eq!(Solution::last_stone_weight_ii(vec![49, 98, 147, 196, 245, 294, 343, 392, 441, 490]), 49);
}

#[test]
fn test_29() {
    assert_eq!(Solution::last_stone_weight_ii(vec![50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40]), 15);
}

#[test]
fn test_30() {
    assert_eq!(Solution::last_stone_weight_ii(vec![30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30]), 0);
}

#[test]
fn test_31() {
    assert_eq!(Solution::last_stone_weight_ii(vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90]), 1);
}

#[test]
fn test_32() {
    assert_eq!(Solution::last_stone_weight_ii(vec![29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10]), 0);
}

#[test]
fn test_33() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 1, 200, 2, 300, 3, 400, 4, 500, 5, 600, 6, 700, 7, 800, 8, 900, 9, 1000, 10, 1100, 11, 1200, 12, 1300, 13, 1400, 14]), 1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::last_stone_weight_ii(vec![8, 14, 22, 26, 28, 30, 35, 40, 43, 45, 47, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100]), 1);
}

#[test]
fn test_35() {
    assert_eq!(Solution::last_stone_weight_ii(vec![45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_36() {
    assert_eq!(Solution::last_stone_weight_ii(vec![50, 45, 40, 35, 30, 25, 20, 15, 10, 5]), 5);
}

#[test]
fn test_37() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 100, 2, 99, 3, 98, 4, 97, 5, 96, 6, 95, 7, 94, 8, 93, 9, 92, 10, 91, 11, 90, 12, 89, 13, 88]), 1);
}

#[test]
fn test_38() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113]), 1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 22, 33, 44, 55, 66, 77, 88, 99, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300]), 0);
}

#[test]
fn test_40() {
    assert_eq!(Solution::last_stone_weight_ii(vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_41() {
    assert_eq!(Solution::last_stone_weight_ii(vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80]), 0);
}

#[test]
fn test_42() {
    assert_eq!(Solution::last_stone_weight_ii(vec![80, 20, 60, 40, 30, 70, 50, 10, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290]), 10);
}

#[test]
fn test_43() {
    assert_eq!(Solution::last_stone_weight_ii(vec![88, 77, 66, 55, 44, 33, 22, 11, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 300]), 0);
}

#[test]
fn test_44() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::last_stone_weight_ii(vec![91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 0);
}

#[test]
fn test_46() {
    assert_eq!(Solution::last_stone_weight_ii(vec![8, 3, 5, 7, 11, 9, 14, 16, 2, 6]), 1);
}

#[test]
fn test_47() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 100, 1, 100, 1, 100, 1, 100, 1, 100]), 95);
}

#[test]
fn test_48() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 10);
}

#[test]
fn test_49() {
    assert_eq!(Solution::last_stone_weight_ii(vec![29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_50() {
    assert_eq!(Solution::last_stone_weight_ii(vec![8, 8, 8, 8, 8, 8, 8, 8]), 0);
}

#[test]
fn test_51() {
    assert_eq!(Solution::last_stone_weight_ii(vec![50, 50, 50, 50, 50, 50, 50]), 50);
}

#[test]
fn test_52() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]), 1);
}

#[test]
fn test_53() {
    assert_eq!(Solution::last_stone_weight_ii(vec![8, 6, 7, 5, 3, 0, 9]), 0);
}

#[test]
fn test_54() {
    assert_eq!(Solution::last_stone_weight_ii(vec![12, 13, 14, 15, 16, 17, 18, 19, 20]), 0);
}

#[test]
fn test_55() {
    assert_eq!(Solution::last_stone_weight_ii(vec![99, 1, 100, 1, 100, 1, 100, 1, 100]), 95);
}

#[test]
fn test_56() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_57() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 45);
}

#[test]
fn test_58() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19]), 1);
}

#[test]
fn test_59() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6]), 0);
}

#[test]
fn test_60() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 10);
}

#[test]
fn test_61() {
    assert_eq!(Solution::last_stone_weight_ii(vec![29, 10, 15, 30, 25, 5]), 4);
}

#[test]
fn test_62() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 14, 10, 4, 7, 12]), 1);
}

#[test]
fn test_63() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91]), 1);
}

#[test]
fn test_64() {
    assert_eq!(Solution::last_stone_weight_ii(vec![5, 8, 12, 9, 15, 20, 18]), 1);
}

#[test]
fn test_65() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71]), 1);
}

#[test]
fn test_66() {
    assert_eq!(Solution::last_stone_weight_ii(vec![30, 30, 30, 30, 30, 30, 30, 30, 30, 30]), 0);
}

#[test]
fn test_67() {
    assert_eq!(Solution::last_stone_weight_ii(vec![30, 30, 30, 30, 30, 30, 30, 30, 30, 30]), 0);
}

#[test]
fn test_68() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 1, 2, 3, 99]), 1);
}

#[test]
fn test_69() {
    assert_eq!(Solution::last_stone_weight_ii(vec![20, 30, 40, 50, 60, 70]), 10);
}

#[test]
fn test_70() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 72);
}

#[test]
fn test_71() {
    assert_eq!(Solution::last_stone_weight_ii(vec![30, 20, 10, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_72() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 0);
}

#[test]
fn test_73() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 1, 99, 2, 98, 3, 97, 4]), 0);
}

#[test]
fn test_74() {
    assert_eq!(Solution::last_stone_weight_ii(vec![29, 22, 24, 27, 30, 31, 28, 25, 21, 20]), 1);
}

#[test]
fn test_75() {
    assert_eq!(Solution::last_stone_weight_ii(vec![12, 13, 14, 15, 16, 17, 18, 19, 20, 21]), 1);
}

#[test]
fn test_76() {
    assert_eq!(Solution::last_stone_weight_ii(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50]), 0);
}

#[test]
fn test_77() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 0);
}

#[test]
fn test_78() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6]), 0);
}

#[test]
fn test_79() {
    assert_eq!(Solution::last_stone_weight_ii(vec![20, 15, 10, 5, 30, 25, 40, 50]), 5);
}

#[test]
fn test_80() {
    assert_eq!(Solution::last_stone_weight_ii(vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000]), 8888889);
}

#[test]
fn test_81() {
    assert_eq!(Solution::last_stone_weight_ii(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]), 5);
}

#[test]
fn test_82() {
    assert_eq!(Solution::last_stone_weight_ii(vec![30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30]), 0);
}

#[test]
fn test_83() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 50, 25, 12, 6, 3, 1]), 3);
}

#[test]
fn test_84() {
    assert_eq!(Solution::last_stone_weight_ii(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50]), 0);
}

#[test]
fn test_85() {
    assert_eq!(Solution::last_stone_weight_ii(vec![25, 25, 25, 25, 25, 25, 25, 25]), 0);
}

#[test]
fn test_86() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10]), 0);
}

#[test]
fn test_87() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 55);
}

#[test]
fn test_88() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 10]), 0);
}

#[test]
fn test_89() {
    assert_eq!(Solution::last_stone_weight_ii(vec![50, 49, 51, 48, 52]), 44);
}

#[test]
fn test_90() {
    assert_eq!(Solution::last_stone_weight_ii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), 0);
}

#[test]
fn test_91() {
    assert_eq!(Solution::last_stone_weight_ii(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 100]), 13);
}

#[test]
fn test_92() {
    assert_eq!(Solution::last_stone_weight_ii(vec![8, 14, 17, 31, 56, 13, 29]), 2);
}

#[test]
fn test_93() {
    assert_eq!(Solution::last_stone_weight_ii(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50]), 50);
}

#[test]
fn test_94() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]), 1);
}

#[test]
fn test_95() {
    assert_eq!(Solution::last_stone_weight_ii(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 1);
}

#[test]
fn test_96() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113]), 1);
}

#[test]
fn test_97() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 3, 7, 8, 10, 12, 15, 18, 20, 25]), 0);
}

#[test]
fn test_98() {
    assert_eq!(Solution::last_stone_weight_ii(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50]), 50);
}

#[test]
fn test_99() {
    assert_eq!(Solution::last_stone_weight_ii(vec![100, 99, 98, 97, 96, 95]), 1);
}

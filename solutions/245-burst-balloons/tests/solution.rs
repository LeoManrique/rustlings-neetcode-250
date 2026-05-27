include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::max_coins(vec![5, 4, 3, 2, 1]), 110);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_coins(vec![1, 5]), 10);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_coins(vec![1]), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_coins(vec![7, 9, 8, 0, 7, 1, 3, 5, 5, 2, 3]), 1654);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_coins(vec![7, 9, 8, 0, 7, 1, 3, 5, 5, 7, 4, 5, 5, 5, 4]), 2886);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_coins(vec![0, 0, 0]), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_coins(vec![9, 76, 66, 18, 49, 79, 11, 31, 2, 83, 45, 12, 50, 88, 67, 34, 73, 39, 100, 87, 30, 6, 41, 72, 84, 17, 29, 63, 52, 75, 58, 92, 37, 35, 61, 43, 89, 64, 55, 19, 32, 62, 57, 90, 91, 33, 44, 27, 3, 76, 65, 68, 42, 8, 54, 60, 10, 80, 70, 12, 3, 5, 82, 46, 30, 81, 13, 26, 93, 14, 20, 78, 86, 25, 56, 1, 36, 59, 74, 15, 95, 16, 4, 7, 22, 69, 51, 38, 85, 23, 40, 94, 48, 6, 97, 24, 53, 9, 96, 21, 47, 77, 99, 31, 28, 45, 32]), 35112384);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_coins(vec![10, 10, 10, 10]), 2110);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_coins(vec![100]), 100);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_coins(vec![9, 76, 64, 21, 97, 60]), 1086136);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_coins(vec![8, 3, 8, 3, 8]), 968);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_coins(vec![0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_coins(vec![0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5]), 110);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_coins(vec![7, 9, 8, 0, 7, 1, 3, 5, 5, 7, 3]), 2107);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_coins(vec![35, 16, 83, 87, 52, 15, 24, 91, 36, 80, 59, 27, 9, 81, 33, 17, 5, 74, 40, 85, 23, 47, 89, 69]), 6802248);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_coins(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100]), 5700600);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_coins(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 10120);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 2420);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_coins(vec![5, 8, 6, 2, 3, 7, 4, 1, 9]), 1704);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_coins(vec![1, 3, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100]), 32339900);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_coins(vec![100, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 99]), 474000);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_coins(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5, 0, 2, 8, 8, 4, 1, 9, 7, 1, 6, 9, 3, 9, 9, 3, 7, 5, 1, 0, 5, 8, 2, 0, 9, 7, 4, 9, 4, 4, 5, 9, 2, 3, 0, 7, 8, 1, 6, 4, 0, 6, 2, 8, 6, 2, 0, 4, 9, 0, 3, 0, 1]), 23783);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_coins(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 2427280);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_coins(vec![8, 1, 9, 2, 10, 3, 11, 4, 12, 5, 13, 6, 14, 7, 15]), 15717);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_coins(vec![1, 3, 1, 5, 1, 7, 1, 9, 1, 11]), 1510);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_coins(vec![50, 40, 30, 20, 10, 5, 2, 1, 2, 5, 10, 20, 30, 40, 50]), 309374);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_coins(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81]), 14636200);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 243660);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_coins(vec![3, 2, 1, 4, 6, 5, 8, 7, 10, 9, 12, 11, 14, 13, 16, 15, 18, 17, 20, 19]), 43888);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 4830);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_coins(vec![1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100]), 13150200);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_coins(vec![5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1]), 2010);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_coins(vec![9, 75, 1, 99, 2, 98, 3, 97, 4, 96, 5]), 2737551);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_coins(vec![100, 1, 100, 1, 100, 1, 100, 1, 100]), 3050100);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 13680);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_coins(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81, 80]), 15284100);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_coins(vec![99, 1, 98, 2, 97, 3, 96, 4, 95, 5]), 2906611);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_coins(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), 741);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_coins(vec![50, 20, 30, 10, 40, 60, 70, 80, 90, 10]), 1428550);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_coins(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 30);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_coins(vec![50, 25, 75, 40, 60, 10, 80, 30, 90]), 1779840);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_coins(vec![20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20]), 72420);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_coins(vec![9, 7, 5, 3, 1, 2, 4, 6, 8, 10]), 2100);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_coins(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100, 105, 110, 115, 120, 125, 130, 135, 140, 145, 150, 155, 160, 165, 170, 175, 180, 185, 190, 195, 200]), 98801200);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_coins(vec![50, 25, 75, 20, 80, 15, 85, 10, 90, 5, 95, 40, 60, 35, 65, 30, 70, 45, 55, 2]), 4854150);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_coins(vec![82, 9, 60, 27, 69, 64, 53, 80, 74, 97, 22, 5, 35, 46, 91, 16, 51, 86, 58, 3, 53, 29, 37, 24, 36, 72, 39, 68, 55, 76, 59, 79, 85, 43, 87, 66, 89, 25, 47, 20, 90, 83, 33, 38, 92, 48, 57, 93, 95, 70, 56, 88, 45, 26, 75, 98, 65, 4, 42, 77, 18, 23, 31, 19, 94, 49, 32, 21, 100, 30, 17, 28, 40, 11, 63, 67, 7, 62, 13, 73, 12, 14, 78, 2, 54, 71, 15, 6, 41, 81, 52, 96, 34, 44, 99, 84, 50, 8, 39]), 35359128);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_coins(vec![2, 3, 2, 4, 2, 3, 2]), 120);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_coins(vec![1, 3, 1, 5, 1, 3]), 90);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_coins(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300]), 243603300);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_coins(vec![25, 50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300]), 82507800);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_coins(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 2401100);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_coins(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 2420);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_coins(vec![9, 7, 6, 5, 4, 3, 2, 1]), 1026);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_coins(vec![10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 940);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_coins(vec![9, 7, 5, 3, 1]), 495);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_coins(vec![10, 100, 1000, 10000]), 1010110000);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_coins(vec![30, 20, 10, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 88180);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_coins(vec![30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 243660);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_coins(vec![99, 1, 100, 1, 99, 1, 100]), 2019700);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_coins(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 2401100);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_coins(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5820);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 13680);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 2420);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_coins(vec![100, 50, 100, 50, 100, 50, 100, 50, 100, 50, 100, 50, 100, 50, 100, 50, 100, 50, 100, 50]), 13005100);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_coins(vec![23, 11, 77, 32, 45, 62, 88, 12, 56, 78, 91, 29, 48, 50, 65, 73, 82, 90, 18, 27]), 5732245);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_coins(vec![50, 24, 31, 29, 96, 9, 18, 45, 32, 27, 95, 38, 57, 47, 52, 56, 83, 40, 87, 91, 30, 72, 4, 36, 66, 6, 1, 49, 59, 27, 9, 81, 33, 17, 5, 74, 40, 85, 23, 47, 89, 69, 35, 16, 83, 87, 52, 15, 24, 91, 36, 80, 59, 27, 9, 81, 33, 17, 5, 74]), 17249443);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_coins(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200]), 45602200);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 15220);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_coins(vec![2, 3, 7, 5, 4, 1, 9, 6, 8, 2]), 1723);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_coins(vec![1, 3, 1, 5, 1, 7, 1, 9, 1]), 617);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_coins(vec![30, 20, 10, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), 49060);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 45640);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_coins(vec![20, 30, 40, 50, 60, 70, 80, 90, 100]), 2382100);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_coins(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 2670380);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_coins(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_coins(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5820);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_coins(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 3520);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_coins(vec![50, 20, 50, 20, 50, 20, 50, 20, 50, 20, 50, 20, 50, 20, 50, 20, 50, 20, 50, 20, 50, 20]), 1676050);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_coins(vec![2, 5, 1, 3, 4, 6, 8, 7, 10, 9, 12, 11, 14, 13, 15]), 13653);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_coins(vec![50, 24, 30, 98, 99, 5, 91, 41, 72, 7, 42, 62, 95, 69, 32, 24, 38, 80, 44, 79, 9, 26, 6, 47, 93, 64, 39, 87, 63, 77, 85, 48, 52, 82, 35, 73, 12, 23, 59, 3, 78, 54, 75, 94, 19, 13, 71, 68, 28, 31, 5, 46, 89, 37, 90, 8, 60, 25, 97, 10, 30, 67, 49, 81, 20, 76, 61, 34, 14, 88, 17, 22, 4, 51, 15, 70, 18, 43, 40, 96, 36, 65, 83, 29, 57, 56, 21, 53, 92, 27, 33, 84, 45, 86, 16, 58, 74]), 33856230);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_coins(vec![9, 7, 5, 8, 6, 4, 2, 1, 3, 10]), 2474);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_coins(vec![100, 0, 100, 0, 100, 0, 100, 0, 100, 0, 100, 0, 100, 0, 100, 0, 100, 0, 100, 0]), 8010100);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_coins(vec![5, 3, 8, 6, 2, 9, 1, 4, 7, 10, 11, 13, 12, 15, 14]), 13102);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_coins(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 9530);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_coins(vec![5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1]), 1260);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_coins(vec![23, 45, 67, 89, 12, 34, 56, 78, 90, 11, 33, 55, 77, 22, 44, 66, 88, 10, 30, 50]), 5055964);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_coins(vec![30, 20, 40, 50, 10]), 99330);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_coins(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 2401100);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_coins(vec![10, 1, 100, 5, 10]), 16110);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_coins(vec![30, 20, 10, 5, 1, 2, 3, 4, 5, 10, 20, 30]), 37940);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_coins(vec![9, 7, 3, 4, 6, 1, 2, 8, 5]), 1614);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_coins(vec![10, 1, 9, 2, 8, 3, 7, 4, 6, 5]), 2630);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_coins(vec![9, 7, 3, 1, 8, 6, 5, 4, 2]), 1677);
}

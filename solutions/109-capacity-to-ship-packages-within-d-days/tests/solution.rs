include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::ship_within_days(vec![10, 50, 100, 100, 50, 10], 2), 160);
}

#[test]
fn test_2() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
}

#[test]
fn test_3() {
    assert_eq!(Solution::ship_within_days(vec![10, 50, 100, 100, 50, 10], 3), 160);
}

#[test]
fn test_4() {
    assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
}

#[test]
fn test_5() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
}

#[test]
fn test_6() {
    assert_eq!(Solution::ship_within_days(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), 2);
}

#[test]
fn test_7() {
    assert_eq!(Solution::ship_within_days(vec![5, 5, 5, 5, 5], 2), 15);
}

#[test]
fn test_8() {
    assert_eq!(Solution::ship_within_days(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47], 5), 83);
}

#[test]
fn test_9() {
    assert_eq!(Solution::ship_within_days(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 4), 170);
}

#[test]
fn test_10() {
    assert_eq!(Solution::ship_within_days(vec![4, 3, 2, 5, 8, 2, 3, 5, 6, 1, 2, 4, 3, 7, 5], 8), 10);
}

#[test]
fn test_11() {
    assert_eq!(Solution::ship_within_days(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], 5), 52);
}

#[test]
fn test_12() {
    assert_eq!(Solution::ship_within_days(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], 6), 48);
}

#[test]
fn test_13() {
    assert_eq!(Solution::ship_within_days(vec![9, 10, 10, 10, 10, 10, 10, 10, 10, 10], 3), 39);
}

#[test]
fn test_14() {
    assert_eq!(Solution::ship_within_days(vec![450, 450, 450, 450, 450, 450, 450, 450, 450, 450], 2), 2250);
}

#[test]
fn test_15() {
    assert_eq!(Solution::ship_within_days(vec![30, 40, 20, 5, 10, 80, 25, 45, 60, 35, 50, 20, 40, 30, 50, 15], 8), 90);
}

#[test]
fn test_16() {
    assert_eq!(Solution::ship_within_days(vec![300, 300, 300, 300, 300, 300, 300, 300, 300, 300], 3), 1200);
}

#[test]
fn test_17() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 7), 23);
}

#[test]
fn test_18() {
    assert_eq!(Solution::ship_within_days(vec![50, 100, 150, 200, 250, 300, 350, 400, 450, 500], 4), 850);
}

#[test]
fn test_19() {
    assert_eq!(Solution::ship_within_days(vec![500, 500, 500, 500, 500, 500, 500, 500, 500, 500], 5), 1000);
}

#[test]
fn test_20() {
    assert_eq!(Solution::ship_within_days(vec![150, 300, 450, 600, 750, 900, 1050], 3), 1650);
}

#[test]
fn test_21() {
    assert_eq!(Solution::ship_within_days(vec![250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250], 5), 750);
}

#[test]
fn test_22() {
    assert_eq!(Solution::ship_within_days(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 3), 210);
}

#[test]
fn test_23() {
    assert_eq!(Solution::ship_within_days(vec![3, 5, 8, 4, 2, 10, 1, 7, 6, 9, 11, 13, 15, 12, 14], 6), 26);
}

#[test]
fn test_24() {
    assert_eq!(Solution::ship_within_days(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 15), 7);
}

#[test]
fn test_25() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 3), 42);
}

#[test]
fn test_26() {
    assert_eq!(Solution::ship_within_days(vec![1, 5, 9, 14, 20, 25, 30, 35, 40, 45, 50], 10), 50);
}

#[test]
fn test_27() {
    assert_eq!(Solution::ship_within_days(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 15), 20);
}

#[test]
fn test_28() {
    assert_eq!(Solution::ship_within_days(vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000], 2), 28000);
}

#[test]
fn test_29() {
    assert_eq!(Solution::ship_within_days(vec![30, 50, 20, 40, 60, 10, 90, 80, 70, 100, 120, 130, 110, 140], 6), 240);
}

#[test]
fn test_30() {
    assert_eq!(Solution::ship_within_days(vec![50, 40, 30, 20, 10, 10, 20, 30, 40, 50], 4), 90);
}

#[test]
fn test_31() {
    assert_eq!(Solution::ship_within_days(vec![100, 200, 300, 400, 500, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 6), 500);
}

#[test]
fn test_32() {
    assert_eq!(Solution::ship_within_days(vec![5, 10, 20, 30, 25, 40, 15, 10, 5, 30], 3), 65);
}

#[test]
fn test_33() {
    assert_eq!(Solution::ship_within_days(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 10), 20);
}

#[test]
fn test_34() {
    assert_eq!(Solution::ship_within_days(vec![100, 200, 300, 100, 200, 300, 100, 200, 300, 100], 5), 500);
}

#[test]
fn test_35() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 5), 28);
}

#[test]
fn test_36() {
    assert_eq!(Solution::ship_within_days(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50], 30), 100);
}

#[test]
fn test_37() {
    assert_eq!(Solution::ship_within_days(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 5), 1500);
}

#[test]
fn test_38() {
    assert_eq!(Solution::ship_within_days(vec![47, 2, 20, 7, 2, 19, 23, 30, 6, 12, 9, 4, 30, 26, 8, 7], 10), 47);
}

#[test]
fn test_39() {
    assert_eq!(Solution::ship_within_days(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20], 5), 30);
}

#[test]
fn test_40() {
    assert_eq!(Solution::ship_within_days(vec![10, 15, 10, 15, 10, 15, 10, 15], 6), 25);
}

#[test]
fn test_41() {
    assert_eq!(Solution::ship_within_days(vec![31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31], 5), 155);
}

#[test]
fn test_42() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 100, 200, 300, 100, 200, 300, 1, 2, 3, 100, 200, 300, 100, 200, 300, 1, 2, 3], 7), 500);
}

#[test]
fn test_43() {
    assert_eq!(Solution::ship_within_days(vec![30, 50, 20, 100, 5, 75, 30, 25, 10, 60, 40, 80, 90, 10, 20], 7), 120);
}

#[test]
fn test_44() {
    assert_eq!(Solution::ship_within_days(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 2), 28);
}

#[test]
fn test_45() {
    assert_eq!(Solution::ship_within_days(vec![500, 400, 300, 200, 100, 50, 40, 30, 20, 10, 5, 4, 3, 2, 1], 5), 500);
}

#[test]
fn test_46() {
    assert_eq!(Solution::ship_within_days(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 5), 1500);
}

#[test]
fn test_47() {
    assert_eq!(Solution::ship_within_days(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100], 2), 500);
}

#[test]
fn test_48() {
    assert_eq!(Solution::ship_within_days(vec![300, 200, 100, 50, 40, 30, 20, 10, 5, 4, 3, 2, 1, 1, 1], 7), 300);
}

#[test]
fn test_49() {
    assert_eq!(Solution::ship_within_days(vec![250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250, 250], 10), 500);
}

#[test]
fn test_50() {
    assert_eq!(Solution::ship_within_days(vec![10, 5, 1, 7, 8, 12, 4, 7], 6), 12);
}

#[test]
fn test_51() {
    assert_eq!(Solution::ship_within_days(vec![300, 200, 100, 200, 300, 100, 200, 300, 100, 200], 4), 600);
}

#[test]
fn test_52() {
    assert_eq!(Solution::ship_within_days(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], 3), 420);
}

#[test]
fn test_53() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 4), 57);
}

#[test]
fn test_54() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 7), 23);
}

#[test]
fn test_55() {
    assert_eq!(Solution::ship_within_days(vec![4, 3, 5, 6, 8, 10, 3, 1, 5, 6], 5), 13);
}

#[test]
fn test_56() {
    assert_eq!(Solution::ship_within_days(vec![48, 99, 37, 11, 37, 42, 46, 20, 7, 13, 11, 50, 88, 33, 60, 10], 7), 103);
}

#[test]
fn test_57() {
    assert_eq!(Solution::ship_within_days(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 3), 70);
}

#[test]
fn test_58() {
    assert_eq!(Solution::ship_within_days(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 5), 11);
}

#[test]
fn test_59() {
    assert_eq!(Solution::ship_within_days(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 5), 40);
}

#[test]
fn test_60() {
    assert_eq!(Solution::ship_within_days(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 15), 10);
}

#[test]
fn test_61() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3], 10), 3);
}

#[test]
fn test_62() {
    assert_eq!(Solution::ship_within_days(vec![10, 5, 2, 7, 3, 4, 11, 6, 9], 4), 15);
}

#[test]
fn test_63() {
    assert_eq!(Solution::ship_within_days(vec![300, 200, 100, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300], 15), 470);
}

#[test]
fn test_64() {
    assert_eq!(Solution::ship_within_days(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 20), 1);
}

#[test]
fn test_65() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3], 10), 6);
}

#[test]
fn test_66() {
    assert_eq!(Solution::ship_within_days(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10], 10), 15);
}

#[test]
fn test_67() {
    assert_eq!(Solution::ship_within_days(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50], 15), 100);
}

#[test]
fn test_68() {
    assert_eq!(Solution::ship_within_days(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 7), 100);
}

#[test]
fn test_69() {
    assert_eq!(Solution::ship_within_days(vec![25, 47, 42, 77, 72, 46, 42, 44, 63, 59, 51, 55, 53, 91, 93, 95, 97, 99], 10), 149);
}

#[test]
fn test_70() {
    assert_eq!(Solution::ship_within_days(vec![91, 41, 54, 63, 17, 5, 58, 57, 98, 46], 10), 98);
}

#[test]
fn test_71() {
    assert_eq!(Solution::ship_within_days(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8], 5), 32);
}

#[test]
fn test_72() {
    assert_eq!(Solution::ship_within_days(vec![450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450, 450], 15), 900);
}

#[test]
fn test_73() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10), 28);
}

#[test]
fn test_74() {
    assert_eq!(Solution::ship_within_days(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 2), 2800);
}

#[test]
fn test_75() {
    assert_eq!(Solution::ship_within_days(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 3), 2100);
}

#[test]
fn test_76() {
    assert_eq!(Solution::ship_within_days(vec![450, 450, 450, 450, 450, 450, 450, 450, 450, 450], 5), 900);
}

#[test]
fn test_77() {
    assert_eq!(Solution::ship_within_days(vec![500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500], 10), 1000);
}

#[test]
fn test_78() {
    assert_eq!(Solution::ship_within_days(vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10), 27);
}

#[test]
fn test_79() {
    assert_eq!(Solution::ship_within_days(vec![150, 100, 50, 200, 250, 300, 100, 50, 200, 150], 3), 550);
}

#[test]
fn test_80() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 9), 31);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::car_fleet(1000, vec![100, 200, 300], vec![100, 50, 25]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::car_fleet(100, vec![0, 1, 2, 3, 4], vec![100, 100, 100, 100, 100]), 5);
}

#[test]
fn test_3() {
    assert_eq!(Solution::car_fleet(300, vec![0, 50, 100, 150, 200, 250], vec![50, 40, 30, 20, 10, 5]), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::car_fleet(100, vec![90, 80, 70, 60, 50], vec![10, 20, 30, 40, 50]), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::car_fleet(50, vec![0, 10, 20, 30, 40], vec![10, 9, 8, 7, 6]), 5);
}

#[test]
fn test_6() {
    assert_eq!(Solution::car_fleet(50, vec![10, 20, 30], vec![1, 2, 3]), 3);
}

#[test]
fn test_7() {
    assert_eq!(Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
}

#[test]
fn test_8() {
    assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::car_fleet(1, vec![0], vec![1]), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::car_fleet(50, vec![10, 20, 30, 40], vec![5, 4, 3, 2]), 4);
}

#[test]
fn test_11() {
    assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::car_fleet(200, vec![50, 60, 70, 80, 90], vec![10, 9, 8, 7, 6]), 1);
}

#[test]
fn test_13() {
    assert_eq!(Solution::car_fleet(100, vec![10, 20, 30, 40, 50, 60, 70, 80, 90], vec![10, 10, 10, 10, 10, 10, 10, 10, 10]), 9);
}

#[test]
fn test_14() {
    assert_eq!(Solution::car_fleet(20, vec![0, 18, 5], vec![4, 4, 4]), 3);
}

#[test]
fn test_15() {
    assert_eq!(Solution::car_fleet(20, vec![6, 2, 17], vec![3, 9, 2]), 2);
}

#[test]
fn test_16() {
    assert_eq!(Solution::car_fleet(1000, vec![100, 200, 300, 400], vec![10, 20, 30, 40]), 4);
}

#[test]
fn test_17() {
    assert_eq!(Solution::car_fleet(10, vec![6, 8], vec![3, 2]), 2);
}

#[test]
fn test_18() {
    assert_eq!(Solution::car_fleet(12, vec![0, 4, 2], vec![1, 2, 3]), 2);
}

#[test]
fn test_19() {
    assert_eq!(Solution::car_fleet(20, vec![5, 15, 10], vec![5, 1, 3]), 1);
}

#[test]
fn test_20() {
    assert_eq!(Solution::car_fleet(100, vec![10, 20, 30], vec![1, 2, 3]), 3);
}

#[test]
fn test_21() {
    assert_eq!(Solution::car_fleet(999999, vec![1, 2, 3, 4, 5], vec![999998, 999997, 999996, 999995, 999994]), 1);
}

#[test]
fn test_22() {
    assert_eq!(Solution::car_fleet(200, vec![10, 50, 90, 130, 170], vec![1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_23() {
    assert_eq!(Solution::car_fleet(1000, vec![500, 600, 700, 800, 900], vec![1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_24() {
    assert_eq!(Solution::car_fleet(100, vec![10, 20, 30, 40, 50, 60, 70, 80, 90], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 9);
}

#[test]
fn test_25() {
    assert_eq!(Solution::car_fleet(600, vec![50, 150, 250, 350, 450, 550], vec![5, 10, 15, 20, 25, 30]), 6);
}

#[test]
fn test_26() {
    assert_eq!(Solution::car_fleet(100, vec![5, 15, 25, 35, 45, 55, 65, 75, 85, 95], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 10);
}

#[test]
fn test_27() {
    assert_eq!(Solution::car_fleet(1000, vec![100, 200, 300, 400, 500], vec![10, 20, 30, 40, 50]), 5);
}

#[test]
fn test_28() {
    assert_eq!(Solution::car_fleet(120, vec![10, 30, 50, 70, 90], vec![10, 20, 30, 40, 50]), 5);
}

#[test]
fn test_29() {
    assert_eq!(Solution::car_fleet(100, vec![0, 25, 50, 75, 99], vec![5, 4, 3, 2, 1]), 5);
}

#[test]
fn test_30() {
    assert_eq!(Solution::car_fleet(1000, vec![900, 800, 700, 600, 500, 400, 300, 200, 100], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 1);
}

#[test]
fn test_31() {
    assert_eq!(Solution::car_fleet(1000, vec![10, 200, 300, 400, 500, 600, 700, 800, 900], vec![90, 80, 70, 60, 50, 40, 30, 20, 10]), 2);
}

#[test]
fn test_32() {
    assert_eq!(Solution::car_fleet(300, vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 20);
}

#[test]
fn test_33() {
    assert_eq!(Solution::car_fleet(250, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 15);
}

#[test]
fn test_34() {
    assert_eq!(Solution::car_fleet(1234, vec![123, 456, 789], vec![321, 654, 987]), 3);
}

#[test]
fn test_35() {
    assert_eq!(Solution::car_fleet(50, vec![10, 20, 30, 40], vec![1, 1, 1, 1]), 4);
}

#[test]
fn test_36() {
    assert_eq!(Solution::car_fleet(700, vec![10, 100, 200, 300, 400], vec![5, 15, 25, 35, 45]), 5);
}

#[test]
fn test_37() {
    assert_eq!(Solution::car_fleet(200, vec![50, 100, 150, 0], vec![2, 2, 2, 1]), 4);
}

#[test]
fn test_38() {
    assert_eq!(Solution::car_fleet(100, vec![10, 20, 30, 40, 50, 60, 70, 80, 90], vec![9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::car_fleet(200, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10);
}

#[test]
fn test_40() {
    assert_eq!(Solution::car_fleet(100, vec![10, 20, 30, 40, 50], vec![1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_41() {
    assert_eq!(Solution::car_fleet(300, vec![0, 50, 100, 150, 200, 250], vec![1, 2, 3, 4, 5, 6]), 6);
}

#[test]
fn test_42() {
    assert_eq!(Solution::car_fleet(120, vec![10, 30, 50, 70, 90], vec![10, 8, 6, 4, 2]), 1);
}

#[test]
fn test_43() {
    assert_eq!(Solution::car_fleet(120, vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55], vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 10);
}

#[test]
fn test_44() {
    assert_eq!(Solution::car_fleet(10000, vec![9990, 9980, 9970, 9960, 9950, 9940, 9930, 9920, 9910, 9900], vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_45() {
    assert_eq!(Solution::car_fleet(500, vec![490, 480, 470, 460, 450], vec![10, 10, 10, 10, 10]), 5);
}

#[test]
fn test_46() {
    assert_eq!(Solution::car_fleet(200, vec![0, 50, 100, 150], vec![20, 15, 10, 5]), 1);
}

#[test]
fn test_47() {
    assert_eq!(Solution::car_fleet(100, vec![5, 15, 25, 35, 45], vec![2, 4, 6, 8, 10]), 5);
}

#[test]
fn test_48() {
    assert_eq!(Solution::car_fleet(150, vec![0, 25, 50, 75, 100], vec![5, 10, 15, 20, 25]), 5);
}

#[test]
fn test_49() {
    assert_eq!(Solution::car_fleet(150, vec![10, 20, 30, 40, 50, 60], vec![6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_50() {
    assert_eq!(Solution::car_fleet(100, vec![90, 80, 70, 60, 50], vec![10, 10, 10, 10, 10]), 5);
}

#[test]
fn test_51() {
    assert_eq!(Solution::car_fleet(120, vec![0, 10, 20, 30, 40, 50], vec![1, 2, 3, 4, 5, 6]), 6);
}

#[test]
fn test_52() {
    assert_eq!(Solution::car_fleet(2000, vec![500, 1000, 1500], vec![1, 1, 1]), 3);
}

#[test]
fn test_53() {
    assert_eq!(Solution::car_fleet(600, vec![0, 150, 300, 450], vec![600, 300, 150, 100]), 2);
}

#[test]
fn test_54() {
    assert_eq!(Solution::car_fleet(1000, vec![100, 200, 300, 400], vec![10, 20, 30, 40]), 4);
}

#[test]
fn test_55() {
    assert_eq!(Solution::car_fleet(400, vec![20, 40, 60, 80, 100, 120, 140, 160, 180, 200, 220, 240, 260, 280, 300, 320, 340, 360, 380], vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 19);
}

#[test]
fn test_56() {
    assert_eq!(Solution::car_fleet(120, vec![10, 20, 30, 40, 50, 60], vec![10, 9, 8, 7, 6, 5]), 1);
}

#[test]
fn test_57() {
    assert_eq!(Solution::car_fleet(800, vec![10, 30, 50, 70, 90], vec![10, 20, 30, 40, 50]), 5);
}

#[test]
fn test_58() {
    assert_eq!(Solution::car_fleet(750, vec![50, 100, 150, 200, 250, 300, 350, 400, 450, 500, 550, 600, 650, 700], vec![14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_59() {
    assert_eq!(Solution::car_fleet(1000, vec![0, 100, 200, 300, 400, 500, 600, 700, 800, 900], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_60() {
    assert_eq!(Solution::car_fleet(400, vec![10, 30, 50, 70, 90, 110, 130, 150, 170, 190, 210, 230, 250, 270, 290, 310, 330, 350, 370, 390], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 20);
}

#[test]
fn test_61() {
    assert_eq!(Solution::car_fleet(500, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 10);
}

#[test]
fn test_62() {
    assert_eq!(Solution::car_fleet(1000, vec![990, 980, 970, 960, 950, 940, 930, 920, 910], vec![10, 10, 10, 10, 10, 10, 10, 10, 10]), 9);
}

#[test]
fn test_63() {
    assert_eq!(Solution::car_fleet(150, vec![5, 10, 20, 30, 40, 50, 60, 70, 80, 90], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_64() {
    assert_eq!(Solution::car_fleet(200, vec![10, 50, 90, 130, 170], vec![10, 20, 30, 40, 50]), 5);
}

#[test]
fn test_65() {
    assert_eq!(Solution::car_fleet(800, vec![100, 200, 300, 400, 500, 600, 700], vec![1, 2, 3, 4, 5, 6, 7]), 7);
}

#[test]
fn test_66() {
    assert_eq!(Solution::car_fleet(500, vec![0, 50, 100, 150, 200, 250, 300, 350, 400, 450], vec![50, 45, 40, 35, 30, 25, 20, 15, 10, 5]), 1);
}

#[test]
fn test_67() {
    assert_eq!(Solution::car_fleet(1000, vec![10, 50, 90, 130, 170], vec![10, 10, 10, 10, 10]), 5);
}

#[test]
fn test_68() {
    assert_eq!(Solution::car_fleet(250, vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100, 105, 110, 115, 120, 125], vec![25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_69() {
    assert_eq!(Solution::car_fleet(2000, vec![1900, 1800, 1700, 1600, 1500, 1400, 1300, 1200, 1100, 1000], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1);
}

#[test]
fn test_70() {
    assert_eq!(Solution::car_fleet(1000, vec![0, 250, 500, 750], vec![50, 100, 150, 200]), 4);
}

#[test]
fn test_71() {
    assert_eq!(Solution::car_fleet(3000, vec![100, 300, 500, 700, 900, 1100, 1300, 1500, 1700, 1900], vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55]), 10);
}

#[test]
fn test_72() {
    assert_eq!(Solution::car_fleet(150, vec![10, 20, 30, 40, 50], vec![5, 5, 5, 5, 5]), 5);
}

#[test]
fn test_73() {
    assert_eq!(Solution::car_fleet(5000, vec![10, 200, 3000, 4000, 4990], vec![100, 90, 80, 70, 60]), 4);
}

#[test]
fn test_74() {
    assert_eq!(Solution::car_fleet(100, vec![10, 20, 30, 40, 50], vec![10, 9, 8, 7, 6]), 5);
}

#[test]
fn test_75() {
    assert_eq!(Solution::car_fleet(1500, vec![100, 300, 500, 700, 900], vec![100, 200, 300, 400, 500]), 5);
}

#[test]
fn test_76() {
    assert_eq!(Solution::car_fleet(600, vec![590, 580, 570, 560, 550], vec![1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_77() {
    assert_eq!(Solution::car_fleet(120, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_78() {
    assert_eq!(Solution::car_fleet(600, vec![30, 150, 270, 390, 510], vec![5, 10, 15, 20, 25]), 5);
}

#[test]
fn test_79() {
    assert_eq!(Solution::car_fleet(400, vec![100, 200, 300], vec![1, 1, 1]), 3);
}

#[test]
fn test_80() {
    assert_eq!(Solution::car_fleet(300, vec![290, 280, 270, 260, 250], vec![1, 2, 3, 4, 5]), 1);
}

#[test]
fn test_81() {
    assert_eq!(Solution::car_fleet(200, vec![10, 30, 50, 70, 90], vec![50, 40, 30, 20, 10]), 1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::car_fleet(350, vec![50, 100, 150, 200, 250, 300], vec![10, 20, 30, 40, 50, 60]), 6);
}

#[test]
fn test_83() {
    assert_eq!(Solution::car_fleet(150, vec![10, 30, 50, 70, 90, 110, 130], vec![10, 8, 6, 4, 2, 1, 5]), 2);
}

#[test]
fn test_84() {
    assert_eq!(Solution::car_fleet(200, vec![5, 15, 25, 35, 45, 55, 65, 75, 85, 95], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_85() {
    assert_eq!(Solution::car_fleet(900, vec![10, 100, 200, 300, 400, 500, 600, 700, 800], vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 9);
}

#[test]
fn test_86() {
    assert_eq!(Solution::car_fleet(500, vec![0, 100, 200, 300, 400], vec![1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_87() {
    assert_eq!(Solution::car_fleet(500, vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 15);
}

#[test]
fn test_88() {
    assert_eq!(Solution::car_fleet(300, vec![20, 40, 60, 80, 100, 120, 140, 160, 180, 200, 220, 240, 260, 280], vec![14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_89() {
    assert_eq!(Solution::car_fleet(300, vec![0, 100, 200], vec![100, 50, 1]), 1);
}

#[test]
fn test_90() {
    assert_eq!(Solution::car_fleet(1200, vec![900, 600, 300, 0], vec![10, 20, 30, 40]), 1);
}

#[test]
fn test_91() {
    assert_eq!(Solution::car_fleet(10000, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_92() {
    assert_eq!(Solution::car_fleet(800, vec![100, 200, 300, 400, 500, 600, 700], vec![700, 600, 500, 400, 300, 200, 100]), 1);
}

#[test]
fn test_93() {
    assert_eq!(Solution::car_fleet(500, vec![100, 200, 300, 400], vec![100, 100, 100, 100]), 4);
}

#[test]
fn test_94() {
    assert_eq!(Solution::car_fleet(900, vec![800, 700, 600, 500, 400, 300, 200, 100, 0], vec![5, 10, 15, 20, 25, 30, 35, 40, 45]), 1);
}

#[test]
fn test_95() {
    assert_eq!(Solution::car_fleet(1000, vec![50, 250, 450, 650, 850], vec![10, 20, 30, 40, 50]), 5);
}

#[test]
fn test_96() {
    assert_eq!(Solution::car_fleet(500, vec![100, 150, 200, 250, 300], vec![5, 10, 15, 20, 25]), 5);
}

#[test]
fn test_97() {
    assert_eq!(Solution::car_fleet(1000, vec![500, 600, 700, 800, 900], vec![100, 200, 300, 400, 500]), 5);
}

#[test]
fn test_98() {
    assert_eq!(Solution::car_fleet(500, vec![10, 100, 200, 300, 400], vec![1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_99() {
    assert_eq!(Solution::car_fleet(550, vec![25, 50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300, 325, 350, 375, 400, 425, 450, 475, 500, 525, 550], vec![22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 21);
}

#[test]
fn test_100() {
    assert_eq!(Solution::car_fleet(300, vec![10, 50, 90, 130, 170, 210, 250, 290], vec![1, 2, 3, 4, 5, 6, 7, 8]), 8);
}

#[test]
fn test_101() {
    assert_eq!(Solution::car_fleet(500, vec![400, 300, 200, 100, 0], vec![5, 4, 3, 2, 1]), 5);
}

#[test]
fn test_102() {
    assert_eq!(Solution::car_fleet(300, vec![25, 50, 75, 100, 125, 150, 175, 200, 225, 250, 275], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1]), 2);
}

#[test]
fn test_103() {
    assert_eq!(Solution::car_fleet(500, vec![0, 100, 200, 300, 400], vec![100, 100, 100, 100, 100]), 5);
}

#[test]
fn test_104() {
    assert_eq!(Solution::car_fleet(200, vec![50, 100, 150], vec![1, 2, 3]), 3);
}

#[test]
fn test_105() {
    assert_eq!(Solution::car_fleet(800, vec![10, 100, 200, 300, 400, 500, 600, 700], vec![1, 2, 3, 4, 5, 6, 7, 8]), 8);
}

#[test]
fn test_106() {
    assert_eq!(Solution::car_fleet(1000, vec![50, 150, 250, 350, 450], vec![100, 200, 300, 400, 500]), 5);
}

#[test]
fn test_107() {
    assert_eq!(Solution::car_fleet(1000, vec![50, 150, 250, 350, 450, 550, 650, 750, 850, 950], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_108() {
    assert_eq!(Solution::car_fleet(1500, vec![0, 150, 300, 450, 600, 750, 900, 1050, 1200, 1350], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 10]), 10);
}

#[test]
fn test_109() {
    assert_eq!(Solution::car_fleet(200, vec![0, 20, 40, 60, 80, 100, 120, 140, 160, 180], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_110() {
    assert_eq!(Solution::car_fleet(200, vec![10, 40, 70, 100], vec![10, 5, 20, 15]), 2);
}

#[test]
fn test_111() {
    assert_eq!(Solution::car_fleet(300, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_112() {
    assert_eq!(Solution::car_fleet(1000, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_113() {
    assert_eq!(Solution::car_fleet(300, vec![25, 50, 75, 100, 125, 150, 175, 200, 225, 250, 275], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 11);
}

#[test]
fn test_114() {
    assert_eq!(Solution::car_fleet(2000, vec![1000, 1500], vec![500, 250]), 1);
}

#[test]
fn test_115() {
    assert_eq!(Solution::car_fleet(1000, vec![100, 200, 300, 400, 500, 600, 700, 800, 900], vec![9, 8, 7, 6, 5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_116() {
    assert_eq!(Solution::car_fleet(500, vec![50, 150, 250, 350, 450], vec![5, 10, 15, 20, 25]), 5);
}

#[test]
fn test_117() {
    assert_eq!(Solution::car_fleet(700, vec![100, 200, 300, 400, 500, 600], vec![100, 90, 80, 70, 60, 50]), 6);
}

#[test]
fn test_118() {
    assert_eq!(Solution::car_fleet(200, vec![10, 50, 90, 130, 170], vec![10, 20, 30, 40, 50]), 5);
}

#[test]
fn test_119() {
    assert_eq!(Solution::car_fleet(100, vec![10, 20, 30, 40, 50], vec![5, 5, 5, 5, 5]), 5);
}

#[test]
fn test_120() {
    assert_eq!(Solution::car_fleet(200, vec![10, 20, 30, 40, 50], vec![1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_121() {
    assert_eq!(Solution::car_fleet(500, vec![10, 50, 90, 130, 170], vec![10, 20, 30, 40, 50]), 5);
}

#[test]
fn test_122() {
    assert_eq!(Solution::car_fleet(1000, vec![990, 980, 970, 960, 950], vec![1, 2, 3, 4, 5]), 1);
}

#[test]
fn test_123() {
    assert_eq!(Solution::car_fleet(1000, vec![50, 100, 150, 200, 250, 300, 350, 400, 450, 500], vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 10);
}

#[test]
fn test_124() {
    assert_eq!(Solution::car_fleet(1000, vec![200, 400, 600, 800], vec![50, 40, 30, 20]), 4);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_area(vec![8, 10, 14, 0, 13, 10, 9, 9, 8, 9]), 72);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 81);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_area(vec![2, 3, 10, 5, 7, 8, 9]), 36);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 25);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_area(vec![1, 2, 4, 3]), 4);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5]), 6);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_area(vec![1, 3, 2, 5, 25, 24, 5]), 24);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_area(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 25);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_area(vec![1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1]), 220000);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_area(vec![5, 8, 6, 2, 5, 4, 8, 3, 7, 9, 10, 11, 10, 9, 7, 3, 8, 4, 5, 2, 6, 8, 5, 3, 7, 9, 1, 4, 6, 8]), 224);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_area(vec![10, 20, 10, 20, 10, 20, 10, 20, 10, 20]), 160);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_area(vec![10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]), 180);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 120);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_area(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 200);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_area(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 56);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_area(vec![5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1]), 50);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_area(vec![5, 8, 5, 8, 5, 8, 5, 8, 5, 8, 5, 8, 5, 8, 5, 8, 5, 8, 5]), 128);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_area(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 10000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 19);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_area(vec![10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000]), 300000);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_area(vec![1, 3, 2, 5, 25, 24, 5, 2, 3, 1]), 24);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_area(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 190);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 50);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_area(vec![5000, 5000, 5000, 5000, 5000, 5000, 5000, 5000, 5000, 5000]), 45000);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_area(vec![2, 1, 5, 6, 2, 3, 1, 4, 5, 1, 5, 6, 2, 3, 1, 4, 5, 1]), 70);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_area(vec![10000, 9000, 8000, 7000, 6000, 5000, 4000, 3000, 2000, 1000]), 25000);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_area(vec![10000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 25);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_area(vec![5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 90);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_area(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 180);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_area(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 120);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_area(vec![1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7, 9, 8, 10, 9, 11, 10]), 66);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 100);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_area(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 2000);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_area(vec![1000, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 29);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_area(vec![2, 3, 10, 5, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 240);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_area(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 90);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_area(vec![10000, 9999, 9998, 9997, 9996, 9995, 9994, 9993, 9992, 9991, 9990, 9989, 9988, 9987, 9986]), 139804);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_area(vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000]), 19000);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_area(vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 100);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_area(vec![1000, 900, 800, 700, 600, 500, 400, 300, 200, 100, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), 19000);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 400);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_area(vec![50, 40, 30, 20, 10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 90, 80, 70, 60]), 950);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 12, 4, 3, 2, 1, 4, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 176);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_area(vec![3, 9, 3, 4, 7, 2, 12, 6, 5, 10, 1, 8]), 80);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_area(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 180);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_area(vec![5, 10, 8, 3, 7, 6, 10, 4, 1, 9]), 72);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_area(vec![100, 200, 300, 400, 300, 200, 100, 200, 300, 400, 300, 200, 100]), 2400);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 10, 12, 11]), 80);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_area(vec![1, 2, 1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8, 7]), 36);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_area(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 18, 16, 14, 12, 10]), 100);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 56);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_area(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 100);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_area(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 500);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_area(vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000]), 24000);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_area(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81]), 1539);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_area(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1000]), 29);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 50);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_area(vec![9985, 9984, 9983, 9982, 9981, 9980, 9979, 9978, 9977, 9976, 9975, 9974, 9973, 9972, 9971]), 139594);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_area(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 70);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_area(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 250);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_area(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 56);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 450);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 25, 7]), 49);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 136);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_area(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 80);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_area(vec![10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000, 1, 10000]), 240000);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_area(vec![100, 200, 150, 300, 250, 400, 350, 500, 450, 600, 550, 700, 650, 800, 750, 900, 850, 1000]), 5000);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_area(vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 9000, 8000, 7000, 6000, 5000, 4000, 3000, 2000, 1000]), 50000);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 180);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_area(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 95);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_area(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]), 120);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_area(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 10000]), 25);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 1, 1, 1, 1, 1, 1, 5, 4, 3, 2, 1, 2, 3, 4, 5, 1, 1, 1, 1, 1, 1]), 75);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 6, 7, 8, 9, 10]), 25);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_area(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 16);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_area(vec![1, 1, 1, 1, 100, 1, 1, 1, 1, 1, 100, 1, 1, 1, 1, 1, 100, 1, 1, 1]), 1200);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2]), 48);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_area(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), 190);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_area(vec![10, 9, 8, 10, 9, 8, 10, 9, 8, 10, 9, 8, 10, 9, 8, 10, 9, 8, 10]), 180);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 15, 5, 4, 3, 2, 1, 15, 1, 2, 3, 4, 5, 15, 5, 4, 3, 2, 1]), 180);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5]), 50);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_area(vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 36);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_area(vec![5, 3, 8, 4, 2, 7, 9, 6, 1]), 35);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 112);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_area(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 120);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 100);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_area(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 45);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_area(vec![10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]), 140);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 100);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_area(vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 36);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_area(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 14);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_area(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6, 6, 5, 7, 4, 8, 3, 9, 2, 10, 1]), 170);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2, 1, 2]), 48);
}

#[test]
fn test_97() {
    assert_eq!(Solution::max_area(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]), 90);
}

#[test]
fn test_98() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 200);
}

#[test]
fn test_99() {
    assert_eq!(Solution::max_area(vec![8, 10, 12, 10, 6, 5, 4, 3, 2, 1]), 25);
}

#[test]
fn test_100() {
    assert_eq!(Solution::max_area(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 55);
}

#[test]
fn test_101() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 10, 2, 15, 1, 5, 3]), 80);
}

#[test]
fn test_102() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1]), 180);
}

#[test]
fn test_103() {
    assert_eq!(Solution::max_area(vec![100, 20, 300, 40, 500, 60, 700, 80, 900, 1000, 100, 900, 80, 700, 60, 500, 40, 300, 20, 100]), 5500);
}

#[test]
fn test_104() {
    assert_eq!(Solution::max_area(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 44);
}

#[test]
fn test_105() {
    assert_eq!(Solution::max_area(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 800);
}

#[test]
fn test_106() {
    assert_eq!(Solution::max_area(vec![39, 37, 35, 33, 31, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1]), 190);
}

#[test]
fn test_107() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 200);
}

#[test]
fn test_108() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 9, 10, 11, 12, 13, 14]), 104);
}

#[test]
fn test_109() {
    assert_eq!(Solution::max_area(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 29);
}

#[test]
fn test_110() {
    assert_eq!(Solution::max_area(vec![1, 3, 5, 7, 9, 11, 13, 15, 13, 11, 9, 7, 5, 3, 1]), 56);
}

#[test]
fn test_111() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 56);
}

#[test]
fn test_112() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 10, 11, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 144);
}

#[test]
fn test_113() {
    assert_eq!(Solution::max_area(vec![1, 2, 3, 100, 2, 3, 100, 2, 3, 100, 2, 3, 100, 2, 3, 100, 2, 3, 100]), 1500);
}

#[test]
fn test_114() {
    assert_eq!(Solution::max_area(vec![100, 50, 30, 60, 100, 40, 20, 80, 70, 90, 10]), 810);
}

#[test]
fn test_115() {
    assert_eq!(Solution::max_area(vec![8, 7, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 7]), 98);
}

#[test]
fn test_116() {
    assert_eq!(Solution::max_area(vec![1, 3, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 36);
}

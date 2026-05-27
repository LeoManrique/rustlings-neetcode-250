include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
}

#[test]
fn test_2() {
    assert_eq!(Solution::split_array(vec![5, 5, 5, 5, 5], 5), 5);
}

#[test]
fn test_3() {
    assert_eq!(Solution::split_array(vec![10, 10, 10, 10], 2), 20);
}

#[test]
fn test_4() {
    assert_eq!(Solution::split_array(vec![10, 5, 13, 4, 8, 4, 5, 11, 14, 9, 15], 6), 23);
}

#[test]
fn test_5() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
}

#[test]
fn test_6() {
    assert_eq!(Solution::split_array(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 10), 5);
}

#[test]
fn test_7() {
    assert_eq!(Solution::split_array(vec![10, 10, 10, 10, 10], 5), 10);
}

#[test]
fn test_8() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
}

#[test]
fn test_9() {
    assert_eq!(Solution::split_array(vec![1, 1000000, 1], 2), 1000001);
}

#[test]
fn test_10() {
    assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4);
}

#[test]
fn test_11() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3), 17);
}

#[test]
fn test_12() {
    assert_eq!(Solution::split_array(vec![10, 5, 13, 4, 8, 4, 5, 11, 14, 9, 15], 8), 15);
}

#[test]
fn test_13() {
    assert_eq!(Solution::split_array(vec![4, 2, 5, 1, 7], 3), 7);
}

#[test]
fn test_14() {
    assert_eq!(Solution::split_array(vec![10, 5, 13, 4, 8, 4, 5, 11, 14, 9, 15], 3), 38);
}

#[test]
fn test_15() {
    assert_eq!(Solution::split_array(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10), 1);
}

#[test]
fn test_16() {
    assert_eq!(Solution::split_array(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 5), 0);
}

#[test]
fn test_17() {
    assert_eq!(Solution::split_array(vec![1000000], 1), 1000000);
}

#[test]
fn test_18() {
    assert_eq!(Solution::split_array(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), 4);
}

#[test]
fn test_19() {
    assert_eq!(Solution::split_array(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 5), 150);
}

#[test]
fn test_20() {
    assert_eq!(Solution::split_array(vec![3, 2, 5, 7, 1, 10, 4, 2, 8, 6, 3], 4), 17);
}

#[test]
fn test_21() {
    assert_eq!(Solution::split_array(vec![100, 400, 500, 300, 200], 3), 500);
}

#[test]
fn test_22() {
    assert_eq!(Solution::split_array(vec![1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000, 1], 5), 1001);
}

#[test]
fn test_23() {
    assert_eq!(Solution::split_array(vec![4, 2, 5, 3, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 7), 37);
}

#[test]
fn test_24() {
    assert_eq!(Solution::split_array(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096], 7), 4096);
}

#[test]
fn test_25() {
    assert_eq!(Solution::split_array(vec![20, 30, 10, 40, 50, 60, 70, 80], 3), 150);
}

#[test]
fn test_26() {
    assert_eq!(Solution::split_array(vec![100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000], 5), 200000);
}

#[test]
fn test_27() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 6), 87);
}

#[test]
fn test_28() {
    assert_eq!(Solution::split_array(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5], 3), 17);
}

#[test]
fn test_29() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3), 21);
}

#[test]
fn test_30() {
    assert_eq!(Solution::split_array(vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000], 5), 15000);
}

#[test]
fn test_31() {
    assert_eq!(Solution::split_array(vec![1000, 900, 800, 700, 600, 500, 400, 300, 200, 100], 2), 2800);
}

#[test]
fn test_32() {
    assert_eq!(Solution::split_array(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 5), 4);
}

#[test]
fn test_33() {
    assert_eq!(Solution::split_array(vec![1, 10, 100, 1000, 10000, 100000, 1000000], 3), 1000000);
}

#[test]
fn test_34() {
    assert_eq!(Solution::split_array(vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000], 10), 1000);
}

#[test]
fn test_35() {
    assert_eq!(Solution::split_array(vec![5, 1, 4, 2, 3, 6, 7, 8, 9, 10], 4), 17);
}

#[test]
fn test_36() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3), 21);
}

#[test]
fn test_37() {
    assert_eq!(Solution::split_array(vec![50, 30, 20, 10, 40, 60, 70, 80, 90, 100], 4), 170);
}

#[test]
fn test_38() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10), 28);
}

#[test]
fn test_39() {
    assert_eq!(Solution::split_array(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 5), 8);
}

#[test]
fn test_40() {
    assert_eq!(Solution::split_array(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 7), 30);
}

#[test]
fn test_41() {
    assert_eq!(Solution::split_array(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], 6), 250);
}

#[test]
fn test_42() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), 17);
}

#[test]
fn test_43() {
    assert_eq!(Solution::split_array(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 5), 1500);
}

#[test]
fn test_44() {
    assert_eq!(Solution::split_array(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 10), 63);
}

#[test]
fn test_45() {
    assert_eq!(Solution::split_array(vec![1000000, 1, 1000000, 1, 1000000], 2), 2000001);
}

#[test]
fn test_46() {
    assert_eq!(Solution::split_array(vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000], 2), 5000);
}

#[test]
fn test_47() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 3), 74);
}

#[test]
fn test_48() {
    assert_eq!(Solution::split_array(vec![9, 18, 27, 36, 45, 54, 63, 72, 81, 90], 9), 90);
}

#[test]
fn test_49() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 10), 28);
}

#[test]
fn test_50() {
    assert_eq!(Solution::split_array(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 3), 2100);
}

#[test]
fn test_51() {
    assert_eq!(Solution::split_array(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], 7), 230);
}

#[test]
fn test_52() {
    assert_eq!(Solution::split_array(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], 7), 230);
}

#[test]
fn test_53() {
    assert_eq!(Solution::split_array(vec![7, 3, 8, 7, 10, 1, 12, 6, 7, 6, 8, 9], 5), 18);
}

#[test]
fn test_54() {
    assert_eq!(Solution::split_array(vec![3, 5, 8, 10, 15, 18, 20], 3), 33);
}

#[test]
fn test_55() {
    assert_eq!(Solution::split_array(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500], 5), 2800);
}

#[test]
fn test_56() {
    assert_eq!(Solution::split_array(vec![100, 200, 300, 400, 500], 2), 900);
}

#[test]
fn test_57() {
    assert_eq!(Solution::split_array(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 3), 7);
}

#[test]
fn test_58() {
    assert_eq!(Solution::split_array(vec![100, 50, 20, 30, 10, 80, 90, 40, 60, 70], 3), 200);
}

#[test]
fn test_59() {
    assert_eq!(Solution::split_array(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 4), 1700);
}

#[test]
fn test_60() {
    assert_eq!(Solution::split_array(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200], 10), 280);
}

#[test]
fn test_61() {
    assert_eq!(Solution::split_array(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10), 4);
}

#[test]
fn test_62() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], 15), 28);
}

#[test]
fn test_63() {
    assert_eq!(Solution::split_array(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120], 5), 210);
}

#[test]
fn test_64() {
    assert_eq!(Solution::split_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], 3), 17);
}

#[test]
fn test_65() {
    assert_eq!(Solution::split_array(vec![100, 400, 300, 100, 500, 300, 200], 4), 500);
}

#[test]
fn test_66() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 5), 46);
}

#[test]
fn test_67() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 15), 42);
}

#[test]
fn test_68() {
    assert_eq!(Solution::split_array(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15], 20), 15);
}

#[test]
fn test_69() {
    assert_eq!(Solution::split_array(vec![1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000], 2), 3002);
}

#[test]
fn test_70() {
    assert_eq!(Solution::split_array(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 5), 150);
}

#[test]
fn test_71() {
    assert_eq!(Solution::split_array(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], 8), 210);
}

#[test]
fn test_72() {
    assert_eq!(Solution::split_array(vec![3, 2, 4, 1, 5, 9, 7, 6, 8, 10], 4), 16);
}

#[test]
fn test_73() {
    assert_eq!(Solution::split_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 15), 11);
}

#[test]
fn test_74() {
    assert_eq!(Solution::split_array(vec![2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1], 5), 6);
}

#[test]
fn test_75() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 5), 46);
}

#[test]
fn test_76() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
}

#[test]
fn test_77() {
    assert_eq!(Solution::split_array(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 4), 170);
}

#[test]
fn test_78() {
    assert_eq!(Solution::split_array(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 50), 10);
}

#[test]
fn test_79() {
    assert_eq!(Solution::split_array(vec![1000000, 999999, 888888, 777777, 666666, 555555, 444444, 333333, 222222, 111111], 5), 1666665);
}

#[test]
fn test_80() {
    assert_eq!(Solution::split_array(vec![9, 7, 6, 5, 4, 3, 2, 1], 4), 11);
}

#[test]
fn test_81() {
    assert_eq!(Solution::split_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 6), 17);
}

#[test]
fn test_82() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], 15), 28);
}

#[test]
fn test_83() {
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 1), 210);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::max_sub_array(vec![0, 1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4, 7]), 12);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_sub_array(vec![10000, -10000, 10000, -10000, 10000]), 10000);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_sub_array(vec![100, -1, 50, -1, 100]), 248);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_sub_array(vec![0, -3, 5, -2, 1, 3, -1, 2, -4, 2]), 8);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_sub_array(vec![0, -3, 5, -2, 1, 3, -1, 2, -4, 2, 3]), 9);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_sub_array(vec![-5, -4, -3, -2, -1]), -1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_sub_array(vec![-1, 0, -2, 0, -3, 0, -4, 0]), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_sub_array(vec![1, -2, 3, 5, -3, 2]), 8);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_sub_array(vec![-2, 0, -1]), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_sub_array(vec![1, -1, 2, -2, 3, -3, 4, -4]), 4);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_sub_array(vec![10000, -10000, 10000]), 10000);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_sub_array(vec![0, 0, 0, 0]), 0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_sub_array(vec![-1, -2, -3, -4]), -1);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_sub_array(vec![-10000, 10000]), 10000);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_sub_array(vec![-10000, 10000, -10000, 10000, -10000, 10000]), 10000);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4, 1, 2, 3]), 11);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_sub_array(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5]), 5);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_sub_array(vec![5, -1, 3, -2, 4, -3, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5]), 11);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_sub_array(vec![100, -1, -2, -3, 100, -1, -2, -3]), 194);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_sub_array(vec![-10, -20, -30, -40, -50, -60]), -10);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_sub_array(vec![5, -2, -3, 4, -1, -2, 1, 5, -3]), 7);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, -15, 6, 7, 8, 9, 10, -25, 11, 12, 13, 14, 15]), 80);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_sub_array(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15]), 15);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_sub_array(vec![10, -1, 2, -3, 4, -5, 6, -7, 8, -9, 10, -11, 12, -13, 14, -15, 16, -17, 18, -19]), 19);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_sub_array(vec![100, -100, 50, -50, 75, -75, 25, -25, 0, 0, 0]), 100);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_sub_array(vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0]), 1);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_sub_array(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), 1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -100, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_sub_array(vec![2, 3, -2, 5, -3, 4, -1, 2, 1, -5, 4, -6, 7, 8, -9, 10, 11, -12]), 31);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_sub_array(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_sub_array(vec![10000, -10000, 10000, -10000, 10000, -10000, 10000, -10000, 10000, -10000]), 10000);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_sub_array(vec![1, -3, 2, 1, -1, 3, -2, 3, 4, -5, 2, 1, -1, 2, 3, -4, 5, -6, 7, -8]), 14);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_sub_array(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 100]), 100);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5, 6, 7, 8, 9, 10]), 40);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4, 6, -1, 2, -1, 4, -3]), 15);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_sub_array(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), 1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_sub_array(vec![-21, 22, -23, 24, -25, 26, -27, 28, -29, 30, -31, 32, -33, 34, -35, 36, -37, 38, -39, 40]), 40);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_sub_array(vec![100, -1, 2, -3, -4, 5, 6, 7, -8, 9, 10, -11, 12, 13, -14]), 137);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_sub_array(vec![100, -100, 100, -100, 100, -100, 100, -100, 100, -100]), 100);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_sub_array(vec![10, -10, 20, -20, 30, -30, 40, -40, 50]), 50);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_sub_array(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9]), 9);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_sub_array(vec![10, -20, 30, -40, 50, -60, 70, -80, 90, -100, 110, -120, 130, -140, 150]), 150);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_sub_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, -1, -2, -3, -4, -5, -6, -7, -8, -9]), 45);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_sub_array(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10, -10]), 10);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, 20]), 55);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5, 1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_sub_array(vec![10, -5, 10, -5, 10, -5, 10, -5, 10, -5, 10, -5, 10, -5, 10, -5, 10, -5, 10, -5]), 55);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8, -20, 15, -10, 25, -5, 10, -30, 40, -50, 60]), 60);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_sub_array(vec![-1, 0, 1, 2, -1, 0, 1, 2, -1, 0, 1, 2, -1, 0, 1, 2, -1, 0, 1, 2]), 11);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_sub_array(vec![-2, -3, -1, -5, -4]), -1);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5, 1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_sub_array(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), 3);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_sub_array(vec![-1, -2, -3, -4, -5, -6, 7, 8, 9, 10, 11, -1, -2, -3, -4]), 45);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_sub_array(vec![-1000, 500, -300, 200, -100, 50, -25, 12, -6, 3, -1]), 500);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_sub_array(vec![-10000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10000]), 10000);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5, 10]), 15);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_sub_array(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20]), -1);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, -15, 6, 7, 8, 9, 10]), 40);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_sub_array(vec![-10, 0, -1, 3, 4, -5, 1, 2, -1, -2, 3, 4, -5, 6, 7, -8, 9, 10]), 28);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4, -2, 3, 4, -1, 2, 1, -5, 4]), 12);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_sub_array(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), -1);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_sub_array(vec![10000, -5000, 5000, -5000, 5000, -5000, 5000, -5000, 5000, -5000]), 10000);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8, -10, 12, 3, 4, -15, 20, -21, 22]), 38);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, -10, 10, 20, -5, 5]), 30);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_sub_array(vec![-2, -3, -1, -5, -4, -6, -3, -1, -2]), -1);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_sub_array(vec![-2, -3, 4, -1, -2, 1, 5, -3, 4, -1, 2, 1, -5, 4]), 10);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_sub_array(vec![1000, -500, 200, -100, 50, -25, 12, -6, 3, -1, 0, -2, 1, 3, -1, 2]), 1000);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_sub_array(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, 100]), 100);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_sub_array(vec![-10, -20, -30, -40, -50, 100, -1, -2, -3]), 100);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_sub_array(vec![1000, -1000, 500, -500, 250, -250, 125, -125]), 1000);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_sub_array(vec![-1, 0, -2, 0, -3, 0, -4, 0, -5, 0, -6, 0, -7, 0, -8, 0]), 0);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, -6, 1, 2, 3, 4, -10, 5, 6, 7, 8]), 26);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_sub_array(vec![-10000, -9999, -9998, -9997, -9996, -9995, -9994, -9993, -9992, -9991]), -9991);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_sub_array(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]), 10);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_sub_array(vec![1000, -500, 200, -300, 100, -200, 50, -10, 5, 1]), 1000);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, -6, 1, 2, 3, 4, 5, -10, 1, 2, 3, 4, 5, 6, 7]), 33);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_sub_array(vec![1000, -1000, 500, -500, 250, -250, 125, -125, 62, -62, 31, -31, 15, -15]), 1000);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_sub_array(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), -1);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_sub_array(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 100]), 100);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_sub_array(vec![100, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 100);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_sub_array(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15]), 15);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_sub_array(vec![10, 20, 30, 40, 50, -100, 60, 70, 80, 90, 100]), 450);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_sub_array(vec![-1, 2, -1, 2, -1, 2, -1, 2, -1, 2, -1, 2, -1, 2]), 8);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_sub_array(vec![100, -1, -2, -3, 100]), 194);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4, 6, -1, 2, -1, 2, 3]), 16);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 210);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_sub_array(vec![100, -1, -2, -3, 100, -1, -2, -3, 100]), 288);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, -15, 6, 7, 8, 9, 10, -25, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 170);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5, 10, 20, 30, -10, -20, -30]), 60);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_sub_array(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14]), -1);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 210);
}

#[test]
fn test_97() {
    assert_eq!(Solution::max_sub_array(vec![0, -1, -2, -3, -4, -5, 6, 7, 8, 9, 10, -1, -2, -3, -4, -5, 6, 7, 8, 9, 10]), 65);
}

#[test]
fn test_98() {
    assert_eq!(Solution::max_sub_array(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15, -16, 17, -18, 19, -20]), 19);
}

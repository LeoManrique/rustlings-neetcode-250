include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::remove_duplicates(vec![-100, 0, 100]), 3);
}

#[test]
fn test_3() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 4, 4, 4, 5]), 5);
}

#[test]
fn test_4() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_5() {
    assert_eq!(Solution::remove_duplicates(vec![1]), 1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::remove_duplicates(vec![0, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4]), 5);
}

#[test]
fn test_7() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, -1, -1, -1]), 1);
}

#[test]
fn test_8() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, 0, 0, 1, 1, 2]), 4);
}

#[test]
fn test_9() {
    assert_eq!(Solution::remove_duplicates(vec![-10, -10, -9, -9, -8, -7, -6, -6, -6, -5]), 6);
}

#[test]
fn test_10() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -100, -50, -50, 0, 0, 50, 50, 100, 100]), 5);
}

#[test]
fn test_11() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 0, 0, 0]), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 10);
}

#[test]
fn test_13() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_15() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 2]), 2);
}

#[test]
fn test_16() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 4, 4, 5]), 5);
}

#[test]
fn test_17() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_18() {
    assert_eq!(Solution::remove_duplicates(vec![-1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]), 7);
}

#[test]
fn test_19() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -100, -99, -99, 0, 0, 1, 1, 2, 2, 3, 3]), 6);
}

#[test]
fn test_20() {
    assert_eq!(Solution::remove_duplicates(vec![-100, 100]), 2);
}

#[test]
fn test_21() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]), 5);
}

#[test]
fn test_22() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 0, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4]), 5);
}

#[test]
fn test_23() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, 0, 0, 1, 2, 2, 3, 4]), 6);
}

#[test]
fn test_24() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14]), 15);
}

#[test]
fn test_25() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 9, 9]), 9);
}

#[test]
fn test_26() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 8, 8]), 8);
}

#[test]
fn test_27() {
    assert_eq!(Solution::remove_duplicates(vec![0, 1, 2, 2, 2, 3, 4, 4, 5, 5, 5, 5, 6, 7, 8, 8, 9, 10, 10, 10]), 11);
}

#[test]
fn test_28() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 5, 6, 6, 6, 6, 7, 7, 8, 8, 8, 9, 9]), 9);
}

#[test]
fn test_29() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 3, 4, 4, 4, 4, 4]), 4);
}

#[test]
fn test_30() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -99, -98, -98, -97, -97, -96, -95, -94, -94, -93, -92, -91, -90, -89, -89, -88, -87, -86, -85]), 16);
}

#[test]
fn test_31() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, 0, 0, 0, 0, 1, 1, 2, 2, 3, 4, 5, 5, 6, 7, 7, 7, 8, 9, 10]), 12);
}

#[test]
fn test_32() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5]), 5);
}

#[test]
fn test_33() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9]), 9);
}

#[test]
fn test_34() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 10, 10]), 10);
}

#[test]
fn test_35() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7]), 7);
}

#[test]
fn test_36() {
    assert_eq!(Solution::remove_duplicates(vec![-10, -10, -9, -9, -8, -8, -7, -7, -6, -6, -5, -5, -4, -4, -3, -3, -2, -2, -1, -1]), 10);
}

#[test]
fn test_37() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_38() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_39() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2]), 2);
}

#[test]
fn test_40() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_41() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10]), 10);
}

#[test]
fn test_42() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 1);
}

#[test]
fn test_43() {
    assert_eq!(Solution::remove_duplicates(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 21);
}

#[test]
fn test_44() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_45() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]), 2);
}

#[test]
fn test_46() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15]), 15);
}

#[test]
fn test_47() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8]), 8);
}

#[test]
fn test_48() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20]), 20);
}

#[test]
fn test_49() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -100, -50, -50, -50, -50, -50, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 100, 100, 100, 100, 100]), 5);
}

#[test]
fn test_50() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10]), 11);
}

#[test]
fn test_51() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_52() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_53() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_54() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 10);
}

#[test]
fn test_55() {
    assert_eq!(Solution::remove_duplicates(vec![-10, -10, -5, -5, -1, 0, 0, 0, 3, 3, 3, 3, 3, 5, 5, 8]), 7);
}

#[test]
fn test_56() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 9, 9, 9]), 9);
}

#[test]
fn test_57() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 3, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10]), 10);
}

#[test]
fn test_58() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 10);
}

#[test]
fn test_59() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 3, 4, 5, 5, 6, 7, 7, 8, 8, 9, 10, 10, 10]), 10);
}

#[test]
fn test_60() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -50, 0, 50, 100]), 5);
}

#[test]
fn test_61() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -100, -50, -50, 0, 0, 0, 10, 10, 20, 20, 20, 50, 50, 100, 100]), 7);
}

#[test]
fn test_62() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3]), 3);
}

#[test]
fn test_63() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_64() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9]), 11);
}

#[test]
fn test_65() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 4, 4, 4, 5, 6, 6, 7, 8, 8, 9, 10, 10]), 10);
}

#[test]
fn test_66() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7]), 7);
}

#[test]
fn test_67() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 5, 5]), 5);
}

#[test]
fn test_68() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4]), 5);
}

#[test]
fn test_69() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 3, 3, 3, 4, 4, 5, 6, 6, 6, 7, 8, 8, 9, 10, 10, 11]), 11);
}

#[test]
fn test_70() {
    assert_eq!(Solution::remove_duplicates(vec![-50, -50, -50, -25, -25, -25, 0, 0, 25, 25, 50, 50, 50]), 5);
}

#[test]
fn test_71() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 9, 9, 10, 10, 10, 10]), 10);
}

#[test]
fn test_72() {
    assert_eq!(Solution::remove_duplicates(vec![-50, -40, -30, -20, -10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 16);
}

#[test]
fn test_73() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 10);
}

#[test]
fn test_74() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_75() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 10, 10]), 10);
}

#[test]
fn test_76() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 20);
}

#[test]
fn test_77() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_78() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9]), 9);
}

#[test]
fn test_79() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 3, 3, 3]), 5);
}

#[test]
fn test_80() {
    assert_eq!(Solution::remove_duplicates(vec![0, 1, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]), 6);
}

#[test]
fn test_81() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 30);
}

#[test]
fn test_82() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, -1, 0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 5, 6, 6, 7, 8, 8, 9, 9, 9, 10, 10, 10]), 12);
}

#[test]
fn test_83() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -100, -50, -50, 0, 0, 50, 50, 100, 100]), 5);
}

#[test]
fn test_84() {
    assert_eq!(Solution::remove_duplicates(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 1);
}

#[test]
fn test_85() {
    assert_eq!(Solution::remove_duplicates(vec![0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5, 5, 5, 6]), 7);
}

#[test]
fn test_86() {
    assert_eq!(Solution::remove_duplicates(vec![-5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, 0, 0, 0, 1, 2, 3, 4, 5]), 7);
}

#[test]
fn test_87() {
    assert_eq!(Solution::remove_duplicates(vec![-10, -5, -3, -3, -2, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 16);
}

#[test]
fn test_88() {
    assert_eq!(Solution::remove_duplicates(vec![0, 1, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 6, 7, 7, 8, 8, 8, 9, 9]), 10);
}

#[test]
fn test_89() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 3, 3, 4, 5, 5, 6, 6, 6, 6, 7, 8, 8, 8, 9, 9, 9, 9, 9, 10, 10, 10, 10]), 10);
}

#[test]
fn test_90() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 7, 8, 8, 9, 9]), 10);
}

#[test]
fn test_91() {
    assert_eq!(Solution::remove_duplicates(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100]), 1);
}

#[test]
fn test_92() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -50, -50, -50, -25, -10, 0, 0, 0, 0, 5, 10, 10, 25, 50, 50, 50, 100, 100]), 10);
}

#[test]
fn test_93() {
    assert_eq!(Solution::remove_duplicates(vec![0, 1, 2, 2, 3, 3, 4, 5, 5, 5, 6, 7, 8, 8, 9, 10, 10, 10, 10]), 11);
}

#[test]
fn test_94() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 1);
}

#[test]
fn test_95() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 3, 4, 5, 5, 5, 6, 7, 7, 7, 7, 8, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_96() {
    assert_eq!(Solution::remove_duplicates(vec![-5, -5, -5, -5, -4, -4, -3, -3, -2, -2, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8]), 14);
}

#[test]
fn test_97() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_98() {
    assert_eq!(Solution::remove_duplicates(vec![-5, -4, -4, -3, -2, -2, -1, 0, 1, 2, 2, 2, 3, 3, 4, 4, 4, 5, 6, 7, 8, 8, 9, 10]), 16);
}

#[test]
fn test_99() {
    assert_eq!(Solution::remove_duplicates(vec![-5, -4, -4, -4, -3, -2, -2, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3, 4]), 10);
}

#[test]
fn test_100() {
    assert_eq!(Solution::remove_duplicates(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 1);
}

#[test]
fn test_101() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -100, -99, -98, -98, -97, -97, -97, -96]), 5);
}

#[test]
fn test_102() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_103() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -100, -50, -50, 0, 0, 0, 50, 50, 50, 100, 100]), 5);
}

#[test]
fn test_104() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, -1, -1, -1, 0, 0, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]), 7);
}

#[test]
fn test_105() {
    assert_eq!(Solution::remove_duplicates(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11]), 2);
}

#[test]
fn test_106() {
    assert_eq!(Solution::remove_duplicates(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100]), 1);
}

#[test]
fn test_107() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 8, 9, 10]), 12);
}

#[test]
fn test_108() {
    assert_eq!(Solution::remove_duplicates(vec![-1, -1, 0, 0, 0, 1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 8, 9, 10]), 12);
}

#[test]
fn test_109() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 5);
}

#[test]
fn test_110() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 4]), 4);
}

#[test]
fn test_111() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 10);
}

#[test]
fn test_112() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -100, -50, -50, 0, 0, 0, 50, 50, 100, 100]), 5);
}

#[test]
fn test_113() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 11, 12, 12, 13, 14, 15, 15, 16, 17]), 17);
}

#[test]
fn test_114() {
    assert_eq!(Solution::remove_duplicates(vec![-10, -10, -10, -10, -10, -9, -9, -9, -8, -8, -7, -7, -6, -6, -5, -5, -4, -4, -3, -3, -2, -2, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3]), 14);
}

#[test]
fn test_115() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_116() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 5, 5, 6, 6, 6, 7, 7, 8, 8, 9, 9]), 9);
}

#[test]
fn test_117() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 10, 10, 10, 10, 10]), 10);
}

#[test]
fn test_118() {
    assert_eq!(Solution::remove_duplicates(vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 10, 10, 10, 11, 11, 12, 12]), 12);
}

#[test]
fn test_119() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -50, -50, -25, -25, -10, -10, 0, 0, 50, 50, 100, 100]), 7);
}

#[test]
fn test_120() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -50, -50, 0, 0, 0, 25, 25, 50, 75, 75, 100]), 7);
}

#[test]
fn test_121() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -99, -98, -97, -96, -95, -95, -95, -94, -93, -92, -91, -90, -89, -88, -87, -86, -85, -84, -83]), 18);
}

#[test]
fn test_122() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 6, 6, 6, 6, 6]), 6);
}

#[test]
fn test_123() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 2, 2, 3, 4, 4, 5, 5, 5, 6, 6, 7, 8, 8, 9, 9, 10, 10]), 10);
}

#[test]
fn test_124() {
    assert_eq!(Solution::remove_duplicates(vec![-100, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90, -89, -88, -87, -86, -85, -84, -83, -82, -81]), 20);
}

#[test]
fn test_125() {
    assert_eq!(Solution::remove_duplicates(vec![-10, -10, -9, -9, -8, -8, -7, -7, -6, -6, -5, -5, -4, -4, -3, -3, -2, -2, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]), 21);
}

#[test]
fn test_126() {
    assert_eq!(Solution::remove_duplicates(vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 7, 8, 9, 10]), 10);
}

#[test]
fn test_127() {
    assert_eq!(Solution::remove_duplicates(vec![-50, -50, -50, -25, -25, -25, -10, -10, -10, 0, 0, 0, 50, 50, 50, 100, 100, 100]), 6);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::max_product(vec![2, -5, -2, -4, 3]), 24);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_product(vec![1]), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_product(vec![-2, 3, -4]), 24);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_product(vec![10, -20, 0, 5, 1]), 10);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_product(vec![3, -1, 4]), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_product(vec![-1, -2, -3, 0]), 6);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_product(vec![0, 2]), 2);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_product(vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10]), 3628800);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_product(vec![1, 2, -1, 4]), 4);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_product(vec![0, 2, 0]), 2);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_product(vec![-1, -2, -3, -4]), 24);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_product(vec![-1]), -1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_product(vec![-2, 3, -4, 5, 7, -8, 2, 3]), 20160);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_product(vec![1, 2, 3, 4, 5]), 120);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_product(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 3628800);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_product(vec![0, 2, 3, -2, 4, -1, 5]), 240);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_product(vec![-10, 0, 10, 20, 30, -40, 50, 60, -70, 80]), 4032000000000);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_product(vec![-1, 0, -1, 0, -1, 0, -1, 0, -1, 0]), 0);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4, -1, 5, 6]), 1440);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_product(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 1);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_product(vec![10, -10, 20, -20, 30, -30, 40, -40, 50]), 2880000000000);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_product(vec![1, -1, 1, -1, 1, -1, 1, -1]), 1);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_product(vec![-2, -3, 7, -4, 0, 5, -8, 3, 6]), 84);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_product(vec![5, 0, 5, -1, 0, 5, -1, 0, 5, -1]), 5);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_product(vec![0, 2, -3, 4, -5, 6]), 720);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_product(vec![0, 2, -3, 4, -5, 6]), 720);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_product(vec![2, 3, 0, -1, 4, 5, 0, -2, 3, 0]), 20);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_product(vec![2, 3, 0, -1, -2, 4, 0, 5, 6, 0, -7, 8, -9]), 504);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_product(vec![1, 0, -1, 0, 1, -1, 0, 1, 0]), 1);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_product(vec![5, 6, -3, 4, 0, 2, 3, -2, 4]), 30);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_product(vec![10, -10, 10, -10, 10, -10]), 100000);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_product(vec![5, 6, -3, 4, 0, -1, 2, -5]), 30);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_product(vec![0, 2, 3, -2, 4, -1, 5, 6]), 1440);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_product(vec![2, -5, 3, 1, -4, 2]), 240);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_product(vec![5, 3, -1, 2, 0, -6, -2, 0, 5, 3, -1, 2, 0, -6, -2]), 15);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_product(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_product(vec![-2, -3, 0, -2, -40, 0, -10]), 80);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_product(vec![0, 2, -3, 4, -1, 2, 1, -5, 4]), 160);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_product(vec![10, -2, -3, 5, -10, 0, 9, 6]), 300);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_product(vec![100, -100, 50, -50, 25, -25, 10, -10, 5, -5]), 7812500000000);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_product(vec![-2, 0, -1, 0, 1, 2, -3]), 2);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_product(vec![5, -2, -3, -4, -5, -6, -7, -8, -9, -10]), 1814400);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_product(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5]), 2880);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_product(vec![-1, -2, 0, 1, -3, 4, -5, 6, -7, 8, -9, 10]), 1814400);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_product(vec![1, 0, -1, 0, 1, 0, -1, 0, 1]), 1);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_product(vec![10, -10, 10, -10, 10, -10, 10]), 100000);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_product(vec![-10, 0, 0, 0, 0, 0, 0, 0, 0, -10]), 0);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_product(vec![-10, 0, 10, 0, -10, 0, 10, 0, -10, 0]), 10);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_product(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 3628800);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_product(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 1);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_product(vec![-3, -1, 0, 2, 4, -2, 0, -1]), 8);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_product(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 3628800);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_product(vec![3, -1, 4, 1, 5, -9, 2, 6, 5, 3, 5]), 486000);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_product(vec![5, 3, -2, 5, -1, 5, -1, 0, 5]), 750);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_product(vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1]), 1);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_product(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10]), 1814400);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_product(vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]), 120);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_product(vec![-10, 0, 5, 2, -3, -2, 4, 5, 0, -1, 2]), 1200);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_product(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_product(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10]), 1814400);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_product(vec![3, -1, 4, 1, 5, -9, 2, 6, -5, 3, 5]), 162000);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_product(vec![0, 2, -3, 4, -1, 0, 5, -2]), 24);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_product(vec![2, -5, -2, -4, 3]), 24);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_product(vec![-10, 9, -10, 10, -1, -100]), 900000);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_product(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), 3628800);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_product(vec![3, -1, 4, -1, 5, -9, 2, 6, -5, 3, 5]), 486000);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_product(vec![1, 0, -1, 0, -2, 0, 1, 0, -1, 0, -2, 0]), 1);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_product(vec![-1, -1, -1, -1, -1, -1, -1, -1]), 1);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_product(vec![100, -100, 100, -100, 100, -100, 100, -100, 100, -100]), 1000000000000000000);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_product(vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10]), 3628800);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_product(vec![10, -10, 0, 10, -10, 0, 10]), 10);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_product(vec![1, -2, 3, -4, 5, -6, 7, -8, 9]), 362880);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_product(vec![0, 2, -3, -4, 5, 0, 1]), 120);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_product(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), 1);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_product(vec![-10, -10, -10, -10, -10]), 10000);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_product(vec![-2, 3, -4, 5, -6, 7, -8, 9, -10, 11]), 19958400);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_product(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 1);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_product(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), 3628800);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_product(vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1]), 1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_product(vec![1, 2, 3, -4, -5, -6, 7, 8, 9, 10, -11]), 39916800);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_product(vec![10, -10, 10, -10, 10]), 100000);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_product(vec![5, -3, 1, -2, 0, 4, -2, 3, -1]), 30);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_product(vec![10, -10, 10, -10, 10, -10]), 100000);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_product(vec![-10, -20, 0, -5, -7, 0, 2, -1]), 200);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_product(vec![1, 2, 3, 0, -1, -2, -3, -4, 5, 6]), 720);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_product(vec![1, 2, 3, 0, -1, -2, -3, 0, 4, 5, 6, 0, -7, -8, -9, 0, 10, 11, 12]), 1320);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_product(vec![1, -2, 3, -4, 5, -6, 7, -8]), 40320);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_product(vec![10, -10, 10, -10, 10, -10, 10, -10]), 100000000);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_product(vec![2, 0, 1, 0, 2, 3, -2, 4, -1, 5]), 240);
}

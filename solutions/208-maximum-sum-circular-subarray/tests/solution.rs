include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, -2, 5, 1, -4, 3, -1]), 16);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![2, 3, -4, 5, 7]), 17);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
}

#[test]
fn test_5() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -1, 2, -1]), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![2, -2, 2, 7]), 11);
}

#[test]
fn test_7() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, 4]), 4);
}

#[test]
fn test_8() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![9, -4, 7, 3, -2]), 17);
}

#[test]
fn test_9() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, 0, 3, 5, -10, 2, 4]), 24);
}

#[test]
fn test_10() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, -4]), -1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -4, 7, 2]), 10);
}

#[test]
fn test_13() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, -4, -5]), -1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![9, -4, 7, 2, -1]), 17);
}

#[test]
fn test_15() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
}

#[test]
fn test_16() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![100, -1, -2, -3, -4, -5, -6, -7, -8, -9, 100, 100, -200, 100, 100]), 455);
}

#[test]
fn test_17() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-10, -20, -30, 15, 20, 25, -10, -5]), 60);
}

#[test]
fn test_18() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5, -2, 3, -4, 7, -6, 8, -1]), 18);
}

#[test]
fn test_19() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -1, 4, -1, 5, -9, 2, 6, -3, 8]), 23);
}

#[test]
fn test_20() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, -3, 4, -2, -1, 10, -10, 5, 6, -5, 2]), 26);
}

#[test]
fn test_21() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
}

#[test]
fn test_22() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![2, 4, -2, -3, 8]), 14);
}

#[test]
fn test_23() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, -6, 4, 5, 6, -10, 7, 8, 9]), 39);
}

#[test]
fn test_24() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, -20, 1, 2, 3, 4, 5, -20, 1, 2, 3, 4, 5]), 30);
}

#[test]
fn test_25() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -2, 2, 5, -3]), 8);
}

#[test]
fn test_26() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -10, 3, 4, -2, 5]), 11);
}

#[test]
fn test_27() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![100, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]), 100);
}

#[test]
fn test_28() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-2, 4, -3, 5, 1, -5, 3, -1, 2, 4, -5]), 10);
}

#[test]
fn test_29() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -1, -1, -1, -1, 100, -1, -1, -1, -1]), 100);
}

#[test]
fn test_30() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-5, -1, -8, 2, 6, -3, 4, 2, -8, 3, 7]), 13);
}

#[test]
fn test_31() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, 4, -2, 3, -2, 4, -2, 3, -1, 2, -5, 6]), 14);
}

#[test]
fn test_32() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![2, 3, -4, 5, 7, 1, -1, -3, 4]), 18);
}

#[test]
fn test_33() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![8, -1, 3, -4, 3, 2, -2, 5, 7, -4, 5, -2, 1, 2, -1, 3, 4, -3, 4, -1, -2, 1, 5]), 37);
}

#[test]
fn test_34() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, 20, -1, -2, -3, 20, -1, -2, -3, 20]), 48);
}

#[test]
fn test_35() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -1, 2, -2, 3, -3, 4, -4]), 4);
}

#[test]
fn test_36() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, -5, 6, 7, -8, 9, 10, -11, 12]), 37);
}

#[test]
fn test_37() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -2, 2, -3, 4, -1, 2, 1, -5, 4]), 10);
}

#[test]
fn test_38() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5, -3, 5, -3, 5, -3, 5, -3]), 13);
}

#[test]
fn test_39() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1000, 1000, -2000, 2000, -3000, 3000, -4000, 4000, -5000, 5000]), 5000);
}

#[test]
fn test_40() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10000, -1000, 5000, -500, 2000, -300]), 16200);
}

#[test]
fn test_41() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2, 4, -1, 2, 1, -5, 4]), 10);
}

#[test]
fn test_42() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![8, -1, 3, -2, 4, -3, 2, 1, -5, 4]), 16);
}

#[test]
fn test_43() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, -20, 5, 5, -3, 2, -1]), 18);
}

#[test]
fn test_44() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![7, 5, -3, 6, -1, 8, -4, 2]), 24);
}

#[test]
fn test_45() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, 4, 5, -1, 2, -1]), 10);
}

#[test]
fn test_46() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_47() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -1, 4, -1, 5, -9, 2, 6, -5, 3, -8]), 10);
}

#[test]
fn test_48() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, -15, 1, 2, 3, 4, 5]), 30);
}

#[test]
fn test_49() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-5, 1, 5, 0, -7, 3, -1, 2]), 6);
}

#[test]
fn test_50() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, 0, 1, -2, 2, -3, 3, -4, 4, -5, 5]), 5);
}

#[test]
fn test_51() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-2, -3, 4, -1, -2, 1, 5, -3, 2, -5]), 7);
}

#[test]
fn test_52() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-10, 15, -20, 25, -30, 35, -40, 45, -50, 55]), 75);
}

#[test]
fn test_53() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![7, 5, -3, 6, -5, 2, -7, 8, -2, 4, 5]), 30);
}

#[test]
fn test_54() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2, 5, 6, -4, 2]), 13);
}

#[test]
fn test_55() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-2, -3, -1, -4, -5, -7, -8, -6, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20]), -1);
}

#[test]
fn test_56() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, -10, 1, 2, 3, 4]), 20);
}

#[test]
fn test_57() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5]), 15);
}

#[test]
fn test_58() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6]), 6);
}

#[test]
fn test_59() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, -4, -5, 5, 4, 3, 2, 1, -1, -2]), 15);
}

#[test]
fn test_60() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, 4, 5, -10, 6, 7]), 16);
}

#[test]
fn test_61() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-10, -20, -30, -40, -50, -60, -70, -80, -90, -100, 1000]), 1000);
}

#[test]
fn test_62() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-5, -1, -8, -9, -2, -6, -3, -8, -4, -5]), -1);
}

#[test]
fn test_63() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, 20, 30, 40, 50, -5, -10, -15, -20, -25, 50, 100, -50, -100, 150, -200, 250, -300, 350, -400]), 350);
}

#[test]
fn test_64() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -1, 2, -1, 4, -2, 5, -3, 6, -4, 7, -5, 8, -6, 9, -7, 10, -8]), 25);
}

#[test]
fn test_65() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -2, 2, -3, 4, -1, 2, 1, -5, 4, 3, -2, 2, -3, 4, -1, 2, 1, -5, 4]), 15);
}

#[test]
fn test_66() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-5, 5, -5, 5, -5, 5]), 5);
}

#[test]
fn test_67() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, -10, 20, -20, 30, -30, 40, -40, 50, -50]), 50);
}

#[test]
fn test_68() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-2, 4, -3, 4, -1, -2, 1, 5, -3]), 8);
}

#[test]
fn test_69() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![100, -200, 300, -400, 500, -600, 700, -800, 900, -1000, 1000]), 1500);
}

#[test]
fn test_70() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10, -10]), 10);
}

#[test]
fn test_71() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, -6, 7, 8, 9, 10]), 49);
}

#[test]
fn test_72() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![15, -2, 20, 4, -5, -9, -10]), 37);
}

#[test]
fn test_73() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5, 10]), 25);
}

#[test]
fn test_74() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -10, 3, 4, -5, 2, 6, -1, 4]), 14);
}

#[test]
fn test_75() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
}

#[test]
fn test_76() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13]), 20);
}

#[test]
fn test_77() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, -7, 9, -7, 9, -7, 9, 9, -6, -4, 6]), 31);
}

#[test]
fn test_78() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![100, -1, -100, 100, -1, -100, 100, -1, -100, 100]), 200);
}

#[test]
fn test_79() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, -2, 5, -2, 5, -2, 5, -2]), 14);
}

#[test]
fn test_80() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-10, 2, 3, -2, 5, -6, 7, -8, 9, -10]), 10);
}

#[test]
fn test_81() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, 10, -20, 5, 10, 5]), 35);
}

#[test]
fn test_82() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![8, -1, 3, 4, -2, -3, 6, 1]), 21);
}

#[test]
fn test_83() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, 5, 5, 5, -10, 5, 5, 5, 5]), 40);
}

#[test]
fn test_84() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, -5, 4, -1, 12, -3, 7, -2, 8, -6]), 30);
}

#[test]
fn test_85() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), -1);
}

#[test]
fn test_86() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![8, -1, 2, -3, 4, -5, 6, -7, 8, -9, 10, -11, 12, -13, 14, -15]), 15);
}

#[test]
fn test_87() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, -20, 30, -40, 50, -60, 70, -80, 90, -100]), 90);
}

#[test]
fn test_88() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![2, 3, -4, 5, 7, -10, 20, -5, 3, -2, 1, -1, 4, -3, 2, -2, 5, -7, 8, -6]), 30);
}

#[test]
fn test_89() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, 3, 4, -5, 6, -7, 8, -9, 10]), 16);
}

#[test]
fn test_90() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, -6, 1, 2, 3, 4, 5, -10, 1, 2, 3, 4, 5]), 30);
}

#[test]
fn test_91() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -3, -5, 2, 1, -1, -2, 3, 4]), 7);
}

#[test]
fn test_92() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, -4, -5, 1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_93() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![100, -100, 100, -100, 100, -100, 100]), 200);
}

#[test]
fn test_94() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-5, 5, -5, 5, -5, 5, -5, 5]), 5);
}

#[test]
fn test_95() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -1, -1, 3, 5, -7, 5]), 14);
}

#[test]
fn test_96() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, 5, -6, 4, 2]), 13);
}

#[test]
fn test_97() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1]), 1);
}

#[test]
fn test_98() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, -5, -4, -3, -2, -1]), 15);
}

#[test]
fn test_99() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-5000, -1000, -3000, -2000, -4000, -1500]), -1000);
}

#[test]
fn test_100() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, -10, 20, -20, 30, -30, 40, -40, 50]), 90);
}

#[test]
fn test_101() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5, 1, 2, 3, 4, 5]), 30);
}

#[test]
fn test_102() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, -6, 4, 5, 6, -10, 1, 2, 3, 4]), 25);
}

#[test]
fn test_103() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, -1, 5, -2, 3, 4, -5]), 14);
}

#[test]
fn test_104() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, -4, -5, 15, 10, 5, -10, -5, 20, 25, -30]), 60);
}

#[test]
fn test_105() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -4, 5, 2, -6, 1, 4, -3, 2, -5]), 7);
}

#[test]
fn test_106() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), -1);
}

#[test]
fn test_107() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, -4, -5, 1, 2, 3, 4, 5, -10]), 15);
}

#[test]
fn test_108() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, -100, 10, 20, 30, 40, 50]), 195);
}

#[test]
fn test_109() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, 1, -2, 3, -1, 2]), 10);
}

#[test]
fn test_110() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![10, -2, -3, -4, 7, 9, -20, 10, 15]), 42);
}

#[test]
fn test_111() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), 1);
}

#[test]
fn test_112() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![3, -2, 2, 5, -3, 3, 5, -2, 3, 2, 5, -3]), 21);
}

#[test]
fn test_113() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15]), 22);
}

#[test]
fn test_114() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, -1, 5, -1, 5, -1, 5, -1, 5, -1]), 21);
}

#[test]
fn test_115() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![100, -1, 100, -1, 100, -1, 100, -1, 100]), 497);
}

#[test]
fn test_116() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, 4, -1, 2, 1, -5, 4]), 12);
}

#[test]
fn test_117() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, 1]), 1);
}

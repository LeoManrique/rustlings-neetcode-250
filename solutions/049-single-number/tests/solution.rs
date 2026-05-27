include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::single_number(vec![-1, 2, -1, -2, 2]), -2);
}

#[test]
fn test_2() {
    assert_eq!(Solution::single_number(vec![1]), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::single_number(vec![10, 1, 10, 2, 2]), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::single_number(vec![3, 3, 7, 7, 10]), 10);
}

#[test]
fn test_5() {
    assert_eq!(Solution::single_number(vec![10, 10, 20, 20, 30]), 30);
}

#[test]
fn test_6() {
    assert_eq!(Solution::single_number(vec![3, 3, 5, 7, 5, 7, 9]), 9);
}

#[test]
fn test_7() {
    assert_eq!(Solution::single_number(vec![-1, -1, -2]), -2);
}

#[test]
fn test_8() {
    assert_eq!(Solution::single_number(vec![100000, 100000, 50000]), 50000);
}

#[test]
fn test_9() {
    assert_eq!(Solution::single_number(vec![5, 7, 5, 7, 9]), 9);
}

#[test]
fn test_10() {
    assert_eq!(Solution::single_number(vec![3, 3, 2, 2, 5]), 5);
}

#[test]
fn test_11() {
    assert_eq!(Solution::single_number(vec![2147483647, -2147483648, -2147483648, 2147483647, 0]), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 2]), 2);
}

#[test]
fn test_13() {
    assert_eq!(Solution::single_number(vec![100000, 100000, -100000]), -100000);
}

#[test]
fn test_14() {
    assert_eq!(Solution::single_number(vec![100000, -100000, 100000]), -100000);
}

#[test]
fn test_15() {
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 99]), 99);
}

#[test]
fn test_16() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
}

#[test]
fn test_17() {
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
}

#[test]
fn test_18() {
    assert_eq!(Solution::single_number(vec![100, 100, 200, 200, 300, 300, 400, 400, 500, 500, 600]), 600);
}

#[test]
fn test_19() {
    assert_eq!(Solution::single_number(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 11]), 11);
}

#[test]
fn test_20() {
    assert_eq!(Solution::single_number(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1]), 21);
}

#[test]
fn test_21() {
    assert_eq!(Solution::single_number(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), 1);
}

#[test]
fn test_22() {
    assert_eq!(Solution::single_number(vec![-1, -1, -2, -2, -3, -3, -4, -4, -5, -5, -6, -6, -7, -7, -8]), -8);
}

#[test]
fn test_23() {
    assert_eq!(Solution::single_number(vec![100, 100, 200, 200, 300, 300, 400, 400, 500, 500, 600, 600, 700, 700, 800, 800, 900, 900, 1000, 1000, 1]), 1);
}

#[test]
fn test_24() {
    assert_eq!(Solution::single_number(vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15]), 14);
}

#[test]
fn test_25() {
    assert_eq!(Solution::single_number(vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 100]), 100);
}

#[test]
fn test_26() {
    assert_eq!(Solution::single_number(vec![123, 123, 456, 456, 789, 789, 101, 101, 202, 202, 303, 303, 404]), 404);
}

#[test]
fn test_27() {
    assert_eq!(Solution::single_number(vec![10, 20, 10, 30, 30, 50, 50]), 20);
}

#[test]
fn test_28() {
    assert_eq!(Solution::single_number(vec![12345, 12345, 67890, 67890, 24680, 55555, 24680, 77777, 77777, 99999, 99999]), 55555);
}

#[test]
fn test_29() {
    assert_eq!(Solution::single_number(vec![123456, 123456, 789012, 789012, 345678, 345678, 901234]), 901234);
}

#[test]
fn test_30() {
    assert_eq!(Solution::single_number(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 10);
}

#[test]
fn test_31() {
    assert_eq!(Solution::single_number(vec![-10000, 10000, -10000, 20000, 20000, 30000, 30000]), 10000);
}

#[test]
fn test_32() {
    assert_eq!(Solution::single_number(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20]), 0);
}

#[test]
fn test_33() {
    assert_eq!(Solution::single_number(vec![100000, 100000, 200000, 200000, 300000, 300000, 400000]), 400000);
}

#[test]
fn test_34() {
    assert_eq!(Solution::single_number(vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9]), 4);
}

#[test]
fn test_35() {
    assert_eq!(Solution::single_number(vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16]), 17);
}

#[test]
fn test_36() {
    assert_eq!(Solution::single_number(vec![10000, -10000, 10000, -10000, 20000, -20000, 20000, -20000, 30000]), 30000);
}

#[test]
fn test_37() {
    assert_eq!(Solution::single_number(vec![-1000, 500, -1000, 500, 10000, 9999, 10000, 9999, 8888]), 8888);
}

#[test]
fn test_38() {
    assert_eq!(Solution::single_number(vec![1000, 2000, 1000, 3000, 4000, 3000, 5000, 6000, 5000, 7000, 8000, 6000, 7000, 8000, 9000]), 11096);
}

#[test]
fn test_39() {
    assert_eq!(Solution::single_number(vec![100, 100, 200, 200, 300, 300, 400, 400, 500, 500, 600, 600, 700, 700, 800, 800, 900, 900, 1000, 1000, 1100]), 1100);
}

#[test]
fn test_40() {
    assert_eq!(Solution::single_number(vec![5, 7, 5, 7, 9, 10, 11, 11, 10, 12, 12, 13, 13, 14, 14, 15, 15]), 9);
}

#[test]
fn test_41() {
    assert_eq!(Solution::single_number(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8]), 9);
}

#[test]
fn test_42() {
    assert_eq!(Solution::single_number(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21]), 21);
}

#[test]
fn test_43() {
    assert_eq!(Solution::single_number(vec![-1, -2, -3, -4, -5, -1, -2, -3, -4, -5, 100000]), 100000);
}

#[test]
fn test_44() {
    assert_eq!(Solution::single_number(vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8]), 8);
}

#[test]
fn test_45() {
    assert_eq!(Solution::single_number(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]), 20);
}

#[test]
fn test_46() {
    assert_eq!(Solution::single_number(vec![99999, 99999, 88888, 88888, 77777, 77777]), 0);
}

#[test]
fn test_47() {
    assert_eq!(Solution::single_number(vec![1, 2, 3, 4, 5, 1, 2, 3, 4]), 5);
}

#[test]
fn test_48() {
    assert_eq!(Solution::single_number(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 0);
}

#[test]
fn test_49() {
    assert_eq!(Solution::single_number(vec![100000, 100000, -50000, -50000, 23456, 23456, 78901]), 78901);
}

#[test]
fn test_50() {
    assert_eq!(Solution::single_number(vec![10000, 10000, 5000, 5000, 7, 7, 3, 3, 2]), 2);
}

#[test]
fn test_51() {
    assert_eq!(Solution::single_number(vec![-1000, 1000, -1000, 2000, 2000, 3000, 3000, 4000]), 3144);
}

#[test]
fn test_52() {
    assert_eq!(Solution::single_number(vec![-100, -100, -50, -50, 1, 1, 2, 2, 3, 3, 4]), 4);
}

#[test]
fn test_53() {
    assert_eq!(Solution::single_number(vec![-1, -1, -2, -2, -3, -3, -4, -4, -5, -5, -6, -6, -7]), -7);
}

#[test]
fn test_54() {
    assert_eq!(Solution::single_number(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13]), 13);
}

#[test]
fn test_55() {
    assert_eq!(Solution::single_number(vec![5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10]), 10);
}

#[test]
fn test_56() {
    assert_eq!(Solution::single_number(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11]), 11);
}

#[test]
fn test_57() {
    assert_eq!(Solution::single_number(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), 1);
}

#[test]
fn test_58() {
    assert_eq!(Solution::single_number(vec![999, 999, 888, 888, 777, 777, 666, 666, 555, 555, 444, 444, 333, 333, 222, 222, 111, 111, 101]), 101);
}

#[test]
fn test_59() {
    assert_eq!(Solution::single_number(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]), 15);
}

#[test]
fn test_60() {
    assert_eq!(Solution::single_number(vec![1000, 1000, 2000, 2000, 3000, 3000, 4000, 4000, 5000, 5000, 6000, 6000, 7000, 7000, 8000, 8000, 9000, 9000, 10000, 10000, 10001]), 10001);
}

#[test]
fn test_61() {
    assert_eq!(Solution::single_number(vec![98765, 87654, 76543, 65432, 54321, 43210, 32109, 21098, 10987, 98765, 87654, 76543, 65432, 54321, 43210, 32109, 21098, 10987, 9876, 8765, 7654, 6543, 5432, 4321, 3210, 2109, 1098, 987, 876, 765, 654, 543, 432, 321, 210, 109, 98, 87, 76, 65, 54, 43, 32, 21, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 1898);
}

#[test]
fn test_62() {
    assert_eq!(Solution::single_number(vec![9999, -9999, 8888, -8888, 7777, -7777, 6666, -6666, 5555]), 5567);
}

#[test]
fn test_63() {
    assert_eq!(Solution::single_number(vec![0, 0, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9]), 1);
}

#[test]
fn test_64() {
    assert_eq!(Solution::single_number(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7]), 7);
}

#[test]
fn test_65() {
    assert_eq!(Solution::single_number(vec![3, 3, 7, 7, 5, 5, 2, 2, 8, 8, 4, 4, 6, 6, 9, 9, 1, 1, 11, 11, 10]), 10);
}

#[test]
fn test_66() {
    assert_eq!(Solution::single_number(vec![-10000, -10000, -9999, -9999, -9998, -9998, 0]), 0);
}

#[test]
fn test_67() {
    assert_eq!(Solution::single_number(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]), 1);
}

#[test]
fn test_68() {
    assert_eq!(Solution::single_number(vec![-10000, -10000, 0, 0, 5000, 5000, 7000, 8000, 8000]), 7000);
}

#[test]
fn test_69() {
    assert_eq!(Solution::single_number(vec![12345, -12345, 12345, -12345, 6789]), 6789);
}

#[test]
fn test_70() {
    assert_eq!(Solution::single_number(vec![-30000, -30000, -20000, -20000, -10000, -10000, 0, 0, 10000]), 10000);
}

#[test]
fn test_71() {
    assert_eq!(Solution::single_number(vec![100000, -100000, 100000, 200000, -200000, 200000, 300000, -300000, 300000, 400000]), -768);
}

#[test]
fn test_72() {
    assert_eq!(Solution::single_number(vec![123, 456, 789, 123, 456, 789, 101112, -101112, -101112, 131415, 131415]), 101112);
}

#[test]
fn test_73() {
    assert_eq!(Solution::single_number(vec![23456, 23456, 78901, 22222, 22222, 11111, 11111, 33333, 33333, 44444, 44444, 55555, 55555, 66666, 66666, 77777]), 7140);
}

#[test]
fn test_74() {
    assert_eq!(Solution::single_number(vec![-1, -1, -2, -2, -3, -3, -4, -4, -5, -5, -6, -6, -7, -7, -8, -8, -9, -9, -10]), -10);
}

#[test]
fn test_75() {
    assert_eq!(Solution::single_number(vec![100000, 100000, 200000, 300000, 200000, 300000, 400000]), 400000);
}

#[test]
fn test_76() {
    assert_eq!(Solution::single_number(vec![10, 20, 10, 20, 30, 40, 50, 40, 50, 60, 70, 60, 70]), 30);
}

#[test]
fn test_77() {
    assert_eq!(Solution::single_number(vec![1, 1, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10]), 8);
}

#[test]
fn test_78() {
    assert_eq!(Solution::single_number(vec![100000, 100000, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10]), 10);
}

#[test]
fn test_79() {
    assert_eq!(Solution::single_number(vec![10, 20, 30, 40, 50, 10, 20, 30, 40, 50, 60]), 60);
}

#[test]
fn test_80() {
    assert_eq!(Solution::single_number(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]), 2);
}

#[test]
fn test_81() {
    assert_eq!(Solution::single_number(vec![100000, -100000, 100000, -100000, 1]), 1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::single_number(vec![10, 20, 10, 2, 2, 30, 30]), 20);
}

#[test]
fn test_83() {
    assert_eq!(Solution::single_number(vec![12345, -12345, 6789, -6789, 11111]), 11111);
}

#[test]
fn test_84() {
    assert_eq!(Solution::single_number(vec![1234, 1234, 5678, 5678, 91011, 91011, 121314]), 121314);
}

#[test]
fn test_85() {
    assert_eq!(Solution::single_number(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10]), 10);
}

#[test]
fn test_86() {
    assert_eq!(Solution::single_number(vec![100, 200, 300, 400, 500, 100, 200, 300, 400]), 500);
}

#[test]
fn test_87() {
    assert_eq!(Solution::single_number(vec![-1, -1, -2, -2, -3, -3, -4, -4, -5]), -5);
}

#[test]
fn test_88() {
    assert_eq!(Solution::single_number(vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7]), 7);
}

#[test]
fn test_89() {
    assert_eq!(Solution::single_number(vec![10, 10, 20, 20, 30, 30, 40, 40, 50, 50, 60, 60, 70, 70, 80, 80, 90, 90, 100, 100, 110]), 110);
}

#[test]
fn test_90() {
    assert_eq!(Solution::single_number(vec![1000, 1000, 2000, 2000, 3000, 3000, 4000, 4000, 5000, 5000, 6000, 6000, 7000, 7000, 8000, 8000, 9000, 9000, 9999]), 9999);
}

#[test]
fn test_91() {
    assert_eq!(Solution::single_number(vec![100000, 200000, 300000, 100000, 200000]), 300000);
}

#[test]
fn test_92() {
    assert_eq!(Solution::single_number(vec![100000, 100000, 20000, 20000, 3000, 3000, 40]), 40);
}

#[test]
fn test_93() {
    assert_eq!(Solution::single_number(vec![5, 7, 7, 5, 3, 9, 3, 8, 8, 6, 6]), 9);
}

#[test]
fn test_94() {
    assert_eq!(Solution::single_number(vec![-30000, -30000, -29999, -29999, -29998, -29998, 0, 0, 1, 1, 2, 2, 3, 3]), 0);
}

#[test]
fn test_95() {
    assert_eq!(Solution::single_number(vec![7, 7, 8, 8, 9, 9, 10, 10, 11]), 11);
}

#[test]
fn test_96() {
    assert_eq!(Solution::single_number(vec![1000, 1000, 2000, 2000, 3000, 3000, 4000, 4000, 5000, 5000, 6000, 6000, 7000, 7000, 8000, 8000, 9000, 9000, 10000]), 10000);
}

#[test]
fn test_97() {
    assert_eq!(Solution::single_number(vec![100, 200, 300, 100, 200, 300, 400]), 400);
}

#[test]
fn test_98() {
    assert_eq!(Solution::single_number(vec![10, 20, 10, 20, 30, 30, 40]), 40);
}

#[test]
fn test_99() {
    assert_eq!(Solution::single_number(vec![5, 7, 5, 7, 8, 8, 9, 10, 9, 10, 11]), 11);
}

#[test]
fn test_100() {
    assert_eq!(Solution::single_number(vec![-1000, -1000, -2000, -2000, -3000, -3000, -4000, -4000, -5000, -5000, -6000]), -6000);
}

#[test]
fn test_101() {
    assert_eq!(Solution::single_number(vec![1000, 2000, 3000, 1000, 4000, 2000, 5000, 3000, 4000]), 5000);
}

#[test]
fn test_102() {
    assert_eq!(Solution::single_number(vec![9999, 8888, 7777, 6666, 5555, 4444, 3333, 2222, 1111, 0, 0, 1111, 2222, 3333, 4444, 5555, 6666, 7777, 8888]), 9999);
}

#[test]
fn test_103() {
    assert_eq!(Solution::single_number(vec![98765, 98765, 87654, 87654, 76543, 76543, 65432, 65432, 54321, 54321, 12345]), 12345);
}

#[test]
fn test_104() {
    assert_eq!(Solution::single_number(vec![-1, -1, -2, -2, -3, -3, -4, -4, -5, -5, -6, -6, -7, -7, -8, -8, -9]), -9);
}

#[test]
fn test_105() {
    assert_eq!(Solution::single_number(vec![12345, 12345, 67890, 67890, -1, -1, -2, -2, -3, -3, 5]), 5);
}

#[test]
fn test_106() {
    assert_eq!(Solution::single_number(vec![-100, -200, -100, -300, -200, -400, -300]), -400);
}

#[test]
fn test_107() {
    assert_eq!(Solution::single_number(vec![12345, 67890, 12345, 67890, -11111, -22222, -11111, -33333, -22222, -33333, 44444]), 44444);
}

#[test]
fn test_108() {
    assert_eq!(Solution::single_number(vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6]), 6);
}

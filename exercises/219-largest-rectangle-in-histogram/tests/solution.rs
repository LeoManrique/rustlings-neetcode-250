include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::largest_rectangle_area(vec![1]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 0, 1, 0, 1]), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::largest_rectangle_area(vec![10000, 10000, 10000, 10000, 10000]), 50000);
}

#[test]
fn test_4() {
    assert_eq!(Solution::largest_rectangle_area(vec![0]), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5]), 9);
}

#[test]
fn test_6() {
    assert_eq!(Solution::largest_rectangle_area(vec![0, 1, 0, 1, 0]), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
}

#[test]
fn test_8() {
    assert_eq!(Solution::largest_rectangle_area(vec![4, 2, 0, 3, 2, 5, 6, 0, 0, 1]), 10);
}

#[test]
fn test_9() {
    assert_eq!(Solution::largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0]), 20);
}

#[test]
fn test_10() {
    assert_eq!(Solution::largest_rectangle_area(vec![3]), 3);
}

#[test]
fn test_11() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
}

#[test]
fn test_12() {
    assert_eq!(Solution::largest_rectangle_area(vec![10000, 10000, 10000, 10000]), 40000);
}

#[test]
fn test_13() {
    assert_eq!(Solution::largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0, 9]), 20);
}

#[test]
fn test_14() {
    assert_eq!(Solution::largest_rectangle_area(vec![0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::largest_rectangle_area(vec![0, 9, 0, 8, 0, 7, 0, 6, 0, 5, 0, 4, 0, 3, 0, 2, 0, 1]), 9);
}

#[test]
fn test_16() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
}

#[test]
fn test_17() {
    assert_eq!(Solution::largest_rectangle_area(vec![3, 3, 3, 3, 3, 3, 3, 3, 3]), 27);
}

#[test]
fn test_18() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_19() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 4, 3, 2, 1]), 9);
}

#[test]
fn test_20() {
    assert_eq!(Solution::largest_rectangle_area(vec![3, 1, 3, 1, 3]), 5);
}

#[test]
fn test_21() {
    assert_eq!(Solution::largest_rectangle_area(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 30);
}

#[test]
fn test_22() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3]), 16);
}

#[test]
fn test_23() {
    assert_eq!(Solution::largest_rectangle_area(vec![10, 15, 10, 20, 25, 30, 20, 15, 10]), 90);
}

#[test]
fn test_24() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 4, 3, 2, 1, 2, 3, 4, 5]), 9);
}

#[test]
fn test_25() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 2, 1, 4, 3, 2, 1, 5, 4, 3, 2, 1]), 14);
}

#[test]
fn test_26() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 2, 1, 1, 2, 3, 2, 1, 1, 2, 3, 2, 1]), 15);
}

#[test]
fn test_27() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 60);
}

#[test]
fn test_28() {
    assert_eq!(Solution::largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0, 2]), 20);
}

#[test]
fn test_29() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5]), 9);
}

#[test]
fn test_30() {
    assert_eq!(Solution::largest_rectangle_area(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 30);
}

#[test]
fn test_31() {
    assert_eq!(Solution::largest_rectangle_area(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 30);
}

#[test]
fn test_32() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 25);
}

#[test]
fn test_33() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1]), 30);
}

#[test]
fn test_34() {
    assert_eq!(Solution::largest_rectangle_area(vec![10000, 9999, 9998, 9997, 9996, 9995, 9994, 9993, 9992, 9991]), 99910);
}

#[test]
fn test_35() {
    assert_eq!(Solution::largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0, 2, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 55);
}

#[test]
fn test_36() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 2, 5, 2, 5, 2, 5, 2, 5, 2]), 20);
}

#[test]
fn test_37() {
    assert_eq!(Solution::largest_rectangle_area(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 55);
}

#[test]
fn test_38() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 110);
}

#[test]
fn test_39() {
    assert_eq!(Solution::largest_rectangle_area(vec![6, 2, 5, 4, 5, 1, 6, 2, 3, 3, 2, 6, 5, 3, 2, 1]), 18);
}

#[test]
fn test_40() {
    assert_eq!(Solution::largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0, 9, 2, 5, 6, 4, 3, 7, 8, 9, 10, 11, 12]), 42);
}

#[test]
fn test_41() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 113);
}

#[test]
fn test_42() {
    assert_eq!(Solution::largest_rectangle_area(vec![10, 20, 10, 30, 10, 40, 10, 50, 10, 60, 10]), 110);
}

#[test]
fn test_43() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 66);
}

#[test]
fn test_44() {
    assert_eq!(Solution::largest_rectangle_area(vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 27, 24, 21, 18, 15, 12, 9, 6, 3]), 165);
}

#[test]
fn test_45() {
    assert_eq!(Solution::largest_rectangle_area(vec![10000, 0, 10000, 0, 10000, 0]), 10000);
}

#[test]
fn test_46() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 55);
}

#[test]
fn test_47() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 3, 5, 6, 7, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1]), 28);
}

#[test]
fn test_48() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 30);
}

#[test]
fn test_49() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 3, 2, 1, 5, 4, 2, 1]), 8);
}

#[test]
fn test_50() {
    assert_eq!(Solution::largest_rectangle_area(vec![4, 2, 0, 3, 2, 5]), 6);
}

#[test]
fn test_51() {
    assert_eq!(Solution::largest_rectangle_area(vec![100, 200, 300, 400, 300, 200, 100]), 1000);
}

#[test]
fn test_52() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2, 1, 2, 1, 2, 1, 2, 1]), 10);
}

#[test]
fn test_53() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 1, 3, 2, 1, 4, 3, 2, 1, 5, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1, 7, 6, 5, 4, 3, 2, 1]), 28);
}

#[test]
fn test_54() {
    assert_eq!(Solution::largest_rectangle_area(vec![6, 2, 5, 4, 5, 1, 6, 3]), 12);
}

#[test]
fn test_55() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3, 2, 1, 5, 6, 2, 3]), 12);
}

#[test]
fn test_56() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
}

#[test]
fn test_57() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3, 1, 4, 2, 1]), 10);
}

#[test]
fn test_58() {
    assert_eq!(Solution::largest_rectangle_area(vec![0, 1, 0, 1, 0, 1, 0]), 1);
}

#[test]
fn test_59() {
    assert_eq!(Solution::largest_rectangle_area(vec![4, 2, 0, 3, 2, 5, 3, 1]), 8);
}

#[test]
fn test_60() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 5, 1, 5, 5]), 10);
}

#[test]
fn test_61() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10, 0]), 10);
}

#[test]
fn test_62() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 3, 2, 1, 2, 3, 1]), 7);
}

#[test]
fn test_63() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 6, 2, 8, 9, 1, 4, 3, 7, 10]), 16);
}

#[test]
fn test_64() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 5, 5, 5, 5, 0, 5, 5, 5, 5, 5]), 25);
}

#[test]
fn test_65() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 8, 6, 2, 7, 8, 5, 9, 5, 3, 8, 6, 7, 9, 5, 2, 8]), 34);
}

#[test]
fn test_66() {
    assert_eq!(Solution::largest_rectangle_area(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 1);
}

#[test]
fn test_67() {
    assert_eq!(Solution::largest_rectangle_area(vec![10000, 0, 10000, 0, 10000, 0, 10000, 0]), 10000);
}

#[test]
fn test_68() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3, 2, 5, 6, 7, 8, 9, 10]), 30);
}

#[test]
fn test_69() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21]), 66);
}

#[test]
fn test_70() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]), 21);
}

#[test]
fn test_71() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4]), 40);
}

#[test]
fn test_72() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]), 18);
}

#[test]
fn test_73() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 4, 3, 2, 1, 1, 2, 3, 4, 5]), 10);
}

#[test]
fn test_74() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10]), 20);
}

#[test]
fn test_75() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 0, 1, 0, 1, 0, 1, 0, 1]), 1);
}

#[test]
fn test_76() {
    assert_eq!(Solution::largest_rectangle_area(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 25);
}

#[test]
fn test_77() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1]), 26);
}

#[test]
fn test_78() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 3, 2, 1, 3, 2, 1]), 7);
}

#[test]
fn test_79() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 5, 1, 7, 1, 1, 5, 2, 7, 6]), 12);
}

#[test]
fn test_80() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5]), 25);
}

#[test]
fn test_81() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]), 25);
}

#[test]
fn test_82() {
    assert_eq!(Solution::largest_rectangle_area(vec![6, 2, 5, 4, 5, 1, 6]), 12);
}

#[test]
fn test_83() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1]), 21);
}

#[test]
fn test_84() {
    assert_eq!(Solution::largest_rectangle_area(vec![5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 55);
}

#[test]
fn test_85() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]), 15);
}

#[test]
fn test_86() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 1);
}

#[test]
fn test_87() {
    assert_eq!(Solution::largest_rectangle_area(vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8]), 160);
}

#[test]
fn test_88() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 3, 2, 1, 2, 3, 1, 2, 3, 4, 3, 2, 1, 2, 3, 1]), 16);
}

#[test]
fn test_89() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]), 21);
}

#[test]
fn test_90() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]), 1);
}

#[test]
fn test_91() {
    assert_eq!(Solution::largest_rectangle_area(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 900, 800, 700, 600, 500, 400, 300, 200, 100]), 5500);
}

#[test]
fn test_92() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 0, 0, 0, 1, 0, 0, 0, 1]), 1);
}

#[test]
fn test_93() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 20);
}

#[test]
fn test_94() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), 1);
}

#[test]
fn test_95() {
    assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 17);
}

#[test]
fn test_96() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 3, 2, 3, 4, 3, 2, 3, 4, 5, 4, 3, 2, 3, 2, 1, 2, 3, 2, 1]), 28);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_sub_array_len(8, vec![1, 2, 3, 4, 5]), 2);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_sub_array_len(9, vec![4, 3, 3, 2, 1]), 3);
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_sub_array_len(100, vec![10, 20, 30, 40, 50]), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_sub_array_len(8, vec![2, 2, 2, 2, 2, 2, 2, 2]), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_sub_array_len(8, vec![3, 4, 3, 1, 1, 1, 1, 1]), 3);
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_sub_array_len(3, vec![1, 1, 1]), 3);
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_sub_array_len(15, vec![5, 5, 5, 5, 5]), 3);
}

#[test]
fn test_11() {
    assert_eq!(Solution::min_sub_array_len(5, vec![2, 3, 1, 1, 1, 1, 1]), 2);
}

#[test]
fn test_12() {
    assert_eq!(Solution::min_sub_array_len(15, vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8]), 2);
}

#[test]
fn test_13() {
    assert_eq!(Solution::min_sub_array_len(10, vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8]), 1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::min_sub_array_len(8, vec![2, 3, 4, 2, 3, 4, 2, 3, 4]), 3);
}

#[test]
fn test_15() {
    assert_eq!(Solution::min_sub_array_len(3, vec![1, 1, 1, 1, 1, 1, 1, 1]), 3);
}

#[test]
fn test_16() {
    assert_eq!(Solution::min_sub_array_len(3, vec![1, 1, 1, 1, 1]), 3);
}

#[test]
fn test_17() {
    assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_18() {
    assert_eq!(Solution::min_sub_array_len(6, vec![1, 2, 3, 4, 5]), 2);
}

#[test]
fn test_19() {
    assert_eq!(Solution::min_sub_array_len(50, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 50);
}

#[test]
fn test_20() {
    assert_eq!(Solution::min_sub_array_len(1, vec![10000, 9999, 9998, 9997, 9996, 9995, 9994, 9993, 9992, 9991]), 1);
}

#[test]
fn test_21() {
    assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 1);
}

#[test]
fn test_22() {
    assert_eq!(Solution::min_sub_array_len(3, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 3);
}

#[test]
fn test_23() {
    assert_eq!(Solution::min_sub_array_len(100, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 6);
}

#[test]
fn test_24() {
    assert_eq!(Solution::min_sub_array_len(1000, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::min_sub_array_len(20, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 3);
}

#[test]
fn test_26() {
    assert_eq!(Solution::min_sub_array_len(5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_27() {
    assert_eq!(Solution::min_sub_array_len(55, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 3);
}

#[test]
fn test_28() {
    assert_eq!(Solution::min_sub_array_len(250, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 50);
}

#[test]
fn test_29() {
    assert_eq!(Solution::min_sub_array_len(20, vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 10);
}

#[test]
fn test_30() {
    assert_eq!(Solution::min_sub_array_len(100, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_31() {
    assert_eq!(Solution::min_sub_array_len(25, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 25);
}

#[test]
fn test_32() {
    assert_eq!(Solution::min_sub_array_len(50, vec![10, 2, 3, 8, 5, 10, 2, 1, 3, 5, 7, 8]), 10);
}

#[test]
fn test_33() {
    assert_eq!(Solution::min_sub_array_len(500, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 0);
}

#[test]
fn test_34() {
    assert_eq!(Solution::min_sub_array_len(25, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 2);
}

#[test]
fn test_35() {
    assert_eq!(Solution::min_sub_array_len(18, vec![1, 2, 3, 4, 5, 6, 7]), 3);
}

#[test]
fn test_36() {
    assert_eq!(Solution::min_sub_array_len(99, vec![50, 25, 20, 5, 10, 10, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 4);
}

#[test]
fn test_37() {
    assert_eq!(Solution::min_sub_array_len(25, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 5]), 5);
}

#[test]
fn test_38() {
    assert_eq!(Solution::min_sub_array_len(150, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 4);
}

#[test]
fn test_39() {
    assert_eq!(Solution::min_sub_array_len(1000, vec![100, 200, 300, 400, 50, 60, 70, 80, 90, 10, 20, 30, 40, 50, 100, 200, 300, 400, 50, 60, 70, 80, 90, 10, 20, 30, 40, 50]), 4);
}

#[test]
fn test_40() {
    assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_41() {
    assert_eq!(Solution::min_sub_array_len(200, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 8);
}

#[test]
fn test_42() {
    assert_eq!(Solution::min_sub_array_len(100, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 4);
}

#[test]
fn test_43() {
    assert_eq!(Solution::min_sub_array_len(25, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 4);
}

#[test]
fn test_44() {
    assert_eq!(Solution::min_sub_array_len(30, vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), 10);
}

#[test]
fn test_45() {
    assert_eq!(Solution::min_sub_array_len(5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1);
}

#[test]
fn test_46() {
    assert_eq!(Solution::min_sub_array_len(10, vec![100, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1);
}

#[test]
fn test_47() {
    assert_eq!(Solution::min_sub_array_len(120, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51]), 3);
}

#[test]
fn test_48() {
    assert_eq!(Solution::min_sub_array_len(42, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42]), 1);
}

#[test]
fn test_49() {
    assert_eq!(Solution::min_sub_array_len(23, vec![2, 3, 1, 2, 4, 3, 5, 6, 7, 8, 9, 10]), 3);
}

#[test]
fn test_50() {
    assert_eq!(Solution::min_sub_array_len(100, vec![10, 20, 30, 40, 50, 60, 70, 80, 90]), 2);
}

#[test]
fn test_51() {
    assert_eq!(Solution::min_sub_array_len(100, vec![50, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20]), 4);
}

#[test]
fn test_52() {
    assert_eq!(Solution::min_sub_array_len(12, vec![2, 3, 1, 2, 4, 3, 5]), 3);
}

#[test]
fn test_53() {
    assert_eq!(Solution::min_sub_array_len(200, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_54() {
    assert_eq!(Solution::min_sub_array_len(5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 1);
}

#[test]
fn test_55() {
    assert_eq!(Solution::min_sub_array_len(100, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 0);
}

#[test]
fn test_56() {
    assert_eq!(Solution::min_sub_array_len(100000, vec![10000, 20000, 30000, 40000, 1000, 2000, 3000, 4000, 5000, 1000, 2000, 3000, 4000, 5000, 1000, 2000, 3000, 4000, 5000, 1000, 2000, 3000, 4000, 5000, 1000, 2000, 3000, 4000, 5000]), 4);
}

#[test]
fn test_57() {
    assert_eq!(Solution::min_sub_array_len(7, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 7);
}

#[test]
fn test_58() {
    assert_eq!(Solution::min_sub_array_len(50, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 4);
}

#[test]
fn test_59() {
    assert_eq!(Solution::min_sub_array_len(120, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 5);
}

#[test]
fn test_60() {
    assert_eq!(Solution::min_sub_array_len(25, vec![25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25]), 1);
}

#[test]
fn test_61() {
    assert_eq!(Solution::min_sub_array_len(500, vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 0);
}

#[test]
fn test_62() {
    assert_eq!(Solution::min_sub_array_len(5, vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 3);
}

#[test]
fn test_63() {
    assert_eq!(Solution::min_sub_array_len(50, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 50);
}

#[test]
fn test_64() {
    assert_eq!(Solution::min_sub_array_len(7, vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]), 5);
}

#[test]
fn test_65() {
    assert_eq!(Solution::min_sub_array_len(500, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]), 12);
}

#[test]
fn test_66() {
    assert_eq!(Solution::min_sub_array_len(100, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 1);
}

#[test]
fn test_67() {
    assert_eq!(Solution::min_sub_array_len(45, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 6);
}

#[test]
fn test_68() {
    assert_eq!(Solution::min_sub_array_len(20, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 4);
}

#[test]
fn test_69() {
    assert_eq!(Solution::min_sub_array_len(20, vec![5, 1, 3, 5, 2, 1, 4]), 7);
}

#[test]
fn test_70() {
    assert_eq!(Solution::min_sub_array_len(1000, vec![250, 250, 250, 250]), 4);
}

#[test]
fn test_71() {
    assert_eq!(Solution::min_sub_array_len(40, vec![10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20]), 3);
}

#[test]
fn test_72() {
    assert_eq!(Solution::min_sub_array_len(30, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 6);
}

#[test]
fn test_73() {
    assert_eq!(Solution::min_sub_array_len(120, vec![10, 20, 30, 40, 50, 10, 20, 30, 40, 50]), 3);
}

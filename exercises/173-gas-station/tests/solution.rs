include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), 3);
}

#[test]
fn test_3() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]), 4);
}

#[test]
fn test_4() {
    assert_eq!(Solution::can_complete_circuit(vec![5, 1, 2, 3, 4], vec![4, 5, 1, 2, 3]), 2);
}

#[test]
fn test_5() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]), 3);
}

#[test]
fn test_6() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::can_complete_circuit(vec![5, 8, 2, 8], vec![6, 5, 6, 6]), 3);
}

#[test]
fn test_8() {
    assert_eq!(Solution::can_complete_circuit(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]), 4);
}

#[test]
fn test_9() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50], vec![15, 25, 35, 45, 55]), -1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 1, 1], vec![1, 2, 2]), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 1, 2, 3, 4], vec![3, 4, 5, 1, 2]), 4);
}

#[test]
fn test_12() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 3, 4], vec![3, 4, 4]), -1);
}

#[test]
fn test_13() {
    assert_eq!(Solution::can_complete_circuit(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100], vec![101, 99, 101, 99, 101, 99, 101, 99, 101, 99]), 9);
}

#[test]
fn test_14() {
    assert_eq!(Solution::can_complete_circuit(vec![100, 50, 20, 5, 10], vec![10, 20, 50, 100, 5]), 4);
}

#[test]
fn test_15() {
    assert_eq!(Solution::can_complete_circuit(vec![20, 20, 20, 20, 20, 20, 20, 20, 20, 20], vec![19, 19, 19, 19, 19, 19, 19, 19, 19, 20]), 9);
}

#[test]
fn test_16() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10], vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9]), 9);
}

#[test]
fn test_17() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 1]), -1);
}

#[test]
fn test_18() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 1);
}

#[test]
fn test_19() {
    assert_eq!(Solution::can_complete_circuit(vec![100, 50, 25, 75, 200, 300], vec![200, 150, 200, 20, 100, 50]), 3);
}

#[test]
fn test_20() {
    assert_eq!(Solution::can_complete_circuit(vec![50, 60, 70, 80, 90, 100, 110, 120, 130, 140], vec![100, 110, 120, 130, 140, 150, 160, 170, 180, 190]), -1);
}

#[test]
fn test_21() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50], vec![50, 10, 20, 30, 40]), 1);
}

#[test]
fn test_22() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]), -1);
}

#[test]
fn test_23() {
    assert_eq!(Solution::can_complete_circuit(vec![6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6]), 0);
}

#[test]
fn test_24() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 30, 20, 10, 5, 40, 10, 30, 20, 10], vec![15, 25, 10, 15, 10, 30, 10, 20, 15, 15]), 7);
}

#[test]
fn test_25() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 1]), 9);
}

#[test]
fn test_26() {
    assert_eq!(Solution::can_complete_circuit(vec![5, 8, 2, 8, 4], vec![6, 5, 6, 6, 8]), -1);
}

#[test]
fn test_27() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 3, 4, 4, 1, 1, 2, 2], vec![3, 3, 3, 3, 2, 2, 2, 2]), 7);
}

#[test]
fn test_28() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 10, 10, 10, 1], vec![10, 1, 1, 1, 10]), 2);
}

#[test]
fn test_29() {
    assert_eq!(Solution::can_complete_circuit(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 9);
}

#[test]
fn test_30() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 19);
}

#[test]
fn test_31() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 3, 4, 4, 5, 5, 6, 6, 7, 7], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 9);
}

#[test]
fn test_32() {
    assert_eq!(Solution::can_complete_circuit(vec![100, 200, 300, 400, 500], vec![500, 100, 200, 300, 400]), 1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::can_complete_circuit(vec![30, 25, 20, 15, 10, 5, 0], vec![5, 10, 15, 20, 25, 30, 1]), -1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 10);
}

#[test]
fn test_35() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50], vec![9, 19, 29, 39, 49]), 4);
}

#[test]
fn test_36() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50], vec![5, 10, 15, 20, 25]), 4);
}

#[test]
fn test_37() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 8);
}

#[test]
fn test_38() {
    assert_eq!(Solution::can_complete_circuit(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 5);
}

#[test]
fn test_40() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![90, 80, 70, 60, 50, 40, 30, 20, 10, 150]), -1);
}

#[test]
fn test_41() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4, 5, 1, 6], vec![3, 4, 5, 1, 6, 2]), 5);
}

#[test]
fn test_42() {
    assert_eq!(Solution::can_complete_circuit(vec![15, 20, 5, 20, 15, 20, 15, 20, 15, 20], vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 9);
}

#[test]
fn test_43() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5], vec![5, 3, 5, 3, 9, 2, 6, 5, 3, 5, 3]), -1);
}

#[test]
fn test_44() {
    assert_eq!(Solution::can_complete_circuit(vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100], vec![99, 99, 99, 99, 99, 99, 99, 99, 99, 100]), 9);
}

#[test]
fn test_45() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 0, 10, 0, 10], vec![0, 10, 0, 10, 0]), 4);
}

#[test]
fn test_46() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 100]), -1);
}

#[test]
fn test_47() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50], vec![40, 30, 20, 10, 5]), 4);
}

#[test]
fn test_48() {
    assert_eq!(Solution::can_complete_circuit(vec![15, 10, 10, 10, 10], vec![10, 10, 10, 15, 10]), 4);
}

#[test]
fn test_49() {
    assert_eq!(Solution::can_complete_circuit(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5], vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 5]), 9);
}

#[test]
fn test_50() {
    assert_eq!(Solution::can_complete_circuit(vec![8, 6, 7, 4, 5], vec![5, 6, 7, 3, 4]), 4);
}

#[test]
fn test_51() {
    assert_eq!(Solution::can_complete_circuit(vec![5, 8, 2, 8, 4, 7, 6, 6, 1, 4], vec![4, 4, 5, 7, 1, 5, 4, 4, 5, 4]), 9);
}

#[test]
fn test_52() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], vec![150, 140, 130, 120, 110, 100, 90, 80, 70, 60, 50, 40, 30, 20, 10]), 8);
}

#[test]
fn test_53() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2]), -1);
}

#[test]
fn test_54() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 10, 5, 15], vec![20, 15, 10, 5, 15, 10]), 2);
}

#[test]
fn test_55() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 9);
}

#[test]
fn test_56() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
}

#[test]
fn test_57() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 0);
}

#[test]
fn test_58() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 3, 4, 5, 1, 2], vec![4, 1, 5, 1, 2, 3]), 3);
}

#[test]
fn test_59() {
    assert_eq!(Solution::can_complete_circuit(vec![20, 30, 10, 40, 50, 60], vec![50, 10, 60, 10, 20, 30]), 4);
}

#[test]
fn test_60() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 5, 7, 2, 8, 10, 6, 4, 12, 9], vec![4, 4, 8, 5, 5, 6, 7, 8, 3, 2]), 9);
}

#[test]
fn test_61() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 20]), -1);
}

#[test]
fn test_62() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]), 19);
}

#[test]
fn test_63() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 10, 20, 10, 20, 10, 20, 10, 20], vec![11, 9, 11, 9, 11, 9, 11, 9, 11, 9]), 9);
}

#[test]
fn test_64() {
    assert_eq!(Solution::can_complete_circuit(vec![5, 5, 5, 5, 5, 5, 5], vec![6, 6, 6, 6, 6, 6, 6]), -1);
}

#[test]
fn test_65() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1, 2, 3, 4]), 11);
}

#[test]
fn test_66() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 20]), 7);
}

#[test]
fn test_67() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 2, 3, 4, 5, 6, 7, 8, 9]), 1);
}

#[test]
fn test_68() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 2, 2, 2, 2], vec![1, 1, 1, 1, 1]), 4);
}

#[test]
fn test_69() {
    assert_eq!(Solution::can_complete_circuit(vec![4, 3, 2, 1, 5, 6, 7], vec![5, 4, 3, 2, 6, 7, 1]), 6);
}

#[test]
fn test_70() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4, 5, 6, 7, 8, 9, 10], vec![3, 4, 5, 6, 7, 8, 9, 10, 2]), 8);
}

#[test]
fn test_71() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 5);
}

#[test]
fn test_72() {
    assert_eq!(Solution::can_complete_circuit(vec![30, 20, 10, 40, 50, 20, 10, 60, 70, 80], vec![20, 30, 40, 50, 60, 70, 80, 90, 10, 10]), -1);
}

#[test]
fn test_73() {
    assert_eq!(Solution::can_complete_circuit(vec![50, 100, 150, 200, 250, 300, 350, 400, 450, 500], vec![400, 450, 500, 50, 100, 150, 200, 250, 300, 350]), 3);
}

#[test]
fn test_74() {
    assert_eq!(Solution::can_complete_circuit(vec![100, 20, 10, 15, 5], vec![70, 15, 10, 10, 5]), 4);
}

#[test]
fn test_75() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 6, 3, 4, 5, 2, 8], vec![2, 5, 4, 3, 6, 1, 5]), 6);
}

#[test]
fn test_76() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 6, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 7);
}

#[test]
fn test_77() {
    assert_eq!(Solution::can_complete_circuit(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![110, 9, 19, 29, 39, 49, 59, 69, 79, 89]), -1);
}

#[test]
fn test_78() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 9);
}

#[test]
fn test_79() {
    assert_eq!(Solution::can_complete_circuit(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0]), 9);
}

#[test]
fn test_80() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 5, 7, 3, 8, 4, 6], vec![6, 2, 4, 8, 7, 3, 5]), 1);
}

#[test]
fn test_81() {
    assert_eq!(Solution::can_complete_circuit(vec![15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15], vec![14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14]), 19);
}

#[test]
fn test_82() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 1]), -1);
}

#[test]
fn test_83() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 5);
}

#[test]
fn test_84() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 2, 6, 5, 10, 1], vec![6, 5, 4, 3, 1, 2]), 4);
}

#[test]
fn test_85() {
    assert_eq!(Solution::can_complete_circuit(vec![20, 20, 20, 20, 20], vec![15, 15, 15, 15, 15]), 4);
}

#[test]
fn test_86() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3], vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), 9);
}

#[test]
fn test_87() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 4, 5, 3, 1], vec![3, 4, 3, 2, 2]), 2);
}

#[test]
fn test_88() {
    assert_eq!(Solution::can_complete_circuit(vec![3, 2, 6, 5, 4], vec![2, 3, 4, 5, 3]), 4);
}

#[test]
fn test_89() {
    assert_eq!(Solution::can_complete_circuit(vec![20, 30, 10, 40, 50], vec![35, 15, 25, 20, 40]), 3);
}

#[test]
fn test_90() {
    assert_eq!(Solution::can_complete_circuit(vec![8, 1, 3, 5, 2, 6, 7], vec![7, 2, 5, 4, 1, 3, 6]), 5);
}

#[test]
fn test_91() {
    assert_eq!(Solution::can_complete_circuit(vec![50, 30, 10, 20, 40], vec![30, 40, 15, 25, 30]), 4);
}

#[test]
fn test_92() {
    assert_eq!(Solution::can_complete_circuit(vec![5, 5, 5, 5, 5], vec![4, 4, 4, 4, 4]), 4);
}

#[test]
fn test_93() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1]), 14);
}

#[test]
fn test_94() {
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 6, 5, 1, 3, 4], vec![3, 4, 3, 1, 2, 4, 3]), 3);
}

#[test]
fn test_95() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 3, 5, 7, 9], vec![9, 7, 5, 3, 1]), 3);
}

#[test]
fn test_96() {
    assert_eq!(Solution::can_complete_circuit(vec![1000, 1000, 1000, 1000, 1000], vec![999, 999, 999, 999, 5000]), -1);
}

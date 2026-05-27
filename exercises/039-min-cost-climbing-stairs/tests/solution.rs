include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 999, 999, 999]), 1998);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 4, 5]), 6);
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]), 6);
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 25);
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 0, 0, 0]), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2]), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 999, 999, 999, 999, 999]), 2997);
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 4, 5]), 6);
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20, 25, 30]), 40);
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15]), 10);
}

#[test]
fn test_11() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 20, 10, 5]), 20);
}

#[test]
fn test_12() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 5, 20, 25, 30, 35, 40]), 65);
}

#[test]
fn test_13() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 20, 15, 20, 30]), 40);
}

#[test]
fn test_14() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15]), 10);
}

#[test]
fn test_15() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 8, 6, 3, 4, 2, 8, 5, 1]), 18);
}

#[test]
fn test_16() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 999]), 999);
}

#[test]
fn test_17() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
}

#[test]
fn test_18() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20, 10, 15, 20]), 40);
}

#[test]
fn test_19() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_20() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 100, 100, 1]), 101);
}

#[test]
fn test_21() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82, 81]), 900);
}

#[test]
fn test_22() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20, 30, 40, 50, 60, 70, 80, 90]), 210);
}

#[test]
fn test_23() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![3, 5, 2, 1, 8, 9, 3, 5, 6, 2, 1, 4, 7, 3, 2, 8, 6, 4, 1, 7]), 38);
}

#[test]
fn test_24() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![50, 10, 20, 100, 30, 40, 10, 20, 30, 40]), 100);
}

#[test]
fn test_25() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![3, 2, 4, 3, 2, 5, 1, 2, 4, 3, 2, 5, 1, 2, 4]), 17);
}

#[test]
fn test_26() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1]), 13);
}

#[test]
fn test_27() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 8, 3, 12, 2, 7, 6, 10, 1, 4, 9, 11, 13, 14, 15]), 46);
}

#[test]
fn test_28() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 100, 2, 100, 3, 100, 4, 100, 5, 100, 6, 100, 7, 100, 8]), 36);
}

#[test]
fn test_29() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90]), 450);
}

#[test]
fn test_30() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 2]), 13);
}

#[test]
fn test_31() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 8, 2, 3, 7, 1, 4, 6, 9, 0, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 92);
}

#[test]
fn test_32() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![50, 10, 5, 100, 20, 15, 10, 5, 20, 10, 5, 100, 20, 15, 10, 5, 20, 10, 5, 100, 20, 15, 10, 5, 20]), 150);
}

#[test]
fn test_33() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140]), 495);
}

#[test]
fn test_34() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999]), 8992);
}

#[test]
fn test_35() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![3, 2, 10, 2, 3, 5, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]), 52);
}

#[test]
fn test_36() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 0, 999, 0, 999, 0, 999, 0, 999, 0]), 0);
}

#[test]
fn test_37() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 999, 1, 999, 999, 2, 999, 999, 3, 999]), 3001);
}

#[test]
fn test_38() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999]), 0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1]), 12);
}

#[test]
fn test_40() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 8, 10, 2, 9, 1, 3, 7, 4, 6, 11, 12]), 29);
}

#[test]
fn test_41() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0]), 999);
}

#[test]
fn test_42() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90]), 475);
}

#[test]
fn test_43() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 8, 3, 7, 2, 8, 3, 7, 2, 8, 3, 7, 2, 8, 3, 7, 2]), 25);
}

#[test]
fn test_44() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), 156);
}

#[test]
fn test_45() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]), 125);
}

#[test]
fn test_46() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500]), 5600);
}

#[test]
fn test_47() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![30, 5, 20, 10, 25, 15, 40, 35, 50, 45, 60, 55, 70, 65, 80, 75]), 305);
}

#[test]
fn test_48() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), 60);
}

#[test]
fn test_49() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 50);
}

#[test]
fn test_50() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 100, 100, 1, 100, 100, 1, 100, 100, 1]), 302);
}

#[test]
fn test_51() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 100, 2, 99, 3, 98, 4, 97, 5, 96]), 15);
}

#[test]
fn test_52() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80]), 315);
}

#[test]
fn test_53() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 3, 8, 6, 7, 2, 4, 9, 1, 10]), 16);
}

#[test]
fn test_54() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55]), 150);
}

#[test]
fn test_55() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 998, 997, 996, 995, 994, 993, 992, 991, 990]), 4970);
}

#[test]
fn test_56() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999]), 75924);
}

#[test]
fn test_57() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![50, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), 570);
}

#[test]
fn test_58() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 2, 7, 4, 9, 6, 11, 8, 13, 10, 15, 12, 17, 14, 19]), 56);
}

#[test]
fn test_59() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0]), 0);
}

#[test]
fn test_60() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 100);
}

#[test]
fn test_61() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984, 983, 982, 981, 980]), 9890);
}

#[test]
fn test_62() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1, 2, 3, 4, 5]), 2507);
}

#[test]
fn test_63() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 999, 0, 0, 999, 999, 0, 0, 999, 999, 0, 0]), 2997);
}

#[test]
fn test_64() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_65() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 56);
}

#[test]
fn test_66() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), 560);
}

#[test]
fn test_67() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![400, 5, 3, 10, 1, 1, 1, 1, 1, 1, 1, 1]), 12);
}

#[test]
fn test_68() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![20, 1, 3, 2, 4, 2, 5, 3, 6, 4, 7, 5, 8, 6, 9, 7, 10]), 30);
}

#[test]
fn test_69() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100]), 2500);
}

#[test]
fn test_70() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1, 1, 1, 1, 1]), 2503);
}

#[test]
fn test_71() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20, 10, 15, 20]), 760);
}

#[test]
fn test_72() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0, 999, 0]), 0);
}

#[test]
fn test_73() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![20, 1, 5, 10, 1, 1, 5, 10, 1, 1, 5, 10, 1, 1, 5, 10, 1, 1, 5, 10]), 30);
}

#[test]
fn test_74() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 1000, 1, 1000, 1, 1000, 1, 1000, 1, 1000]), 5);
}

#[test]
fn test_75() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 100);
}

#[test]
fn test_76() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 36);
}

#[test]
fn test_77() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100]), 7);
}

#[test]
fn test_78() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1]), 10);
}

#[test]
fn test_79() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 8, 2, 3, 6, 1, 4, 7, 9, 10]), 24);
}

#[test]
fn test_80() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 0);
}

#[test]
fn test_81() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100]), 495);
}

#[test]
fn test_82() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999]), 12);
}

#[test]
fn test_83() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 56);
}

#[test]
fn test_84() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 75);
}

#[test]
fn test_85() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![2, 1, 4, 5, 6, 3, 2, 5, 4, 3, 1, 2, 4, 3, 2, 1, 4, 5, 6, 3, 2, 5, 4, 3, 1, 2, 4, 3, 2, 1, 4, 5, 6, 3, 2, 5, 4, 3, 1, 2, 4, 3, 2, 1, 4, 5, 6, 3, 2, 5, 4, 3, 1, 2, 4, 3, 2, 1, 4, 5, 6, 3, 2, 5, 4, 3, 1, 2, 4, 3, 2, 1, 4, 5, 6, 3, 2, 5, 4, 3, 1, 2, 4, 3, 2, 1, 4, 5, 6, 3, 2, 5, 4, 3, 1, 2, 4, 3, 2, 1, 4, 5, 6, 3, 2, 5, 4, 3, 1, 2, 4]), 165);
}

#[test]
fn test_86() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 20, 30, 40, 50, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 65);
}

#[test]
fn test_87() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 150, 100, 150, 100, 150, 100, 150, 100, 150]), 500);
}

#[test]
fn test_88() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]), 225);
}

#[test]
fn test_89() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![3, 2, 1, 5, 4, 1, 3, 2, 1, 5, 4, 1, 3, 2, 1, 5, 4, 1]), 23);
}

#[test]
fn test_90() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 20, 10, 20, 10, 20, 10, 20, 10, 20]), 50);
}

#[test]
fn test_91() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 156);
}

#[test]
fn test_92() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10, 20, 10]), 130);
}

#[test]
fn test_93() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 200, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100]), 106);
}

#[test]
fn test_94() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100]), 14);
}

#[test]
fn test_95() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![100, 50, 25, 12, 6, 3, 1, 0, 0, 1, 3, 6, 12, 25, 50, 100, 150, 200, 250, 300]), 530);
}

#[test]
fn test_96() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_97() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![4, 2, 6, 7, 8, 1, 3, 5, 10, 2, 6, 7, 8, 1, 3, 5]), 28);
}

#[test]
fn test_98() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999, 1, 999]), 10);
}

#[test]
fn test_99() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040]), 832039);
}

#[test]
fn test_100() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50]), 2150);
}

#[test]
fn test_101() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![999, 999, 999, 999, 999, 0, 0, 0, 0, 0]), 1998);
}

#[test]
fn test_102() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::asteroid_collision(vec![1, -1, 2, -2, 3, -3]), vec![]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::asteroid_collision(vec![-2, -2, 1, 1]), vec![-2, -2, 1, 1]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5]), vec![1, 2, 3, 4]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::asteroid_collision(vec![-1000, 1000]), vec![-1000, 1000]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::asteroid_collision(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, -4, -3, -2, -1]), vec![-4, -3, -2, -1]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::asteroid_collision(vec![5, -1, 5, -1]), vec![5, 5]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::asteroid_collision(vec![-2, -1, 1, 2]), vec![-2, -1, 1, 2]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, -3, -2, -1]), vec![]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::asteroid_collision(vec![-2, -2, 1, -2]), vec![-2, -2, -2]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, -1, -2, -3]), vec![1, 2]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::asteroid_collision(vec![1, -2, -2, -2]), vec![-2, -2, -2]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, -4, -5]), vec![-4, -5]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::asteroid_collision(vec![1000, -1000]), vec![]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::asteroid_collision(vec![-3, 3, 1, -1, -2, 2]), vec![-3, 3, 2]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::asteroid_collision(vec![10, 1, 2, -3, -4, -5, 6, 7, 8, 9]), vec![10, 6, 7, 8, 9]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::asteroid_collision(vec![-2, -1, 1, 2, 3, 4]), vec![-2, -1, 1, 2, 3, 4]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::asteroid_collision(vec![5, -10, 10, -5]), vec![-10, 10]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::asteroid_collision(vec![-5, -4, -3, -2, -1, 1, 2, 3, 4, 5]), vec![-5, -4, -3, -2, -1, 1, 2, 3, 4, 5]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::asteroid_collision(vec![6, 2, -3, -4, -5]), vec![6]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -10, -5]), vec![]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, -3, -4]), vec![-3, -4]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::asteroid_collision(vec![10, 2, -3, 5, -1, -4, 6, -6]), vec![10, 5]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::asteroid_collision(vec![-7, 7, -8, 8, -9, 9, 10, -10, 11, -11]), vec![-7, -8, -9, 9]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::asteroid_collision(vec![100, -100, 50, -50, 25, -25, 10, -10, 5, -5, 3, -3, 2, -2, 1, -1]), vec![]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::asteroid_collision(vec![50, 40, 30, 20, 10, -10, -20, -30, -40, -50]), vec![]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::asteroid_collision(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15]), vec![-2, -4, -6, -8, -10, -12, -14, 15]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, -6, -7, -8, -9, -10]), vec![-6, -7, -8, -9, -10]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::asteroid_collision(vec![10, 20, 30, 40, 50, -15, -25, -35, -45, -55, 60, 70]), vec![-55, 60, 70]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::asteroid_collision(vec![5, 5, 5, 5, 5, -5, -5, -5, -5, -5]), vec![]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::asteroid_collision(vec![3, 3, 3, 3, -3, -3, -3, -3, 3, 3, 3, 3, -3, -3, -3, -3]), vec![]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::asteroid_collision(vec![50, -40, 30, -20, 10, -5, 0, 5, -10, 20, -30, 40, -50]), vec![]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::asteroid_collision(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), vec![]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::asteroid_collision(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, -29, -27, -25, -23, -21, -19, -17, -15, -13, -11, -9, -7, -5, -3, -1]), vec![]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::asteroid_collision(vec![-50, 50, -40, 40, -30, 30, -20, 20, -10, 10]), vec![-50, 50, 40, 30, 20, 10]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::asteroid_collision(vec![100, -90, -80, 70, -60, 50, -40, 30, -20, 10]), vec![100, 70, 50, 30, 10]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::asteroid_collision(vec![-5, 5, -10, 10, -15, 15, -20, 20, 25]), vec![-5, -10, -15, -20, 20, 25]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::asteroid_collision(vec![10, 20, 30, 40, 50, -50, -40, -30, -20, -10]), vec![]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::asteroid_collision(vec![1, -1, -1, 1, 2, -2, -2, 2, 3, -3, -3, 3, 4, -4, -4, 4]), vec![-1, -2, -3, -4, 4]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::asteroid_collision(vec![-10, -20, -30, -40, -50, 50, 40, 30, 20, 10]), vec![-10, -20, -30, -40, -50, 50, 40, 30, 20, 10]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::asteroid_collision(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5]), vec![]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::asteroid_collision(vec![-1, -2, -3, 1, 2, 3, -4, -5, 4, 5]), vec![-1, -2, -3, -4, -5, 4, 5]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::asteroid_collision(vec![5, 15, 25, 35, 45, 55, -5, -15, -25, -35, -45, -55]), vec![5, 15, 25, 35, 45]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::asteroid_collision(vec![7, 6, 5, 4, 3, 2, 1, -1, -2, -3, -4, -5, -6, -7]), vec![]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::asteroid_collision(vec![100, -200, 50, -50, 300, -150, 250, -100]), vec![-200, 300, 250]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::asteroid_collision(vec![-10, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5]), vec![-10]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, 15, 20, -20, -15, -10, -5]), vec![]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::asteroid_collision(vec![8, -2, 9, -3, 4, -1, 7, -5, 6, -4]), vec![8, 9, 4, 7, 6]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -10, -9, -8, -7, -6, -5, -4, -3, -2, -1]), vec![]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::asteroid_collision(vec![5, 5, 5, 5, -5, -5, -5, -5, 5, 5]), vec![5, 5]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::asteroid_collision(vec![5, 7, 3, -7, 4, -4, 2, -2, 1, -1]), vec![5]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::asteroid_collision(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, -1, -2, -3, -4, -5, -6, -7, -8, -9]), vec![]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::asteroid_collision(vec![-5, 5, -10, 10, -15, 15, -20, 20, -25, 25]), vec![-5, -10, -15, -20, -25, 25]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::asteroid_collision(vec![2, -2, 4, -4, 6, -6, 8, -8, 10, -10, 12, -12, 14, -14, 16, -16]), vec![]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::asteroid_collision(vec![1, 3, 5, 7, 9, -9, -7, -5, -3, -1, 2, 4, 6, 8, 10, -10, -8, -6, -4, -2]), vec![]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::asteroid_collision(vec![-1, -2, -3, 4, 5, 6, -7, -8, 9, 10, -10, 11, -11, 12, -12]), vec![-1, -2, -3, -7, -8, 9]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::asteroid_collision(vec![-10, 10, -9, 9, -8, 8, -7, 7, -6, 6, -5, 5, -4, 4, -3, 3, -2, 2, -1, 1]), vec![-10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::asteroid_collision(vec![50, 40, 30, 20, 10, -10, -20, -30, -40, -50, 60, 70, 80, 90]), vec![60, 70, 80, 90]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::asteroid_collision(vec![5, -3, 7, -2, 1, -10, 15, -15, 20]), vec![-10, 20]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::asteroid_collision(vec![2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10, -10]), vec![]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::asteroid_collision(vec![5, -10, 15, -20, 25, -30, 35, -40]), vec![-10, -20, -30, -40]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::asteroid_collision(vec![5, -5, 5, -5, 5, -5, 5, -5, 5, -5]), vec![]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::asteroid_collision(vec![5, -5, 5, -5, 5, -5]), vec![]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::asteroid_collision(vec![100, -99, 98, -97, 96, -95, 94, -93, 92, -91, 90]), vec![100, 98, 96, 94, 92, 90]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::asteroid_collision(vec![-10, -20, -30, 30, 20, 10, -10, -20, -30]), vec![-10, -20, -30]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::asteroid_collision(vec![10, 20, -15, -20, 5, 15, -10, 25, -30]), vec![-30]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::asteroid_collision(vec![10, 20, 30, -5, -15, -25, -35, 40, -45, 50]), vec![-35, -45, 50]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::asteroid_collision(vec![-10, -20, -30, 10, 20, 30, -15, -25, 5, 15, -5, -10, -20, 25, 35, -40, 40, -45, 45]), vec![-10, -20, -30, -40, -45, 45]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::asteroid_collision(vec![-9, -8, -7, -6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9]), vec![-9, -8, -7, -6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -10, -5, 20, -20, 15, -15, 25, -25]), vec![]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::asteroid_collision(vec![10, -10, 20, -20, 30, -30, 40, -40, 50, -50]), vec![]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::asteroid_collision(vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10, -11, 12]), vec![-1, -3, -5, -7, -9, -11, 12]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, -5, -4, -3, -2, -1]), vec![]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::asteroid_collision(vec![10, -1, 9, -2, 8, -3, 7, -4, 6, -5]), vec![10, 9, 8, 7, 6]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::asteroid_collision(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, -10, -20, -30, -40, -50, -60, -70, -80, -90, -100]), vec![]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::asteroid_collision(vec![100, -99, 98, -97, 96, -95, 94, -93, 92, -91, 90, -89]), vec![100, 98, 96, 94, 92, 90]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, -9, -8, -7, -6, -5, -4, -3, -2, -1]), vec![]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::asteroid_collision(vec![1, 1, 1, 1, 1, -1, -1, -1, -1, -1]), vec![]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -5, 10, -10, 5, -5, 10]), vec![5, 10, 10]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::asteroid_collision(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), vec![]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, -5, -4, -3, -2, -1]), vec![]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::asteroid_collision(vec![100, 200, 300, -100, -200, -300, 400, -400, 500, -500]), vec![100, 200]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::asteroid_collision(vec![10, -20, 30, -40, 50, -60, 70, -80, 90, -100, 100, -90, 80, -70, 60, -50, 40, -30, 20, -10]), vec![-20, -40, -60, -80, -100, 100, 80, 60, 40, 20]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::asteroid_collision(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10, -10, 11, -11]), vec![]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::asteroid_collision(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20]), vec![]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::asteroid_collision(vec![100, -1, 99, -2, 98, -3, 97, -4, 96, -5, 95, -6, 94, -7, 93, -8, 92, -9, 91, -10]), vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::asteroid_collision(vec![10, -5, 5, -10, 10, -10, 5, -5, 2, -2]), vec![]);
}

#[test]
fn test_92() {
    assert_eq!(Solution::asteroid_collision(vec![-10, 20, -20, 30, -30, 40, -40, 50, -50, 60, -60]), vec![-10]);
}

#[test]
fn test_93() {
    assert_eq!(Solution::asteroid_collision(vec![100, -50, 50, -100, 200, -200, 300, -300, 400, -400]), vec![]);
}

#[test]
fn test_94() {
    assert_eq!(Solution::asteroid_collision(vec![1000, -1000, 500, -500, 250, -250, 125, -125, 63, -63, 31, -31, 15, -15, 7, -7, 3, -3, 1, -1]), vec![]);
}

#[test]
fn test_95() {
    assert_eq!(Solution::asteroid_collision(vec![1, -2, 2, -3, 3, -4, 4, -5, 5, -6, 6, -7, 7, -8, 8, -9, 9, -10, 10]), vec![-2, -3, -4, -5, -6, -7, -8, -9, -10, 10]);
}

#[test]
fn test_96() {
    assert_eq!(Solution::asteroid_collision(vec![-10, -20, -30, -40, -50, 50, 40, 30, 20, 10]), vec![-10, -20, -30, -40, -50, 50, 40, 30, 20, 10]);
}

#[test]
fn test_97() {
    assert_eq!(Solution::asteroid_collision(vec![100, 200, 300, 400, 500, -100, -200, -300, -400, -500, 600, 700, 800, 900, 1000]), vec![100, 200, 300, 400, 600, 700, 800, 900, 1000]);
}

#[test]
fn test_98() {
    assert_eq!(Solution::asteroid_collision(vec![10, 20, 30, 40, 50, -50, -40, -30, -20, -10]), vec![]);
}

#[test]
fn test_99() {
    assert_eq!(Solution::asteroid_collision(vec![-1, -2, -3, -4, -5, 5, 4, 3, 2, 1]), vec![-1, -2, -3, -4, -5, 5, 4, 3, 2, 1]);
}

#[test]
fn test_100() {
    assert_eq!(Solution::asteroid_collision(vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10, -11, 12, -13, 14, -15, 16]), vec![-1, -3, -5, -7, -9, -11, -13, -15, 16]);
}

#[test]
fn test_101() {
    assert_eq!(Solution::asteroid_collision(vec![5, -5, 5, -5, 5, -5, 5, -5, 5, -5, 5, -5, 5, -5, 5, -5]), vec![]);
}

#[test]
fn test_102() {
    assert_eq!(Solution::asteroid_collision(vec![10, -10, 5, 3, -3, -5, 8, 8, -8, 2, -2]), vec![8]);
}

#[test]
fn test_103() {
    assert_eq!(Solution::asteroid_collision(vec![10, -20, 30, -40, 50, -60, 70, -80, 90, -100, 110, -120, 130, -140, 150]), vec![-20, -40, -60, -80, -100, -120, -140, 150]);
}

#[test]
fn test_104() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, -3, -2, -1, 4, 5, -5, -4]), vec![]);
}

#[test]
fn test_105() {
    assert_eq!(Solution::asteroid_collision(vec![-1, -2, -3, -4, -5, 5, 4, 3, 2, 1]), vec![-1, -2, -3, -4, -5, 5, 4, 3, 2, 1]);
}

#[test]
fn test_106() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, 15, -10, 20, -15, -20, 25, -25, 30, -30, 35, -35]), vec![5, 10, 15]);
}

#[test]
fn test_107() {
    assert_eq!(Solution::asteroid_collision(vec![100, -50, 50, -25, 25, -10, 10, -5, 5, -2, 2, -1, 1]), vec![100, 50, 25, 10, 5, 2, 1]);
}

#[test]
fn test_108() {
    assert_eq!(Solution::asteroid_collision(vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10]), vec![-1, -3, -5, -7, -9, 10]);
}

#[test]
fn test_109() {
    assert_eq!(Solution::asteroid_collision(vec![100, -50, 20, -10, 5, -5, 10, -20, 30, -40]), vec![100]);
}

#[test]
fn test_110() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, -1, -2, -3, -4, -5]), vec![1, 2, 3, 4]);
}

#[test]
fn test_111() {
    assert_eq!(Solution::asteroid_collision(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, -90, -80, -70, -60, -50, -40, -30, -20, -10]), vec![]);
}

#[test]
fn test_112() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15]), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_113() {
    assert_eq!(Solution::asteroid_collision(vec![10, 20, 30, -15, -25, 5, 15, -5, -10, -20, 25, 35, -40, 40, -45, 45]), vec![-40, -45, 45]);
}

#[test]
fn test_114() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, -9, -8, -7, -6, -5, -4, -3, -2, -1]), vec![]);
}

#[test]
fn test_115() {
    assert_eq!(Solution::asteroid_collision(vec![10, 20, 30, 40, 50, -10, -20, -30, -40, -50]), vec![10, 20, 30, 40]);
}

#[test]
fn test_116() {
    assert_eq!(Solution::asteroid_collision(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_117() {
    assert_eq!(Solution::asteroid_collision(vec![50, -50, 49, -49, 48, -48, 47, -47, 46, -46, 45, -45, 44, -44, 43, -43]), vec![]);
}

#[test]
fn test_118() {
    assert_eq!(Solution::asteroid_collision(vec![-100, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50]), vec![-100, 50]);
}

#[test]
fn test_119() {
    assert_eq!(Solution::asteroid_collision(vec![-1, -2, -3, -4, -5, 5, 4, 3, 2, 1, -6, 6, -7, 7, -8, 8, -9, 9]), vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, 9]);
}

#[test]
fn test_120() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, 15, -15, -10, -5, 20, -20, 25, -25, 30, -30]), vec![]);
}

#[test]
fn test_121() {
    assert_eq!(Solution::asteroid_collision(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10, -10]), vec![]);
}

#[test]
fn test_122() {
    assert_eq!(Solution::asteroid_collision(vec![1, 1, 1, 1, 1, -1, -1, -1, -1, -1]), vec![]);
}

#[test]
fn test_123() {
    assert_eq!(Solution::asteroid_collision(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15, -15]), vec![-2, -4, -6, -8, -10, -12, -14]);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::stone_game_iii(vec![0, 0, 0, 0, 0]), "Tie".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::stone_game_iii(vec![1]), "Alice".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::stone_game_iii(vec![1, -100, 1, 100]), "Tie".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::stone_game_iii(vec![5, 3, 7, 1]), "Alice".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::stone_game_iii(vec![5, 3, 1, 4, 2]), "Alice".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::stone_game_iii(vec![0, 0, 0, 0]), "Tie".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::stone_game_iii(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), "Alice".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), "Tie".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::stone_game_iii(vec![-1, -2, -3, -4]), "Tie".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 6]), "Tie".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::stone_game_iii(vec![0, 0, 0, 0, 0]), "Tie".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::stone_game_iii(vec![10, -10, 20, -20, 30, -30, 40, -40, 50, -50]), "Alice".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::stone_game_iii(vec![5, 3, -6, 7, 2, 8, -1, 4, 10]), "Alice".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), "Bob".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::stone_game_iii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "Alice".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::stone_game_iii(vec![1000, -1000, 1000, -1000]), "Alice".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::stone_game_iii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "Alice".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 7]), "Bob".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::stone_game_iii(vec![1000, -1000, 1000, -1000, 1000]), "Alice".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, -9]), "Alice".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::stone_game_iii(vec![-1, -2, -3, -4, -5]), "Bob".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::stone_game_iii(vec![3, 2, 10, 4]), "Alice".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::stone_game_iii(vec![1000, -1000, 1000, -1000, 1000]), "Alice".to_string());
}

#[test]
fn test_24() {
    assert_eq!(Solution::stone_game_iii(vec![5, 10, -5, 1]), "Alice".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]), "Alice".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::stone_game_iii(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8]), "Alice".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::stone_game_iii(vec![3, -2, 5, 1, -4, 6, -3, 2, 5, -1, 4, -6, 3, 2, -5, 1, 4, -6, 3, 2, -5, 1, 4, -6, 3, 2, -5]), "Alice".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::stone_game_iii(vec![100, -200, 300, -400, 500, -600, 700, -800, 900, -1000, 1100, -1200, 1300, -1400, 1500]), "Alice".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::stone_game_iii(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), "Tie".to_string());
}

#[test]
fn test_30() {
    assert_eq!(Solution::stone_game_iii(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1]), "Alice".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::stone_game_iii(vec![1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984, 983, 982, 981]), "Alice".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::stone_game_iii(vec![1000, -999, 998, -997, 996, -995, 994, -993, 992, -991]), "Alice".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::stone_game_iii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]), "Tie".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::stone_game_iii(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), "Alice".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::stone_game_iii(vec![20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), "Alice".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::stone_game_iii(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), "Tie".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::stone_game_iii(vec![5, 10, -5, 20, 25, -30, 35, -40, 45, 50]), "Alice".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::stone_game_iii(vec![5, -1, 3, -4, 2, 10, -7, 8, -2, 6]), "Alice".to_string());
}

#[test]
fn test_39() {
    assert_eq!(Solution::stone_game_iii(vec![5, 3, 2, 4, 1, 3, 2, 1, 2, 3, 4, 5, 1, 2, 3]), "Alice".to_string());
}

#[test]
fn test_40() {
    assert_eq!(Solution::stone_game_iii(vec![100, -99, 100, -99, 100, -99, 100, -99, 100, -99]), "Alice".to_string());
}

#[test]
fn test_41() {
    assert_eq!(Solution::stone_game_iii(vec![500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500]), "Alice".to_string());
}

#[test]
fn test_42() {
    assert_eq!(Solution::stone_game_iii(vec![10, 15, -5, 20, -10, 25, -15, 30, -20, 35]), "Alice".to_string());
}

#[test]
fn test_43() {
    assert_eq!(Solution::stone_game_iii(vec![100, -50, 200, -150, 300, -200, 400, -250, 500, -300, 600, -350, 700, -400, 800]), "Alice".to_string());
}

#[test]
fn test_44() {
    assert_eq!(Solution::stone_game_iii(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), "Tie".to_string());
}

#[test]
fn test_45() {
    assert_eq!(Solution::stone_game_iii(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]), "Bob".to_string());
}

#[test]
fn test_46() {
    assert_eq!(Solution::stone_game_iii(vec![3, 1, 5, 4, 2, 6, 7, 8, 9, 10]), "Alice".to_string());
}

#[test]
fn test_47() {
    assert_eq!(Solution::stone_game_iii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, -100, -90, -80, -70, -60, -50, -40, -30, -20, -10]), "Tie".to_string());
}

#[test]
fn test_48() {
    assert_eq!(Solution::stone_game_iii(vec![-1, 1, -2, 2, -3, 3, -4, 4, -5, 5, -6, 6, -7, 7, -8]), "Alice".to_string());
}

#[test]
fn test_49() {
    assert_eq!(Solution::stone_game_iii(vec![-1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1]), "Tie".to_string());
}

#[test]
fn test_50() {
    assert_eq!(Solution::stone_game_iii(vec![-5, -10, -15, -20, -25, -30, -35, -40, -45, -50]), "Alice".to_string());
}

#[test]
fn test_51() {
    assert_eq!(Solution::stone_game_iii(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), "Bob".to_string());
}

#[test]
fn test_52() {
    assert_eq!(Solution::stone_game_iii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "Alice".to_string());
}

#[test]
fn test_53() {
    assert_eq!(Solution::stone_game_iii(vec![1000, -500, 250, -125, 62, -31, 15, -7, 3, -1, 0, 1, -3, 7, -15, 31, -62, 125, -250, 500, -1000]), "Alice".to_string());
}

#[test]
fn test_54() {
    assert_eq!(Solution::stone_game_iii(vec![-10, 100, -20, 200, -30, 300, -40, 400, -50, 500]), "Alice".to_string());
}

#[test]
fn test_55() {
    assert_eq!(Solution::stone_game_iii(vec![5, 7, -3, 12, 15, -10, 8, 11, -2, 6, 14, 1, -5, 9, -7]), "Alice".to_string());
}

#[test]
fn test_56() {
    assert_eq!(Solution::stone_game_iii(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15, -16, 17, -18, 19, -20, 21, -22, 23, -24, 25]), "Alice".to_string());
}

#[test]
fn test_57() {
    assert_eq!(Solution::stone_game_iii(vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]), "Tie".to_string());
}

#[test]
fn test_58() {
    assert_eq!(Solution::stone_game_iii(vec![-100, -200, -300, 400, 500, 600, -700, -800, 900, 1000]), "Alice".to_string());
}

#[test]
fn test_59() {
    assert_eq!(Solution::stone_game_iii(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), "Alice".to_string());
}

#[test]
fn test_60() {
    assert_eq!(Solution::stone_game_iii(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), "Alice".to_string());
}

#[test]
fn test_61() {
    assert_eq!(Solution::stone_game_iii(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]), "Bob".to_string());
}

#[test]
fn test_62() {
    assert_eq!(Solution::stone_game_iii(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40]), "Alice".to_string());
}

#[test]
fn test_63() {
    assert_eq!(Solution::stone_game_iii(vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000]), "Alice".to_string());
}

#[test]
fn test_64() {
    assert_eq!(Solution::stone_game_iii(vec![100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89]), "Alice".to_string());
}

#[test]
fn test_65() {
    assert_eq!(Solution::stone_game_iii(vec![-10, 20, -30, 40, -50, 60, -70, 80, -90, 100]), "Alice".to_string());
}

#[test]
fn test_66() {
    assert_eq!(Solution::stone_game_iii(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), "Tie".to_string());
}

#[test]
fn test_67() {
    assert_eq!(Solution::stone_game_iii(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), "Alice".to_string());
}

#[test]
fn test_68() {
    assert_eq!(Solution::stone_game_iii(vec![100, 200, 300, 400, -1000, 500, 600, 700, -800, 900, 1000]), "Alice".to_string());
}

#[test]
fn test_69() {
    assert_eq!(Solution::stone_game_iii(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), "Alice".to_string());
}

#[test]
fn test_70() {
    assert_eq!(Solution::stone_game_iii(vec![100, -200, 300, -400, 500, -600, 700, -800, 900, -1000]), "Alice".to_string());
}

#[test]
fn test_71() {
    assert_eq!(Solution::stone_game_iii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200]), "Alice".to_string());
}

#[test]
fn test_72() {
    assert_eq!(Solution::stone_game_iii(vec![10, -20, 30, -40, 50, -60, 70, -80, 90, -100, 110, -120, 130, -140, 150, -160, 170, -180, 190, -200, 210, -220, 230, -240, 250]), "Alice".to_string());
}

#[test]
fn test_73() {
    assert_eq!(Solution::stone_game_iii(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20]), "Tie".to_string());
}

#[test]
fn test_74() {
    assert_eq!(Solution::stone_game_iii(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), "Tie".to_string());
}

#[test]
fn test_75() {
    assert_eq!(Solution::stone_game_iii(vec![-5, 10, -15, 20, -25, 30, -35, 40, -45, 50]), "Alice".to_string());
}

#[test]
fn test_76() {
    assert_eq!(Solution::stone_game_iii(vec![-10, -20, -30, -40, -50, -60, -70, -80, -90, -100, -110, -120, -130, -140, -150]), "Tie".to_string());
}

#[test]
fn test_77() {
    assert_eq!(Solution::stone_game_iii(vec![1000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), "Alice".to_string());
}

#[test]
fn test_78() {
    assert_eq!(Solution::stone_game_iii(vec![-100, -200, -300, -400, -500, -600, -700, -800, -900, -1000]), "Alice".to_string());
}

#[test]
fn test_79() {
    assert_eq!(Solution::stone_game_iii(vec![-10, 20, -30, 40, -50, 60, -70, 80, -90, 100, 110, -120, 130, -140, 150, -160, 170, -180, 190, -200]), "Alice".to_string());
}

#[test]
fn test_80() {
    assert_eq!(Solution::stone_game_iii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "Alice".to_string());
}

#[test]
fn test_81() {
    assert_eq!(Solution::stone_game_iii(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, -30]), "Alice".to_string());
}

#[test]
fn test_82() {
    assert_eq!(Solution::stone_game_iii(vec![-10, 10, -20, 20, -30, 30, -40, 40, -50, 50]), "Tie".to_string());
}

#[test]
fn test_83() {
    assert_eq!(Solution::stone_game_iii(vec![-50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50]), "Tie".to_string());
}

#[test]
fn test_84() {
    assert_eq!(Solution::stone_game_iii(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15, -16, 17, -18, 19, -20, 21, -22, 23, -24, 25, -26, 27, -28, 29, -30]), "Alice".to_string());
}

#[test]
fn test_85() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), "Alice".to_string());
}

#[test]
fn test_86() {
    assert_eq!(Solution::stone_game_iii(vec![5, -2, 4, -1, 3, -3, 2, 1, -5, 6, 7, -8, 9]), "Alice".to_string());
}

#[test]
fn test_87() {
    assert_eq!(Solution::stone_game_iii(vec![500, 250, -100, 200, -150, 300, -50, 100, -200, 300, 500]), "Alice".to_string());
}

#[test]
fn test_88() {
    assert_eq!(Solution::stone_game_iii(vec![5, -3, 2, 7, -8, 10, -15, 20, -25, 30]), "Alice".to_string());
}

#[test]
fn test_89() {
    assert_eq!(Solution::stone_game_iii(vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0, -10, -20, -30, -40, -50, -60, -70, -80, -90, -100]), "Alice".to_string());
}

#[test]
fn test_90() {
    assert_eq!(Solution::stone_game_iii(vec![-5, 10, 20, -15, 30, 25, -20, 40, -35, 50, -45, 60, -55]), "Alice".to_string());
}

#[test]
fn test_91() {
    assert_eq!(Solution::stone_game_iii(vec![100, 200, 300, -100, -200, -300, 400, 500, 600, -400, -500, -600]), "Alice".to_string());
}

#[test]
fn test_92() {
    assert_eq!(Solution::stone_game_iii(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, -1000, -900, -800, -700, -600, -500, -400, -300, -200, -100]), "Tie".to_string());
}

#[test]
fn test_93() {
    assert_eq!(Solution::stone_game_iii(vec![-50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50, 50, -50]), "Alice".to_string());
}

#[test]
fn test_94() {
    assert_eq!(Solution::stone_game_iii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "Tie".to_string());
}

#[test]
fn test_95() {
    assert_eq!(Solution::stone_game_iii(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15]), "Alice".to_string());
}

#[test]
fn test_96() {
    assert_eq!(Solution::stone_game_iii(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "Alice".to_string());
}

#[test]
fn test_97() {
    assert_eq!(Solution::stone_game_iii(vec![1000, -500, 2000, -1000, 3000, -1500, 4000, -2000, 5000, -2500, 6000, -3000, 7000, -3500, 8000, -4000, 9000, -4500, 10000, -5000]), "Alice".to_string());
}

#[test]
fn test_98() {
    assert_eq!(Solution::stone_game_iii(vec![5, -2, 3, 1, 4, -1, 7, -3, 2, 6, -5]), "Alice".to_string());
}

#[test]
fn test_99() {
    assert_eq!(Solution::stone_game_iii(vec![-999, 1000, -999, 1000, -999, 1000, -999, 1000, -999, 1000]), "Alice".to_string());
}

#[test]
fn test_100() {
    assert_eq!(Solution::stone_game_iii(vec![10, -10, 20, -20, 30, -30, 40, -40, 50, -50]), "Alice".to_string());
}

#[test]
fn test_101() {
    assert_eq!(Solution::stone_game_iii(vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10, 11, -12, 13, -14, 15, -16, 17, -18, 19, -20]), "Alice".to_string());
}

#[test]
fn test_102() {
    assert_eq!(Solution::stone_game_iii(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120]), "Alice".to_string());
}

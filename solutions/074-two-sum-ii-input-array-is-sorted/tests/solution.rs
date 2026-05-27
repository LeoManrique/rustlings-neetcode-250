include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::two_sum(vec![-3, -2, -1, 0, 0, 1, 2, 3], 0), vec![1, 8]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::two_sum(vec![-10, -5, 0, 5, 10], 0), vec![1, 5]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 17), vec![8, 9]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::two_sum(vec![-10, -5, -3, 3, 5, 6, 7, 8, 9], 0), vec![2, 5]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::two_sum(vec![5, 25, 75], 100), vec![2, 3]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::two_sum(vec![-10, -5, -2, -1, 0, 1, 2, 5, 10], 0), vec![1, 9]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::two_sum(vec![-10, -5, -3, -1, 0, 2, 3, 5, 6, 9], -8), vec![1, 6]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 4, 4, 9, 56, 90], 8), vec![4, 5]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::two_sum(vec![1, 3, 5, 7, 9], 10), vec![1, 5]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::two_sum(vec![-1000, -500, 0, 500, 1000], 0), vec![1, 5]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::two_sum(vec![-50, -25, 0, 25, 50, 75, 100], 25), vec![1, 6]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::two_sum(vec![1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34, 37, 40], 65), vec![9, 14]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::two_sum(vec![100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110], 219), vec![10, 11]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::two_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 1), vec![1, 11]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::two_sum(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024], 768), vec![9, 10]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::two_sum(vec![-500, -450, -400, -350, -300, -250, -200, -150, -100, -50, 0], -250), vec![6, 11]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::two_sum(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], 40), vec![6, 15]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30], 51), vec![9, 12]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::two_sum(vec![-999, -500, -100, 0, 100, 500, 999], 500), vec![4, 6]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::two_sum(vec![-3, -2, -1, 0, 1, 2, 3], 0), vec![1, 7]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::two_sum(vec![-10, -5, 0, 3, 7, 9, 11], 2), vec![2, 5]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 13), vec![1, 12]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::two_sum(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40], 70), vec![15, 20]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::two_sum(vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5], 0), vec![1, 11]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 29), vec![14, 15]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::two_sum(vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5], 1), vec![2, 11]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::two_sum(vec![-10, -5, 0, 3, 7, 9, 11, 15], 8), None);
}

#[test]
fn test_31() {
    assert_eq!(Solution::two_sum(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59], 108), vec![25, 30]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::two_sum(vec![-10, -5, 0, 5, 10, 15], 5), vec![1, 6]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987], 1597), vec![14, 15]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::two_sum(vec![-1000, -900, -800, -700, -600, -500, -400, -300, -200, -100], -1800), vec![1, 3]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 38), vec![18, 20]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::two_sum(vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], 1500), vec![5, 10]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::two_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), vec![1, 2]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::two_sum(vec![-50, -20, -10, 0, 10, 20, 30, 40, 50], 0), vec![1, 9]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::two_sum(vec![-499, -333, -222, -111, 0, 111, 222, 333, 499, 666, 888, 1111, 1333, 1666, 1999], 1000), vec![2, 13]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::two_sum(vec![1, 1, 2, 2, 3, 4, 5], 4), vec![1, 5]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::two_sum(vec![1, 3, 3, 4, 4, 4, 5, 5, 5, 5, 6, 6, 7, 8, 9], 8), vec![1, 13]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::two_sum(vec![-10, -5, 0, 5, 10], 0), vec![1, 5]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::two_sum(vec![-1000, -999, -998, -997, -996, -995, -994, -993, -992, -991], -1981), None);
}

#[test]
fn test_44() {
    assert_eq!(Solution::two_sum(vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120], 210), None);
}

#[test]
fn test_45() {
    assert_eq!(Solution::two_sum(vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 4, 5, 6, 7], 5), vec![1, 12]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::two_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 1), vec![1, 15]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::two_sum(vec![-1000, -500, -250, -125, -62, -31, -15, -7, -3, -1, 0, 1, 3, 7, 15, 31, 62, 125, 250, 500, 1000], -1562), None);
}

#[test]
fn test_48() {
    assert_eq!(Solution::two_sum(vec![1, 4, 6, 8, 10, 12, 14, 16, 18, 20], 30), vec![5, 10]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::two_sum(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 10), vec![1, 10]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::two_sum(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31], 58), vec![14, 16]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::two_sum(vec![1, 4, 6, 8, 10, 12, 14, 16, 18, 20], 38), vec![9, 10]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::two_sum(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2], 3), vec![1, 15]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::two_sum(vec![-10, -5, -1, 0, 2, 5, 8, 12], -3), vec![2, 5]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::two_sum(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], 58), None);
}

#[test]
fn test_55() {
    assert_eq!(Solution::two_sum(vec![-50, -25, -10, -5, 0, 5, 10, 25, 50], 0), vec![1, 9]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::two_sum(vec![-1000, -999, -998, -997, -996], -1998), vec![1, 3]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::two_sum(vec![-50, -25, 0, 25, 50], 25), vec![2, 5]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::two_sum(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75], 130), vec![11, 15]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::two_sum(vec![3, 3, 3, 3, 3], 6), vec![1, 2]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::two_sum(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 2), vec![1, 2]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::two_sum(vec![995, 996, 997, 998, 999, 1000], 1995), vec![1, 6]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::two_sum(vec![-100, -90, -80, -70, -60, -50, -40, -30, -20, -10], -80), vec![4, 10]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::two_sum(vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100], 120), vec![3, 19]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 28), vec![13, 15]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::two_sum(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2], 3), vec![1, 21]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::two_sum(vec![-500, -250, -100, -50, -25, -10, -5, 0, 5, 10, 25, 50, 100, 250, 500], 0), vec![1, 15]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::two_sum(vec![500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511], 1011), vec![1, 12]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::two_sum(vec![100, 150, 200, 250, 300, 350, 400, 450, 500], 750), vec![4, 9]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::two_sum(vec![-999, -998, -997, -996, -995, -994, -993, -992, -991, -990, 990, 991, 992, 993, 994, 995, 996, 997, 998, 999], 0), vec![1, 20]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::two_sum(vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10], 15), vec![1, 15]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::two_sum(vec![-1000, -900, -800, -700, -600, -500, -400, -300, -200, -100, 0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], -500), vec![1, 16]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::two_sum(vec![-100, -50, -25, -10, -5, 0, 5, 10, 25, 50, 100], 0), vec![1, 11]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::two_sum(vec![100, 150, 200, 250, 300, 350, 400, 450, 500], 800), vec![5, 9]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::two_sum(vec![-100, -50, -25, 0, 25, 50, 100, 150, 200, 250, 300], 200), vec![1, 11]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::two_sum(vec![-50, -20, -10, -5, 0, 5, 10, 20, 50], 0), vec![1, 9]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::two_sum(vec![-1000, -900, -800, -700, -600, -500, -400, -300, -200, -100, 0, 100, 200, 300, 400], -500), vec![2, 15]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987], 1094), None);
}

#[test]
fn test_78() {
    assert_eq!(Solution::two_sum(vec![-10, -5, 0, 3, 8, 12, 15], 5), vec![1, 7]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 39), vec![19, 20]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::two_sum(vec![-5, 0, 2, 3, 7, 11], 5), vec![3, 4]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::two_sum(vec![10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100], 190), vec![17, 19]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 18), vec![8, 10]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::two_sum(vec![1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67], 94), vec![14, 17]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::two_sum(vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 8), vec![1, 11]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::two_sum(vec![-100, -50, -10, 0, 50, 100, 150], 0), vec![1, 6]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::two_sum(vec![1, 3, 6, 8, 12, 14, 16, 18, 20, 23, 26, 28, 30, 32, 35, 37, 40, 42, 45, 47], 87), vec![17, 20]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::two_sum(vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 7), vec![1, 10]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::two_sum(vec![-100, -50, -10, 0, 10, 50, 100], 0), vec![1, 7]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::two_sum(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512], 768), vec![9, 10]);
}

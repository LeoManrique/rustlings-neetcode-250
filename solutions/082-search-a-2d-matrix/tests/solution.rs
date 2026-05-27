include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20]], 10), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3], vec![0, 3, 10], vec![15, 20, 25]], 3), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -1], vec![0, 2, 3, 5], vec![7, 8, 11, 13], vec![15, 16, 18, 20]], 0), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 5), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::search_matrix(vec![vec![1]], 1), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -8, -6, -4], vec![-3, -1, 1, 3], vec![5, 7, 9, 11]], 10), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]], 0), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -1], vec![0, 2, 3, 5], vec![7, 8, 11, 13], vec![15, 16, 18, 20]], 20), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -8, -6, -4], vec![-3, -1, 1, 3], vec![5, 7, 9, 11]], -5), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15]], 8), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 13), false);
}

#[test]
fn test_12() {
    assert_eq!(Solution::search_matrix(vec![vec![1]], 2), false);
}

#[test]
fn test_13() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -1], vec![0, 2, 3, 5], vec![8, 9, 10, 12]], -3), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7]], 5), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3], vec![0, 3, 10], vec![15, 20, 25]], 1), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5]], 5), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -2], vec![-1, 0, 2, 3], vec![4, 5, 7, 8]], 11), false);
}

#[test]
fn test_18() {
    assert_eq!(Solution::search_matrix(vec![vec![1]], 0), false);
}

#[test]
fn test_19() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3]], 3), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -1], vec![0, 2, 3, 5], vec![7, 8, 11, 13], vec![15, 16, 18, 20]], -10), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7]], 8), false);
}

#[test]
fn test_22() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 10), false);
}

#[test]
fn test_23() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -2], vec![-1, 0, 2, 3], vec![4, 5, 7, 8]], -3), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 0), false);
}

#[test]
fn test_25() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -1], vec![0, 2, 3, 5], vec![8, 9, 10, 12]], 15), false);
}

#[test]
fn test_26() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -1], vec![0, 2, 3, 5], vec![7, 8, 11, 13], vec![15, 16, 18, 20]], 21), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 3), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -1], vec![0, 2, 3, 5], vec![7, 8, 11, 13], vec![15, 16, 18, 20]], 11), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, -3, -1], vec![0, 2, 3, 5], vec![7, 8, 11, 13], vec![15, 16, 18, 20]], -11), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 6), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5]], 6), false);
}

#[test]
fn test_32() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 60), true);
}

#[test]
fn test_33() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3]], 1), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]], 10), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3]], 2), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 7), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 1), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::search_matrix(vec![vec![-1000, -950, -900, -850], vec![-800, -750, -700, -650], vec![-600, -550, -500, -450], vec![-400, -350, -300, -250], vec![-200, -150, -100, -50]], -325), false);
}

#[test]
fn test_39() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60], vec![61, 62, 63, 64], vec![65, 66, 67, 68]], 1), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5], vec![7, 9, 11], vec![13, 15, 17]], 18), false);
}

#[test]
fn test_41() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -5, 0, 5, 10], vec![-4, -2, 2, 4, 8], vec![6, 7, 12, 13, 14], vec![15, 16, 18, 19, 20], vec![21, 22, 23, 24, 25]], 12), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::search_matrix(vec![vec![-1, 0, 1, 2, 3], vec![4, 5, 6, 7, 8], vec![9, 10, 11, 12, 13], vec![14, 15, 16, 17, 18], vec![19, 20, 21, 22, 23]], 0), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::search_matrix(vec![vec![10, 20, 30, 40, 50], vec![60, 70, 80, 90, 100], vec![110, 120, 130, 140, 150], vec![160, 170, 180, 190, 200]], 150), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9, 11, 13, 15], vec![17, 19, 21, 23, 25, 27, 29, 31], vec![33, 35, 37, 39, 41, 43, 45, 47], vec![49, 51, 53, 55, 57, 59, 61, 63]], 62), false);
}

#[test]
fn test_45() {
    assert_eq!(Solution::search_matrix(vec![vec![-10000]], -10000), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30], vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40], vec![41, 42, 43, 44, 45, 46, 47, 48, 49, 50]], 25), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19], vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29]], 20), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60], vec![61, 62, 63, 64], vec![65, 66, 67, 68]], 35), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -90, -80, -70], vec![-60, -50, -40, -30], vec![-20, -10, 0, 10], vec![20, 30, 40, 50]], 5), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::search_matrix(vec![vec![-1, -2, -3], vec![-4, -5, -6], vec![-7, -8, -9]], 0), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -99, -98, -97, -96], vec![-95, -94, -93, -92, -91], vec![-90, -89, -88, -87, -86], vec![-85, -84, -83, -82, -81], vec![-80, -79, -78, -77, -76]], -88), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]], 10), true);
}

#[test]
fn test_53() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 61), false);
}

#[test]
fn test_54() {
    assert_eq!(Solution::search_matrix(vec![vec![-4, -3, -2, -1, 0], vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20]], -2), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 5), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 4, 7, 10, 13], vec![2, 5, 8, 11, 14], vec![3, 6, 9, 12, 15], vec![16, 19, 22, 25, 28], vec![17, 20, 23, 26, 29]], 18), false);
}

#[test]
fn test_57() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![9, 11, 13, 15], vec![17, 19, 21, 23], vec![25, 27, 29, 31], vec![33, 35, 37, 39], vec![41, 43, 45, 47], vec![49, 51, 53, 55], vec![57, 59, 61, 63]], 64), false);
}

#[test]
fn test_58() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -90, -80, -70], vec![-60, -50, -40, -30], vec![-20, -10, 0, 10], vec![20, 30, 40, 50]], -55), false);
}

#[test]
fn test_59() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], vec![31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59], vec![61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89], vec![91, 93, 95, 97, 99, 101, 103, 105, 107, 109, 111, 113, 115, 117, 119]], 119), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::search_matrix(vec![vec![-5, -4, -3, -2, -1], vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]], -6), false);
}

#[test]
fn test_61() {
    assert_eq!(Solution::search_matrix(vec![vec![-1, 0, 3, 5, 9, 12], vec![13, 18, 20, 23, 28, 31], vec![32, 34, 36, 39, 42, 45], vec![46, 48, 51, 54, 57, 60]], 61), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 5), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -90, -80, -70], vec![-60, -50, -40, -30], vec![-20, -10, 0, 10], vec![20, 30, 40, 50]], -45), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -99, -98, -97, -96], vec![-95, -94, -93, -92, -91], vec![-90, -89, -88, -87, -86], vec![-85, -84, -83, -82, -81], vec![-80, -79, -78, -77, -76]], -83), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::search_matrix(vec![vec![-5, -2, -1], vec![0, 3, 5], vec![8, 9, 10]], -3), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16], vec![17, 18, 19, 20]], 1), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 20), false);
}

#[test]
fn test_68() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5], vec![7, 9, 11], vec![13, 15, 17]], 14), false);
}

#[test]
fn test_69() {
    assert_eq!(Solution::search_matrix(vec![vec![-1000, -999, -998, -997, -996], vec![-995, -994, -993, -992, -991], vec![-990, -989, -988, -987, -986], vec![-985, -984, -983, -982, -981], vec![-980, -979, -978, -977, -976]], -987), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9], vec![11, 13, 15, 17, 19], vec![21, 23, 25, 27, 29], vec![31, 33, 35, 37, 39], vec![41, 43, 45, 47, 49]], 38), false);
}

#[test]
fn test_71() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 20), false);
}

#[test]
fn test_72() {
    assert_eq!(Solution::search_matrix(vec![vec![-1000, -999, -998, -997, -996], vec![-995, -994, -993, -992, -991], vec![-990, -989, -988, -987, -986]], -987), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30]], 15), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9], vec![11, 13, 15, 17, 19], vec![21, 23, 25, 27, 29], vec![31, 33, 35, 37, 39], vec![41, 43, 45, 47, 49]], 0), false);
}

#[test]
fn test_75() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18], vec![19, 20, 21], vec![22, 23, 24], vec![25, 26, 27], vec![28, 29, 30], vec![31, 32, 33]], 17), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -99, -98, -97], vec![-96, -95, -94, -93], vec![-92, -91, -90, -89], vec![-88, -87, -86, -85]], -90), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::search_matrix(vec![vec![-1, -1, -1, -1], vec![-1, -1, -1, -1], vec![-1, -1, -1, -1], vec![-1, -1, -1, -1]], -1), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9], vec![11, 13, 15, 17, 19], vec![21, 23, 25, 27, 29], vec![31, 33, 35, 37, 39]], 20), false);
}

#[test]
fn test_79() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9, 11], vec![13, 15, 17, 19, 21, 23], vec![25, 27, 29, 31, 33, 35], vec![37, 39, 41, 43, 45, 47], vec![49, 51, 53, 55, 57, 59]], 30), false);
}

#[test]
fn test_80() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 25), false);
}

#[test]
fn test_81() {
    assert_eq!(Solution::search_matrix(vec![vec![10, 11, 12, 13, 14, 15]], 15), true);
}

#[test]
fn test_82() {
    assert_eq!(Solution::search_matrix(vec![vec![-5, -4, -3, -2, -1], vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]], 10), false);
}

#[test]
fn test_83() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]], 27), true);
}

#[test]
fn test_84() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]], 13), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]], 31), false);
}

#[test]
fn test_86() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -90, -80, -70, -60], vec![-50, -40, -30, -20, -10], vec![0, 10, 20, 30, 40], vec![50, 60, 70, 80, 90], vec![100, 110, 120, 130, 140]], -35), false);
}

#[test]
fn test_87() {
    assert_eq!(Solution::search_matrix(vec![vec![-1, 0, 3, 5, 9, 12], vec![13, 18, 20, 23, 28, 31], vec![32, 34, 36, 39, 42, 45], vec![46, 48, 51, 54, 57, 60]], 45), true);
}

#[test]
fn test_88() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45]], 22), true);
}

#[test]
fn test_89() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30], vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40]], 25), true);
}

#[test]
fn test_90() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 35), false);
}

#[test]
fn test_91() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60], vec![61, 62, 63, 64], vec![65, 66, 67, 68]], 68), true);
}

#[test]
fn test_92() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -98, -95, -93], vec![-88, -86, -82, -80], vec![-74, -70, -66, -62], vec![-58, -54, -50, -46]], -66), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]], 26), false);
}

#[test]
fn test_94() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 5, 9, 13], vec![17, 21, 25, 29], vec![33, 37, 41, 45], vec![49, 53, 57, 61]], 18), false);
}

#[test]
fn test_95() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60], vec![61, 62, 63, 64], vec![65, 66, 67, 68], vec![69, 70, 71, 72]], 67), true);
}

#[test]
fn test_96() {
    assert_eq!(Solution::search_matrix(vec![vec![-5, -2, -1], vec![0, 3, 5], vec![8, 9, 10]], 11), false);
}

#[test]
fn test_97() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -90, -80, -70], vec![-60, -50, -40, -30], vec![-20, -10, 0, 10], vec![20, 30, 40, 50]], 0), true);
}

#[test]
fn test_98() {
    assert_eq!(Solution::search_matrix(vec![vec![-10000, -9999, -9998, -9997], vec![-9996, -9995, -9994, -9993], vec![-9992, -9991, -9990, -9989], vec![-9988, -9987, -9986, -9985]], -9990), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -9, -8, -7, -6], vec![-5, -4, -3, -2, -1], vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9], vec![10, 11, 12, 13, 14]], -5), true);
}

#[test]
fn test_100() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -99, -98, -97, -96], vec![-95, -94, -93, -92, -91], vec![-90, -89, -88, -87, -86], vec![-85, -84, -83, -82, -81], vec![-80, -79, -78, -77, -76]], 0), false);
}

#[test]
fn test_101() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 0), false);
}

#[test]
fn test_102() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]], 13), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -8, -6, -4, -2], vec![0, 2, 4, 6, 8], vec![10, 12, 14, 16, 18], vec![20, 22, 24, 26, 28], vec![30, 32, 34, 36, 38]], -5), false);
}

#[test]
fn test_104() {
    assert_eq!(Solution::search_matrix(vec![vec![10], vec![11], vec![12], vec![13], vec![14], vec![15]], 15), true);
}

#[test]
fn test_105() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], vec![21, 23, 25, 27, 29, 31, 33, 35, 37, 39], vec![41, 43, 45, 47, 49, 51, 53, 55, 57, 59], vec![61, 63, 65, 67, 69, 71, 73, 75, 77, 79]], 40), false);
}

#[test]
fn test_106() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1), true);
}

#[test]
fn test_107() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30], vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40], vec![41, 42, 43, 44, 45, 46, 47, 48, 49, 50]], 51), false);
}

#[test]
fn test_108() {
    assert_eq!(Solution::search_matrix(vec![vec![-1000, -500, -250, -100], vec![-50, -25, -10, 0], vec![5, 10, 25, 50], vec![100, 250, 500, 1000]], 0), true);
}

#[test]
fn test_109() {
    assert_eq!(Solution::search_matrix(vec![vec![-1, 0, 3, 5, 9, 12], vec![13, 18, 20, 23, 28, 31], vec![32, 34, 36, 39, 42, 45], vec![46, 48, 51, 54, 57, 60]], -2), false);
}

#[test]
fn test_110() {
    assert_eq!(Solution::search_matrix(vec![vec![1], vec![2], vec![3]], 2), true);
}

#[test]
fn test_111() {
    assert_eq!(Solution::search_matrix(vec![vec![-5, -3, -1, 1, 3], vec![5, 7, 9, 11, 13], vec![15, 17, 19, 21, 23], vec![25, 27, 29, 31, 33], vec![35, 37, 39, 41, 43]], 42), false);
}

#[test]
fn test_112() {
    assert_eq!(Solution::search_matrix(vec![vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19], vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29], vec![30, 31, 32, 33, 34, 35, 36, 37, 38, 39], vec![40, 41, 42, 43, 44, 45, 46, 47, 48, 49], vec![50, 51, 52, 53, 54, 55, 56, 57, 58, 59], vec![60, 61, 62, 63, 64, 65, 66, 67, 68, 69], vec![70, 71, 72, 73, 74, 75, 76, 77, 78, 79], vec![80, 81, 82, 83, 84, 85, 86, 87, 88, 89]], 75), true);
}

#[test]
fn test_113() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9], vec![11, 13, 15, 17, 19], vec![21, 23, 25, 27, 29], vec![31, 33, 35, 37, 39], vec![41, 43, 45, 47, 49]], 50), false);
}

#[test]
fn test_114() {
    assert_eq!(Solution::search_matrix(vec![vec![-5, -4, -3, -2, -1], vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]], -1), true);
}

#[test]
fn test_115() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 5, 9, 13, 17, 21], vec![25, 29, 33, 37, 41, 45], vec![49, 53, 57, 61, 65, 69], vec![73, 77, 81, 85, 89, 93], vec![97, 101, 105, 109, 113, 117]], 97), true);
}

#[test]
fn test_116() {
    assert_eq!(Solution::search_matrix(vec![vec![-9999, -9997, -9995, -9993], vec![-9991, -9989, -9987, -9985], vec![-9983, -9981, -9979, -9977], vec![-9975, -9973, -9971, -9969]], -9996), false);
}

#[test]
fn test_117() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]], 5), true);
}

#[test]
fn test_118() {
    assert_eq!(Solution::search_matrix(vec![vec![-1000, -500, -250, -100], vec![-50, -25, -10, 0], vec![5, 10, 25, 50], vec![100, 250, 500, 1000]], -300), false);
}

#[test]
fn test_119() {
    assert_eq!(Solution::search_matrix(vec![vec![10000]], 10000), true);
}

#[test]
fn test_120() {
    assert_eq!(Solution::search_matrix(vec![vec![1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009], vec![1010, 1011, 1012, 1013, 1014, 1015, 1016, 1017, 1018, 1019], vec![1020, 1021, 1022, 1023, 1024, 1025, 1026, 1027, 1028, 1029], vec![1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039]], 1025), true);
}

#[test]
fn test_121() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3]], 2), true);
}

#[test]
fn test_122() {
    assert_eq!(Solution::search_matrix(vec![vec![-1, -2, -3], vec![-4, -5, -6], vec![-7, -8, -9]], -5), false);
}

#[test]
fn test_123() {
    assert_eq!(Solution::search_matrix(vec![vec![-10000, -9999, -9998], vec![-9997, -9996, -9995], vec![-9994, -9993, -9992], vec![-9991, -9990, -9989], vec![-9988, -9987, -9986]], -9992), true);
}

#[test]
fn test_124() {
    assert_eq!(Solution::search_matrix(vec![vec![-1000, -500, -250, -100], vec![-50, -25, -10, 0], vec![5, 10, 25, 50], vec![100, 250, 500, 1000]], 2000), false);
}

#[test]
fn test_125() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]], 11), false);
}

#[test]
fn test_126() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60], vec![70, 71, 72, 73]], 70), true);
}

#[test]
fn test_127() {
    assert_eq!(Solution::search_matrix(vec![vec![-5, -4, -3, -2, -1], vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9], vec![10, 11, 12, 13, 14], vec![15, 16, 17, 18, 19]], 18), true);
}

#[test]
fn test_128() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9], vec![10, 11, 12, 13, 14], vec![15, 16, 17, 18, 19], vec![20, 21, 22, 23, 24], vec![25, 26, 27, 28, 29]], 29), true);
}

#[test]
fn test_129() {
    assert_eq!(Solution::search_matrix(vec![vec![-10000, -9000, -8000, -7000], vec![-6000, -5000, -4000, -3000], vec![-2000, -1000, 0, 1000], vec![2000, 3000, 4000, 5000]], -5000), true);
}

#[test]
fn test_130() {
    assert_eq!(Solution::search_matrix(vec![vec![-1, 1, 3], vec![5, 7, 9], vec![11, 13, 15]], 0), false);
}

#[test]
fn test_131() {
    assert_eq!(Solution::search_matrix(vec![vec![-1000, -900, -800], vec![-700, -600, -500], vec![-400, -300, -200], vec![-100, -50, 0], vec![50, 100, 150]], -450), false);
}

#[test]
fn test_132() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93, 95, 97, 99]], 42), false);
}

#[test]
fn test_133() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30]], 25), true);
}

#[test]
fn test_134() {
    assert_eq!(Solution::search_matrix(vec![vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14], vec![15, 16, 17, 18, 19, 20, 21, 22, 23, 24], vec![25, 26, 27, 28, 29, 30, 31, 32, 33, 34]], 17), true);
}

#[test]
fn test_135() {
    assert_eq!(Solution::search_matrix(vec![vec![-3, -2, -1, 0, 1, 2, 3], vec![4, 5, 6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15, 16, 17], vec![18, 19, 20, 21, 22, 23, 24]], 13), true);
}

#[test]
fn test_136() {
    assert_eq!(Solution::search_matrix(vec![vec![-1000, -999, -998, -997, -996], vec![-995, -994, -993, -992, -991], vec![-990, -989, -988, -987, -986], vec![-985, -984, -983, -982, -981]], -993), true);
}

#[test]
fn test_137() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 3, 5, 7, 9, 11, 13], vec![15, 17, 19, 21, 23, 25, 27], vec![29, 31, 33, 35, 37, 39, 41]], 28), false);
}

#[test]
fn test_138() {
    assert_eq!(Solution::search_matrix(vec![vec![10000, 10001, 10002], vec![10003, 10004, 10005], vec![10006, 10007, 10008]], 10004), true);
}

#[test]
fn test_139() {
    assert_eq!(Solution::search_matrix(vec![vec![-50, -49, -48, -47, -46], vec![-45, -44, -43, -42, -41], vec![-40, -39, -38, -37, -36], vec![-35, -34, -33, -32, -31], vec![-30, -29, -28, -27, -26]], -33), true);
}

#[test]
fn test_140() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 31), false);
}

#[test]
fn test_141() {
    assert_eq!(Solution::search_matrix(vec![vec![-5, -4, -3, -2, -1], vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]], 9), true);
}

#[test]
fn test_142() {
    assert_eq!(Solution::search_matrix(vec![vec![-100, -90, -80, -70], vec![-60, -50, -40, -30], vec![-20, -10, 0, 10], vec![20, 30, 40, 50]], -101), false);
}

#[test]
fn test_143() {
    assert_eq!(Solution::search_matrix(vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13, 14, 15], vec![16, 17, 18, 19, 20], vec![21, 22, 23, 24, 25]], 26), false);
}

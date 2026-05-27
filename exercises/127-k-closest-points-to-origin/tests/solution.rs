include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]], 3), vec![vec![0, 0], vec![1, 1], vec![2, 2]]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2), vec![vec![3, 3], vec![-2, 4]]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::k_closest(vec![vec![-3, -3], vec![-2, -2], vec![-1, -1], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3]], 4), vec![vec![0, 0], vec![-1, -1], vec![1, 1], vec![-2, -2]]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::k_closest(vec![vec![10000, -10000], vec![-10000, 10000], vec![0, 0]], 2), vec![vec![0, 0], vec![10000, -10000]]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1), vec![vec![-2, 2]]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::k_closest(vec![vec![1, 3], vec![2, 4], vec![3, 1]], 2), vec![vec![1, 3], vec![3, 1]]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::k_closest(vec![vec![-1, 0], vec![2, 0], vec![-1, 0], vec![2, 0]], 3), vec![vec![-1, 0], vec![-1, 0], vec![2, 0]]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::k_closest(vec![vec![-10000, -10000], vec![10000, 10000]], 1), vec![vec![-10000, -10000]]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::k_closest(vec![vec![-1, 0], vec![2, 0], vec![-1, 0], vec![2, 0]], 2), vec![vec![-1, 0], vec![-1, 0]]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::k_closest(vec![vec![0, 1], vec![1, 0]], 2), vec![vec![0, 1], vec![1, 0]]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::k_closest(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]], 5), vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::k_closest(vec![vec![6, 10], vec![-3, 3], vec![-2, 5], vec![0, 2]], 3), vec![vec![0, 2], vec![-3, 3], vec![-2, 5]]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::k_closest(vec![vec![-10000, 10000], vec![10000, -10000]], 1), vec![vec![-10000, 10000]]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::k_closest(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10]], 5), vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3]], 3), vec![vec![0, 0], vec![1, 1], vec![2, 2]]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3]], 2), vec![vec![0, 0], vec![1, 1]]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::k_closest(vec![vec![-5, 4], vec![4, -5], vec![0, 0], vec![1, 1], vec![-1, -1], vec![5, -5], vec![-5, 5], vec![2, 2], vec![-2, -2]], 5), vec![vec![0, 0], vec![1, 1], vec![-1, -1], vec![2, 2], vec![-2, -2]]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 0], vec![7, 0], vec![8, 0], vec![9, 0], vec![10, 0]], 7), vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 0]]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::k_closest(vec![vec![10000, 0], vec![0, 10000], vec![-10000, 0], vec![0, -10000], vec![2500, 2500], vec![2500, -2500], vec![-2500, 2500], vec![-2500, -2500], vec![0, 0]], 6), vec![vec![0, 0], vec![2500, 2500], vec![2500, -2500], vec![-2500, 2500], vec![-2500, -2500], vec![10000, 0]]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::k_closest(vec![vec![100, 100], vec![200, 200], vec![300, 300], vec![400, 400], vec![500, 500], vec![600, 600], vec![700, 700]], 5), vec![vec![100, 100], vec![200, 200], vec![300, 300], vec![400, 400], vec![500, 500]]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::k_closest(vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 0], vec![7, 0], vec![8, 0], vec![9, 0], vec![10, 0], vec![-1, 0], vec![-2, 0], vec![-3, 0], vec![-4, 0], vec![-5, 0], vec![-6, 0], vec![-7, 0], vec![-8, 0], vec![-9, 0], vec![-10, 0]], 10), vec![vec![1, 0], vec![-1, 0], vec![2, 0], vec![-2, 0], vec![3, 0], vec![-3, 0], vec![4, 0], vec![-4, 0], vec![5, 0], vec![-5, 0]]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::k_closest(vec![vec![-1, 0], vec![0, -1], vec![1, 0], vec![0, 1], vec![-2, 0], vec![0, -2], vec![2, 0], vec![0, 2], vec![-3, 0], vec![0, -3], vec![3, 0], vec![0, 3], vec![-4, 0], vec![0, -4], vec![4, 0], vec![0, 4]], 8), vec![vec![-1, 0], vec![0, -1], vec![1, 0], vec![0, 1], vec![-2, 0], vec![0, -2], vec![2, 0], vec![0, 2]]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::k_closest(vec![vec![100, 100], vec![-100, -100], vec![200, 200], vec![-200, -200], vec![300, 300], vec![-300, -300], vec![0, 0]], 4), vec![vec![0, 0], vec![100, 100], vec![-100, -100], vec![200, 200]]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::k_closest(vec![vec![-10, 10], vec![-20, 20], vec![-30, 30], vec![-40, 40], vec![-50, 50], vec![-60, 60], vec![-70, 70], vec![-80, 80], vec![-90, 90], vec![-100, 100]], 3), vec![vec![-10, 10], vec![-20, 20], vec![-30, 30]]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::k_closest(vec![vec![-1, 0], vec![0, -1], vec![1, 0], vec![0, 1], vec![0, 0]], 3), vec![vec![0, 0], vec![-1, 0], vec![0, -1]]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::k_closest(vec![vec![100, -100], vec![-100, 100], vec![200, 200], vec![-200, -200], vec![300, -300], vec![-300, 300], vec![400, 400], vec![-400, -400], vec![500, -500]], 3), vec![vec![100, -100], vec![-100, 100], vec![200, 200]]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::k_closest(vec![vec![100, 100], vec![200, 200], vec![300, 300], vec![400, 400], vec![500, 500], vec![600, 600], vec![700, 700], vec![800, 800], vec![900, 900], vec![1000, 1000]], 4), vec![vec![100, 100], vec![200, 200], vec![300, 300], vec![400, 400]]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![1, 1], vec![1, -1], vec![-1, -1], vec![-1, 1], vec![2, 2], vec![-2, -2]], 5), vec![vec![0, 0], vec![1, 1], vec![1, -1], vec![-1, -1], vec![-1, 1]]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::k_closest(vec![vec![-5, -5], vec![5, 5], vec![-6, -6], vec![6, 6], vec![-7, -7], vec![7, 7], vec![-8, -8], vec![8, 8], vec![-9, -9], vec![9, 9], vec![-10, -10], vec![10, 10]], 6), vec![vec![-5, -5], vec![5, 5], vec![-6, -6], vec![6, 6], vec![-7, -7], vec![7, 7]]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::k_closest(vec![vec![10, 0], vec![20, 0], vec![30, 0], vec![40, 0], vec![50, 0], vec![60, 0], vec![70, 0], vec![80, 0], vec![90, 0], vec![100, 0]], 7), vec![vec![10, 0], vec![20, 0], vec![30, 0], vec![40, 0], vec![50, 0], vec![60, 0], vec![70, 0]]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::k_closest(vec![vec![-1, -1], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]], 5), vec![vec![0, 0], vec![-1, -1], vec![1, 1], vec![2, 2], vec![3, 3]]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::k_closest(vec![vec![-9999, -9999], vec![9999, 9999], vec![5000, 5000], vec![-5000, -5000], vec![1000, -1000], vec![-1000, 1000], vec![2000, -2000], vec![-2000, 2000]], 4), vec![vec![1000, -1000], vec![-1000, 1000], vec![2000, -2000], vec![-2000, 2000]]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::k_closest(vec![vec![1, -1], vec![2, -2], vec![3, -3], vec![4, -4], vec![5, -5], vec![6, -6], vec![7, -7], vec![8, -8], vec![9, -9], vec![10, -10]], 5), vec![vec![1, -1], vec![2, -2], vec![3, -3], vec![4, -4], vec![5, -5]]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::k_closest(vec![vec![-8000, 8000], vec![8000, -8000], vec![-7000, 7000], vec![7000, -7000], vec![-6000, 6000], vec![6000, -6000], vec![-5000, 5000], vec![5000, -5000], vec![-4000, 4000], vec![4000, -4000]], 3), vec![vec![-4000, 4000], vec![4000, -4000], vec![-5000, 5000]]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1], vec![1, 1], vec![-1, -1], vec![-1, 1], vec![1, -1]], 6), vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1], vec![1, 1]]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::k_closest(vec![vec![100, -100], vec![-100, 100], vec![200, -200], vec![-200, 200], vec![300, -300]], 3), vec![vec![100, -100], vec![-100, 100], vec![200, -200]]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::k_closest(vec![vec![-1000, 0], vec![0, -1000], vec![1000, 0], vec![0, 1000], vec![500, 500], vec![-500, -500]], 3), vec![vec![500, 500], vec![-500, -500], vec![-1000, 0]]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![1, 1], vec![-1, -1], vec![-1, 0], vec![0, -1]], 5), vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::k_closest(vec![vec![1, 1], vec![1, -1], vec![-1, 1], vec![-1, -1], vec![2, 2], vec![2, -2], vec![-2, 2], vec![-2, -2], vec![3, 3], vec![3, -3], vec![-3, 3], vec![-3, -3]], 6), vec![vec![1, 1], vec![1, -1], vec![-1, 1], vec![-1, -1], vec![2, 2], vec![2, -2]]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::k_closest(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11]], 5), vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6]]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::k_closest(vec![vec![-500, 500], vec![500, -500], vec![-100, 100], vec![100, -100], vec![0, 0], vec![200, -200]], 4), vec![vec![0, 0], vec![-100, 100], vec![100, -100], vec![200, -200]]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 0], vec![7, 0], vec![8, 0], vec![9, 0], vec![10, 0]], 8), vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 0], vec![7, 0]]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::k_closest(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16]], 4), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::k_closest(vec![vec![100, 100], vec![200, 200], vec![300, 300], vec![400, 400], vec![500, 500]], 1), vec![vec![100, 100]]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::k_closest(vec![vec![-999, -999], vec![999, 999], vec![0, 0], vec![499, 499], vec![-499, -499], vec![500, 500], vec![-500, -500]], 5), vec![vec![0, 0], vec![499, 499], vec![-499, -499], vec![500, 500], vec![-500, -500]]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::k_closest(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]], 5), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::k_closest(vec![vec![-5000, 5000], vec![-4000, 4000], vec![-3000, 3000], vec![-2000, 2000], vec![-1000, 1000]], 3), vec![vec![-1000, 1000], vec![-2000, 2000], vec![-3000, 3000]]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::k_closest(vec![vec![10, 10], vec![20, 20], vec![30, 30], vec![40, 40], vec![50, 50], vec![60, 60], vec![70, 70], vec![80, 80], vec![90, 90], vec![100, 100]], 8), vec![vec![10, 10], vec![20, 20], vec![30, 30], vec![40, 40], vec![50, 50], vec![60, 60], vec![70, 70], vec![80, 80]]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::k_closest(vec![vec![10, 10], vec![20, 20], vec![30, 30], vec![40, 40], vec![50, 50], vec![60, 60], vec![70, 70], vec![80, 80], vec![90, 90], vec![100, 100]], 3), vec![vec![10, 10], vec![20, 20], vec![30, 30]]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::k_closest(vec![vec![10, 10], vec![-10, -10], vec![5, 5], vec![-5, -5], vec![0, 0], vec![2, 2], vec![-2, -2], vec![3, 3], vec![-3, -3], vec![4, 4], vec![-4, -4]], 7), vec![vec![0, 0], vec![2, 2], vec![-2, -2], vec![3, 3], vec![-3, -3], vec![4, 4], vec![-4, -4]]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::k_closest(vec![vec![-100, -100], vec![100, 100], vec![-200, -200], vec![200, 200], vec![-300, -300], vec![300, 300], vec![-400, -400], vec![400, 400], vec![-500, -500], vec![500, 500]], 7), vec![vec![-100, -100], vec![100, 100], vec![-200, -200], vec![200, 200], vec![-300, -300], vec![300, 300], vec![-400, -400]]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::k_closest(vec![vec![10, 0], vec![-10, 0], vec![0, 10], vec![0, -10], vec![5, 5], vec![-5, -5]], 2), vec![vec![5, 5], vec![-5, -5]]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::k_closest(vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4], vec![-5, -5], vec![-6, -6], vec![-7, -7], vec![-8, -8], vec![-9, -9], vec![-10, -10]], 7), vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4], vec![-5, -5], vec![-6, -6], vec![-7, -7]]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::k_closest(vec![vec![9999, 9999], vec![-9999, -9999], vec![0, 0], vec![5000, 5000], vec![-5000, -5000], vec![1, -1], vec![-1, 1]], 3), vec![vec![0, 0], vec![1, -1], vec![-1, 1]]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::k_closest(vec![vec![-5000, -5000], vec![-4000, -4000], vec![-3000, -3000], vec![-2000, -2000], vec![-1000, -1000], vec![0, 0], vec![1000, 1000], vec![2000, 2000], vec![3000, 3000], vec![4000, 4000], vec![5000, 5000], vec![10000, -10000], vec![-10000, 10000]], 6), vec![vec![0, 0], vec![-1000, -1000], vec![1000, 1000], vec![-2000, -2000], vec![2000, 2000], vec![-3000, -3000]]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::k_closest(vec![vec![-1, 1], vec![1, -1], vec![-2, 2], vec![2, -2], vec![-3, 3], vec![3, -3]], 3), vec![vec![-1, 1], vec![1, -1], vec![-2, 2]]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::k_closest(vec![vec![-5000, 5000], vec![5000, -5000], vec![0, 0], vec![-3000, -3000], vec![3000, 3000]], 3), vec![vec![0, 0], vec![-3000, -3000], vec![3000, 3000]]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::k_closest(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10]], 5), vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::k_closest(vec![vec![-1, -2], vec![-2, -3], vec![-3, -4], vec![-4, -5], vec![-5, -6], vec![-6, -7], vec![-7, -8], vec![-8, -9], vec![-9, -10], vec![-10, -11], vec![-11, -12], vec![-12, -13], vec![-13, -14]], 8), vec![vec![-1, -2], vec![-2, -3], vec![-3, -4], vec![-4, -5], vec![-5, -6], vec![-6, -7], vec![-7, -8], vec![-8, -9]]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::k_closest(vec![vec![1, 1], vec![-1, -1], vec![0, 0], vec![2, 2], vec![-2, -2], vec![3, 3], vec![-3, -3]], 4), vec![vec![0, 0], vec![1, 1], vec![-1, -1], vec![2, 2]]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::k_closest(vec![vec![100, 100], vec![200, 200], vec![300, 300], vec![400, 400], vec![500, 500], vec![-100, -100], vec![-200, -200], vec![-300, -300], vec![-400, -400], vec![-500, -500]], 6), vec![vec![100, 100], vec![-100, -100], vec![200, 200], vec![-200, -200], vec![300, 300], vec![-300, -300]]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::k_closest(vec![vec![-1000, 1000], vec![1000, -1000], vec![0, 0], vec![500, 500], vec![-500, -500], vec![250, 250], vec![-250, -250]], 4), vec![vec![0, 0], vec![250, 250], vec![-250, -250], vec![500, 500]]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::k_closest(vec![vec![-5000, 5000], vec![5000, -5000], vec![10000, 0], vec![0, 10000], vec![-10000, 0], vec![0, -10000]], 3), vec![vec![-5000, 5000], vec![5000, -5000], vec![10000, 0]]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::k_closest(vec![vec![-1000, 0], vec![1000, 0], vec![0, -1000], vec![0, 1000], vec![-500, -500], vec![500, 500], vec![-500, 500], vec![500, -500], vec![250, 250], vec![750, 750]], 6), vec![vec![250, 250], vec![-500, -500], vec![500, 500], vec![-500, 500], vec![500, -500], vec![-1000, 0]]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::k_closest(vec![vec![100, 100], vec![-100, -100], vec![0, 0], vec![200, 200], vec![-200, -200], vec![50, 50], vec![-50, -50], vec![75, 75], vec![-75, -75]], 6), vec![vec![0, 0], vec![50, 50], vec![-50, -50], vec![75, 75], vec![-75, -75], vec![100, 100]]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::k_closest(vec![vec![-9999, 10000], vec![10000, -9999], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7]], 5), vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4]]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::k_closest(vec![vec![1, -1], vec![-1, 1], vec![1, 1], vec![-1, -1], vec![0, 0], vec![2, 2], vec![-2, -2], vec![2, -2], vec![-2, 2]], 4), vec![vec![0, 0], vec![1, -1], vec![-1, 1], vec![1, 1]]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::k_closest(vec![vec![-1, 1], vec![-2, 2], vec![-3, 3], vec![-4, 4], vec![-5, 5], vec![-6, 6], vec![-7, 7], vec![-8, 8], vec![-9, 9], vec![-10, 10]], 5), vec![vec![-1, 1], vec![-2, 2], vec![-3, 3], vec![-4, 4], vec![-5, 5]]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::k_closest(vec![vec![10000, 10000], vec![-10000, -10000], vec![5000, 5000], vec![-5000, -5000], vec![0, 0]], 3), vec![vec![0, 0], vec![5000, 5000], vec![-5000, -5000]]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1], vec![-1, -1], vec![-1, 0], vec![0, -1], vec![1, -1], vec![-1, 1], vec![0, 2], vec![2, 0]], 5), vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![-1, 0], vec![0, -1]]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::k_closest(vec![vec![-5000, -5000], vec![5000, 5000], vec![0, 0], vec![100, 100], vec![-100, -100]], 4), vec![vec![0, 0], vec![100, 100], vec![-100, -100], vec![-5000, -5000]]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::k_closest(vec![vec![-1, -1], vec![-1, 1], vec![1, -1], vec![1, 1], vec![0, 0], vec![0, 1], vec![1, 0], vec![0, -1], vec![-2, 2], vec![2, -2]], 5), vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, -1]]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::k_closest(vec![vec![-1, -1], vec![0, 0], vec![1, 1], vec![-2, -2], vec![0, 1], vec![1, 0], vec![-3, -3], vec![0, 2], vec![2, 0], vec![-4, -4]], 5), vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![-1, -1], vec![1, 1]]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::k_closest(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]], 5), vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::k_closest(vec![vec![500, 500], vec![-500, -500], vec![500, -500], vec![-500, 500], vec![0, 0], vec![1000, 0], vec![0, 1000], vec![-1000, 0], vec![0, -1000]], 5), vec![vec![0, 0], vec![500, 500], vec![-500, -500], vec![500, -500], vec![-500, 500]]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::k_closest(vec![vec![-5000, -5000], vec![-7000, 7000], vec![8000, -8000], vec![6000, 6000]], 2), vec![vec![-5000, -5000], vec![6000, 6000]]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::k_closest(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12]], 3), vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::k_closest(vec![vec![100, 200], vec![200, 300], vec![300, 400], vec![400, 500], vec![500, 600], vec![600, 700], vec![700, 800], vec![800, 900]], 6), vec![vec![100, 200], vec![200, 300], vec![300, 400], vec![400, 500], vec![500, 600], vec![600, 700]]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::k_closest(vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4], vec![-5, -5], vec![-6, -6], vec![-7, -7], vec![-8, -8], vec![-9, -9], vec![-10, -10]], 4), vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4]]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::k_closest(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9]], 5), vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::k_closest(vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4], vec![-5, -5], vec![-6, -6], vec![-7, -7], vec![-8, -8], vec![-9, -9], vec![-10, -10]], 7), vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4], vec![-5, -5], vec![-6, -6], vec![-7, -7]]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::k_closest(vec![vec![-9000, 0], vec![0, -9000], vec![9000, 0], vec![0, 9000], vec![-4500, 4500], vec![4500, -4500], vec![-4500, -4500], vec![4500, 4500]], 4), vec![vec![-4500, 4500], vec![4500, -4500], vec![-4500, -4500], vec![4500, 4500]]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::k_closest(vec![vec![10, 10], vec![20, 20], vec![30, 30], vec![40, 40], vec![50, 50], vec![60, 60], vec![70, 70], vec![80, 80], vec![90, 90], vec![100, 100], vec![0, 0], vec![-10, -10], vec![-20, -20], vec![-30, -30], vec![-40, -40], vec![-50, -50]], 7), vec![vec![0, 0], vec![10, 10], vec![-10, -10], vec![20, 20], vec![-20, -20], vec![30, 30], vec![-30, -30]]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::k_closest(vec![vec![-9999, 9999], vec![9999, -9999], vec![1000, 1000], vec![-1000, -1000]], 2), vec![vec![1000, 1000], vec![-1000, -1000]]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::k_closest(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14]], 7), vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8]]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::k_closest(vec![vec![0, 10000], vec![-10000, 0], vec![10000, 0], vec![0, -10000], vec![2500, 2500], vec![-2500, -2500], vec![5000, -5000], vec![-5000, 5000], vec![7500, 7500]], 5), vec![vec![2500, 2500], vec![-2500, -2500], vec![5000, -5000], vec![-5000, 5000], vec![0, 10000]]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::k_closest(vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6], vec![0, 7], vec![0, 8], vec![0, 9], vec![0, 10]], 6), vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![0, 6]]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::k_closest(vec![vec![-10, -10], vec![-9, -9], vec![-8, -8], vec![-7, -7], vec![-6, -6], vec![-5, -5]], 3), vec![vec![-5, -5], vec![-6, -6], vec![-7, -7]]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::k_closest(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10], vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4], vec![-5, -5], vec![-6, -6], vec![-7, -7], vec![-8, -8], vec![-9, -9], vec![-10, -10]], 15), vec![vec![1, 1], vec![-1, -1], vec![2, 2], vec![-2, -2], vec![3, 3], vec![-3, -3], vec![4, 4], vec![-4, -4], vec![5, 5], vec![-5, -5], vec![6, 6], vec![-6, -6], vec![7, 7], vec![-7, -7], vec![8, 8]]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::k_closest(vec![vec![-1000, 1000], vec![1000, -1000], vec![500, 500], vec![-500, -500], vec![750, -750], vec![-750, 750]], 4), vec![vec![500, 500], vec![-500, -500], vec![750, -750], vec![-750, 750]]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::k_closest(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9]], 4), vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]);
}

#[test]
fn test_92() {
    assert_eq!(Solution::k_closest(vec![vec![-8, 6], vec![5, 0], vec![3, 1], vec![-9, 4], vec![2, -8], vec![-5, -3], vec![7, -7], vec![1, 9], vec![-4, -10]], 4), vec![vec![3, 1], vec![5, 0], vec![-5, -3], vec![2, -8]]);
}

#[test]
fn test_93() {
    assert_eq!(Solution::k_closest(vec![vec![0, 0], vec![1, 2], vec![2, 4], vec![3, 6], vec![4, 8], vec![5, 10], vec![6, 12], vec![7, 14], vec![8, 16], vec![9, 18], vec![10, 20]], 4), vec![vec![0, 0], vec![1, 2], vec![2, 4], vec![3, 6]]);
}

#[test]
fn test_94() {
    assert_eq!(Solution::k_closest(vec![vec![1000, 1000], vec![2000, 2000], vec![3000, 3000], vec![4000, 4000], vec![5000, 5000], vec![6000, 6000], vec![7000, 7000], vec![8000, 8000], vec![9000, 9000], vec![10000, 10000]], 5), vec![vec![1000, 1000], vec![2000, 2000], vec![3000, 3000], vec![4000, 4000], vec![5000, 5000]]);
}

#[test]
fn test_95() {
    assert_eq!(Solution::k_closest(vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4], vec![-5, -5], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]], 6), vec![vec![0, 0], vec![-1, -1], vec![1, 1], vec![-2, -2], vec![2, 2], vec![-3, -3]]);
}

#[test]
fn test_96() {
    assert_eq!(Solution::k_closest(vec![vec![-5, -12], vec![-10, -24], vec![-15, -36], vec![-20, -48], vec![-25, -60], vec![-30, -72], vec![-35, -84], vec![-40, -96], vec![-45, -108], vec![-50, -120]], 6), vec![vec![-5, -12], vec![-10, -24], vec![-15, -36], vec![-20, -48], vec![-25, -60], vec![-30, -72]]);
}

#[test]
fn test_97() {
    assert_eq!(Solution::k_closest(vec![vec![-3, 0], vec![2, 6], vec![0, 5], vec![-4, -4], vec![8, 3]], 3), vec![vec![-3, 0], vec![0, 5], vec![-4, -4]]);
}

#[test]
fn test_98() {
    assert_eq!(Solution::k_closest(vec![vec![-1, -1], vec![1, 1], vec![-2, -2], vec![2, 2], vec![-3, -3], vec![3, 3], vec![-4, -4], vec![4, 4], vec![-5, -5]], 4), vec![vec![-1, -1], vec![1, 1], vec![-2, -2], vec![2, 2]]);
}

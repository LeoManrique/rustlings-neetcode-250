include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![2, 2], vec![4, 4], vec![6, 6], vec![8, 8]]), 16);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 1], vec![0, 0], vec![2, 2], vec![3, 3], vec![4, 4]]), 8);
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1000000, 1000000], vec![1000000, -1000000], vec![0, 0]]), 4000000);
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -1], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3]]), 8);
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]]), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]]), 10);
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![5, 5], vec![10, 10], vec![15, 15], vec![20, 20], vec![25, 25]]), 40);
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1000000, -1000000], vec![1000000, 1000000]]), 4000000);
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4]]), 8);
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4]]), 6);
}

#[test]
fn test_11() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4]]), 6);
}

#[test]
fn test_12() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]]), 6);
}

#[test]
fn test_13() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![3, 12], vec![-2, 5], vec![-4, 1]]), 18);
}

#[test]
fn test_14() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1000000, -1000000], vec![-1000000, 1000000], vec![0, 0]]), 4000000);
}

#[test]
fn test_15() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]), 3);
}

#[test]
fn test_16() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]]), 3);
}

#[test]
fn test_17() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]]), 20);
}

#[test]
fn test_18() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 4);
}

#[test]
fn test_19() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9]]), 19);
}

#[test]
fn test_20() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![100, 200], vec![200, 300], vec![300, 400], vec![400, 500], vec![500, 600], vec![600, 700], vec![700, 800], vec![800, 900], vec![900, 1000]]), 1600);
}

#[test]
fn test_21() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![0, 1000000], vec![1000000, 0], vec![1000000, 1000000], vec![500000, 500000], vec![250000, 750000], vec![750000, 250000], vec![100000, 100000], vec![900000, 900000], vec![100000, 900000], vec![900000, 100000]]), 4000000);
}

#[test]
fn test_22() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2], vec![3, 3], vec![3, 4], vec![4, 3], vec![4, 4], vec![5, 5], vec![5, 6], vec![6, 5], vec![6, 6], vec![7, 7], vec![7, 8], vec![8, 7], vec![8, 8], vec![9, 9], vec![9, 10], vec![10, 9], vec![10, 10]]), 23);
}

#[test]
fn test_23() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, 0], vec![0, -1], vec![1, 0], vec![0, 1], vec![-2, 0], vec![0, -2], vec![2, 0], vec![0, 2], vec![-3, 0], vec![0, -3], vec![3, 0], vec![0, 3], vec![-4, 0], vec![0, -4], vec![4, 0], vec![0, 4]]), 18);
}

#[test]
fn test_24() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![10, 0], vec![0, 10], vec![10, 10], vec![20, 0], vec![20, 10], vec![30, 0], vec![30, 10], vec![40, 0], vec![40, 10]]), 90);
}

#[test]
fn test_25() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-5, 5], vec![0, 0], vec![5, -5], vec![10, 10], vec![-10, -10], vec![15, 15], vec![-15, -15]]), 80);
}

#[test]
fn test_26() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1], vec![2, 0], vec![2, 1], vec![3, 0], vec![3, 1], vec![4, 0], vec![4, 1], vec![5, 0], vec![5, 1], vec![6, 0], vec![6, 1], vec![7, 0], vec![7, 1], vec![8, 0], vec![8, 1], vec![9, 0], vec![9, 1]]), 19);
}

#[test]
fn test_27() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -1], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8]]), 18);
}

#[test]
fn test_28() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9]]), 18);
}

#[test]
fn test_29() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 1], vec![2, 3], vec![4, 5], vec![8, 10], vec![15, 20], vec![25, 30], vec![40, 50]]), 88);
}

#[test]
fn test_30() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4], vec![-5, -5], vec![5, 5], vec![4, 4], vec![3, 3], vec![2, 2], vec![1, 1], vec![0, 0]]), 20);
}

#[test]
fn test_31() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1000000, 1000000], vec![-1000000, -1000000], vec![0, 0], vec![500000, 500000], vec![-500000, -500000]]), 4000000);
}

#[test]
fn test_32() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-50, -100], vec![-150, -200], vec![-250, -300], vec![-350, -400], vec![-450, -500], vec![-550, -600], vec![-650, -700], vec![-750, -800], vec![-850, -900], vec![-950, -1000]]), 1800);
}

#[test]
fn test_33() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1000000, -1000000], vec![0, 0], vec![-1000000, 1000000], vec![500000, 500000], vec![-500000, -500000]]), 6000000);
}

#[test]
fn test_34() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![1, 1], vec![-1, 0], vec![0, -1], vec![-1, -1], vec![-1, 1], vec![1, -1], vec![2, 0], vec![0, 2], vec![-2, 0], vec![0, -2], vec![-2, -1], vec![-1, -2], vec![-2, 1], vec![1, -2]]), 16);
}

#[test]
fn test_35() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![10, 10], vec![20, 10], vec![20, 20], vec![10, 20], vec![15, 15], vec![5, 5], vec![25, 25], vec![10, 5], vec![10, 15], vec![5, 10]]), 60);
}

#[test]
fn test_36() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 2], vec![4, 6], vec![5, 7], vec![8, 9], vec![11, 10], vec![10, 11], vec![3, 5], vec![2, 3], vec![6, 8], vec![9, 4]]), 26);
}

#[test]
fn test_37() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1000000, 1000000], vec![1000000, -1000000], vec![0, 0], vec![-500000, 500000], vec![500000, -500000]]), 4000000);
}

#[test]
fn test_38() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 1000000], vec![1000000, 1], vec![1, 2], vec![2, 1], vec![1000000, 2], vec![2, 1000000], vec![1000000, 1000000]]), 2999998);
}

#[test]
fn test_39() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 2], vec![4, 8], vec![1, 7], vec![6, 1], vec![9, 7], vec![7, 4], vec![9, 1], vec![7, 9], vec![9, 2], vec![7, 6]]), 29);
}

#[test]
fn test_40() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-500000, -500000], vec![0, 0], vec![500000, 500000], vec![1000000, 1000000], vec![1500000, 1500000]]), 4000000);
}

#[test]
fn test_41() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-5, -5], vec![-4, -4], vec![-3, -3], vec![-2, -2], vec![-1, -1], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]), 20);
}

#[test]
fn test_42() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![10, 10], vec![20, 10], vec![30, 10], vec![40, 10], vec![50, 10], vec![60, 10], vec![70, 10], vec![80, 10], vec![90, 10], vec![100, 10]]), 90);
}

#[test]
fn test_43() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 2], vec![4, 6], vec![5, 7], vec![8, 4], vec![2, 1], vec![5, 5], vec![7, 8], vec![8, 9], vec![1, 4], vec![2, 8], vec![7, 1], vec![9, 5]]), 32);
}

#[test]
fn test_44() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -2], vec![-3, -4], vec![-5, -6], vec![-7, -8], vec![-9, -10], vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]), 38);
}

#[test]
fn test_45() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-3, -3], vec![-2, -2], vec![-1, -1], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10], vec![11, 11], vec![12, 12], vec![13, 13], vec![14, 14]]), 34);
}

#[test]
fn test_46() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 1], vec![2, 3], vec![3, 1], vec![4, 3], vec![5, 1], vec![6, 3], vec![7, 1], vec![8, 3], vec![9, 1], vec![10, 3]]), 19);
}

#[test]
fn test_47() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1], vec![2, 0], vec![2, 1], vec![3, 0], vec![3, 1], vec![4, 0], vec![4, 1]]), 9);
}

#[test]
fn test_48() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![100, 100], vec![200, 200], vec![300, 300], vec![400, 400], vec![500, 500], vec![150, 250], vec![250, 150], vec![350, 450], vec![450, 350], vec![550, 450]]), 1300);
}

#[test]
fn test_49() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1], vec![2, 2], vec![2, 0], vec![0, 2], vec![1, 2], vec![2, 1], vec![0, 3], vec![3, 0], vec![3, 3]]), 12);
}

#[test]
fn test_50() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 2], vec![4, 8], vec![5, 0], vec![7, 3], vec![8, 9], vec![9, 6], vec![9, 7], vec![10, 4], vec![10, 8], vec![11, 2]]), 32);
}

#[test]
fn test_51() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1], vec![0, 2], vec![1, 2], vec![2, 0], vec![2, 1], vec![2, 2], vec![3, 0], vec![3, 1], vec![3, 2]]), 11);
}

#[test]
fn test_52() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![100, 200], vec![300, 400], vec![500, 600], vec![700, 800], vec![900, 1000], vec![1100, 1200], vec![1300, 1400], vec![1500, 1600]]), 2800);
}

#[test]
fn test_53() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 1], vec![2, 3], vec![3, 2], vec![4, 5], vec![5, 4], vec![6, 7], vec![7, 6], vec![8, 9], vec![9, 8], vec![10, 10]]), 26);
}

#[test]
fn test_54() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![10, 10], vec![20, 10], vec![20, 20], vec![10, 20], vec![30, 10], vec![30, 20], vec![40, 10], vec![40, 20], vec![15, 15], vec![25, 15], vec![35, 15], vec![15, 25], vec![25, 25], vec![35, 25]]), 130);
}

#[test]
fn test_55() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 0], vec![7, 0], vec![8, 0], vec![9, 0], vec![10, 0], vec![11, 0], vec![12, 0], vec![13, 0], vec![14, 0]]), 14);
}

#[test]
fn test_56() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -1], vec![1, 1], vec![0, 0], vec![2, 2], vec![-2, -2], vec![3, 3], vec![-3, -3], vec![4, 4], vec![-4, -4], vec![5, 5], vec![-5, -5], vec![6, 6], vec![-6, -6]]), 24);
}

#[test]
fn test_57() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-100, 0], vec![0, 100], vec![100, 0], vec![0, -100], vec![50, 50], vec![-50, -50], vec![50, -50], vec![-50, 50], vec![0, 0], vec![0, 0]]), 800);
}

#[test]
fn test_58() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 2], vec![2, 4], vec![4, 8], vec![8, 16], vec![16, 32], vec![32, 64], vec![64, 128], vec![128, 256], vec![256, 512]]), 768);
}

#[test]
fn test_59() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 2], vec![2, 4], vec![3, 6], vec![4, 8], vec![5, 10], vec![6, 12], vec![7, 14], vec![8, 16], vec![9, 18], vec![10, 20], vec![11, 22], vec![12, 24], vec![13, 26], vec![14, 28]]), 42);
}

#[test]
fn test_60() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![500000, 500000], vec![400000, 400000], vec![300000, 300000], vec![200000, 200000], vec![100000, 100000], vec![0, 0], vec![-100000, -100000], vec![-200000, -200000], vec![-300000, -300000], vec![-400000, -400000], vec![-500000, -500000]]), 2000000);
}

#[test]
fn test_61() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 2], vec![3, 1], vec![2, 3], vec![4, 5], vec![6, 4], vec![5, 6], vec![7, 8], vec![9, 7], vec![8, 9], vec![10, 10], vec![12, 11], vec![11, 12], vec![13, 14], vec![15, 13], vec![14, 15], vec![16, 16], vec![18, 17], vec![17, 18], vec![19, 19]]), 54);
}

#[test]
fn test_62() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![10, 20], vec![30, 40], vec![50, 60], vec![70, 80], vec![90, 100], vec![110, 120], vec![130, 140], vec![150, 160], vec![170, 180], vec![190, 200]]), 360);
}

#[test]
fn test_63() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -1], vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9]]), 20);
}

#[test]
fn test_64() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1000000, 0], vec![0, 1000000], vec![500000, 500000], vec![250000, 750000], vec![750000, 250000]]), 2000000);
}

#[test]
fn test_65() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10]]), 21);
}

#[test]
fn test_66() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18], vec![19, 20]]), 36);
}

#[test]
fn test_67() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1000000, 1000000], vec![1000001, 1000001], vec![1000002, 1000002], vec![1000003, 1000003], vec![1000004, 1000004]]), 8);
}

#[test]
fn test_68() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![100, 200], vec![200, 300], vec![300, 400], vec![400, 500], vec![500, 600], vec![600, 700], vec![700, 800], vec![800, 900], vec![900, 1000], vec![1000, 1100]]), 1800);
}

#[test]
fn test_69() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1000000, 1000000], vec![500000, 500000], vec![250000, 250000], vec![750000, 750000], vec![125000, 125000], vec![875000, 875000], vec![312500, 312500], vec![687500, 687500], vec![437500, 437500]]), 2000000);
}

#[test]
fn test_70() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-200000, 200000], vec![-400000, 400000], vec![-600000, 600000], vec![-800000, 800000], vec![-1000000, 1000000], vec![200000, -200000], vec![400000, -400000], vec![600000, -600000], vec![800000, -800000], vec![1000000, -1000000]]), 4000000);
}

#[test]
fn test_71() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![4, 3], vec![5, 6], vec![6, 5], vec![7, 8], vec![8, 7], vec![9, 10], vec![10, 9], vec![11, 12], vec![12, 11]]), 32);
}

#[test]
fn test_72() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1], vec![2, 2], vec![2, 0], vec![0, 2]]), 7);
}

#[test]
fn test_73() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![0, 1000000], vec![1000000, 0], vec![1000000, 1000000], vec![500000, 500000]]), 4000000);
}

#[test]
fn test_74() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 1], vec![2, 5], vec![3, 6], vec![4, 7], vec![5, 8], vec![6, 9], vec![7, 10], vec![8, 1], vec![9, 2], vec![10, 3]]), 42);
}

#[test]
fn test_75() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -1], vec![-2, -2], vec![-3, -3], vec![-4, -4], vec![-5, -5], vec![-6, -6], vec![-7, -7], vec![-8, -8], vec![-9, -9], vec![-10, -10]]), 18);
}

#[test]
fn test_76() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, 2], vec![-4, 6], vec![-5, 7], vec![-8, 9], vec![-11, 10], vec![-10, -11], vec![-3, -5], vec![-2, -3], vec![-6, -8], vec![-9, -4]]), 47);
}

#[test]
fn test_77() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1], vec![2, 2], vec![-2, 2], vec![2, -2], vec![-2, -2], vec![3, 3], vec![-3, 3], vec![3, -3], vec![-3, -3]]), 26);
}

#[test]
fn test_78() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 0], vec![7, 0], vec![8, 0], vec![9, 0], vec![10, 0], vec![11, 0], vec![12, 0], vec![13, 0], vec![14, 0], vec![15, 0]]), 15);
}

#[test]
fn test_79() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1000000, 1000000], vec![1000000, -1000000], vec![-500000, -500000], vec![500000, 500000], vec![0, 0], vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]]), 5999999);
}

#[test]
fn test_80() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![500000, 500000], vec![500001, 500001], vec![500002, 500002], vec![500003, 500003], vec![500004, 500004]]), 8);
}

#[test]
fn test_81() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 2], vec![2, 4], vec![3, 6], vec![4, 8], vec![5, 10], vec![6, 12], vec![7, 14], vec![8, 16], vec![9, 18]]), 27);
}

#[test]
fn test_82() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 3], vec![2, 6], vec![3, 9], vec![4, 12], vec![5, 15], vec![6, 18], vec![7, 21], vec![8, 24], vec![9, 27], vec![10, 30], vec![11, 33], vec![12, 36], vec![13, 39], vec![14, 42]]), 56);
}

#[test]
fn test_83() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9]]), 18);
}

#[test]
fn test_84() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![10, 10], vec![10, 20], vec![10, 30], vec![20, 10], vec![20, 20], vec![20, 30], vec![30, 10], vec![30, 20], vec![30, 30], vec![15, 15], vec![25, 25]]), 100);
}

#[test]
fn test_85() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-3, -3], vec![3, 3], vec![0, 0], vec![-1, 1], vec![1, -1], vec![-2, 2], vec![2, -2], vec![-3, 0], vec![3, 0], vec![0, -3], vec![0, 3], vec![-2, -1], vec![2, 1], vec![1, 2], vec![-1, -2], vec![-2, 1], vec![2, -1], vec![1, -2], vec![-1, 2]]), 32);
}

#[test]
fn test_86() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -1], vec![-2, -2], vec![1, 1], vec![2, 2], vec![-3, -3], vec![3, 3], vec![-4, -4], vec![4, 4], vec![-5, -5], vec![5, 5], vec![-6, -6], vec![6, 6], vec![-7, -7], vec![7, 7], vec![-8, -8], vec![8, 8], vec![-9, -9], vec![9, 9]]), 36);
}

#[test]
fn test_87() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1], vec![0, 2], vec![2, 0], vec![1, 2], vec![2, 1], vec![2, 2], vec![0, 3], vec![3, 0], vec![1, 3], vec![3, 1], vec![2, 3], vec![3, 2], vec![3, 3]]), 15);
}

#[test]
fn test_88() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![1, 0], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9]]), 19);
}

#[test]
fn test_89() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-500000, -500000], vec![-500001, -500001], vec![-500002, -500002], vec![-500003, -500003], vec![-500004, -500004]]), 8);
}

#[test]
fn test_90() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20]]), 38);
}

#[test]
fn test_91() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 0], vec![6, 0], vec![7, 0], vec![8, 0], vec![9, 0], vec![10, 0], vec![11, 0], vec![12, 0], vec![13, 0], vec![14, 0], vec![15, 0], vec![16, 0], vec![17, 0], vec![18, 0], vec![19, 0]]), 19);
}

#[test]
fn test_92() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1000000, 1000000], vec![999999, 1000000], vec![1000000, 999999], vec![1000000, 1000001], vec![999999, 999999], vec![999999, 1000001], vec![1000001, 999999], vec![1000001, 1000001]]), 7);
}

#[test]
fn test_93() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![0, 1000000], vec![1000000, 0], vec![1000000, 1000000], vec![500000, 500000], vec![250000, 250000], vec![750000, 750000], vec![250000, 750000], vec![750000, 250000]]), 4000000);
}

#[test]
fn test_94() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 1], vec![2, 1], vec![3, 1], vec![4, 1], vec![5, 1], vec![6, 1], vec![7, 1], vec![8, 1], vec![9, 1], vec![10, 1], vec![1, 2], vec![2, 2], vec![3, 2], vec![4, 2], vec![5, 2], vec![6, 2], vec![7, 2], vec![8, 2], vec![9, 2], vec![10, 2]]), 19);
}

#[test]
fn test_95() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-100, -200], vec![-50, -50], vec![0, 0], vec![50, 50], vec![100, 200], vec![200, 400], vec![400, 800], vec![800, 1600], vec![1600, 3200], vec![3200, 6400]]), 9900);
}

#[test]
fn test_96() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1000000, -1000000], vec![-500000, -500000], vec![0, 0], vec![500000, 500000], vec![1000000, 1000000]]), 4000000);
}

#[test]
fn test_97() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![1, 2], vec![4, 6], vec![5, 7], vec![8, 4], vec![2, 1], vec![6, 9], vec![7, 8], vec![3, 5], vec![9, 2], vec![0, 3]]), 26);
}

#[test]
fn test_98() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-500000, -500000], vec![500000, 500000], vec![-400000, -400000], vec![400000, 400000], vec![-300000, -300000], vec![300000, 300000], vec![-200000, -200000], vec![200000, 200000], vec![-100000, -100000], vec![100000, 100000]]), 2000000);
}

#[test]
fn test_99() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![500000, 500000], vec![100000, 100000], vec![900000, 900000], vec![200000, 200000], vec![800000, 800000], vec![300000, 300000], vec![700000, 700000], vec![400000, 400000], vec![600000, 600000], vec![550000, 550000]]), 1600000);
}

#[test]
fn test_100() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10], vec![11, 11], vec![12, 12], vec![13, 13], vec![14, 14], vec![15, 15], vec![16, 16], vec![17, 17], vec![18, 18], vec![19, 19]]), 38);
}

#[test]
fn test_101() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![3, 1], vec![1, 4], vec![1, 1], vec![1, 3], vec![2, 6], vec![3, 5], vec![6, 5], vec![4, 3], vec![7, 4], vec![4, 6], vec![5, 4], vec![5, 5], vec![6, 7], vec![8, 4], vec![2, 3]]), 23);
}

#[test]
fn test_102() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![-1, -1], vec![1, 1], vec![-2, -2], vec![2, 2], vec![-3, -3], vec![3, 3], vec![-4, -4], vec![4, 4], vec![-5, -5], vec![5, 5]]), 20);
}

#[test]
fn test_103() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![10, 0], vec![0, 10], vec![10, 10], vec![5, 5], vec![3, 7], vec![8, 2], vec![2, 8], vec![7, 3], vec![4, 6], vec![6, 4]]), 40);
}

#[test]
fn test_104() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0], vec![10, 0], vec![10, 10], vec![0, 10], vec![5, 5], vec![2, 2], vec![8, 8], vec![3, 3], vec![7, 7], vec![6, 6]]), 40);
}

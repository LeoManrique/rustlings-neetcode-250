include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 10], vec![1, 3, 1]], 3, 1), 10);
}

#[test]
fn test_2() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 4]], 3, 1), 3);
}

#[test]
fn test_4() {
    assert_eq!(Solution::network_delay_time(vec![vec![3, 1, 5], vec![3, 2, 2], vec![2, 1, 2], vec![3, 4, 1], vec![4, 5, 1], vec![5, 3, 1], vec![1, 5, 4]], 5, 3), 4);
}

#[test]
fn test_5() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 3, 1], vec![2, 1, 3], vec![2, 3, 1]], 3, 2), 3);
}

#[test]
fn test_6() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 1], vec![2, 3, 1], vec![2, 4, 1], vec![3, 4, 1], vec![4, 5, 1]], 5, 1), 3);
}

#[test]
fn test_7() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
}

#[test]
fn test_8() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 7], vec![1, 3, 4], vec![2, 1, 2]], 3, 2), 6);
}

#[test]
fn test_9() {
    assert_eq!(Solution::network_delay_time(vec![vec![3, 1, 5], vec![3, 2, 5], vec![3, 5, 2], vec![2, 1, 2], vec![2, 4, 1], vec![4, 3, 1], vec![5, 4, 1], vec![4, 5, 3], vec![4, 1, 6]], 5, 3), 5);
}

#[test]
fn test_10() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 1, 3]], 2, 1), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2), 2);
}

#[test]
fn test_12() {
    assert_eq!(Solution::network_delay_time(vec![vec![3, 1, 5], vec![3, 2, 5], vec![1, 2, 2], vec![2, 1, 3], vec![4, 3, 1]], 4, 3), -1);
}

#[test]
fn test_13() {
    assert_eq!(Solution::network_delay_time(vec![vec![3, 1, 5], vec![3, 2, 2], vec![2, 1, 2], vec![3, 4, 1], vec![4, 5, 1], vec![5, 2, 5]], 5, 3), 4);
}

#[test]
fn test_14() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![1, 3, 5], vec![1, 4, 8], vec![2, 3, 2], vec![2, 4, 6], vec![3, 4, 1]], 4, 1), 10);
}

#[test]
fn test_15() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 1, 5]], 5, 1), 10);
}

#[test]
fn test_16() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![1, 4, 3], vec![1, 5, 4], vec![1, 6, 5], vec![1, 7, 6], vec![1, 8, 7], vec![1, 9, 8], vec![1, 10, 9], vec![2, 10, 1], vec![3, 10, 2], vec![4, 10, 3], vec![5, 10, 4], vec![6, 10, 5], vec![7, 10, 6], vec![8, 10, 7], vec![9, 10, 8]], 10, 1), 8);
}

#[test]
fn test_17() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 4], vec![2, 4, 2], vec![3, 4, 3], vec![4, 5, 3]], 5, 1), 6);
}

#[test]
fn test_18() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![1, 3, 10], vec![1, 4, 10], vec![1, 5, 10], vec![1, 6, 10], vec![1, 7, 10], vec![1, 8, 10], vec![1, 9, 10], vec![1, 10, 10]], 10, 1), 10);
}

#[test]
fn test_19() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![1, 3, 20], vec![2, 4, 5], vec![3, 5, 10], vec![4, 6, 20], vec![5, 7, 30], vec![6, 8, 40], vec![7, 9, 50], vec![8, 10, 60]], 10, 1), 135);
}

#[test]
fn test_20() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 4], vec![1, 4, 5], vec![2, 3, 2], vec![2, 4, 4], vec![3, 4, 1], vec![4, 5, 3], vec![5, 6, 2], vec![6, 7, 1]], 7, 1), 10);
}

#[test]
fn test_21() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 20], vec![3, 4, 30], vec![4, 5, 40], vec![5, 6, 50], vec![6, 7, 60], vec![7, 8, 70], vec![8, 9, 80], vec![9, 10, 90], vec![10, 11, 100], vec![11, 12, 110], vec![12, 13, 120]], 13, 1), 780);
}

#[test]
fn test_22() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![2, 4, 1], vec![3, 4, 2], vec![4, 5, 1], vec![2, 5, 2], vec![3, 5, 3]], 5, 1), 3);
}

#[test]
fn test_23() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 3], vec![1, 3, 5], vec![2, 3, 1], vec![3, 4, 4], vec![4, 5, 3], vec![5, 6, 2], vec![6, 7, 1], vec![7, 8, 5], vec![8, 9, 3], vec![9, 10, 2]], 10, 1), 24);
}

#[test]
fn test_24() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 2], vec![1, 3, 3], vec![2, 4, 5], vec![2, 5, 4], vec![3, 4, 1], vec![3, 5, 2], vec![4, 6, 3], vec![5, 6, 3], vec![6, 7, 4], vec![6, 8, 5], vec![7, 9, 6], vec![8, 9, 5], vec![9, 10, 7]], 10, 1), 24);
}

#[test]
fn test_25() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 5], vec![2, 3, 3], vec![3, 4, 2], vec![4, 5, 1], vec![5, 6, 1], vec![6, 7, 2], vec![7, 8, 3], vec![8, 9, 4], vec![9, 10, 5]], 10, 1), 26);
}

#[test]
fn test_26() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![1, 3, 20], vec![1, 4, 30], vec![1, 5, 40], vec![1, 6, 50], vec![1, 7, 60], vec![1, 8, 70], vec![1, 9, 80], vec![1, 10, 90]], 10, 1), 90);
}

#[test]
fn test_27() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 1], vec![4, 5, 1], vec![5, 1, 1], vec![2, 4, 1], vec![3, 5, 1], vec![4, 1, 1], vec![5, 2, 1]], 5, 1), 3);
}

#[test]
fn test_28() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10], vec![5, 6, 10], vec![6, 7, 10], vec![7, 8, 10], vec![8, 9, 10], vec![9, 10, 10], vec![1, 10, 1]], 10, 1), 80);
}

#[test]
fn test_29() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 30], vec![2, 3, 40], vec![3, 4, 50], vec![4, 5, 60], vec![5, 1, 20]], 5, 5), 140);
}

#[test]
fn test_30() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 4], vec![1, 3, 2], vec![2, 4, 7], vec![2, 5, 3], vec![3, 4, 5], vec![3, 5, 4], vec![4, 6, 2], vec![5, 6, 3]], 6, 1), 9);
}

#[test]
fn test_31() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9], vec![10, 11, 10], vec![11, 12, 11], vec![12, 13, 12], vec![13, 14, 13], vec![14, 1, 14]], 14, 1), 91);
}

#[test]
fn test_32() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 1], vec![4, 5, 1], vec![5, 6, 1], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1], vec![10, 11, 1], vec![11, 12, 1], vec![12, 13, 1], vec![13, 14, 1], vec![14, 15, 1]], 15, 1), 14);
}

#[test]
fn test_33() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 3], vec![1, 3, 4], vec![2, 4, 5], vec![3, 5, 10], vec![4, 6, 2], vec![5, 7, 3], vec![6, 8, 1], vec![7, 9, 4], vec![8, 10, 5]], 10, 1), 21);
}

#[test]
fn test_34() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 20], vec![3, 4, 30], vec![4, 5, 40], vec![5, 1, 50]], 5, 1), 100);
}

#[test]
fn test_35() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![1, 3, 4], vec![2, 3, 3], vec![2, 5, 2], vec![3, 5, 1], vec![3, 4, 5], vec![4, 5, 6], vec![5, 6, 1]], 6, 1), 10);
}

#[test]
fn test_36() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 4], vec![1, 3, 2], vec![2, 3, 3], vec![2, 4, 1], vec![3, 4, 2], vec![4, 5, 5], vec![4, 6, 3], vec![5, 6, 2]], 6, 1), 9);
}

#[test]
fn test_37() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 1], vec![3, 1, 1], vec![1, 4, 1], vec![4, 5, 1], vec![5, 6, 1], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1]], 10, 1), 7);
}

#[test]
fn test_38() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 1, 5], vec![6, 7, 1], vec![7, 8, 2], vec![8, 9, 3], vec![9, 10, 4], vec![10, 6, 5]], 10, 1), -1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 4], vec![2, 3, 6], vec![1, 3, 5], vec![3, 4, 2], vec![4, 5, 1], vec![5, 6, 3], vec![6, 7, 4], vec![7, 1, 8]], 7, 1), 15);
}

#[test]
fn test_40() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 2], vec![1, 3, 1], vec![3, 2, 1], vec![2, 4, 3], vec![3, 4, 4], vec![4, 5, 5], vec![5, 3, 6]], 5, 1), 10);
}

#[test]
fn test_41() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 1], vec![2, 4, 1], vec![2, 5, 1], vec![3, 4, 1], vec![3, 5, 1], vec![4, 6, 1], vec![4, 7, 1], vec![5, 6, 1], vec![5, 7, 1], vec![6, 8, 1], vec![6, 9, 1], vec![7, 8, 1], vec![7, 9, 1], vec![8, 10, 1], vec![9, 10, 1], vec![10, 1, 10]], 10, 1), 5);
}

#[test]
fn test_42() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 20], vec![3, 4, 10], vec![4, 5, 20], vec![1, 3, 5], vec![3, 1, 5]], 5, 1), 35);
}

#[test]
fn test_43() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 1, 9]], 9, 1), 36);
}

#[test]
fn test_44() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 20], vec![3, 4, 30], vec![4, 5, 40], vec![5, 6, 50], vec![6, 7, 60], vec![7, 8, 70], vec![8, 1, 80]], 8, 1), 280);
}

#[test]
fn test_45() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 1], vec![2, 4, 1], vec![3, 4, 1], vec![4, 5, 1], vec![1, 5, 1], vec![2, 5, 1], vec![3, 5, 1], vec![4, 1, 1], vec![5, 1, 1]], 5, 1), 2);
}

#[test]
fn test_46() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 100], vec![2, 3, 99], vec![3, 4, 98], vec![4, 5, 97], vec![5, 6, 96], vec![6, 7, 95], vec![7, 8, 94], vec![8, 9, 93], vec![9, 10, 92]], 10, 1), 864);
}

#[test]
fn test_47() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 5], vec![1, 3, 15], vec![3, 4, 10], vec![2, 4, 8], vec![4, 5, 2], vec![5, 6, 5], vec![6, 1, 7]], 6, 1), 25);
}

#[test]
fn test_48() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![2, 4, 3], vec![2, 5, 4], vec![3, 6, 5], vec![3, 7, 6], vec![4, 8, 7], vec![4, 9, 8], vec![5, 10, 9], vec![5, 11, 10], vec![6, 12, 11], vec![7, 12, 12]], 12, 1), 18);
}

#[test]
fn test_49() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 2], vec![2, 3, 3], vec![3, 4, 4], vec![4, 5, 5], vec![1, 5, 10], vec![5, 1, 10]], 5, 1), 10);
}

#[test]
fn test_50() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 1, 5], vec![2, 5, 1], vec![3, 1, 2], vec![4, 2, 3], vec![5, 3, 4]], 5, 1), 6);
}

#[test]
fn test_51() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 2], vec![2, 3, 4], vec![3, 4, 6], vec![4, 5, 8], vec![5, 6, 10], vec![6, 7, 12], vec![7, 8, 14], vec![8, 9, 16], vec![9, 10, 18], vec![10, 1, 20]], 10, 1), 90);
}

#[test]
fn test_52() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 1], vec![1, 4, 1], vec![1, 5, 1], vec![2, 3, 1], vec![2, 4, 1], vec![2, 5, 1], vec![3, 4, 1], vec![3, 5, 1], vec![4, 5, 1]], 5, 1), 1);
}

#[test]
fn test_53() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9], vec![10, 1, 10]], 10, 10), 45);
}

#[test]
fn test_54() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9], vec![10, 1, 10]], 10, 1), 45);
}

#[test]
fn test_55() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 2], vec![1, 3, 4], vec![1, 4, 1], vec![2, 3, 1], vec![2, 4, 5], vec![3, 4, 2], vec![4, 5, 3], vec![5, 6, 2], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1], vec![10, 11, 1], vec![11, 12, 1], vec![12, 1, 1], vec![1, 5, 2]], 12, 1), 10);
}

#[test]
fn test_56() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 100], vec![1, 3, 200], vec![2, 4, 50], vec![2, 5, 150], vec![3, 5, 100], vec![4, 6, 200], vec![5, 7, 250], vec![6, 8, 300], vec![7, 9, 350], vec![8, 10, 400]], 10, 1), 1050);
}

#[test]
fn test_57() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 1, 10], vec![1, 3, 20], vec![3, 1, 20], vec![2, 4, 30], vec![4, 2, 30], vec![3, 4, 40], vec![4, 3, 40], vec![1, 4, 60], vec![4, 1, 60]], 4, 1), 40);
}

#[test]
fn test_58() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![1, 3, 3], vec![2, 3, 1], vec![2, 4, 5], vec![3, 4, 2], vec![3, 5, 8], vec![4, 5, 7], vec![5, 6, 1]], 6, 1), 12);
}

#[test]
fn test_59() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 1], vec![2, 4, 1], vec![3, 4, 1], vec![4, 5, 1], vec![2, 5, 1], vec![3, 5, 1], vec![4, 1, 1], vec![5, 1, 1], vec![1, 4, 1]], 5, 2), 3);
}

#[test]
fn test_60() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10], vec![5, 6, 10], vec![6, 7, 10], vec![7, 8, 10], vec![8, 9, 10], vec![9, 10, 10], vec![10, 1, 10]], 10, 1), 90);
}

#[test]
fn test_61() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![1, 4, 3], vec![2, 3, 1], vec![2, 4, 2], vec![3, 4, 1], vec![4, 5, 1]], 5, 1), 4);
}

#[test]
fn test_62() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 1, 2], vec![2, 4, 3], vec![4, 5, 4], vec![5, 2, 3]], 5, 1), 8);
}

#[test]
fn test_63() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 1], vec![4, 5, 1], vec![5, 1, 1], vec![1, 6, 1], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1]], 10, 1), 5);
}

#[test]
fn test_64() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![1, 3, 10], vec![2, 4, 10], vec![2, 5, 10], vec![3, 6, 10], vec![3, 7, 10], vec![4, 8, 10], vec![4, 9, 10], vec![5, 10, 10], vec![5, 11, 10], vec![6, 12, 10], vec![6, 13, 10]], 13, 1), 30);
}

#[test]
fn test_65() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 1], vec![2, 4, 1], vec![2, 5, 1], vec![3, 4, 1], vec![3, 5, 1], vec![4, 6, 1], vec![4, 7, 1], vec![5, 6, 1], vec![5, 7, 1], vec![6, 8, 1], vec![6, 9, 1], vec![7, 8, 1], vec![7, 9, 1], vec![8, 10, 1], vec![9, 10, 1]], 10, 1), 5);
}

#[test]
fn test_66() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 2], vec![3, 4, 1], vec![4, 5, 2], vec![2, 4, 3], vec![5, 4, 5], vec![4, 2, 2]], 5, 1), 6);
}

#[test]
fn test_67() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 1], vec![1, 4, 1], vec![1, 5, 1], vec![2, 3, 2], vec![2, 4, 2], vec![2, 5, 2], vec![3, 4, 3], vec![3, 5, 3], vec![4, 5, 4]], 5, 1), 1);
}

#[test]
fn test_68() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10], vec![5, 6, 10], vec![6, 7, 10], vec![7, 8, 10], vec![8, 9, 10], vec![9, 10, 10]], 10, 1), 90);
}

#[test]
fn test_69() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 5], vec![1, 4, 10], vec![2, 3, 10], vec![2, 4, 30], vec![3, 4, 5], vec![3, 5, 15], vec![4, 5, 10], vec![5, 6, 5], vec![6, 7, 10], vec![7, 8, 15], vec![8, 9, 20], vec![9, 10, 25]], 10, 3), -1);
}

#[test]
fn test_70() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 3], vec![1, 3, 5], vec![2, 3, 1], vec![2, 4, 4], vec![3, 4, 2], vec![4, 5, 3], vec![5, 6, 2], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1], vec![10, 11, 1], vec![11, 12, 1], vec![12, 1, 1]], 12, 1), 17);
}

#[test]
fn test_71() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 4], vec![2, 3, 3], vec![3, 4, 2], vec![4, 5, 1], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 1, 8]], 8, 1), 28);
}

#[test]
fn test_72() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 4, 2], vec![1, 2, 4], vec![2, 3, 3], vec![3, 4, 1], vec![4, 5, 2], vec![5, 6, 1], vec![6, 7, 5], vec![7, 8, 3], vec![8, 9, 2], vec![9, 10, 1]], 10, 1), 16);
}

#[test]
fn test_73() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![1, 4, 3], vec![2, 3, 4], vec![2, 4, 5], vec![3, 4, 6], vec![4, 5, 7], vec![5, 6, 8], vec![6, 7, 9], vec![7, 8, 10]], 8, 1), 37);
}

#[test]
fn test_74() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9], vec![10, 1, 10]], 10, 5), 51);
}

#[test]
fn test_75() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 9], vec![3, 4, 8], vec![4, 5, 7], vec![5, 6, 6], vec![6, 7, 5], vec![7, 8, 4], vec![8, 9, 3], vec![9, 10, 2], vec![10, 1, 1]], 10, 1), 54);
}

#[test]
fn test_76() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9], vec![10, 1, 10], vec![1, 3, 15], vec![3, 5, 20], vec![5, 7, 25], vec![7, 9, 30], vec![9, 11, 35]], 11, 1), 71);
}

#[test]
fn test_77() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 3, 4], vec![2, 4, 5], vec![3, 4, 2], vec![3, 5, 3], vec![4, 5, 1], vec![5, 1, 1]], 5, 1), 5);
}

#[test]
fn test_78() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 1, 5], vec![1, 3, 3], vec![3, 5, 1]], 5, 1), 6);
}

#[test]
fn test_79() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 5], vec![2, 3, 3], vec![3, 4, 1], vec![4, 5, 2], vec![5, 6, 4], vec![6, 7, 3], vec![7, 8, 2], vec![8, 9, 1], vec![9, 10, 5]], 10, 1), 26);
}

#[test]
fn test_80() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![2, 4, 3], vec![2, 5, 4], vec![3, 4, 5], vec![3, 5, 6], vec![4, 5, 7], vec![5, 6, 8], vec![5, 7, 9], vec![6, 7, 10]], 7, 1), 14);
}

#[test]
fn test_81() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9], vec![10, 11, 10], vec![11, 12, 11], vec![12, 13, 12], vec![13, 14, 13], vec![14, 15, 14], vec![15, 1, 15]], 15, 1), 105);
}

#[test]
fn test_82() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9], vec![10, 11, 10], vec![11, 12, 11], vec![12, 13, 12], vec![13, 14, 13], vec![14, 15, 14], vec![15, 16, 15], vec![16, 17, 16], vec![17, 18, 17], vec![18, 19, 18], vec![19, 20, 19]], 20, 1), 190);
}

#[test]
fn test_83() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9]], 10, 1), 45);
}

#[test]
fn test_84() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 4, 3], vec![4, 5, 4], vec![5, 6, 5], vec![6, 7, 6], vec![7, 8, 7], vec![8, 9, 8], vec![9, 10, 9], vec![10, 11, 10], vec![11, 12, 11], vec![12, 13, 12], vec![13, 14, 13]], 14, 1), 91);
}

#[test]
fn test_85() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 3], vec![1, 3, 2], vec![1, 4, 5], vec![2, 3, 1], vec![2, 5, 4], vec![3, 4, 6], vec![3, 5, 2], vec![4, 5, 3], vec![5, 6, 1]], 6, 1), 5);
}

#[test]
fn test_86() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 2], vec![3, 4, 1], vec![4, 5, 1], vec![5, 6, 1], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1], vec![10, 1, 1]], 10, 1), 10);
}

#[test]
fn test_87() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 4, 2], vec![1, 5, 4], vec![2, 1, 4], vec![2, 4, 1], vec![3, 1, 3], vec![3, 2, 2], vec![4, 3, 3], vec![4, 5, 1], vec![5, 1, 1], vec![5, 2, 5]], 5, 4), 5);
}

#[test]
fn test_88() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![2, 4, 1], vec![3, 4, 1], vec![4, 5, 1], vec![2, 5, 2], vec![3, 5, 2], vec![4, 1, 2], vec![5, 1, 2], vec![1, 4, 2], vec![2, 3, 2]], 5, 1), 3);
}

#[test]
fn test_89() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![1, 3, 3], vec![1, 4, 20], vec![2, 5, 15], vec![3, 2, 20], vec![3, 4, 5], vec![4, 3, 5], vec![4, 5, 7], vec![5, 6, 20], vec![6, 7, 15], vec![7, 8, 10], vec![8, 9, 15], vec![9, 10, 20]], 10, 1), 95);
}

#[test]
fn test_90() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 1], vec![2, 4, 2], vec![2, 5, 2], vec![3, 4, 3], vec![3, 5, 3], vec![4, 6, 4], vec![4, 7, 4], vec![5, 6, 5], vec![5, 7, 5], vec![6, 8, 6], vec![6, 9, 6], vec![7, 8, 7], vec![7, 9, 7], vec![8, 10, 8], vec![9, 10, 8]], 10, 1), 21);
}

#[test]
fn test_91() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 20], vec![3, 4, 30], vec![4, 5, 40], vec![5, 6, 50], vec![6, 7, 60], vec![7, 8, 70], vec![8, 9, 80], vec![9, 10, 90], vec![10, 1, 100]], 10, 1), 450);
}

#[test]
fn test_92() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 2], vec![2, 4, 5], vec![3, 4, 1], vec![4, 5, 3], vec![5, 6, 2], vec![6, 7, 4], vec![7, 8, 3], vec![8, 9, 1], vec![9, 10, 5]], 10, 1), 22);
}

#[test]
fn test_93() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![2, 4, 3], vec![2, 5, 4], vec![3, 4, 2], vec![3, 5, 3], vec![4, 6, 5], vec![4, 7, 6], vec![5, 6, 4], vec![5, 7, 5], vec![6, 8, 7], vec![7, 8, 8]], 8, 1), 16);
}

#[test]
fn test_94() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 1], vec![2, 4, 1], vec![3, 4, 1], vec![4, 5, 1], vec![5, 6, 1], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1], vec![10, 11, 1], vec![11, 12, 1], vec![12, 13, 1], vec![13, 14, 1], vec![14, 15, 1]], 15, 1), 13);
}

#[test]
fn test_95() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 100], vec![2, 3, 90], vec![3, 4, 80], vec![4, 5, 70], vec![5, 6, 60], vec![6, 7, 50], vec![7, 8, 40], vec![8, 9, 30], vec![9, 10, 20]], 10, 1), 540);
}

#[test]
fn test_96() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 10], vec![2, 3, 20], vec![1, 3, 40], vec![2, 4, 30], vec![3, 5, 50], vec![4, 6, 60], vec![5, 7, 70], vec![6, 8, 80], vec![7, 9, 90], vec![8, 10, 100]], 10, 1), 280);
}

#[test]
fn test_97() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![2, 4, 1], vec![3, 4, 2], vec![4, 5, 1], vec![2, 5, 2], vec![3, 5, 3], vec![5, 6, 1], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1]], 9, 1), 7);
}

#[test]
fn test_98() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 2], vec![1, 3, 4], vec![1, 4, 1], vec![2, 3, 1], vec![2, 4, 5], vec![3, 4, 2], vec![4, 5, 3], vec![5, 6, 2], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1]], 10, 1), 10);
}

#[test]
fn test_99() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 1], vec![4, 5, 1], vec![5, 6, 1], vec![6, 7, 1], vec![7, 8, 1], vec![8, 9, 1], vec![9, 10, 1], vec![10, 1, 1]], 10, 1), 9);
}

#[test]
fn test_100() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1], vec![1, 3, 2], vec![2, 4, 3], vec![2, 5, 4], vec![3, 6, 5], vec![3, 7, 6], vec![4, 8, 7], vec![4, 9, 8], vec![5, 10, 9], vec![5, 11, 10]], 11, 1), 15);
}

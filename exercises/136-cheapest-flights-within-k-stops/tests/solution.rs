include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 10], vec![0, 2, 15], vec![1, 2, 5], vec![1, 3, 20], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10]], 0, 5, 2), -1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_cheapest_price(3, vec![vec![0, 1, 100], vec![1, 2, 100]], 0, 2, 2), 200);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_cheapest_price(3, vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]], 0, 2, 1), 200);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 10], vec![1, 2, 20], vec![0, 3, 40], vec![3, 4, 20], vec![4, 2, 10], vec![2, 5, 25]], 0, 5, 2), 55);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_cheapest_price(3, vec![vec![0, 1, 100], vec![1, 2, 100]], 0, 2, 0), -1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_cheapest_price(4, vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]], 0, 3, 1), 6);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_cheapest_price(4, vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1], vec![1, 3, 4]], 0, 3, 1), 5);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 10], vec![0, 2, 15], vec![1, 3, 20], vec![2, 4, 50], vec![3, 4, 10], vec![4, 5, 10]], 0, 5, 1), -1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 10], vec![0, 2, 30], vec![1, 3, 50], vec![2, 3, 20], vec![3, 4, 10], vec![4, 5, 60]], 0, 5, 2), -1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_cheapest_price(4, vec![vec![0, 1, 100], vec![1, 2, 100], vec![2, 0, 100], vec![1, 3, 600], vec![2, 3, 200]], 0, 3, 1), 700);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 10], vec![0, 2, 30], vec![1, 2, 10], vec![1, 3, 40], vec![2, 3, 10], vec![2, 4, 10], vec![3, 4, 10], vec![4, 5, 10]], 0, 5, 1), -1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_cheapest_price(3, vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]], 0, 2, 0), 500);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_cheapest_price(2, vec![vec![0, 1, 100]], 0, 1, 0), 100);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_cheapest_price(5, vec![vec![0, 1, 5], vec![1, 2, 5], vec![0, 3, 2], vec![3, 1, 2], vec![1, 4, 1], vec![4, 2, 1]], 0, 2, 2), 7);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_cheapest_price(9, vec![vec![0, 1, 10], vec![0, 2, 20], vec![0, 3, 30], vec![1, 4, 40], vec![1, 5, 50], vec![2, 5, 60], vec![2, 6, 70], vec![3, 4, 80], vec![3, 6, 90], vec![4, 7, 100], vec![5, 7, 110], vec![5, 8, 120], vec![6, 8, 130], vec![7, 8, 140]], 0, 8, 7), 180);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 10], vec![1, 2, 20], vec![2, 3, 30], vec![3, 4, 40], vec![4, 5, 50], vec![5, 6, 60], vec![6, 7, 70], vec![0, 7, 250], vec![7, 0, 250], vec![1, 3, 50], vec![3, 1, 50], vec![2, 4, 70], vec![4, 2, 70], vec![5, 7, 90], vec![7, 5, 90]], 0, 7, 2), 250);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_cheapest_price(5, vec![vec![0, 1, 500], vec![0, 2, 100], vec![2, 3, 300], vec![1, 4, 200], vec![3, 4, 50], vec![4, 2, 100], vec![2, 0, 200]], 0, 4, 2), 450);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 100], vec![0, 2, 150], vec![1, 3, 100], vec![1, 4, 200], vec![2, 3, 150], vec![2, 5, 200], vec![3, 4, 100], vec![3, 6, 150], vec![4, 7, 200], vec![5, 6, 100], vec![5, 7, 150], vec![6, 7, 100]], 0, 7, 3), 450);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 100], vec![0, 2, 150], vec![1, 3, 100], vec![1, 4, 200], vec![2, 3, 200], vec![2, 5, 300], vec![3, 4, 50], vec![3, 6, 200], vec![4, 6, 150], vec![5, 6, 100]], 0, 6, 4), 400);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_cheapest_price(12, vec![vec![0, 1, 5], vec![1, 2, 10], vec![2, 3, 5], vec![3, 4, 10], vec![4, 5, 5], vec![5, 6, 10], vec![6, 7, 5], vec![7, 8, 10], vec![8, 9, 5], vec![9, 10, 10], vec![10, 11, 5], vec![0, 11, 100]], 0, 11, 6), 100);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 200], vec![0, 2, 300], vec![1, 2, 100], vec![1, 3, 150], vec![2, 3, 200], vec![2, 4, 100], vec![3, 4, 50], vec![3, 5, 100], vec![4, 5, 150], vec![4, 6, 100], vec![5, 6, 200]], 0, 6, 5), 500);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_cheapest_price(9, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 2, 5], vec![1, 3, 20], vec![2, 3, 10], vec![2, 4, 15], vec![3, 4, 5], vec![3, 5, 25], vec![4, 5, 10], vec![4, 6, 20], vec![5, 6, 15], vec![5, 7, 25], vec![6, 7, 10], vec![6, 8, 15], vec![7, 8, 5]], 0, 8, 5), 65);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_cheapest_price(12, vec![vec![0, 1, 10], vec![0, 2, 15], vec![1, 2, 5], vec![1, 3, 10], vec![2, 3, 20], vec![3, 4, 50], vec![4, 5, 20], vec![5, 6, 10], vec![6, 7, 15], vec![7, 8, 30], vec![8, 9, 50], vec![9, 10, 70], vec![10, 11, 90], vec![11, 3, 110]], 0, 11, 6), -1);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_cheapest_price(9, vec![vec![0, 1, 50], vec![0, 2, 30], vec![1, 3, 100], vec![1, 4, 10], vec![2, 5, 50], vec![2, 6, 100], vec![3, 7, 10], vec![4, 8, 100], vec![5, 8, 50], vec![6, 8, 5], vec![7, 8, 50]], 0, 8, 2), 130);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 10], vec![1, 2, 20], vec![2, 3, 30], vec![3, 4, 40], vec![4, 5, 50], vec![5, 6, 60], vec![6, 7, 70], vec![7, 8, 80], vec![8, 9, 90], vec![9, 0, 100]], 0, 9, 9), 450);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_cheapest_price(5, vec![vec![0, 1, 10], vec![0, 2, 5], vec![1, 2, 2], vec![1, 3, 20], vec![2, 3, 10], vec![2, 4, 15], vec![3, 4, 10]], 0, 4, 2), 20);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_cheapest_price(12, vec![vec![0, 1, 200], vec![0, 2, 300], vec![1, 3, 100], vec![1, 4, 200], vec![2, 3, 400], vec![2, 4, 500], vec![3, 4, 100], vec![3, 5, 200], vec![4, 5, 150], vec![4, 6, 300], vec![5, 6, 250], vec![6, 7, 400], vec![6, 8, 500], vec![7, 8, 100], vec![7, 9, 300], vec![8, 9, 200], vec![8, 10, 100], vec![9, 10, 250], vec![9, 11, 400], vec![10, 11, 300]], 0, 11, 2), -1);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 10], vec![0, 2, 100], vec![1, 2, 50], vec![1, 3, 150], vec![1, 4, 120], vec![2, 4, 50], vec![3, 4, 10], vec![3, 5, 60], vec![4, 5, 30], vec![4, 6, 100], vec![5, 6, 50], vec![6, 7, 80], vec![6, 8, 100], vec![7, 8, 20], vec![7, 9, 60], vec![8, 9, 30]], 0, 9, 3), -1);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 10], vec![0, 2, 30], vec![1, 2, 5], vec![1, 3, 40], vec![2, 3, 10], vec![2, 4, 10], vec![3, 4, 20], vec![4, 5, 10], vec![5, 6, 15], vec![6, 7, 25], vec![7, 8, 30], vec![8, 9, 10]], 0, 9, 4), -1);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 100], vec![0, 2, 300], vec![1, 2, 150], vec![1, 3, 400], vec![2, 4, 100], vec![3, 4, 200], vec![3, 5, 500], vec![4, 5, 250], vec![4, 6, 350], vec![5, 6, 100], vec![6, 7, 200], vec![7, 8, 100], vec![8, 9, 150]], 0, 9, 3), -1);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 5], vec![1, 2, 3], vec![2, 3, 8], vec![0, 3, 10], vec![3, 4, 4], vec![4, 5, 6], vec![5, 6, 2], vec![6, 3, 7], vec![0, 4, 20], vec![1, 5, 12], vec![2, 6, 9]], 0, 6, 3), 17);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_cheapest_price(15, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 2, 30], vec![1, 3, 50], vec![2, 3, 10], vec![2, 4, 50], vec![3, 4, 10], vec![3, 5, 60], vec![4, 5, 30], vec![4, 6, 100], vec![5, 6, 50], vec![6, 7, 80], vec![6, 8, 100], vec![7, 8, 20], vec![7, 9, 60], vec![8, 9, 30], vec![8, 10, 50], vec![9, 10, 10], vec![9, 11, 60], vec![10, 11, 30], vec![11, 12, 50], vec![12, 13, 80], vec![13, 14, 100]], 0, 14, 5), -1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_cheapest_price(15, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 3, 5], vec![1, 4, 15], vec![2, 5, 6], vec![2, 6, 12], vec![3, 7, 8], vec![3, 8, 15], vec![4, 9, 15], vec![4, 10, 25], vec![5, 10, 7], vec![5, 11, 12], vec![6, 11, 6], vec![7, 8, 5], vec![8, 9, 5], vec![9, 10, 5], vec![10, 11, 5], vec![10, 12, 15], vec![11, 12, 10], vec![11, 13, 25], vec![12, 13, 15], vec![12, 14, 10], vec![13, 14, 5]], 0, 14, 5), 58);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 2, 5], vec![1, 3, 30], vec![2, 3, 25], vec![3, 4, 50], vec![4, 5, 20], vec![5, 6, 10], vec![6, 7, 15]], 0, 7, 3), -1);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_cheapest_price(12, vec![vec![0, 1, 5], vec![0, 2, 10], vec![1, 3, 15], vec![1, 4, 20], vec![2, 5, 25], vec![2, 6, 30], vec![3, 7, 35], vec![3, 8, 40], vec![4, 8, 45], vec![4, 9, 50], vec![5, 9, 55], vec![5, 10, 60], vec![6, 10, 65], vec![7, 11, 70], vec![8, 11, 75], vec![9, 11, 80]], 0, 11, 4), 125);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 50], vec![1, 2, 30], vec![2, 3, 20], vec![3, 4, 10], vec![4, 5, 15], vec![5, 6, 25], vec![6, 7, 10], vec![7, 8, 5], vec![8, 9, 10]], 0, 9, 3), -1);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 5], vec![0, 2, 10], vec![1, 2, 3], vec![1, 3, 8], vec![1, 4, 15], vec![2, 3, 4], vec![2, 4, 9], vec![3, 5, 20], vec![4, 5, 10], vec![4, 6, 25], vec![5, 6, 5]], 0, 6, 4), 32);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 100], vec![0, 2, 150], vec![1, 2, 50], vec![1, 3, 200], vec![2, 4, 100], vec![3, 4, 200], vec![3, 5, 150], vec![4, 5, 100], vec![5, 6, 200]], 0, 6, 3), 550);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_cheapest_price(11, vec![vec![0, 1, 100], vec![0, 2, 200], vec![1, 2, 100], vec![1, 3, 150], vec![2, 3, 50], vec![2, 4, 50], vec![3, 4, 100], vec![3, 5, 150], vec![4, 5, 200], vec![4, 6, 100], vec![5, 6, 50], vec![6, 7, 100], vec![6, 8, 100], vec![7, 8, 50], vec![7, 9, 150], vec![8, 9, 200], vec![8, 10, 100], vec![9, 10, 150]], 0, 10, 4), 550);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 5], vec![0, 2, 10], vec![0, 3, 15], vec![0, 4, 20], vec![0, 5, 25], vec![0, 6, 30], vec![0, 7, 35], vec![0, 8, 40], vec![0, 9, 45], vec![1, 2, 5], vec![2, 3, 5], vec![3, 4, 5], vec![4, 5, 5], vec![5, 6, 5], vec![6, 7, 5], vec![7, 8, 5], vec![8, 9, 5], vec![1, 9, 95], vec![9, 1, 95], vec![2, 8, 80], vec![8, 2, 80], vec![3, 7, 70], vec![7, 3, 70], vec![4, 6, 60], vec![6, 4, 60]], 0, 9, 5), 45);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_cheapest_price(9, vec![vec![0, 1, 100], vec![0, 2, 200], vec![1, 3, 100], vec![1, 4, 200], vec![2, 3, 150], vec![2, 4, 250], vec![3, 4, 50], vec![3, 5, 300], vec![4, 5, 200], vec![4, 6, 100], vec![5, 6, 150], vec![5, 7, 350], vec![6, 7, 100], vec![6, 8, 250], vec![7, 8, 200]], 0, 8, 2), -1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 1], vec![0, 2, 2], vec![1, 3, 3], vec![1, 4, 4], vec![2, 3, 5], vec![2, 4, 6], vec![3, 5, 7], vec![3, 6, 8], vec![4, 5, 9], vec![4, 6, 10], vec![5, 7, 11], vec![6, 7, 12]], 0, 7, 6), 22);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 10], vec![0, 2, 100], vec![1, 3, 150], vec![1, 4, 200], vec![2, 4, 50], vec![2, 5, 100], vec![3, 6, 10], vec![4, 6, 300], vec![5, 6, 50], vec![6, 7, 100]], 0, 7, 3), 270);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 50], vec![0, 2, 100], vec![1, 2, 10], vec![1, 3, 20], vec![2, 3, 5], vec![3, 4, 100], vec![4, 5, 10], vec![5, 6, 50], vec![4, 6, 200]], 0, 6, 2), -1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 10], vec![0, 2, 10], vec![1, 3, 10], vec![1, 4, 10], vec![2, 5, 10], vec![2, 6, 10], vec![3, 7, 10], vec![3, 8, 10], vec![4, 8, 10], vec![4, 9, 10], vec![5, 9, 10], vec![6, 7, 10], vec![7, 8, 10], vec![8, 9, 10]], 0, 9, 5), 30);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 100], vec![1, 2, 100], vec![2, 3, 100], vec![3, 4, 100], vec![4, 5, 100], vec![0, 2, 150], vec![1, 3, 150], vec![2, 4, 150], vec![3, 5, 150], vec![0, 3, 200], vec![1, 4, 200], vec![2, 5, 200]], 0, 5, 2), 350);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 50], vec![0, 2, 25], vec![1, 2, 10], vec![1, 3, 75], vec![2, 3, 60], vec![2, 4, 80], vec![3, 4, 40], vec![3, 5, 65], vec![4, 5, 30], vec![4, 6, 90], vec![5, 6, 50], vec![5, 7, 70], vec![6, 7, 20]], 0, 7, 4), 205);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_cheapest_price(9, vec![vec![0, 1, 10], vec![0, 2, 10], vec![1, 2, 10], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10], vec![5, 6, 10], vec![6, 7, 10], vec![7, 8, 10], vec![0, 8, 80]], 0, 8, 2), 80);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 10], vec![0, 2, 50], vec![1, 2, 10], vec![1, 3, 40], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10], vec![2, 5, 50]], 0, 5, 2), 70);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 10], vec![1, 2, 20], vec![2, 3, 30], vec![3, 4, 40], vec![4, 5, 50], vec![5, 0, 60]], 0, 5, 5), 150);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 100], vec![0, 4, 10], vec![0, 2, 100], vec![1, 2, 10], vec![1, 3, 50], vec![2, 3, 20], vec![3, 4, 20], vec![4, 5, 10], vec![5, 6, 50], vec![6, 7, 10]], 0, 7, 3), 80);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 10], vec![0, 2, 5], vec![1, 2, 1], vec![1, 3, 4], vec![2, 4, 10], vec![2, 5, 15], vec![3, 5, 25], vec![4, 6, 5], vec![5, 7, 10], vec![6, 8, 20], vec![7, 9, 15]], 0, 9, 5), 45);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 3, 30], vec![1, 4, 40], vec![2, 5, 50], vec![2, 6, 60], vec![3, 7, 70], vec![4, 7, 80], vec![5, 7, 90], vec![6, 7, 100]], 0, 7, 4), 110);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 10], vec![0, 2, 10], vec![1, 2, 5], vec![1, 3, 50], vec![2, 3, 10], vec![3, 4, 20], vec![4, 5, 10], vec![5, 6, 10], vec![6, 0, 50]], 0, 6, 3), -1);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 100], vec![0, 2, 200], vec![0, 3, 300], vec![1, 4, 400], vec![2, 4, 300], vec![3, 5, 200], vec![4, 5, 100]], 0, 5, 2), 500);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 100], vec![0, 2, 200], vec![0, 3, 300], vec![1, 4, 100], vec![2, 4, 200], vec![3, 4, 300], vec![4, 5, 100], vec![4, 6, 200], vec![5, 6, 100]], 0, 6, 3), 400);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 100], vec![0, 2, 100], vec![1, 3, 100], vec![1, 4, 100], vec![2, 4, 100], vec![2, 5, 100], vec![3, 4, 50], vec![4, 5, 50]], 0, 5, 2), 200);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 10], vec![0, 2, 5], vec![0, 3, 15], vec![1, 2, 20], vec![1, 4, 50], vec![1, 5, 10], vec![2, 3, 10], vec![3, 4, 20], vec![4, 5, 10], vec![5, 6, 10], vec![6, 7, 5], vec![7, 8, 10], vec![8, 9, 10]], 0, 9, 4), -1);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 10], vec![0, 2, 20], vec![0, 3, 30], vec![1, 4, 40], vec![1, 5, 50], vec![2, 5, 60], vec![2, 6, 70], vec![3, 4, 80], vec![4, 6, 90], vec![5, 6, 100]], 0, 6, 5), 90);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 10], vec![1, 2, 20], vec![2, 3, 10], vec![3, 4, 20], vec![4, 5, 10], vec![5, 6, 20], vec![6, 7, 10], vec![0, 7, 100], vec![7, 3, 10], vec![3, 6, 50]], 0, 7, 2), 100);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_cheapest_price(9, vec![vec![0, 1, 50], vec![0, 2, 70], vec![1, 3, 100], vec![1, 4, 200], vec![2, 4, 120], vec![2, 5, 140], vec![3, 6, 90], vec![4, 6, 130], vec![4, 7, 150], vec![5, 7, 110], vec![6, 8, 80], vec![7, 8, 70]], 0, 8, 4), 320);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 3, 10], vec![1, 4, 20], vec![2, 4, 15], vec![2, 5, 30], vec![3, 5, 10], vec![4, 5, 5], vec![0, 5, 100]], 0, 5, 1), 50);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 100], vec![0, 2, 300], vec![1, 2, 50], vec![1, 3, 200], vec![2, 3, 150], vec![2, 4, 250], vec![3, 4, 100], vec![3, 5, 200], vec![4, 5, 50], vec![4, 6, 100], vec![5, 6, 200]], 0, 6, 2), 650);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 100], vec![0, 2, 150], vec![1, 2, 50], vec![1, 3, 200], vec![2, 3, 100], vec![2, 4, 150], vec![3, 4, 50], vec![3, 5, 250], vec![4, 5, 100]], 0, 5, 2), 400);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_cheapest_price(12, vec![vec![0, 1, 10], vec![1, 2, 10], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10], vec![5, 6, 10], vec![6, 7, 10], vec![7, 8, 10], vec![8, 9, 10], vec![9, 10, 10], vec![10, 11, 10], vec![11, 0, 10]], 0, 11, 5), -1);
}

#[test]
fn test_66() {
    assert_eq!(Solution::find_cheapest_price(5, vec![vec![0, 1, 100], vec![0, 2, 200], vec![1, 3, 300], vec![1, 4, 400], vec![2, 3, 150], vec![2, 4, 250], vec![3, 4, 100]], 0, 4, 1), 450);
}

#[test]
fn test_67() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 3, 100], vec![1, 4, 300], vec![2, 5, 50], vec![2, 6, 100], vec![3, 7, 10], vec![4, 7, 300], vec![5, 8, 50], vec![6, 8, 100], vec![7, 9, 10], vec![8, 9, 50]], 0, 9, 4), 130);
}

#[test]
fn test_68() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 10], vec![1, 7, 50], vec![0, 7, 100], vec![1, 2, 20], vec![2, 3, 30], vec![3, 4, 40], vec![4, 5, 50], vec![5, 6, 60], vec![6, 7, 10], vec![7, 3, 20]], 0, 7, 2), 60);
}

#[test]
fn test_69() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 50], vec![1, 2, 100], vec![2, 3, 50], vec![3, 4, 100], vec![4, 5, 50], vec![5, 6, 100], vec![0, 3, 200], vec![3, 5, 200], vec![0, 4, 300], vec![4, 6, 100], vec![1, 5, 200], vec![5, 1, 200], vec![2, 6, 250], vec![6, 2, 250], vec![0, 6, 350], vec![6, 0, 350]], 0, 6, 4), 350);
}

#[test]
fn test_70() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 1], vec![0, 2, 1], vec![0, 3, 1], vec![0, 4, 1], vec![0, 5, 1], vec![0, 6, 1], vec![0, 7, 1], vec![0, 8, 1], vec![0, 9, 1], vec![1, 9, 1], vec![2, 9, 1], vec![3, 9, 1], vec![4, 9, 1], vec![5, 9, 1], vec![6, 9, 1], vec![7, 9, 1], vec![8, 9, 1]], 0, 9, 0), 1);
}

#[test]
fn test_71() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 5], vec![0, 2, 10], vec![1, 3, 15], vec![1, 4, 20], vec![2, 5, 25], vec![2, 6, 30], vec![3, 7, 35], vec![3, 8, 40], vec![4, 9, 45], vec![5, 9, 50], vec![6, 9, 55], vec![7, 9, 60], vec![8, 9, 65]], 0, 9, 5), 70);
}

#[test]
fn test_72() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 10], vec![1, 2, 20], vec![2, 3, 30], vec![3, 4, 40], vec![4, 5, 50], vec![5, 6, 60], vec![6, 7, 70], vec![0, 7, 400]], 0, 7, 3), 400);
}

#[test]
fn test_73() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 100], vec![0, 2, 200], vec![1, 2, 100], vec![1, 3, 150], vec![2, 3, 150], vec![2, 4, 200], vec![3, 4, 50], vec![3, 5, 100], vec![4, 5, 50]], 0, 5, 2), 350);
}

#[test]
fn test_74() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 100], vec![0, 2, 200], vec![1, 2, 50], vec![1, 3, 150], vec![2, 3, 100], vec![2, 4, 100], vec![3, 4, 50], vec![3, 5, 100], vec![4, 5, 150], vec![4, 6, 100], vec![5, 6, 50]], 0, 6, 3), 350);
}

#[test]
fn test_75() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 2, 5], vec![1, 3, 20], vec![2, 3, 10], vec![2, 4, 15], vec![3, 4, 5], vec![3, 5, 25], vec![4, 5, 10], vec![4, 6, 20], vec![5, 6, 15], vec![5, 7, 25], vec![6, 7, 10], vec![6, 8, 15], vec![7, 8, 5], vec![7, 9, 25], vec![8, 9, 10]], 0, 9, 4), 80);
}

#[test]
fn test_76() {
    assert_eq!(Solution::find_cheapest_price(12, vec![vec![0, 1, 5], vec![0, 2, 10], vec![1, 3, 20], vec![1, 4, 30], vec![2, 5, 20], vec![2, 6, 30], vec![3, 7, 40], vec![3, 8, 50], vec![4, 9, 40], vec![4, 10, 50], vec![5, 10, 20], vec![5, 11, 30], vec![6, 11, 20], vec![7, 8, 5], vec![8, 9, 5], vec![9, 10, 5], vec![10, 11, 5]], 0, 11, 4), 55);
}

#[test]
fn test_77() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 100], vec![0, 2, 300], vec![1, 3, 200], vec![1, 4, 250], vec![2, 3, 50], vec![2, 5, 150], vec![3, 6, 50], vec![3, 7, 200], vec![4, 6, 150], vec![4, 7, 250], vec![5, 8, 50], vec![5, 9, 200], vec![6, 9, 150], vec![7, 8, 50], vec![8, 9, 50]], 0, 9, 3), 500);
}

#[test]
fn test_78() {
    assert_eq!(Solution::find_cheapest_price(5, vec![vec![0, 1, 10], vec![0, 2, 5], vec![0, 3, 15], vec![1, 2, 20], vec![1, 4, 50], vec![1, 3, 10], vec![2, 4, 10], vec![3, 4, 5]], 0, 4, 2), 15);
}

#[test]
fn test_79() {
    assert_eq!(Solution::find_cheapest_price(5, vec![vec![0, 1, 5], vec![0, 2, 30], vec![1, 2, 5], vec![1, 3, 15], vec![2, 3, 10], vec![2, 4, 50], vec![3, 4, 10], vec![0, 4, 100], vec![1, 4, 20], vec![2, 0, 10], vec![3, 0, 20]], 0, 4, 2), 25);
}

#[test]
fn test_80() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 100], vec![1, 2, 200], vec![2, 3, 150], vec![3, 4, 200], vec![4, 5, 100], vec![5, 6, 250], vec![6, 7, 100], vec![7, 8, 300], vec![8, 9, 50]], 0, 9, 0), -1);
}

#[test]
fn test_81() {
    assert_eq!(Solution::find_cheapest_price(5, vec![vec![0, 1, 20], vec![0, 2, 10], vec![1, 3, 10], vec![1, 4, 20], vec![2, 3, 20], vec![2, 4, 10], vec![3, 4, 5], vec![3, 0, 15], vec![4, 0, 10], vec![4, 1, 5], vec![4, 2, 10]], 0, 4, 2), 20);
}

#[test]
fn test_82() {
    assert_eq!(Solution::find_cheapest_price(12, vec![vec![0, 1, 10], vec![0, 2, 20], vec![0, 3, 30], vec![1, 4, 40], vec![1, 5, 50], vec![2, 6, 60], vec![3, 7, 70], vec![4, 8, 80], vec![5, 9, 90], vec![6, 10, 100], vec![7, 11, 110], vec![8, 11, 120], vec![9, 11, 130]], 0, 11, 5), 210);
}

#[test]
fn test_83() {
    assert_eq!(Solution::find_cheapest_price(9, vec![vec![0, 1, 10], vec![1, 2, 10], vec![2, 3, 10], vec![3, 4, 10], vec![4, 5, 10], vec![5, 6, 10], vec![6, 7, 10], vec![7, 8, 10], vec![8, 0, 10], vec![0, 8, 10], vec![1, 8, 10], vec![2, 8, 10], vec![3, 8, 10], vec![4, 8, 10], vec![5, 8, 10], vec![6, 8, 10], vec![7, 8, 10]], 0, 4, 4), 40);
}

#[test]
fn test_84() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 100], vec![0, 2, 500], vec![1, 2, 100], vec![2, 3, 200], vec![3, 4, 100], vec![4, 5, 300], vec![5, 2, 200], vec![2, 5, 200], vec![1, 3, 300], vec![3, 5, 100]], 0, 5, 3), 400);
}

#[test]
fn test_85() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 10], vec![0, 2, 15], vec![1, 2, 5], vec![1, 3, 10], vec![2, 3, 20], vec![3, 4, 50], vec![4, 5, 20], vec![5, 6, 10], vec![6, 7, 15], vec![7, 8, 30], vec![8, 9, 50]], 0, 9, 5), -1);
}

#[test]
fn test_86() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 100], vec![0, 2, 50], vec![0, 3, 100], vec![1, 4, 100], vec![1, 5, 50], vec![2, 4, 50], vec![2, 5, 100], vec![3, 4, 50], vec![4, 5, 25], vec![4, 6, 50], vec![5, 6, 100]], 0, 6, 3), 150);
}

#[test]
fn test_87() {
    assert_eq!(Solution::find_cheapest_price(5, vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 1], vec![4, 0, 1], vec![1, 4, 1], vec![4, 2, 1], vec![2, 0, 1], vec![0, 3, 1], vec![3, 1, 1]], 0, 4, 10), 2);
}

#[test]
fn test_88() {
    assert_eq!(Solution::find_cheapest_price(6, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 2, 5], vec![1, 3, 30], vec![2, 3, 25], vec![3, 4, 50], vec![4, 5, 20], vec![5, 0, 10]], 0, 5, 2), -1);
}

#[test]
fn test_89() {
    assert_eq!(Solution::find_cheapest_price(7, vec![vec![0, 1, 100], vec![0, 2, 200], vec![1, 3, 100], vec![1, 4, 150], vec![2, 3, 200], vec![2, 5, 300], vec![3, 4, 50], vec![3, 6, 200], vec![4, 6, 150], vec![5, 6, 100]], 0, 6, 3), 400);
}

#[test]
fn test_90() {
    assert_eq!(Solution::find_cheapest_price(10, vec![vec![0, 1, 100], vec![1, 2, 100], vec![2, 3, 100], vec![3, 4, 100], vec![4, 5, 100], vec![5, 6, 100], vec![6, 7, 100], vec![7, 8, 100], vec![8, 9, 100], vec![9, 0, 100], vec![0, 9, 10], vec![1, 8, 10], vec![2, 7, 10], vec![3, 6, 10], vec![4, 5, 10], vec![5, 4, 10], vec![6, 3, 10], vec![7, 2, 10], vec![8, 1, 10], vec![9, 0, 10]], 0, 9, 4), 10);
}

#[test]
fn test_91() {
    assert_eq!(Solution::find_cheapest_price(15, vec![vec![0, 1, 10], vec![0, 2, 20], vec![1, 3, 10], vec![2, 4, 20], vec![3, 5, 10], vec![4, 6, 20], vec![5, 7, 10], vec![6, 8, 20], vec![7, 9, 10], vec![8, 10, 20], vec![9, 11, 10], vec![10, 12, 20], vec![11, 13, 10], vec![12, 14, 20], vec![13, 14, 10]], 0, 14, 7), 80);
}

#[test]
fn test_92() {
    assert_eq!(Solution::find_cheapest_price(9, vec![vec![0, 1, 100], vec![1, 2, 100], vec![2, 3, 100], vec![3, 4, 100], vec![4, 5, 100], vec![5, 6, 100], vec![6, 7, 100], vec![7, 8, 100], vec![8, 0, 100], vec![0, 2, 150], vec![2, 4, 150], vec![4, 6, 150], vec![6, 8, 150], vec![1, 3, 150], vec![3, 5, 150], vec![5, 7, 150], vec![7, 0, 150]], 0, 8, 4), 600);
}

#[test]
fn test_93() {
    assert_eq!(Solution::find_cheapest_price(8, vec![vec![0, 1, 10], vec![0, 2, 10], vec![1, 3, 10], vec![2, 4, 10], vec![3, 5, 10], vec![4, 6, 10], vec![5, 7, 10], vec![6, 7, 10], vec![1, 4, 5], vec![2, 5, 5], vec![3, 6, 5], vec![4, 7, 5]], 0, 7, 3), 20);
}

#[test]
fn test_94() {
    assert_eq!(Solution::find_cheapest_price(9, vec![vec![0, 1, 100], vec![0, 2, 150], vec![1, 3, 50], vec![1, 4, 200], vec![2, 5, 100], vec![3, 6, 50], vec![4, 7, 200], vec![5, 8, 100], vec![6, 8, 100], vec![7, 8, 50]], 0, 8, 4), 300);
}

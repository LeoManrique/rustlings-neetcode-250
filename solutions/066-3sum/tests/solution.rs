include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::three_sum(vec![-2, 0, 0, 2, 2]), vec![vec![-2, 0, 2]]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::three_sum(vec![-2, 0, 1, 1, 2]), vec![vec![-2, 0, 2], vec![-2, 1, 1]]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::three_sum(vec![1, 2, -2, -1]), vec![]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 0, 0, 1, 1, 2]), vec![vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::three_sum(vec![1, -1, -1, 0]), vec![vec![-1, 0, 1]]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::three_sum(vec![3, -2, 1, 0, -1, -2, 1, -2, 1, -2]), vec![vec![-2, -1, 3], vec![-2, 1, 1], vec![-1, 0, 1]]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 0]), vec![vec![-1, 0, 1]]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]), vec![vec![-4, -2, 6], vec![-4, 0, 4], vec![-4, 1, 3], vec![-4, 2, 2], vec![-2, -2, 4], vec![-2, 0, 2]]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::three_sum(vec![-1, 1, 0]), vec![vec![-1, 0, 1]]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), vec![]);
}

#[test]
fn test_13() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), vec![vec![0, 0, 0]]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, -1, 0, 1, -1, 0, 1]), vec![vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_15() {
    assert_eq!(Solution::three_sum(vec![-3, -3, -3, -3, 1, 2, 2, 3, 3, 3]), vec![vec![-3, 1, 2]]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::three_sum(vec![-1, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 1]), vec![vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::three_sum(vec![-3, -2, -1, 0, 0, 0, 1, 2, 3]), vec![vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::three_sum(vec![-5, -3, -2, 0, 1, 2, 3, 5, 7]), vec![vec![-5, -2, 7], vec![-5, 0, 5], vec![-5, 2, 3], vec![-3, -2, 5], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, 0, 2]]);
}

#[test]
fn test_19() {
    assert_eq!(Solution::three_sum(vec![-1, -1, 0, 0, 0, 1, 1, 1, 2, 2]), vec![vec![-1, -1, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_20() {
    assert_eq!(Solution::three_sum(vec![-4, -3, -2, -1, 0, 1, 2, 3, 4]), vec![vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), vec![vec![0, 0, 0]]);
}

#[test]
fn test_22() {
    assert_eq!(Solution::three_sum(vec![-1, -1, 0, 0, 1, 1, 2, -2, 3]), vec![vec![-2, -1, 3], vec![-2, 0, 2], vec![-2, 1, 1], vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_23() {
    assert_eq!(Solution::three_sum(vec![-1, -1, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::three_sum(vec![-1, -1, 0, 1, 1, 2, 2, 3]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_25() {
    assert_eq!(Solution::three_sum(vec![-5, 2, 3, 0, -4, -6, 5, -7, 9, -3, -8, -1, 1, -7, -3, 1, -8, 5, -5, -7, 6, -3, 0, 6, -4, -8, -9, 7, 7, -4, -9, 1, -2, 6, 5, -3, -9, -1, 5, 4, -1, -2, -5, -2, -8, 8, 6, -7, -1, -2, 3, -3, 9, -4, 2, -3, 3, 2, 6, -9, -5, -3, 5, -8, 1, 8, -2, 5, -3, -8, 1, 8, -7, -4, 5, 5, -1, 6, 2, 2, -6, -1, -3, 2, -5, -7, -9, -7, 6, 2, 2, 1, 6, 5, -5, 0, -5, 8, -5, -7, 6, -2, -9, -1, -2, 7, 1, -7, 5, -3, -9, 2, -7, -8, -3, -7, -9, 0, 4, -1, 3, 8, -2, -7, 9, -9, -6, -7, 4, 3, 3, -9, -9, -9, 9, 0, 5, -2, -9, -9, -6, -1, 0, 1, 9, -8, -3, -2, -1, 6, 0, 8, 8, -7, -6, -2, -1, -2, -8, -4, 8, -6, -9, 7, -6, -4, 0, 7, 8, -9, 9, -9, -8, -3, -5, -7, -3, 9, 1, 6, -9, -2, -4, -4, -3, -1, 5, -4, -9, -9, 8, -2, -8, 1, 0, 3, 6, -1, 6, -8, -7, -2, -8, 1, 9, -3, -5, -1, -7, 8, -5, -6, -1, 8, 0, -2, -1, 1, 8, -4, 1, -4, 4, -3, -1, -4, 5, 9, 4, -9, -8, 2, -2, -7, -7, -8, -7, -3, 7, -5, -8, 2, -6, -1, -2, -4, -4, -6, -8, 5, -4, 7, 8, -4, 5, 0, -7, -8, -4, -6, -1, -3, 1, 3, -2, -1, -9, 0, 1, 2, -8, 5, 8, 7, -3, -1, -7, -9, -2, -4, -1, -5, -2, -5, -5, -9, -8, -6, 6, -2, -9, -7, -5, -9, 2, -3, 7, 0, -9, 7, -4, 3, -3, -4, -8, 1, -4, -6, -4, 3, 5, 0, 4, -5, -8, -5, -3, -7, -1, 0, -3, -8, 2, -4, -3, 6, -7, -6, -3, -1, -5, -8, -4, -3, -7, -9, -6, -6, 3, -9, -5, 7, -1, -5, -7, 4, 8, -5, -5, 2, -9, -3, -8, -3, 4, -6, 2, -8, -8, -1, -8, -8, -9, 7, -9, 1, -3, -4, -1, -3, -1, -8, 9, -6, -1, 3, 3, -9, -9, 7, -5, -1, -7, -4, -8, 2, 0, 5, -8, 8, -3, -6, -2, -8, -3, -3, -1, -7, 4, -4, 6, 3, -6, -1, -2, -8, -3, 2, -8, -2, 8, -4, -7, -8, -9, -6, -8, -9, 7, 1, -4, 1, -8, 2, -2, 4, -1, 9, -3, -5, -1, -6, -2, -8, 2, -2, -8, -4, -3, -7, -3, 0, -9, -2, -1, -7, -1, -9, -1, 7, -6, -5, -4, -8, -4, -2, -7, 5, -6, -5, -5, -5, -5, -3, 4, -9, -3, -2, -8, -1, -9, -4, -2, 5, -3, 1, -4, -1, 1, -3, -1, -3, 1, -3, -5, 7, -3, -7, -9, 9, -2, -9, 7, -7, -1, -4, -2, -5, -3, 8, -6, -2, 2, -7, -8, 6, 9, -9, -7, -3, -9, -8, -2, -7, -6, -9, -9, -1, -9, -9, -7, -1, -8, -7, -8, -6, -4, -2, -3, -2, -2, -6, -9, -8, -5, -3, -1, -1, -3, 3, -5, -1, -2, -5, -9, 0, -7, -1, -4, -3, -9, -7, -3, -7, -8, -7, -6, 0, 1, -2, 0, -4, -5, -5, -3, -9, 5, -1, -2, 8, -8, -1, -9, -7, -8, -9, -9, 0, -1, -3, -4, -3, -2, -4, -6, -2, 6, -4, -4, -3, 8, -9, -5, -9, -4, -9, -8, -7, -3, -1, -6, -9, 0, -7, -8, 2, -5, -4, -5, -3, -6, -7, -4, -7, 3, -3, -4, -2, -3, 8, -9, -5, -2, -6, -3, -8, -1, -3, -7, -1, -8, -5, -5, -1, -3, -7, -3, -5, -5, 2, -4, -9, -3, -6, -3, -8, -6, -9, -3, -1, -8, -8, -8, -8, -7, -7, -7, -7, -7, -6, -6, -6, -6, -5, -5, -5, -5, -4, -4, -4, -4, -3, -3, -3, -3, -2, -2, -2, -2, -1, -1, -1, -1, 0, 0, 0, 0]), vec![vec![-9, 0, 9], vec![-9, 1, 8], vec![-9, 2, 7], vec![-9, 3, 6], vec![-9, 4, 5], vec![-8, -1, 9], vec![-8, 0, 8], vec![-8, 1, 7], vec![-8, 2, 6], vec![-8, 3, 5], vec![-8, 4, 4], vec![-7, -2, 9], vec![-7, -1, 8], vec![-7, 0, 7], vec![-7, 1, 6], vec![-7, 2, 5], vec![-7, 3, 4], vec![-6, -3, 9], vec![-6, -2, 8], vec![-6, -1, 7], vec![-6, 0, 6], vec![-6, 1, 5], vec![-6, 2, 4], vec![-6, 3, 3], vec![-5, -4, 9], vec![-5, -3, 8], vec![-5, -2, 7], vec![-5, -1, 6], vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -4, 8], vec![-4, -3, 7], vec![-4, -2, 6], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-4, 2, 2], vec![-3, -3, 6], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -2, 4], vec![-2, -1, 3], vec![-2, 0, 2], vec![-2, 1, 1], vec![-1, -1, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::three_sum(vec![-10, -10, -10, -1, 0, 0, 1, 10, 10, 10]), vec![vec![-10, 0, 10], vec![-1, 0, 1]]);
}

#[test]
fn test_27() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4, 3, 4, 5, -5]), vec![vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::three_sum(vec![-3, -2, -1, 0, 1, 2, 3, 4, 5]), vec![vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::three_sum(vec![1, 2, -2, -1, -1, -3, 3, 3, 2, -2]), vec![vec![-3, 1, 2], vec![-2, -1, 3], vec![-1, -1, 2]]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::three_sum(vec![-1, -1, -1, 0, 0, 1, 1, 1]), vec![vec![-1, 0, 1]]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::three_sum(vec![-5, 2, 5, -1, 1, 0, -2, 2, -3, 3]), vec![vec![-5, 0, 5], vec![-5, 2, 3], vec![-3, -2, 5], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_32() {
    assert_eq!(Solution::three_sum(vec![-3, -2, -1, 0, 1, 2, 3]), vec![vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_33() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), vec![vec![0, 0, 0]]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::three_sum(vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]), vec![vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_35() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4, -3, 3, 0, 1, 2, -1, -4]), vec![vec![-4, 1, 3], vec![-4, 2, 2], vec![-3, 0, 3], vec![-3, 1, 2], vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::three_sum(vec![-1, 1, 2, -2, 3, -3, 4, -4]), vec![vec![-4, 1, 3], vec![-3, -1, 4], vec![-3, 1, 2], vec![-2, -1, 3]]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::three_sum(vec![-10, -5, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), vec![vec![-10, 0, 10], vec![-10, 1, 9], vec![-10, 2, 8], vec![-10, 3, 7], vec![-10, 4, 6], vec![-5, -3, 8], vec![-5, -2, 7], vec![-5, -1, 6], vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::three_sum(vec![1, 2, -2, -1, 0, 0, 0]), vec![vec![-2, 0, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_39() {
    assert_eq!(Solution::three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4]), vec![vec![-4, 0, 4], vec![-4, 1, 3], vec![-4, 2, 2], vec![-2, -2, 4], vec![-2, 0, 2]]);
}

#[test]
fn test_40() {
    assert_eq!(Solution::three_sum(vec![1, -1, -1, 1, 0, 0, 0]), vec![vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_41() {
    assert_eq!(Solution::three_sum(vec![-1, -2, -3, 4, 1, 3, 0]), vec![vec![-3, -1, 4], vec![-3, 0, 3], vec![-2, -1, 3], vec![-1, 0, 1]]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]), vec![vec![0, 0, 0]]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::three_sum(vec![1, 2, -2, -1, -1, -1, 0, 0, 0, 0]), vec![vec![-2, 0, 2], vec![-1, -1, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::three_sum(vec![-10, -9, -8, -7, 7, 8, 9, 10]), vec![]);
}

#[test]
fn test_45() {
    assert_eq!(Solution::three_sum(vec![-10, 0, 10, 5, -5, 3, -3, 2, -2, 1, -1]), vec![vec![-10, 0, 10], vec![-5, 0, 5], vec![-5, 2, 3], vec![-3, -2, 5], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_46() {
    assert_eq!(Solution::three_sum(vec![3, 0, -2, -1, 1, -2, 1, 0, -2, 1, 2]), vec![vec![-2, -1, 3], vec![-2, 0, 2], vec![-2, 1, 1], vec![-1, 0, 1]]);
}

#[test]
fn test_47() {
    assert_eq!(Solution::three_sum(vec![-10, -5, 0, 5, 10, 15, 20]), vec![vec![-10, -5, 15], vec![-10, 0, 10], vec![-5, 0, 5]]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::three_sum(vec![1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, 0, 0, 0]), vec![vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::three_sum(vec![-10, 0, 10, 20, -20, 30, -30, 40, -40, 50, -50, 60, -60, 70, -70, 80, -80, 90, -90]), vec![vec![-90, 0, 90], vec![-90, 10, 80], vec![-90, 20, 70], vec![-90, 30, 60], vec![-90, 40, 50], vec![-80, -10, 90], vec![-80, 0, 80], vec![-80, 10, 70], vec![-80, 20, 60], vec![-80, 30, 50], vec![-70, -20, 90], vec![-70, -10, 80], vec![-70, 0, 70], vec![-70, 10, 60], vec![-70, 20, 50], vec![-70, 30, 40], vec![-60, -30, 90], vec![-60, -20, 80], vec![-60, -10, 70], vec![-60, 0, 60], vec![-60, 10, 50], vec![-60, 20, 40], vec![-50, -40, 90], vec![-50, -30, 80], vec![-50, -20, 70], vec![-50, -10, 60], vec![-50, 0, 50], vec![-50, 10, 40], vec![-50, 20, 30], vec![-40, -30, 70], vec![-40, -20, 60], vec![-40, -10, 50], vec![-40, 0, 40], vec![-40, 10, 30], vec![-30, -20, 50], vec![-30, -10, 40], vec![-30, 0, 30], vec![-30, 10, 20], vec![-20, -10, 30], vec![-20, 0, 20], vec![-10, 0, 10]]);
}

#[test]
fn test_50() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 0, -1, 1, -1, 0, 1]), vec![vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_51() {
    assert_eq!(Solution::three_sum(vec![-4, -2, -1, 0, 1, 2, 3, 5]), vec![vec![-4, -1, 5], vec![-4, 1, 3], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::three_sum(vec![-1, -1, -1, 0, 0, 1, 1, 2, 3]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::three_sum(vec![-3, -3, -3, 1, 2, 3, 4, 4, 4]), vec![vec![-3, 1, 2]]);
}

#[test]
fn test_54() {
    assert_eq!(Solution::three_sum(vec![-2, -3, 4, 3, 2, 1, 0, -1, -2, -3, 3, 2, 1]), vec![vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -2, 4], vec![-2, -1, 3], vec![-2, 0, 2], vec![-2, 1, 1], vec![-1, 0, 1]]);
}

#[test]
fn test_55() {
    assert_eq!(Solution::three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4]), vec![vec![-4, 0, 4], vec![-4, 1, 3], vec![-4, 2, 2], vec![-2, -2, 4], vec![-2, 0, 2]]);
}

#[test]
fn test_56() {
    assert_eq!(Solution::three_sum(vec![-4, -1, -1, 0, 1, 2]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_57() {
    assert_eq!(Solution::three_sum(vec![-1, -2, -3, -4, -5, 5, 4, 3, 2, 1]), vec![vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -1, 5], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 1, 2], vec![-2, -1, 3]]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4, 3, -2, 2]), vec![vec![-4, 1, 3], vec![-4, 2, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::three_sum(vec![-3, -2, -1, 0, 1, 2, 3]), vec![vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::three_sum(vec![-5, -4, -3, -2, -1, 1, 2, 3, 4, 5]), vec![vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -1, 5], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 1, 2], vec![-2, -1, 3]]);
}

#[test]
fn test_61() {
    assert_eq!(Solution::three_sum(vec![-1, -2, -3, 0, 1, 2, 3]), vec![vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_62() {
    assert_eq!(Solution::three_sum(vec![100000, -100000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), vec![vec![-100000, 0, 100000], vec![0, 0, 0]]);
}

#[test]
fn test_63() {
    assert_eq!(Solution::three_sum(vec![3, -3, 4, -4, 5, -5, 6, -6, 0]), vec![vec![-6, 0, 6], vec![-5, 0, 5], vec![-4, 0, 4], vec![-3, 0, 3]]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::three_sum(vec![0, 1, 2, 3, 4, 5, -1, -2, -3, -4, -5]), vec![vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::three_sum(vec![-6, -6, -4, -2, 0, 2, 4, 6, 6]), vec![vec![-6, 0, 6], vec![-6, 2, 4], vec![-4, -2, 6], vec![-4, 0, 4], vec![-2, 0, 2]]);
}

#[test]
fn test_66() {
    assert_eq!(Solution::three_sum(vec![1, 2, 3, -6, 4, -1, -3, 2, 0]), vec![vec![-6, 2, 4], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::three_sum(vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), vec![vec![-5, -4, 9], vec![-5, -3, 8], vec![-5, -2, 7], vec![-5, -1, 6], vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -3, 7], vec![-4, -2, 6], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::three_sum(vec![2, 2, 2, 2, 2, -2, -2, -2, -2, -2]), vec![]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::three_sum(vec![-1, -1, -1, 0, 0, 1, 1, 2, 3]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::three_sum(vec![-5, -3, -1, 0, 0, 0, 0, 0, 1, 2, 3, 5]), vec![vec![-5, 0, 5], vec![-5, 2, 3], vec![-3, 0, 3], vec![-3, 1, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::three_sum(vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]), vec![vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::three_sum(vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), vec![vec![-10, 0, 10], vec![-10, 1, 9], vec![-10, 2, 8], vec![-10, 3, 7], vec![-10, 4, 6], vec![-9, -1, 10], vec![-9, 0, 9], vec![-9, 1, 8], vec![-9, 2, 7], vec![-9, 3, 6], vec![-9, 4, 5], vec![-8, -2, 10], vec![-8, -1, 9], vec![-8, 0, 8], vec![-8, 1, 7], vec![-8, 2, 6], vec![-8, 3, 5], vec![-7, -3, 10], vec![-7, -2, 9], vec![-7, -1, 8], vec![-7, 0, 7], vec![-7, 1, 6], vec![-7, 2, 5], vec![-7, 3, 4], vec![-6, -4, 10], vec![-6, -3, 9], vec![-6, -2, 8], vec![-6, -1, 7], vec![-6, 0, 6], vec![-6, 1, 5], vec![-6, 2, 4], vec![-5, -4, 9], vec![-5, -3, 8], vec![-5, -2, 7], vec![-5, -1, 6], vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -3, 7], vec![-4, -2, 6], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_73() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), vec![vec![0, 0, 0]]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::three_sum(vec![1, 2, 3, 4, 5, 6, -1, -2, -3, -4, -5, -6]), vec![vec![-6, 1, 5], vec![-6, 2, 4], vec![-5, -1, 6], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -2, 6], vec![-4, -1, 5], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 1, 2], vec![-2, -1, 3]]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::three_sum(vec![-3, -2, -1, 0, 1, 2, 3, 4, 5, 6]), vec![vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::three_sum(vec![-10, -5, -5, -3, -2, -1, 0, 1, 2, 3, 4, 5, 5, 6, 7, 8]), vec![vec![-10, 2, 8], vec![-10, 3, 7], vec![-10, 4, 6], vec![-10, 5, 5], vec![-5, -3, 8], vec![-5, -2, 7], vec![-5, -1, 6], vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::three_sum(vec![1, 2, -3, 4, -1, -2, 3, -4, 0, 0, 0, 0]), vec![vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::three_sum(vec![-5, 2, 1, -2, 3, -1, 2, -3, 0, 0, 0]), vec![vec![-5, 2, 3], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::three_sum(vec![-3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -4, -5, -6, -7, -8, -9]), vec![vec![-9, 0, 9], vec![-9, 1, 8], vec![-9, 2, 7], vec![-9, 3, 6], vec![-9, 4, 5], vec![-8, -1, 9], vec![-8, 0, 8], vec![-8, 1, 7], vec![-8, 2, 6], vec![-8, 3, 5], vec![-7, -2, 9], vec![-7, -1, 8], vec![-7, 0, 7], vec![-7, 1, 6], vec![-7, 2, 5], vec![-7, 3, 4], vec![-6, -3, 9], vec![-6, -2, 8], vec![-6, -1, 7], vec![-6, 0, 6], vec![-6, 1, 5], vec![-6, 2, 4], vec![-5, -4, 9], vec![-5, -3, 8], vec![-5, -2, 7], vec![-5, -1, 6], vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -3, 7], vec![-4, -2, 6], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::three_sum(vec![-2, 0, 1, 1, 2, 3, 4, 5, -3, -4, -5, 6, 7, 8, 9, -6, -7, -8, -9]), vec![vec![-9, 0, 9], vec![-9, 1, 8], vec![-9, 2, 7], vec![-9, 3, 6], vec![-9, 4, 5], vec![-8, 0, 8], vec![-8, 1, 7], vec![-8, 2, 6], vec![-8, 3, 5], vec![-7, -2, 9], vec![-7, 0, 7], vec![-7, 1, 6], vec![-7, 2, 5], vec![-7, 3, 4], vec![-6, -3, 9], vec![-6, -2, 8], vec![-6, 0, 6], vec![-6, 1, 5], vec![-6, 2, 4], vec![-5, -4, 9], vec![-5, -3, 8], vec![-5, -2, 7], vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -3, 7], vec![-4, -2, 6], vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, 0, 2], vec![-2, 1, 1]]);
}

#[test]
fn test_81() {
    assert_eq!(Solution::three_sum(vec![3, -1, -3, 0, 1, 2, 4, -2, -4, 5, -5, 6, -6]), vec![vec![-6, 0, 6], vec![-6, 1, 5], vec![-6, 2, 4], vec![-5, -1, 6], vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -2, 6], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::three_sum(vec![1, 1, 1, 1, -1, -1, -1, -1, 0, 0, 0, 0]), vec![vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::three_sum(vec![-5, -3, -1, 0, 0, 0, 1, 3, 5]), vec![vec![-5, 0, 5], vec![-3, 0, 3], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::three_sum(vec![1, 2, -2, -1, 0, -1, 2]), vec![vec![-2, 0, 2], vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::three_sum(vec![-7, -5, -5, -1, 0, 1, 1, 5, 7]), vec![vec![-7, 0, 7], vec![-5, 0, 5], vec![-1, 0, 1]]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 0, 1, 0, -1, -1, -1, 0]), vec![vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::three_sum(vec![1, 2, -2, -1, 0, 3, -3]), vec![vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::three_sum(vec![-1, -1, -1, 0, 0, 0, 1, 1, 1]), vec![vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::three_sum(vec![1, 2, 3, 4, 5, -5, -4, -3, -2, -1]), vec![vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -1, 5], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 1, 2], vec![-2, -1, 3]]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::three_sum(vec![-100000, -100000, -100000, -100000, -100000, -100000, -100000, -100000, -100000, -100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), vec![vec![-100000, 0, 100000], vec![0, 0, 0]]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::three_sum(vec![100000, -100000, 0, -50000, 50000, -25000, 25000]), vec![vec![-100000, 0, 100000], vec![-50000, 0, 50000], vec![-25000, 0, 25000]]);
}

#[test]
fn test_92() {
    assert_eq!(Solution::three_sum(vec![-2, 0, 0, 0, 1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9]), vec![vec![-2, 0, 2], vec![-2, 1, 1], vec![0, 0, 0]]);
}

#[test]
fn test_93() {
    assert_eq!(Solution::three_sum(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5]), vec![vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -1, 5], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 1, 2], vec![-2, -1, 3]]);
}

#[test]
fn test_94() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0, 0, 0]), vec![vec![0, 0, 0]]);
}

#[test]
fn test_95() {
    assert_eq!(Solution::three_sum(vec![-4, -2, -2, -1, 0, 1, 2, 2, 4]), vec![vec![-4, 0, 4], vec![-4, 2, 2], vec![-2, -2, 4], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_96() {
    assert_eq!(Solution::three_sum(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), vec![]);
}

#[test]
fn test_97() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4, 3, 4, -3, 2, -2, -5, 5]), vec![vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-4, 2, 2], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_98() {
    assert_eq!(Solution::three_sum(vec![1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1]), vec![]);
}

#[test]
fn test_99() {
    assert_eq!(Solution::three_sum(vec![-6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6]), vec![vec![-6, 0, 6], vec![-6, 1, 5], vec![-6, 2, 4], vec![-5, -1, 6], vec![-5, 0, 5], vec![-5, 1, 4], vec![-5, 2, 3], vec![-4, -2, 6], vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_100() {
    assert_eq!(Solution::three_sum(vec![-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0]), vec![vec![-5, 1, 4], vec![-4, 0, 4], vec![-4, 1, 3], vec![-2, -2, 4], vec![-2, 1, 1], vec![0, 0, 0]]);
}

#[test]
fn test_101() {
    assert_eq!(Solution::three_sum(vec![-1, 2, 1, -4, 3, 0, -2, 1, 1, -1, 0, 0, 0, 0, 0]), vec![vec![-4, 1, 3], vec![-2, -1, 3], vec![-2, 0, 2], vec![-2, 1, 1], vec![-1, -1, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
}

#[test]
fn test_102() {
    assert_eq!(Solution::three_sum(vec![-1, 2, 1, -4, 3, 0, -2, 2, 1, -1, -3, 3]), vec![vec![-4, 1, 3], vec![-4, 2, 2], vec![-3, 0, 3], vec![-3, 1, 2], vec![-2, -1, 3], vec![-2, 0, 2], vec![-2, 1, 1], vec![-1, -1, 2], vec![-1, 0, 1]]);
}

#[test]
fn test_103() {
    assert_eq!(Solution::three_sum(vec![-5, -3, -1, 0, 2, 4, 6]), vec![vec![-5, -1, 6], vec![-3, -1, 4]]);
}

#[test]
fn test_104() {
    assert_eq!(Solution::three_sum(vec![1, 2, -2, -1, 0, -4, 3, 4, 5]), vec![vec![-4, -1, 5], vec![-4, 0, 4], vec![-4, 1, 3], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]]);
}

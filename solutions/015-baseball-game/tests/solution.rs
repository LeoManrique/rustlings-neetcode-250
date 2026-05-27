include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "D".to_string(), "+".to_string(), "2".to_string(), "C".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "C".to_string(), "C".to_string()]), 9);
}

#[test]
fn test_2() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "20".to_string(), "30".to_string(), "D".to_string(), "+".to_string()]), 210);
}

#[test]
fn test_3() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "6".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "3".to_string(), "D".to_string(), "+".to_string(), "-2".to_string(), "D".to_string(), "9".to_string(), "+".to_string()]), 44);
}

#[test]
fn test_4() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "20".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "5".to_string(), "D".to_string(), "+".to_string()]), 90);
}

#[test]
fn test_5() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "10".to_string()]), 55);
}

#[test]
fn test_6() {
    assert_eq!(Solution::cal_points(vec!["-1".to_string(), "-2".to_string(), "-3".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-4".to_string(), "-5".to_string(), "+".to_string(), "-6".to_string()]), -37);
}

#[test]
fn test_7() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "8".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "2".to_string()]), 24);
}

#[test]
fn test_8() {
    assert_eq!(Solution::cal_points(vec!["5".to_string(), "-2".to_string(), "4".to_string(), "C".to_string(), "D".to_string(), "9".to_string(), "+".to_string(), "+".to_string()]), 27);
}

#[test]
fn test_9() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "D".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "C".to_string(), "+".to_string()]), 130);
}

#[test]
fn test_10() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "C".to_string(), "10".to_string()]), 13);
}

#[test]
fn test_11() {
    assert_eq!(Solution::cal_points(vec!["5".to_string(), "2".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 30);
}

#[test]
fn test_12() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "D".to_string(), "C".to_string(), "1".to_string(), "D".to_string(), "C".to_string(), "1".to_string(), "D".to_string(), "C".to_string(), "1".to_string()]), 4);
}

#[test]
fn test_13() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "20".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "+".to_string(), "-10".to_string(), "C".to_string(), "100".to_string()]), 530);
}

#[test]
fn test_14() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "D".to_string(), "+".to_string(), "5".to_string(), "C".to_string(), "D".to_string()]), 120);
}

#[test]
fn test_15() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "200".to_string(), "300".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "+".to_string()]), 4300);
}

#[test]
fn test_16() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "-50".to_string(), "D".to_string(), "C".to_string(), "+".to_string()]), 100);
}

#[test]
fn test_17() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "C".to_string()]), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::cal_points(vec!["10000".to_string(), "-10000".to_string(), "D".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string()]), -300000);
}

#[test]
fn test_19() {
    assert_eq!(Solution::cal_points(vec!["1000".to_string(), "500".to_string(), "250".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "C".to_string()]), 12500);
}

#[test]
fn test_20() {
    assert_eq!(Solution::cal_points(vec!["-1".to_string(), "-2".to_string(), "3".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 36);
}

#[test]
fn test_21() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "C".to_string(), "2".to_string(), "C".to_string(), "3".to_string(), "C".to_string(), "4".to_string(), "C".to_string(), "5".to_string(), "C".to_string(), "6".to_string(), "C".to_string(), "7".to_string(), "C".to_string(), "8".to_string(), "C".to_string(), "9".to_string(), "C".to_string(), "10".to_string(), "C".to_string(), "11".to_string(), "C".to_string(), "12".to_string(), "C".to_string(), "13".to_string(), "C".to_string(), "14".to_string(), "C".to_string(), "15".to_string(), "C".to_string(), "16".to_string(), "C".to_string(), "17".to_string(), "C".to_string(), "18".to_string(), "C".to_string(), "19".to_string(), "C".to_string(), "20".to_string(), "C".to_string()]), 0);
}

#[test]
fn test_22() {
    assert_eq!(Solution::cal_points(vec!["50000".to_string(), "-50000".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "25000".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "-25000".to_string(), "D".to_string(), "+".to_string()]), 125000);
}

#[test]
fn test_23() {
    assert_eq!(Solution::cal_points(vec!["5".to_string(), "-2".to_string(), "4".to_string(), "C".to_string(), "D".to_string(), "9".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 73);
}

#[test]
fn test_24() {
    assert_eq!(Solution::cal_points(vec!["30".to_string(), "-10".to_string(), "C".to_string(), "D".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "5".to_string(), "C".to_string(), "+".to_string()]), 1170);
}

#[test]
fn test_25() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 61572651155455);
}

#[test]
fn test_26() {
    assert_eq!(Solution::cal_points(vec!["-10".to_string(), "-20".to_string(), "+".to_string(), "C".to_string(), "-30".to_string(), "D".to_string(), "+".to_string(), "-40".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "50".to_string(), "D".to_string(), "+".to_string(), "+".to_string()]), 300);
}

#[test]
fn test_27() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "D".to_string(), "D".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "D".to_string(), "D".to_string(), "100".to_string(), "200".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string()]), 74401);
}

#[test]
fn test_28() {
    assert_eq!(Solution::cal_points(vec!["5".to_string(), "10".to_string(), "C".to_string(), "20".to_string(), "D".to_string(), "+".to_string(), "30".to_string(), "C".to_string(), "40".to_string(), "D".to_string(), "+".to_string(), "50".to_string()]), 415);
}

#[test]
fn test_29() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "-100".to_string(), "D".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "+".to_string()]), -2200);
}

#[test]
fn test_30() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string()]), 130);
}

#[test]
fn test_31() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "50".to_string(), "25".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-1".to_string(), "C".to_string(), "-2".to_string(), "C".to_string(), "-3".to_string(), "C".to_string(), "-4".to_string(), "C".to_string(), "-5".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 82575490);
}

#[test]
fn test_32() {
    assert_eq!(Solution::cal_points(vec!["5".to_string(), "C".to_string(), "10".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 550);
}

#[test]
fn test_33() {
    assert_eq!(Solution::cal_points(vec!["5".to_string(), "C".to_string(), "5".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "5".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "5".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 135);
}

#[test]
fn test_34() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "C".to_string(), "20".to_string(), "C".to_string(), "30".to_string(), "C".to_string(), "40".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "+".to_string()]), 840);
}

#[test]
fn test_35() {
    assert_eq!(Solution::cal_points(vec!["-32768".to_string(), "32767".to_string(), "D".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-32768".to_string(), "32767".to_string(), "D".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-32768".to_string(), "32767".to_string(), "D".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 2555823);
}

#[test]
fn test_36() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "6".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "2".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "-3".to_string(), "D".to_string(), "+".to_string()]), 22);
}

#[test]
fn test_37() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "-10".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), -73400300);
}

#[test]
fn test_38() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "3".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "D".to_string()]), 16401);
}

#[test]
fn test_39() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string(), "50".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "60".to_string(), "70".to_string(), "80".to_string(), "90".to_string(), "100".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "110".to_string(), "120".to_string(), "130".to_string(), "140".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "150".to_string()]), 2210);
}

#[test]
fn test_40() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "C".to_string(), "2".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "+".to_string(), "D".to_string()]), 2185);
}

#[test]
fn test_41() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "200".to_string(), "300".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 2202009600);
}

#[test]
fn test_42() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "3".to_string(), "+".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 2241);
}

#[test]
fn test_43() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "100".to_string(), "-100".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 606);
}

#[test]
fn test_44() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "-50".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "25".to_string(), "D".to_string(), "+".to_string(), "150".to_string(), "C".to_string(), "-200".to_string(), "D".to_string(), "+".to_string()]), -1100);
}

#[test]
fn test_45() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "-3".to_string(), "5".to_string(), "-5".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string()]), 5);
}

#[test]
fn test_46() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "3".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "10".to_string()]), 112);
}

#[test]
fn test_47() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "6".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "12".to_string(), "+".to_string(), "-10".to_string(), "D".to_string(), "+".to_string(), "5".to_string(), "C".to_string(), "+".to_string(), "20".to_string()]), -33);
}

#[test]
fn test_48() {
    assert_eq!(Solution::cal_points(vec!["-1".to_string(), "-2".to_string(), "-3".to_string(), "-4".to_string(), "-5".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-6".to_string(), "-7".to_string(), "-8".to_string(), "-9".to_string(), "-10".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-11".to_string(), "-12".to_string(), "-13".to_string(), "-14".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-15".to_string()]), -221);
}

#[test]
fn test_49() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "3".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "4".to_string(), "5".to_string(), "+".to_string(), "6".to_string(), "D".to_string(), "C".to_string(), "7".to_string(), "8".to_string(), "+".to_string(), "9".to_string(), "D".to_string(), "C".to_string(), "10".to_string(), "11".to_string(), "+".to_string(), "12".to_string(), "D".to_string(), "C".to_string(), "13".to_string(), "14".to_string(), "+".to_string(), "15".to_string(), "D".to_string(), "C".to_string(), "16".to_string(), "17".to_string(), "+".to_string(), "18".to_string(), "D".to_string(), "C".to_string(), "19".to_string(), "20".to_string(), "+".to_string(), "21".to_string(), "D".to_string(), "C".to_string(), "22".to_string(), "23".to_string(), "+".to_string(), "24".to_string(), "D".to_string(), "C".to_string(), "25".to_string(), "26".to_string(), "+".to_string(), "27".to_string(), "D".to_string(), "C".to_string(), "28".to_string(), "29".to_string(), "+".to_string(), "30".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "-50".to_string()]), 804);
}

#[test]
fn test_50() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "6".to_string(), "D".to_string(), "+".to_string(), "-2".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "5".to_string(), "D".to_string(), "+".to_string()]), 83);
}

#[test]
fn test_51() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 55);
}

#[test]
fn test_52() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "C".to_string(), "-100".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), -183500700);
}

#[test]
fn test_53() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "200".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "100".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "100".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "100".to_string(), "D".to_string(), "+".to_string()]), 2400);
}

#[test]
fn test_54() {
    assert_eq!(Solution::cal_points(vec!["-100".to_string(), "C".to_string(), "-50".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), -2750);
}

#[test]
fn test_55() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "-50".to_string(), "D".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "20".to_string(), "-20".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "1000".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "-500".to_string(), "+".to_string(), "D".to_string(), "+".to_string()]), 3280);
}

#[test]
fn test_56() {
    assert_eq!(Solution::cal_points(vec!["5".to_string(), "2".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "3".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 1371);
}

#[test]
fn test_57() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "-1".to_string(), "-1".to_string(), "C".to_string(), "2".to_string(), "D".to_string(), "5".to_string(), "+".to_string(), "10".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-5".to_string()]), 62);
}

#[test]
fn test_58() {
    assert_eq!(Solution::cal_points(vec!["-1".to_string(), "-2".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "-3".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "-4".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "-5".to_string(), "D".to_string(), "+".to_string()]), -78);
}

#[test]
fn test_59() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "200".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "1000".to_string(), "-1000".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 7300);
}

#[test]
fn test_60() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string(), "1".to_string(), "C".to_string()]), 0);
}

#[test]
fn test_61() {
    assert_eq!(Solution::cal_points(vec!["-1".to_string(), "-2".to_string(), "C".to_string(), "-3".to_string(), "D".to_string(), "+".to_string(), "-4".to_string(), "C".to_string(), "-5".to_string(), "D".to_string(), "+".to_string(), "-6".to_string()]), -55);
}

#[test]
fn test_62() {
    assert_eq!(Solution::cal_points(vec!["3".to_string(), "-1".to_string(), "-4".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-5".to_string(), "D".to_string(), "C".to_string(), "+".to_string()]), -84);
}

#[test]
fn test_63() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "C".to_string(), "1".to_string(), "D".to_string(), "+".to_string(), -1, "D".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "-10".to_string(), "C".to_string(), "100".to_string(), "D".to_string(), "+".to_string(), "C".to_string()]), 331);
}

#[test]
fn test_64() {
    assert_eq!(Solution::cal_points(vec!["1000".to_string(), "-1000".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "500".to_string(), "D".to_string(), "+".to_string(), "1500".to_string(), "C".to_string(), "-2000".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), -3670013000);
}

#[test]
fn test_65() {
    assert_eq!(Solution::cal_points(vec!["-5".to_string(), "D".to_string(), "-10".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "5".to_string(), "D".to_string(), "C".to_string(), "+".to_string()]), -95);
}

#[test]
fn test_66() {
    assert_eq!(Solution::cal_points(vec!["5".to_string(), "2".to_string(), "3".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "5".to_string(), "D".to_string(), "+".to_string(), "-5".to_string(), "C".to_string(), "10".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "-10".to_string(), "D".to_string(), "+".to_string(), "-20".to_string(), "+".to_string(), "-30".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "-40".to_string(), "+".to_string(), "-50".to_string(), "D".to_string(), "C".to_string(), "+".to_string()]), -539);
}

#[test]
fn test_67() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "20".to_string(), "+".to_string(), "30".to_string(), "+".to_string(), "C".to_string(), "40".to_string(), "D".to_string(), "+".to_string(), "50".to_string(), "+".to_string(), "60".to_string(), "+".to_string(), "C".to_string(), "70".to_string(), "D".to_string(), "+".to_string(), "80".to_string(), "+".to_string(), "90".to_string()]), 1490);
}

#[test]
fn test_68() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "50".to_string(), "D".to_string(), "+".to_string(), "-25".to_string(), "C".to_string(), "75".to_string(), "D".to_string(), "+".to_string(), "25".to_string(), "C".to_string(), "+".to_string(), "D".to_string()]), 1975);
}

#[test]
fn test_69() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "10".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "11".to_string(), "12".to_string(), "13".to_string(), "14".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "15".to_string()]), 221);
}

#[test]
fn test_70() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "10".to_string(), "+".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 2164);
}

#[test]
fn test_71() {
    assert_eq!(Solution::cal_points(vec!["100".to_string(), "50".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "20".to_string(), "D".to_string(), "+".to_string(), "10".to_string(), "C".to_string(), "+".to_string()]), 470);
}

#[test]
fn test_72() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]), 1794);
}

#[test]
fn test_73() {
    assert_eq!(Solution::cal_points(vec!["10".to_string(), "-10".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string(), "D".to_string(), "+".to_string(), "D".to_string(), "C".to_string(), "C".to_string(), "C".to_string()]), -20);
}

#[test]
fn test_74() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "C".to_string(), "2".to_string(), "D".to_string(), "C".to_string(), "3".to_string(), "D".to_string(), "C".to_string(), "4".to_string(), "D".to_string(), "C".to_string(), "5".to_string(), "D".to_string(), "C".to_string(), "6".to_string(), "D".to_string(), "C".to_string(), "7".to_string(), "D".to_string(), "+".to_string()]), 62);
}

#[test]
fn test_75() {
    assert_eq!(Solution::cal_points(vec!["5".to_string(), "10".to_string(), "D".to_string(), "+".to_string(), "C".to_string(), "5".to_string(), "D".to_string(), "C".to_string(), "+".to_string(), "-5".to_string(), "D".to_string(), "+".to_string()]), 35);
}

#[test]
fn test_76() {
    assert_eq!(Solution::cal_points(vec!["1".to_string(), "D".to_string(), "2".to_string(), "D".to_string(), "3".to_string(), "D".to_string(), "+".to_string(), "+".to_string(), "+".to_string(), "C".to_string(), "C".to_string(), "C".to_string()]), 18);
}

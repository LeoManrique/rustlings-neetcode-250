include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::eval_rpn(vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()]), 6);
}

#[test]
fn test_2() {
    assert_eq!(Solution::eval_rpn(vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]), 9);
}

#[test]
fn test_3() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "6".to_string(), "9".to_string(), "3".to_string(), "+".to_string(), "-11".to_string(), "*".to_string(), "/".to_string(), "*".to_string(), "17".to_string(), "+".to_string(), "5".to_string(), "+".to_string()]), 22);
}

#[test]
fn test_4() {
    assert_eq!(Solution::eval_rpn(vec!["20".to_string(), "5".to_string(), "/".to_string(), "2".to_string(), "+".to_string(), "10".to_string(), "3".to_string(), "/".to_string(), "-".to_string(), "3".to_string(), "4".to_string(), "*".to_string(), "+".to_string(), "2".to_string(), "-".to_string(), "1".to_string(), "*".to_string()]), 13);
}

#[test]
fn test_5() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "50".to_string(), "/".to_string(), "3".to_string(), "-".to_string(), "2".to_string(), "*".to_string(), "4".to_string(), "1".to_string(), "+".to_string(), "/".to_string(), "5".to_string(), "-".to_string(), "2".to_string(), "*".to_string()]), -10);
}

#[test]
fn test_6() {
    assert_eq!(Solution::eval_rpn(vec!["-10".to_string(), "-3".to_string(), "/".to_string(), "2".to_string(), "*".to_string()]), 6);
}

#[test]
fn test_7() {
    assert_eq!(Solution::eval_rpn(vec!["20".to_string(), "5".to_string(), "*".to_string()]), 100);
}

#[test]
fn test_8() {
    assert_eq!(Solution::eval_rpn(vec!["3".to_string(), "-4".to_string(), "+".to_string(), "2".to_string(), "*".to_string(), "5".to_string(), "/".to_string(), "10".to_string(), "+".to_string()]), 10);
}

#[test]
fn test_9() {
    assert_eq!(Solution::eval_rpn(vec!["8".to_string(), "4".to_string(), "+".to_string(), "3".to_string(), "*".to_string(), "2".to_string(), "/".to_string(), "7".to_string(), "-".to_string(), "1".to_string(), "+".to_string(), "5".to_string(), "*".to_string()]), 60);
}

#[test]
fn test_10() {
    assert_eq!(Solution::eval_rpn(vec!["17".to_string(), "5".to_string(), "+".to_string(), "3".to_string(), "8".to_string(), "*".to_string(), "-".to_string(), "10".to_string(), "2".to_string(), "/".to_string(), "+".to_string()]), 3);
}

#[test]
fn test_11() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "-5".to_string(), "*".to_string(), "2".to_string(), "-".to_string(), "3".to_string(), "*".to_string()]), -156);
}

#[test]
fn test_12() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "50".to_string(), "-".to_string(), "20".to_string(), "*".to_string(), "2".to_string(), "/".to_string(), "3".to_string(), "+".to_string(), "5".to_string(), "-".to_string(), "6".to_string(), "*".to_string(), "7".to_string(), "/".to_string(), "8".to_string(), "+".to_string(), "9".to_string(), "-".to_string()]), 425);
}

#[test]
fn test_13() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "50".to_string(), "/".to_string(), "20".to_string(), "10".to_string(), "/".to_string(), "*".to_string(), "30".to_string(), "2".to_string(), "*".to_string(), "+".to_string()]), 64);
}

#[test]
fn test_14() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "-50".to_string(), "/".to_string(), "20".to_string(), "-10".to_string(), "/".to_string(), "*".to_string()]), 4);
}

#[test]
fn test_15() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "2".to_string(), "+".to_string(), "3".to_string(), "4".to_string(), "+".to_string(), "*".to_string(), "5".to_string(), "6".to_string(), "+".to_string(), "7".to_string(), "8".to_string(), "+".to_string(), "*".to_string(), "9".to_string(), "10".to_string(), "+".to_string(), "*".to_string()]), 3135);
}

#[test]
fn test_16() {
    assert_eq!(Solution::eval_rpn(vec!["8".to_string(), "4".to_string(), "*".to_string(), "6".to_string(), "2".to_string(), "/".to_string(), "-".to_string(), "5".to_string(), "+".to_string(), "3".to_string(), "2".to_string(), "*".to_string()]), 6);
}

#[test]
fn test_17() {
    assert_eq!(Solution::eval_rpn(vec!["-10".to_string(), "-5".to_string(), "/".to_string(), "2".to_string(), "*".to_string(), "-1".to_string(), "+".to_string()]), 3);
}

#[test]
fn test_18() {
    assert_eq!(Solution::eval_rpn(vec!["8".to_string(), "12".to_string(), "+".to_string(), "3".to_string(), "/".to_string(), "4".to_string(), "*".to_string(), "5".to_string(), "2".to_string(), "/".to_string(), "-".to_string(), "7".to_string(), "+".to_string(), "2".to_string(), "*".to_string()]), 58);
}

#[test]
fn test_19() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "2".to_string(), "+".to_string(), "3".to_string(), "4".to_string(), "+".to_string(), "*".to_string()]), 21);
}

#[test]
fn test_20() {
    assert_eq!(Solution::eval_rpn(vec!["3".to_string(), "8".to_string(), "2".to_string(), "/".to_string(), "+".to_string(), "4".to_string(), "*".to_string(), "10".to_string(), "5".to_string(), "/".to_string(), "-".to_string(), "2".to_string(), "*".to_string()]), 52);
}

#[test]
fn test_21() {
    assert_eq!(Solution::eval_rpn(vec!["20".to_string(), "5".to_string(), "+".to_string(), "3".to_string(), "*".to_string(), "2".to_string(), "-".to_string(), "8".to_string(), "/".to_string(), "4".to_string(), "*".to_string()]), 36);
}

#[test]
fn test_22() {
    assert_eq!(Solution::eval_rpn(vec!["5".to_string(), "2".to_string(), "/".to_string(), "1".to_string(), "+".to_string(), "4".to_string(), "*".to_string()]), 12);
}

#[test]
fn test_23() {
    assert_eq!(Solution::eval_rpn(vec!["7".to_string(), "3".to_string(), "+".to_string(), "2".to_string(), "*".to_string(), "5".to_string(), "-".to_string()]), 15);
}

#[test]
fn test_24() {
    assert_eq!(Solution::eval_rpn(vec!["8".to_string(), "2".to_string(), "/".to_string(), "3".to_string(), "4".to_string(), "+".to_string(), "*".to_string(), "1".to_string(), "5".to_string(), "-".to_string(), "*".to_string()]), -112);
}

#[test]
fn test_25() {
    assert_eq!(Solution::eval_rpn(vec!["20".to_string(), "10".to_string(), "/".to_string(), "5".to_string(), "*".to_string()]), 10);
}

#[test]
fn test_26() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "3".to_string(), "2".to_string(), "*".to_string(), "+".to_string(), "5".to_string(), "-".to_string(), "1".to_string(), "2".to_string(), "+".to_string(), "/".to_string()]), 0);
}

#[test]
fn test_27() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "5".to_string(), "2".to_string(), "/".to_string(), "+".to_string(), "3".to_string(), "-14".to_string(), "*".to_string(), "8".to_string(), "/".to_string()]), -5);
}

#[test]
fn test_28() {
    assert_eq!(Solution::eval_rpn(vec!["-1".to_string(), "3".to_string(), "*".to_string(), "-1".to_string(), "2".to_string(), "+".to_string(), "*".to_string(), "4".to_string(), "1".to_string(), "-".to_string()]), 3);
}

#[test]
fn test_29() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "5".to_string(), "+".to_string(), "20".to_string(), "-".to_string(), "2".to_string(), "/".to_string(), "15".to_string(), "*".to_string(), "3".to_string(), "-".to_string(), "10".to_string(), "/".to_string(), "2".to_string(), "+".to_string(), "1".to_string(), "-".to_string(), "7".to_string(), "*".to_string(), "3".to_string(), "/".to_string(), "2".to_string(), "*".to_string()]), -8);
}

#[test]
fn test_30() {
    assert_eq!(Solution::eval_rpn(vec!["12".to_string(), "-3".to_string(), "*".to_string(), "4".to_string(), "/".to_string(), "8".to_string(), "+".to_string(), "2".to_string(), "-".to_string(), "5".to_string(), "*".to_string(), "-2".to_string(), "/".to_string()]), 7);
}

#[test]
fn test_31() {
    assert_eq!(Solution::eval_rpn(vec!["5".to_string(), "8".to_string(), "4".to_string(), "/".to_string(), "+".to_string()]), 7);
}

#[test]
fn test_32() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "4".to_string(), "2".to_string(), "+".to_string(), "*".to_string()]), 60);
}

#[test]
fn test_33() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "3".to_string(), "5".to_string(), "/".to_string(), "2".to_string(), "*".to_string(), "+".to_string(), "8".to_string(), "3".to_string(), "2".to_string(), "*".to_string(), "-".to_string(), "4".to_string(), "*".to_string(), "2".to_string(), "/".to_string(), "+".to_string()]), 14);
}

#[test]
fn test_34() {
    assert_eq!(Solution::eval_rpn(vec!["12".to_string(), "3".to_string(), "*".to_string()]), 36);
}

#[test]
fn test_35() {
    assert_eq!(Solution::eval_rpn(vec!["17".to_string(), "5".to_string(), "+".to_string(), "3".to_string(), "*".to_string(), "2".to_string(), "-".to_string(), "1".to_string(), "+".to_string(), "4".to_string(), "/".to_string(), "2".to_string(), "*".to_string(), "3".to_string(), "+".to_string(), "1".to_string(), "-".to_string(), "9".to_string(), "/".to_string(), "3".to_string(), "*".to_string(), "-11".to_string(), "+".to_string(), "-2".to_string(), "*".to_string(), "-3".to_string(), "+".to_string(), "17".to_string(), "/".to_string(), "5".to_string(), "-".to_string(), "7".to_string(), "+".to_string(), "-4".to_string(), "*".to_string()]), -8);
}

#[test]
fn test_36() {
    assert_eq!(Solution::eval_rpn(vec!["18".to_string(), "4".to_string(), "-".to_string()]), 14);
}

#[test]
fn test_37() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "20".to_string(), "+".to_string(), "30".to_string(), "*".to_string(), "40".to_string(), "+".to_string(), "50".to_string(), "*".to_string(), "60".to_string(), "-".to_string()]), 46940);
}

#[test]
fn test_38() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "-5".to_string(), "*".to_string(), "2".to_string(), "+".to_string(), "20".to_string(), "-10".to_string(), "/".to_string(), "*".to_string(), "3".to_string(), "+".to_string(), "2".to_string(), "*".to_string()]), 198);
}

#[test]
fn test_39() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string(), "+".to_string(), "*".to_string()]), 1400);
}

#[test]
fn test_40() {
    assert_eq!(Solution::eval_rpn(vec!["8".to_string(), "3".to_string(), "-".to_string(), "2".to_string(), "1".to_string(), "/".to_string(), "*".to_string()]), 10);
}

#[test]
fn test_41() {
    assert_eq!(Solution::eval_rpn(vec!["9".to_string(), "3".to_string(), "+".to_string(), "6".to_string(), "2".to_string(), "/".to_string(), "-".to_string(), "11".to_string(), "*".to_string(), "7".to_string(), "/".to_string(), "2".to_string(), "+".to_string()]), 16);
}

#[test]
fn test_42() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "+".to_string(), "*".to_string(), "5".to_string(), "/".to_string()]), 2);
}

#[test]
fn test_43() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "2".to_string(), "3".to_string(), "+".to_string(), "4".to_string(), "/".to_string(), "+".to_string(), "5".to_string(), "-".to_string(), "6".to_string(), "*".to_string()]), -18);
}

#[test]
fn test_44() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "1".to_string(), "+".to_string(), "1".to_string(), "1".to_string(), "+".to_string(), "1".to_string(), "1".to_string(), "+".to_string(), "1".to_string(), "1".to_string(), "+".to_string(), "1".to_string(), "1".to_string(), "+".to_string()]), 2);
}

#[test]
fn test_45() {
    assert_eq!(Solution::eval_rpn(vec!["-7".to_string(), "3".to_string(), "/".to_string(), "2".to_string(), "-3".to_string(), "*".to_string()]), -6);
}

#[test]
fn test_46() {
    assert_eq!(Solution::eval_rpn(vec!["9".to_string(), "3".to_string(), "+".to_string(), "6".to_string(), "2".to_string(), "/".to_string(), "*".to_string()]), 36);
}

#[test]
fn test_47() {
    assert_eq!(Solution::eval_rpn(vec!["9".to_string(), "3".to_string(), "-".to_string(), "4".to_string(), "/".to_string(), "8".to_string(), "2".to_string(), "*".to_string(), "+".to_string(), "6".to_string(), "1".to_string(), "-".to_string(), "*".to_string(), "3".to_string(), "+".to_string(), "5".to_string(), "-".to_string(), "11".to_string(), "/".to_string()]), 7);
}

#[test]
fn test_48() {
    assert_eq!(Solution::eval_rpn(vec!["7".to_string(), "-8".to_string(), "*".to_string(), "2".to_string(), "/".to_string(), "3".to_string(), "+".to_string(), "1".to_string(), "-".to_string(), "5".to_string(), "11".to_string(), "+".to_string(), "2".to_string(), "/".to_string(), "*".to_string(), "4".to_string(), "-".to_string()]), -212);
}

#[test]
fn test_49() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "50".to_string(), "+".to_string(), "25".to_string(), "-".to_string(), "10".to_string(), "*".to_string(), "5".to_string(), "/".to_string()]), 250);
}

#[test]
fn test_50() {
    assert_eq!(Solution::eval_rpn(vec!["7".to_string(), "-3".to_string(), "*".to_string(), "2".to_string(), "1".to_string(), "+".to_string(), "*".to_string(), "5".to_string(), "-".to_string(), "3".to_string(), "+".to_string(), "2".to_string(), "*".to_string()]), -130);
}

#[test]
fn test_51() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "3".to_string(), "/".to_string(), "5".to_string(), "+".to_string(), "-3".to_string(), "*".to_string(), "7".to_string(), "2".to_string(), "/".to_string(), "+".to_string(), "8".to_string(), "-".to_string(), "2".to_string(), "*".to_string(), "3".to_string(), "+".to_string(), "6".to_string(), "-".to_string(), "4".to_string(), "/".to_string(), "9".to_string(), "+".to_string(), "11".to_string(), "-".to_string(), "13".to_string(), "*".to_string(), "15".to_string(), "/".to_string(), "17".to_string(), "+".to_string(), "19".to_string(), "-".to_string()]), -16);
}

#[test]
fn test_52() {
    assert_eq!(Solution::eval_rpn(vec!["5".to_string(), "8".to_string(), "4".to_string(), "2".to_string(), "+".to_string(), "*".to_string()]), 48);
}

#[test]
fn test_53() {
    assert_eq!(Solution::eval_rpn(vec!["5".to_string(), "3".to_string(), "+".to_string(), "2".to_string(), "*".to_string(), "8".to_string(), "4".to_string(), "/".to_string(), "-".to_string()]), 14);
}

#[test]
fn test_54() {
    assert_eq!(Solution::eval_rpn(vec!["-1".to_string(), "-2".to_string(), "*".to_string()]), 2);
}

#[test]
fn test_55() {
    assert_eq!(Solution::eval_rpn(vec!["15".to_string(), "7".to_string(), "1".to_string(), "1".to_string(), "+".to_string(), "-".to_string(), "3".to_string(), "/".to_string(), "2".to_string(), "1".to_string(), "1".to_string(), "+".to_string(), "+".to_string(), "-".to_string()]), -3);
}

#[test]
fn test_56() {
    assert_eq!(Solution::eval_rpn(vec!["20".to_string(), "4".to_string(), "/".to_string(), "5".to_string(), "2".to_string(), "*".to_string(), "+".to_string()]), 15);
}

#[test]
fn test_57() {
    assert_eq!(Solution::eval_rpn(vec!["8".to_string(), "4".to_string(), "+".to_string(), "2".to_string(), "/".to_string(), "3".to_string(), "*".to_string(), "1".to_string(), "+".to_string(), "6".to_string(), "-".to_string()]), 13);
}

#[test]
fn test_58() {
    assert_eq!(Solution::eval_rpn(vec!["9".to_string(), "3".to_string(), "/".to_string(), "2".to_string(), "1".to_string(), "+".to_string(), "*".to_string()]), 9);
}

#[test]
fn test_59() {
    assert_eq!(Solution::eval_rpn(vec!["18".to_string(), "7".to_string(), "+".to_string(), "3".to_string(), "-".to_string(), "1".to_string(), "*".to_string(), "2".to_string(), "+".to_string(), "5".to_string(), "/".to_string(), "9".to_string(), "+".to_string(), "1".to_string(), "+".to_string(), "-11".to_string(), "*".to_string(), "13".to_string(), "-".to_string(), "5".to_string(), "+".to_string(), "9".to_string(), "+".to_string(), "15".to_string(), "+".to_string(), "1".to_string(), "-".to_string(), "7".to_string(), "*".to_string(), "8".to_string(), "+".to_string(), "-3".to_string(), "-".to_string(), "2".to_string(), "-".to_string()]), -964);
}

#[test]
fn test_60() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "-5".to_string(), "+".to_string(), "20".to_string(), "-10".to_string(), "/".to_string(), "2".to_string(), "*".to_string()]), -4);
}

#[test]
fn test_61() {
    assert_eq!(Solution::eval_rpn(vec!["5".to_string(), "-2".to_string(), "*".to_string()]), -10);
}

#[test]
fn test_62() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "20".to_string(), "5".to_string(), "/".to_string(), "2".to_string(), "*".to_string()]), 8);
}

#[test]
fn test_63() {
    assert_eq!(Solution::eval_rpn(vec!["13".to_string(), "7".to_string(), "2".to_string(), "+".to_string(), "/".to_string()]), 1);
}

#[test]
fn test_64() {
    assert_eq!(Solution::eval_rpn(vec!["-1".to_string(), "-1".to_string(), "*".to_string(), "-2".to_string(), "-".to_string(), "2".to_string(), "*".to_string(), "-3".to_string(), "/".to_string(), "4".to_string(), "-".to_string()]), -6);
}

#[test]
fn test_65() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "3".to_string(), "-".to_string()]), -2);
}

#[test]
fn test_66() {
    assert_eq!(Solution::eval_rpn(vec!["15".to_string(), "11".to_string(), "+".to_string()]), 26);
}

#[test]
fn test_67() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "20".to_string(), "+".to_string(), "30".to_string(), "40".to_string(), "+".to_string(), "50".to_string(), "60".to_string(), "+".to_string(), "70".to_string(), "80".to_string(), "+".to_string(), "90".to_string(), "*".to_string()]), 13500);
}

#[test]
fn test_68() {
    assert_eq!(Solution::eval_rpn(vec!["6".to_string(), "3".to_string(), "1".to_string(), "+".to_string(), "*".to_string(), "4".to_string(), "/".to_string()]), 6);
}

#[test]
fn test_69() {
    assert_eq!(Solution::eval_rpn(vec!["15".to_string(), "7".to_string(), "1".to_string(), "1".to_string(), "+".to_string(), "-".to_string(), "/".to_string(), "3".to_string(), "*".to_string(), "2".to_string(), "1".to_string(), "1".to_string(), "+".to_string(), "+".to_string(), "-".to_string()]), 5);
}

#[test]
fn test_70() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "50".to_string(), "+".to_string(), "20".to_string(), "*".to_string(), "4".to_string(), "/".to_string(), "2".to_string(), "+".to_string(), "8".to_string(), "-".to_string(), "3".to_string(), "*".to_string()]), 2232);
}

#[test]
fn test_71() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "2".to_string(), "+".to_string(), "3".to_string(), "4".to_string(), "+".to_string(), "*".to_string(), "5".to_string(), "6".to_string(), "+".to_string(), "*".to_string(), "7".to_string(), "8".to_string(), "+".to_string(), "*".to_string(), "9".to_string(), "10".to_string(), "+".to_string(), "*".to_string()]), 65835);
}

#[test]
fn test_72() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "50".to_string(), "-".to_string()]), 50);
}

#[test]
fn test_73() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "200".to_string(), "+".to_string(), "50".to_string(), "-".to_string(), "2".to_string(), "/".to_string(), "3".to_string(), "*".to_string(), "10".to_string(), "+".to_string()]), 385);
}

#[test]
fn test_74() {
    assert_eq!(Solution::eval_rpn(vec!["5".to_string(), "3".to_string(), "+".to_string(), "7".to_string(), "4".to_string(), "-".to_string(), "*".to_string()]), 24);
}

#[test]
fn test_75() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "2".to_string(), "+".to_string(), "3".to_string(), "*".to_string(), "4".to_string(), "+".to_string(), "5".to_string(), "*".to_string(), "6".to_string(), "+".to_string(), "7".to_string(), "*".to_string(), "8".to_string(), "+".to_string(), "9".to_string(), "+".to_string(), "10".to_string(), "+".to_string(), "11".to_string(), "+".to_string(), "12".to_string(), "+".to_string(), "13".to_string(), "+".to_string(), "14".to_string(), "+".to_string(), "15".to_string(), "+".to_string(), "16".to_string(), "+".to_string(), "17".to_string(), "+".to_string(), "18".to_string(), "+".to_string(), "19".to_string(), "+".to_string()]), 659);
}

#[test]
fn test_76() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "50".to_string(), "/".to_string(), "20".to_string(), "*".to_string(), "10".to_string(), "-".to_string(), "5".to_string(), "+".to_string(), "2".to_string(), "*".to_string(), "3".to_string(), "/".to_string(), "1".to_string(), "+".to_string(), "4".to_string(), "-".to_string()]), 20);
}

#[test]
fn test_77() {
    assert_eq!(Solution::eval_rpn(vec!["18".to_string(), "5".to_string(), "+".to_string(), "12".to_string(), "-".to_string(), "3".to_string(), "/".to_string(), "2".to_string(), "*".to_string(), "4".to_string(), "-".to_string()]), 2);
}

#[test]
fn test_78() {
    assert_eq!(Solution::eval_rpn(vec!["8".to_string(), "3".to_string(), "2".to_string(), "*".to_string(), "1".to_string(), "-".to_string(), "2".to_string(), "/".to_string()]), 2);
}

#[test]
fn test_79() {
    assert_eq!(Solution::eval_rpn(vec!["-11".to_string(), "-12".to_string(), "-".to_string(), "13".to_string(), "-14".to_string(), "-".to_string(), "*".to_string()]), 27);
}

#[test]
fn test_80() {
    assert_eq!(Solution::eval_rpn(vec!["20".to_string(), "5".to_string(), "2".to_string(), "/".to_string(), "*".to_string()]), 40);
}

#[test]
fn test_81() {
    assert_eq!(Solution::eval_rpn(vec!["0".to_string(), "3".to_string(), "/".to_string(), "5".to_string(), "+".to_string()]), 5);
}

#[test]
fn test_82() {
    assert_eq!(Solution::eval_rpn(vec!["3".to_string(), "-4".to_string(), "*".to_string(), "2".to_string(), "/".to_string(), "5".to_string(), "-".to_string()]), -11);
}

#[test]
fn test_83() {
    assert_eq!(Solution::eval_rpn(vec!["1".to_string(), "2".to_string(), "3".to_string(), "+".to_string(), "*".to_string()]), 5);
}

#[test]
fn test_84() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "50".to_string(), "2".to_string(), "*".to_string(), "-".to_string(), "30".to_string(), "20".to_string(), "+".to_string(), "/".to_string(), "10".to_string(), "+".to_string()]), 10);
}

#[test]
fn test_85() {
    assert_eq!(Solution::eval_rpn(vec!["7".to_string(), "-3".to_string(), "*".to_string(), "11".to_string(), "5".to_string(), "-".to_string(), "/".to_string(), "2".to_string(), "2".to_string(), "+".to_string(), "*".to_string()]), -12);
}

#[test]
fn test_86() {
    assert_eq!(Solution::eval_rpn(vec!["7".to_string(), "-3".to_string(), "*".to_string(), "2".to_string(), "1".to_string(), "+".to_string(), "-".to_string(), "15".to_string(), "5".to_string(), "/".to_string(), "+".to_string()]), -21);
}

#[test]
fn test_87() {
    assert_eq!(Solution::eval_rpn(vec!["2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "+".to_string(), "*".to_string(), "6".to_string(), "/".to_string()]), 4);
}

#[test]
fn test_88() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "6".to_string(), "9".to_string(), "3".to_string(), "+".to_string(), "-11".to_string(), "*".to_string()]), -132);
}

#[test]
fn test_89() {
    assert_eq!(Solution::eval_rpn(vec!["7".to_string(), "-3".to_string(), "*".to_string()]), -21);
}

#[test]
fn test_90() {
    assert_eq!(Solution::eval_rpn(vec!["10".to_string(), "5".to_string(), "-".to_string(), "3".to_string(), "*".to_string(), "4".to_string(), "+".to_string(), "2".to_string(), "/".to_string()]), 9);
}

#[test]
fn test_91() {
    assert_eq!(Solution::eval_rpn(vec!["7".to_string(), "-3".to_string(), "*".to_string(), "2".to_string(), "+".to_string(), "15".to_string(), "5".to_string(), "/".to_string(), "*".to_string()]), -57);
}

#[test]
fn test_92() {
    assert_eq!(Solution::eval_rpn(vec!["15".to_string(), "-3".to_string(), "/".to_string(), "7".to_string(), "+".to_string(), "2".to_string(), "*".to_string(), "10".to_string(), "-".to_string(), "5".to_string(), "*".to_string()]), -30);
}

#[test]
fn test_93() {
    assert_eq!(Solution::eval_rpn(vec!["100".to_string(), "50".to_string(), "/".to_string(), "25".to_string(), "*".to_string(), "5".to_string(), "+".to_string(), "3".to_string(), "-".to_string(), "2".to_string(), "*".to_string()]), 104);
}

#[test]
fn test_94() {
    assert_eq!(Solution::eval_rpn(vec!["7".to_string(), "-8".to_string(), "*".to_string(), "5".to_string(), "+".to_string(), "3".to_string(), "-12".to_string(), "/".to_string(), "4".to_string(), "*".to_string()]), 0);
}

#[test]
fn test_95() {
    assert_eq!(Solution::eval_rpn(vec!["7".to_string(), "8".to_string(), "3".to_string(), "*".to_string(), "+".to_string(), "2".to_string(), "/".to_string(), "4".to_string(), "-".to_string(), "10".to_string(), "5".to_string(), "*".to_string(), "+".to_string()]), 61);
}

#[test]
fn test_96() {
    assert_eq!(Solution::eval_rpn(vec!["5".to_string(), "2".to_string(), "/".to_string(), "10".to_string(), "2".to_string(), "/".to_string(), "-".to_string(), "3".to_string(), "*".to_string(), "7".to_string(), "+".to_string(), "-1".to_string(), "*".to_string()]), 2);
}

#[test]
fn test_97() {
    assert_eq!(Solution::eval_rpn(vec!["5".to_string(), "2".to_string(), "/".to_string(), "-1".to_string(), "*".to_string(), "3".to_string(), "-".to_string()]), -5);
}

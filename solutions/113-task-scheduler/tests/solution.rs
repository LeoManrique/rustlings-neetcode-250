include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D'], 3), 13);
}

#[test]
fn test_2() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D'], 0), 4);
}

#[test]
fn test_3() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G'], 2), 7);
}

#[test]
fn test_4() {
    assert_eq!(Solution::least_interval(vec!['A'], 5), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D'], 2), 16);
}

#[test]
fn test_6() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'B', 'B'], 0), 4);
}

#[test]
fn test_7() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G'], 0), 7);
}

#[test]
fn test_8() {
    assert_eq!(Solution::least_interval(vec!['A'], 0), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
}

#[test]
fn test_10() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 3), 10);
}

#[test]
fn test_11() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D', 'E', 'F'], 0), 6);
}

#[test]
fn test_12() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C'], 2), 13);
}

#[test]
fn test_13() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'A', 'B', 'A', 'B'], 2), 8);
}

#[test]
fn test_14() {
    assert_eq!(Solution::least_interval(vec!['A', 'C', 'A', 'B', 'D', 'B'], 1), 6);
}

#[test]
fn test_15() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D'], 4), 14);
}

#[test]
fn test_16() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'], 2), 16);
}

#[test]
fn test_17() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D'], 2), 20);
}

#[test]
fn test_18() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C'], 6), 45);
}

#[test]
fn test_19() {
    assert_eq!(Solution::least_interval(vec!['X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z'], 3), 35);
}

#[test]
fn test_20() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], 10), 52);
}

#[test]
fn test_21() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'E', 'E', 'E', 'E'], 3), 20);
}

#[test]
fn test_22() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'B', 'B', 'C', 'C', 'D', 'D', 'E', 'E', 'F', 'F', 'G', 'G', 'H', 'H', 'I', 'I', 'J', 'J', 'K', 'K', 'L', 'L', 'M', 'M', 'N', 'N', 'O', 'O', 'P', 'P', 'Q', 'Q', 'R', 'R', 'S', 'S', 'T', 'T', 'U', 'U', 'V', 'V', 'W', 'W', 'X', 'X', 'Y', 'Y', 'Z', 'Z'], 1), 52);
}

#[test]
fn test_23() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'F', 'F', 'F', 'G', 'G', 'G', 'H', 'H', 'H', 'I', 'I', 'I', 'J', 'J', 'J', 'K', 'K', 'K', 'L', 'L', 'L'], 10), 36);
}

#[test]
fn test_24() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], 5), 52);
}

#[test]
fn test_25() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'D', 'D', 'E', 'E', 'E', 'E', 'E', 'E'], 4), 27);
}

#[test]
fn test_26() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'E', 'E', 'E', 'E', 'E'], 4), 25);
}

#[test]
fn test_27() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D'], 5), 58);
}

#[test]
fn test_28() {
    assert_eq!(Solution::least_interval(vec!['X', 'X', 'X', 'Y', 'Y', 'Y', 'Z', 'Z', 'W', 'W', 'W', 'W', 'W', 'V', 'V', 'V', 'V', 'V'], 5), 26);
}

#[test]
fn test_29() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B'], 2), 35);
}

#[test]
fn test_30() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], 0), 26);
}

#[test]
fn test_31() {
    assert_eq!(Solution::least_interval(vec!['X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z', 'X', 'Y', 'Z'], 2), 24);
}

#[test]
fn test_32() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'F', 'F', 'F', 'G', 'G', 'G', 'H', 'H', 'H', 'I', 'I', 'I', 'J', 'J', 'J', 'K', 'K', 'K', 'L', 'L', 'L', 'M', 'M', 'M', 'N', 'N', 'N', 'O', 'O', 'O', 'P', 'P', 'P', 'Q', 'Q', 'Q', 'R', 'R', 'R', 'S', 'S', 'S', 'T', 'T', 'T', 'U', 'U', 'U', 'V', 'V', 'V', 'W', 'W', 'W', 'X', 'X', 'X', 'Y', 'Y', 'Y', 'Z', 'Z', 'Z'], 10), 78);
}

#[test]
fn test_33() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'D', 'D', 'E', 'E', 'F', 'F', 'G', 'G'], 3), 17);
}

#[test]
fn test_34() {
    assert_eq!(Solution::least_interval(vec!['P', 'P', 'P', 'P', 'P', 'Q', 'Q', 'Q', 'Q', 'Q', 'R', 'R', 'R', 'R', 'S', 'S', 'S', 'S', 'T', 'T', 'T', 'T', 'U', 'U', 'U', 'U'], 4), 26);
}

#[test]
fn test_35() {
    assert_eq!(Solution::least_interval(vec!['X', 'X', 'X', 'X', 'Y', 'Y', 'Y', 'Y', 'Z', 'Z', 'Z', 'Z', 'Z', 'W', 'W', 'W', 'W', 'W', 'W'], 6), 36);
}

#[test]
fn test_36() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C'], 2), 24);
}

#[test]
fn test_37() {
    assert_eq!(Solution::least_interval(vec!['X', 'X', 'X', 'X', 'X', 'Y', 'Y', 'Y', 'Y', 'Y', 'Z', 'Z', 'Z', 'Z', 'Z', 'W', 'W', 'W', 'W', 'W'], 5), 28);
}

#[test]
fn test_38() {
    assert_eq!(Solution::least_interval(vec!['X', 'X', 'X', 'Y', 'Y', 'Y', 'Z', 'Z', 'Z', 'Z', 'Z'], 5), 25);
}

#[test]
fn test_39() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'F', 'F', 'F', 'G', 'G', 'G', 'H', 'H', 'H', 'I', 'I', 'I', 'J', 'J', 'J'], 7), 30);
}

#[test]
fn test_40() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'F', 'F', 'F', 'G', 'G', 'G', 'H', 'H', 'H', 'I', 'I', 'I', 'J', 'J', 'J', 'K', 'K', 'K'], 2), 33);
}

#[test]
fn test_41() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'F', 'F', 'F', 'F', 'F', 'F', 'F', 'F', 'G', 'G', 'G', 'G', 'G', 'G', 'G', 'G', 'H', 'H', 'H', 'H', 'H', 'H', 'H', 'H'], 7), 64);
}

#[test]
fn test_42() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], 7), 78);
}

#[test]
fn test_43() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C'], 5), 33);
}

#[test]
fn test_44() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'E', 'E', 'E'], 5), 31);
}

#[test]
fn test_45() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D'], 4), 19);
}

#[test]
fn test_46() {
    assert_eq!(Solution::least_interval(vec!['X', 'Y', 'X', 'Y', 'X', 'Y', 'X', 'Y', 'X', 'Y', 'Z', 'Z', 'Z', 'Z', 'Z', 'W', 'W', 'W', 'W', 'W'], 3), 20);
}

#[test]
fn test_47() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B'], 1), 62);
}

#[test]
fn test_48() {
    assert_eq!(Solution::least_interval(vec!['F', 'G', 'H', 'F', 'G', 'H', 'F', 'G', 'H', 'F', 'G', 'H', 'F', 'G', 'H', 'F', 'G', 'H', 'F', 'G', 'H', 'F', 'G', 'H', 'F', 'G', 'H'], 2), 27);
}

#[test]
fn test_49() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'B', 'B', 'C', 'C', 'D', 'D', 'E', 'E', 'F', 'F', 'G', 'G', 'H', 'H', 'I', 'I', 'J', 'J', 'K', 'K', 'L', 'L', 'M', 'M', 'N', 'N', 'O', 'O', 'P', 'P', 'Q', 'Q', 'R', 'R', 'S', 'S', 'T', 'T', 'U', 'U', 'V', 'V', 'W', 'W', 'X', 'X', 'Y', 'Y', 'Z', 'Z'], 10), 52);
}

#[test]
fn test_50() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'F', 'F', 'F', 'G', 'G', 'G', 'H', 'H', 'H', 'I', 'I', 'I', 'J', 'J', 'J', 'K', 'K', 'K'], 4), 33);
}

#[test]
fn test_51() {
    assert_eq!(Solution::least_interval(vec!['Q', 'Q', 'Q', 'R', 'R', 'R', 'S', 'S', 'T', 'T', 'U', 'U', 'V', 'V', 'W', 'W', 'X', 'X', 'Y', 'Y', 'Z', 'Z'], 7), 22);
}

#[test]
fn test_52() {
    assert_eq!(Solution::least_interval(vec!['P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O'], 25), 26);
}

#[test]
fn test_53() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'D', 'D'], 8), 58);
}

#[test]
fn test_54() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'], 0), 68);
}

#[test]
fn test_55() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'F', 'F', 'F', 'F', 'F', 'F', 'F', 'F', 'G', 'G', 'G', 'G', 'G', 'G', 'G', 'G'], 15), 119);
}

#[test]
fn test_56() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C', 'A', 'B', 'C'], 1), 30);
}

#[test]
fn test_57() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'F', 'F', 'F'], 3), 18);
}

#[test]
fn test_58() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'A', 'C', 'A', 'D', 'A', 'E', 'A', 'F', 'A', 'G', 'A', 'H', 'A', 'I', 'A', 'J', 'A'], 5), 55);
}

#[test]
fn test_59() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'E'], 4), 17);
}

#[test]
fn test_60() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C'], 5), 27);
}

#[test]
fn test_61() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'F', 'F', 'F', 'G', 'G', 'G', 'H', 'H', 'H', 'I', 'I', 'I', 'J', 'J', 'J', 'K', 'K', 'K', 'L', 'L', 'L', 'M', 'M', 'M', 'N', 'N', 'N', 'O', 'O', 'O', 'P', 'P', 'P', 'Q', 'Q', 'Q', 'R', 'R', 'R', 'S', 'S', 'S', 'T', 'T', 'T', 'U', 'U', 'U', 'V', 'V', 'V', 'W', 'W', 'W', 'X', 'X', 'X', 'Y', 'Y', 'Y', 'Z', 'Z', 'Z'], 2), 78);
}

#[test]
fn test_62() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C'], 1), 27);
}

#[test]
fn test_63() {
    assert_eq!(Solution::least_interval(vec!['U', 'V', 'W', 'X', 'Y', 'Z', 'U', 'V', 'W', 'X', 'Y', 'Z', 'U', 'V', 'W', 'X', 'Y', 'Z', 'U', 'V', 'W', 'X', 'Y', 'Z'], 6), 27);
}

#[test]
fn test_64() {
    assert_eq!(Solution::least_interval(vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Y', 'Z', 'Z', 'Z', 'Z', 'Z', 'Z', 'Z', 'Z'], 6), 52);
}

#[test]
fn test_65() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'F', 'F', 'F', 'G', 'G', 'G', 'H', 'H', 'H', 'I', 'I', 'I', 'J', 'J', 'J'], 2), 31);
}

#[test]
fn test_66() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C'], 4), 23);
}

#[test]
fn test_67() {
    assert_eq!(Solution::least_interval(vec!['M', 'M', 'M', 'M', 'M', 'M', 'N', 'N', 'N', 'N', 'N', 'N', 'O', 'O', 'O', 'O', 'O', 'O', 'P', 'P', 'P', 'P', 'P', 'P'], 6), 39);
}

#[test]
fn test_68() {
    assert_eq!(Solution::least_interval(vec!['M', 'M', 'M', 'N', 'N', 'N', 'N', 'N', 'O', 'O', 'O', 'O', 'O', 'P', 'P', 'P', 'P', 'P', 'P', 'P', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q'], 5), 38);
}

#[test]
fn test_69() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'F', 'F', 'F', 'F', 'F', 'F', 'F', 'F', 'G', 'G', 'G', 'G', 'G', 'G', 'G', 'G', 'H', 'H', 'H', 'H', 'H', 'H', 'H', 'H'], 6), 59);
}

#[test]
fn test_70() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D'], 3), 32);
}

#[test]
fn test_71() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'E', 'E', 'E', 'E', 'F', 'F', 'F', 'F', 'G', 'G', 'G', 'G', 'H', 'H', 'H', 'H'], 7), 32);
}

#[test]
fn test_72() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E', 'E', 'F', 'F', 'F', 'G', 'G', 'G', 'H', 'H', 'H'], 2), 24);
}

#[test]
fn test_73() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D'], 5), 28);
}

#[test]
fn test_74() {
    assert_eq!(Solution::least_interval(vec!['Q', 'Q', 'Q', 'Q', 'Q', 'R', 'R', 'R', 'R', 'R', 'R', 'S', 'S', 'S', 'S', 'S', 'T', 'T', 'T', 'T', 'T'], 5), 31);
}

#[test]
fn test_75() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], 10), 26);
}

#[test]
fn test_76() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'D', 'D'], 6), 45);
}

#[test]
fn test_77() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'E', 'E', 'E', 'E', 'F', 'F', 'F', 'F', 'G', 'G', 'G', 'G', 'H', 'H', 'H', 'H'], 8), 37);
}

#[test]
fn test_78() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'E', 'E', 'E', 'E', 'F', 'F', 'F', 'F', 'G', 'G', 'G', 'G', 'H', 'H', 'H', 'H', 'I', 'I', 'I', 'I'], 10), 42);
}

#[test]
fn test_79() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], 25), 26);
}

#[test]
fn test_80() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'D', 'E', 'E', 'F', 'F', 'F'], 4), 16);
}

#[test]
fn test_81() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'D', 'E', 'E'], 4), 26);
}

#[test]
fn test_82() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'E', 'E', 'E', 'E', 'F', 'F', 'F', 'F'], 3), 24);
}

#[test]
fn test_83() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'C', 'C'], 5), 45);
}

#[test]
fn test_84() {
    assert_eq!(Solution::least_interval(vec!['A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B', 'A', 'B'], 0), 20);
}

#[test]
fn test_85() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D'], 6), 50);
}

#[test]
fn test_86() {
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', 'D', 'D', 'D'], 3), 25);
}

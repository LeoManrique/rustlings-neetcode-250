include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::longest_diverse_string(7, 1, 0), "aabaa".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::longest_diverse_string(100, 100, 100), "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::longest_diverse_string(33, 33, 34), "cabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::longest_diverse_string(2, 2, 2), "abcabc".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::longest_diverse_string(5, 5, 5), "abcabcabcabcabc".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::longest_diverse_string(5, 3, 2), "aabaabcabc".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::longest_diverse_string(50, 50, 0), "abababababababababababababababababababababababababababababababababababababababababababababababababab".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::longest_diverse_string(50, 30, 20), "aabaabaabaabaabaabaabaabaabaabaabaacaabaacaabaacaabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::longest_diverse_string(1, 1, 7), "ccaccbcc".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::longest_diverse_string(0, 0, 1), "c".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::longest_diverse_string(0, 0, 0), "".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::longest_diverse_string(100, 0, 0), "aa".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::longest_diverse_string(0, 0, 3), "cc".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::longest_diverse_string(40, 30, 30), "aabaacaabaacaabaacaabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::longest_diverse_string(0, 10, 10), "bcbcbcbcbcbcbcbcbcbc".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::longest_diverse_string(10, 10, 1), "ababababababababababc".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::longest_diverse_string(20, 10, 15), "aacaacaacaacaacabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::longest_diverse_string(15, 20, 10), "bbabbabbabbabababcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::longest_diverse_string(45, 10, 45), "acacacacacacacacacacacacacacacacacacacacacacacacacacacacacacacacacacacabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::longest_diverse_string(6, 6, 6), "abcabcabcabcabcabc".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::longest_diverse_string(3, 3, 3), "abcabcabc".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::longest_diverse_string(40, 20, 40), "acacacacacacacacacacacacacacacacacacacacabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::longest_diverse_string(15, 15, 15), "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_24() {
    assert_eq!(Solution::longest_diverse_string(33, 34, 33), "babcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::longest_diverse_string(5, 5, 4), "ababcabcabcabc".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::longest_diverse_string(10, 10, 20), "ccaccbccaccbccaccbcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::longest_diverse_string(60, 40, 5), "aabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaabababababababababababababababababcabcabcabcabc".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::longest_diverse_string(4, 4, 3), "ababcabcabc".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::longest_diverse_string(15, 15, 5), "abababababababababababcabcabcabcabc".to_string());
}

#[test]
fn test_30() {
    assert_eq!(Solution::longest_diverse_string(5, 4, 4), "aabcabcabcabc".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::longest_diverse_string(5, 5, 0), "ababababab".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::longest_diverse_string(10, 10, 5), "ababababababcabcabcabcabc".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::longest_diverse_string(5, 0, 5), "acacacacac".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::longest_diverse_string(20, 20, 10), "abababababababababababcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::longest_diverse_string(34, 33, 33), "aabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::longest_diverse_string(0, 0, 100), "cc".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::longest_diverse_string(10, 20, 10), "bbabbcbbabbcbbabbcbabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::longest_diverse_string(30, 20, 10), "aabaabaabaabaabaabaabaabaabaababcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_39() {
    assert_eq!(Solution::longest_diverse_string(30, 30, 20), "abababababababababababcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_40() {
    assert_eq!(Solution::longest_diverse_string(20, 25, 20), "bbabbcbbabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_41() {
    assert_eq!(Solution::longest_diverse_string(0, 50, 50), "bcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbc".to_string());
}

#[test]
fn test_42() {
    assert_eq!(Solution::longest_diverse_string(90, 5, 5), "aabaacaabaacaabaacaabaacaabaacaa".to_string());
}

#[test]
fn test_43() {
    assert_eq!(Solution::longest_diverse_string(20, 19, 18), "aababcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_44() {
    assert_eq!(Solution::longest_diverse_string(6, 7, 8), "cbcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_45() {
    assert_eq!(Solution::longest_diverse_string(1, 1, 5), "ccaccbc".to_string());
}

#[test]
fn test_46() {
    assert_eq!(Solution::longest_diverse_string(4, 4, 5), "cabcabcabcabc".to_string());
}

#[test]
fn test_47() {
    assert_eq!(Solution::longest_diverse_string(2, 1, 1), "aabc".to_string());
}

#[test]
fn test_48() {
    assert_eq!(Solution::longest_diverse_string(50, 50, 50), "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_49() {
    assert_eq!(Solution::longest_diverse_string(99, 1, 0), "aabaa".to_string());
}

#[test]
fn test_50() {
    assert_eq!(Solution::longest_diverse_string(34, 34, 32), "abababcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_51() {
    assert_eq!(Solution::longest_diverse_string(10, 45, 45), "bcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_52() {
    assert_eq!(Solution::longest_diverse_string(2, 3, 5), "ccbcabcabc".to_string());
}

#[test]
fn test_53() {
    assert_eq!(Solution::longest_diverse_string(15, 5, 10), "aacaacaacaacaacabcabcabcabcabc".to_string());
}

#[test]
fn test_54() {
    assert_eq!(Solution::longest_diverse_string(1, 1, 2), "cabc".to_string());
}

#[test]
fn test_55() {
    assert_eq!(Solution::longest_diverse_string(7, 8, 9), "cbcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_56() {
    assert_eq!(Solution::longest_diverse_string(8, 8, 8), "abcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_57() {
    assert_eq!(Solution::longest_diverse_string(33, 33, 33), "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_58() {
    assert_eq!(Solution::longest_diverse_string(25, 25, 24), "ababcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_59() {
    assert_eq!(Solution::longest_diverse_string(100, 50, 50), "aabaacaabaacaabaacaabaacaabaacaabaacaabaacaabaacaabaacaabaacaabaacaabaacaabaacaabaacaabaacaabaacaabacabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_60() {
    assert_eq!(Solution::longest_diverse_string(5, 2, 2), "aabaacabc".to_string());
}

#[test]
fn test_61() {
    assert_eq!(Solution::longest_diverse_string(25, 25, 25), "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_62() {
    assert_eq!(Solution::longest_diverse_string(99, 99, 99), "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_63() {
    assert_eq!(Solution::longest_diverse_string(15, 15, 10), "ababababababcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_64() {
    assert_eq!(Solution::longest_diverse_string(3, 3, 9), "ccaccbccacbcabc".to_string());
}

#[test]
fn test_65() {
    assert_eq!(Solution::longest_diverse_string(15, 15, 1), "abababababababababababababababc".to_string());
}

#[test]
fn test_66() {
    assert_eq!(Solution::longest_diverse_string(10, 10, 10), "abcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_67() {
    assert_eq!(Solution::longest_diverse_string(9, 8, 7), "aababcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_68() {
    assert_eq!(Solution::longest_diverse_string(3, 3, 4), "cabcabcabc".to_string());
}

#[test]
fn test_69() {
    assert_eq!(Solution::longest_diverse_string(1, 2, 1), "babc".to_string());
}

#[test]
fn test_70() {
    assert_eq!(Solution::longest_diverse_string(3, 2, 1), "aababc".to_string());
}

#[test]
fn test_71() {
    assert_eq!(Solution::longest_diverse_string(10, 1, 1), "aabaacaa".to_string());
}

#[test]
fn test_72() {
    assert_eq!(Solution::longest_diverse_string(10, 5, 15), "ccaccaccaccacacabcabcabcabcabc".to_string());
}

#[test]
fn test_73() {
    assert_eq!(Solution::longest_diverse_string(1, 1, 1), "abc".to_string());
}

#[test]
fn test_74() {
    assert_eq!(Solution::longest_diverse_string(49, 50, 51), "cbcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_75() {
    assert_eq!(Solution::longest_diverse_string(20, 10, 10), "aabaacaabaacaabaacaabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_76() {
    assert_eq!(Solution::longest_diverse_string(51, 50, 49), "aababcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_77() {
    assert_eq!(Solution::longest_diverse_string(20, 30, 20), "bbabbcbbabbcbbabbcbabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_78() {
    assert_eq!(Solution::longest_diverse_string(1, 100, 1), "bbabbcbb".to_string());
}

#[test]
fn test_79() {
    assert_eq!(Solution::longest_diverse_string(5, 5, 10), "ccaccbccabcabcabcabc".to_string());
}

#[test]
fn test_80() {
    assert_eq!(Solution::longest_diverse_string(50, 50, 1), "ababababababababababababababababababababababababababababababababababababababababababababababababababc".to_string());
}

#[test]
fn test_81() {
    assert_eq!(Solution::longest_diverse_string(0, 100, 0), "bb".to_string());
}

#[test]
fn test_82() {
    assert_eq!(Solution::longest_diverse_string(24, 25, 25), "bcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_83() {
    assert_eq!(Solution::longest_diverse_string(10, 10, 15), "ccaccbccabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_84() {
    assert_eq!(Solution::longest_diverse_string(1, 1, 100), "ccaccbcc".to_string());
}

#[test]
fn test_85() {
    assert_eq!(Solution::longest_diverse_string(1, 2, 3), "cbcabc".to_string());
}

#[test]
fn test_86() {
    assert_eq!(Solution::longest_diverse_string(20, 20, 19), "ababcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_87() {
    assert_eq!(Solution::longest_diverse_string(45, 45, 10), "ababababababababababababababababababababababababababababababababababababcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_88() {
    assert_eq!(Solution::longest_diverse_string(0, 5, 5), "bcbcbcbcbc".to_string());
}

#[test]
fn test_89() {
    assert_eq!(Solution::longest_diverse_string(60, 40, 0), "aabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaabaababababababababababababababababababababab".to_string());
}

#[test]
fn test_90() {
    assert_eq!(Solution::longest_diverse_string(3, 7, 5), "bbcbbcabcabcabc".to_string());
}

#[test]
fn test_91() {
    assert_eq!(Solution::longest_diverse_string(30, 30, 30), "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

#[test]
fn test_92() {
    assert_eq!(Solution::longest_diverse_string(30, 25, 45), "ccaccaccaccaccaccaccbccaccbccaccbcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc".to_string());
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
}

#[test]
fn test_4() {
    assert_eq!(Solution::length_of_longest_substring("abcdabcabcabcd".to_string()), 4);
}

#[test]
fn test_5() {
    assert_eq!(Solution::length_of_longest_substring("abcdefgabcdefgabcdefgabcdefg".to_string()), 7);
}

#[test]
fn test_6() {
    assert_eq!(Solution::length_of_longest_substring("aabbccddeeff".to_string()), 2);
}

#[test]
fn test_7() {
    assert_eq!(Solution::length_of_longest_substring("sldfjldskfjdslkfjsdkljflkjsdfljfsdlkflskdjflsdjflskdjflsdkjflsdfjlsd".to_string()), 6);
}

#[test]
fn test_8() {
    assert_eq!(Solution::length_of_longest_substring("racecar".to_string()), 4);
}

#[test]
fn test_9() {
    assert_eq!(Solution::length_of_longest_substring("abcdefghijklmnopqrstuvwxyz".to_string()), 26);
}

#[test]
fn test_10() {
    assert_eq!(Solution::length_of_longest_substring("aabacbebebe".to_string()), 4);
}

#[test]
fn test_11() {
    assert_eq!(Solution::length_of_longest_substring("ekdvdfis".to_string()), 5);
}

#[test]
fn test_12() {
    assert_eq!(Solution::length_of_longest_substring("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz1234567890abcdefghijklmnopqrstuvwxyz".to_string()), 36);
}

#[test]
fn test_13() {
    assert_eq!(Solution::length_of_longest_substring("abbaabbaabba".to_string()), 2);
}

#[test]
fn test_14() {
    assert_eq!(Solution::length_of_longest_substring("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), 2);
}

#[test]
fn test_15() {
    assert_eq!(Solution::length_of_longest_substring("abcdefghihgfedcba".to_string()), 9);
}

#[test]
fn test_16() {
    assert_eq!(Solution::length_of_longest_substring("abcdeffedcba".to_string()), 6);
}

#[test]
fn test_17() {
    assert_eq!(Solution::length_of_longest_substring("aaaaaaaabbbbbbbccccccdddddeeeeeeffffffffggggggg".to_string()), 2);
}

#[test]
fn test_18() {
    assert_eq!(Solution::length_of_longest_substring("tmmzuxt".to_string()), 5);
}

#[test]
fn test_19() {
    assert_eq!(Solution::length_of_longest_substring("nfpdmpi".to_string()), 5);
}

#[test]
fn test_20() {
    assert_eq!(Solution::length_of_longest_substring("anviaj".to_string()), 5);
}

#[test]
fn test_21() {
    assert_eq!(Solution::length_of_longest_substring("abcdeabcde".to_string()), 5);
}

#[test]
fn test_22() {
    assert_eq!(Solution::length_of_longest_substring("abcdabcabcd".to_string()), 4);
}

#[test]
fn test_23() {
    assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
}

#[test]
fn test_24() {
    assert_eq!(Solution::length_of_longest_substring("zyxwvutsrqponmlkjihgfedcba".to_string()), 26);
}

#[test]
fn test_25() {
    assert_eq!(Solution::length_of_longest_substring("abcdabcdeabcdabcdeabcd".to_string()), 5);
}

#[test]
fn test_26() {
    assert_eq!(Solution::length_of_longest_substring("rjqzupkoz".to_string()), 8);
}

#[test]
fn test_27() {
    assert_eq!(Solution::length_of_longest_substring("ababababababababab".to_string()), 2);
}

#[test]
fn test_28() {
    assert_eq!(Solution::length_of_longest_substring("!@#$%^&*()_+!@#$%^&*()_+".to_string()), 12);
}

#[test]
fn test_29() {
    assert_eq!(Solution::length_of_longest_substring("cdddddddddddddd".to_string()), 2);
}

#[test]
fn test_30() {
    assert_eq!(Solution::length_of_longest_substring("wobgrovw".to_string()), 6);
}

#[test]
fn test_31() {
    assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
}

#[test]
fn test_32() {
    assert_eq!(Solution::length_of_longest_substring("abcbacabc".to_string()), 3);
}

#[test]
fn test_33() {
    assert_eq!(Solution::length_of_longest_substring("ohvhjdml".to_string()), 6);
}

#[test]
fn test_34() {
    assert_eq!(Solution::length_of_longest_substring("123456789012345678901234567890".to_string()), 10);
}

#[test]
fn test_35() {
    assert_eq!(Solution::length_of_longest_substring("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz1234567890!@#$%^&*()_+".to_string()), 23);
}

#[test]
fn test_36() {
    assert_eq!(Solution::length_of_longest_substring("12345678901234567890".to_string()), 10);
}

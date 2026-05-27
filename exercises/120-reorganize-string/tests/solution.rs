include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::reorganize_string("aaabc".to_string()), "abaca".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::reorganize_string("aabbccc".to_string()), "cacbcba".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::reorganize_string("abcdefg".to_string()), "aebfcgd".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::reorganize_string("aaabbbccc".to_string()), "abacacbcb".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnop".to_string()), "agagbhbhcicjdkdlemenfofp".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::reorganize_string("abbabbaaa".to_string()), "ababababa".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::reorganize_string("abcdefghijklmnopqrstuvwxyz".to_string()), "anbocpdqerfsgthuivjwkxlymz".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::reorganize_string("aabac".to_string()), "abaca".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::reorganize_string("zzzza".to_string()), "".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopqrstuvwxy".to_string()), "ajakblbmcncodpdqeresftfugvgwhxhyi".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::reorganize_string("aabbaa".to_string()), "".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::reorganize_string("aabbbb".to_string()), "".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::reorganize_string("abababab".to_string()), "abababab".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::reorganize_string("aaabbbcccd".to_string()), "abacacbcbd".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::reorganize_string("aabbbcccc".to_string()), "cbcbcacab".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopqr".to_string()), "agahbhbicjckdldmeneofpfqgr".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopqrs".to_string()), "ahahbibjckcldmdneoepfqfrgsg".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopqrstuvwxyz".to_string()), "ajakblbmcncodpdqeresftfugvgwhxhyiz".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhij".to_string()), "aeafbfbgcgchdhdiej".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::reorganize_string("aaaaabc".to_string()), "".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::reorganize_string("zzzzzzz".to_string()), "".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::reorganize_string("a".to_string()), "a".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::reorganize_string("aabbc".to_string()), "abacb".to_string());
}

#[test]
fn test_24() {
    assert_eq!(Solution::reorganize_string("zzzzzaabbcc".to_string()), "zazbzbzczca".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::reorganize_string("aa".to_string()), "".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhi".to_string()), "aeafbfbgcgchdhdie".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::reorganize_string("aaabbcc".to_string()), "abacacb".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::reorganize_string("abcabc".to_string()), "abacbc".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::reorganize_string("aabb".to_string()), "abab".to_string());
}

#[test]
fn test_30() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopqrstuvw".to_string()), "aiajbkblcmcndodpeqerfsftgugvhwh".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::reorganize_string("vvvlo".to_string()), "vlvov".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::reorganize_string("abc".to_string()), "acb".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopqrstuvwx".to_string()), "aiajbkblcmcndodpeqerfsftgugvhwhx".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijkl".to_string()), "afafbgbgchchdidjekel".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopqrst".to_string()), "ahahbibjckcldmdneoepfqfrgsgt".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::reorganize_string("geeksforgeeks".to_string()), "ekesesefgogrk".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklm".to_string()), "afagbgbhchcidjdkelemf".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgg".to_string()), "adaebebfcfcgdg".to_string());
}

#[test]
fn test_39() {
    assert_eq!(Solution::reorganize_string("aaabcbb".to_string()), "ababacb".to_string());
}

#[test]
fn test_40() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghh".to_string()), "aeaebfbfcgcgdhdh".to_string());
}

#[test]
fn test_41() {
    assert_eq!(Solution::reorganize_string("bfrbs".to_string()), "brbsf".to_string());
}

#[test]
fn test_42() {
    assert_eq!(Solution::reorganize_string("aab".to_string()), "aba".to_string());
}

#[test]
fn test_43() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopqrstuv".to_string()), "ahaibjbkclcmdndoepeqfrfsgtguhv".to_string());
}

#[test]
fn test_44() {
    assert_eq!(Solution::reorganize_string("aabbcc".to_string()), "abacbc".to_string());
}

#[test]
fn test_45() {
    assert_eq!(Solution::reorganize_string("mississippi".to_string()), "isisipipsms".to_string());
}

#[test]
fn test_46() {
    assert_eq!(Solution::reorganize_string("zzzzz".to_string()), "".to_string());
}

#[test]
fn test_47() {
    assert_eq!(Solution::reorganize_string("zmrlllllll".to_string()), "".to_string());
}

#[test]
fn test_48() {
    assert_eq!(Solution::reorganize_string("ababab".to_string()), "ababab".to_string());
}

#[test]
fn test_49() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), "ananbobocpcpdqdqererfsfsgtgthuhuivivjwjwkxkxlylymzmz".to_string());
}

#[test]
fn test_50() {
    assert_eq!(Solution::reorganize_string("programming".to_string()), "rprogagimnm".to_string());
}

#[test]
fn test_51() {
    assert_eq!(Solution::reorganize_string("abacaba".to_string()), "ababaca".to_string());
}

#[test]
fn test_52() {
    assert_eq!(Solution::reorganize_string("zzzzzzzz".to_string()), "".to_string());
}

#[test]
fn test_53() {
    assert_eq!(Solution::reorganize_string("aaaabc".to_string()), "".to_string());
}

#[test]
fn test_54() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopq".to_string()), "agahbhbicjckdldmeneofpfqg".to_string());
}

#[test]
fn test_55() {
    assert_eq!(Solution::reorganize_string("aabbccddeeffgghhijklmnopqrstu".to_string()), "ahaibjbkclcmdndoepeqfrfsgtguh".to_string());
}

#[test]
fn test_56() {
    assert_eq!(Solution::reorganize_string("aaab".to_string()), "".to_string());
}

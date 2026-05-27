include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_window("acbbaca".to_string(), "aba".to_string()), "baca".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_window("aabbcc".to_string(), "abc".to_string()), "abbc".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_window("aaaaaaa".to_string(), "aa".to_string()), "aa".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_window("abcd".to_string(), "bd".to_string()), "bcd".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_window("ab".to_string(), "b".to_string()), "b".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_window("aa".to_string(), "aa".to_string()), "aa".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_window("fgrheahtfeqcrha".to_string(), "harf".to_string()), "fgrhea".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_window("aaaaaaaaaaaabbbbbcdd".to_string(), "abcdd".to_string()), "abbbbbcdd".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::min_window("abcde".to_string(), "f".to_string()), "".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::min_window("ab".to_string(), "a".to_string()), "a".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::min_window("abababab".to_string(), "abab".to_string()), "abab".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::min_window("aafffrbb".to_string(), "ffab".to_string()), "afffrb".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::min_window("bba".to_string(), "ab".to_string()), "ba".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::min_window("cbbbaaaaabbbcccccbbaa".to_string(), "aaabbbccc".to_string()), "aaabbbccc".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::min_window("abcabcabc".to_string(), "abc".to_string()), "abc".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::min_window("cabwefgewcwaefgcf".to_string(), "cae".to_string()), "cwae".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::min_window("abcabcabc".to_string(), "aaa".to_string()), "abcabca".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::min_window("abc".to_string(), "abc".to_string()), "abc".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::min_window("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(), "aaaaaaaaaaaaaaaaaaaaaaaaa".to_string()), "aaaaaaaaaaaaaaaaaaaaaaaaa".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::min_window("zjwsxeyrhtlnejzjwsxeyrhtlnej".to_string(), "nejxyz".to_string()), "nejzjwsxey".to_string());
}

#[test]
fn test_24() {
    assert_eq!(Solution::min_window("abcdefghijk".to_string(), "jihgfedcba".to_string()), "abcdefghij".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::min_window("abacabadabacaba".to_string(), "abc".to_string()), "bac".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::min_window("abcabcabcabcabcabcabcabcabcabc".to_string(), "cba".to_string()), "abc".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::min_window("aabbccddeeffgghhii".to_string(), "abcdefghi".to_string()), "abbccddeeffgghhi".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::min_window("aaaaaaaaaabbbbbbcccccc".to_string(), "abc".to_string()), "abbbbbbc".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::min_window("mississippi".to_string(), "issip".to_string()), "issip".to_string());
}

#[test]
fn test_30() {
    assert_eq!(Solution::min_window("zzzzzzzzzzzzzzzzz".to_string(), "zzz".to_string()), "zzz".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::min_window("xyxzyxzyxzyxzyx".to_string(), "xzy".to_string()), "yxz".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::min_window("abcdefg".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), "".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::min_window("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), "abbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyz".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::min_window("xyzxzyzxzyzzyzyxzyzyxzyzyx".to_string(), "xyzzyxzyzyzx".to_string()), "xyzxzyzxzyzzy".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::min_window("aaabbbaaabbbccc".to_string(), "aabbcc".to_string()), "aabbbcc".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::min_window("zzzzzzzzzzz".to_string(), "z".to_string()), "z".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::min_window("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "abcdef".to_string()), "abbccddeef".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::min_window("aabbccddeeffgghhii".to_string(), "aabbccddeeffgghhii".to_string()), "aabbccddeeffgghhii".to_string());
}

#[test]
fn test_39() {
    assert_eq!(Solution::min_window("abcabcabcabcabcabcabcabcabcabcabcabc".to_string(), "aabbcc".to_string()), "abcabc".to_string());
}

#[test]
fn test_40() {
    assert_eq!(Solution::min_window("bbaabbbbbaabbbbaaabbbbaaabbababababab".to_string(), "bbbbaaaa".to_string()), "aabbbbaa".to_string());
}

#[test]
fn test_41() {
    assert_eq!(Solution::min_window("mississippi".to_string(), "issi".to_string()), "issi".to_string());
}

#[test]
fn test_42() {
    assert_eq!(Solution::min_window("aaabbcccccccdddeee".to_string(), "abcde".to_string()), "abbcccccccddde".to_string());
}

#[test]
fn test_43() {
    assert_eq!(Solution::min_window("zxcvbnmasdfghjklqwertyuiop".to_string(), "opq".to_string()), "qwertyuiop".to_string());
}

#[test]
fn test_44() {
    assert_eq!(Solution::min_window("aaaaaaaaaaaabbbbbbcccccccc".to_string(), "abc".to_string()), "abbbbbbc".to_string());
}

#[test]
fn test_45() {
    assert_eq!(Solution::min_window("abcabcabcabcabcabc".to_string(), "cba".to_string()), "abc".to_string());
}

#[test]
fn test_46() {
    assert_eq!(Solution::min_window("xyzzxyzzxyzz".to_string(), "xyz".to_string()), "xyz".to_string());
}

#[test]
fn test_47() {
    assert_eq!(Solution::min_window("aaaaaaaaaaaabbbbbbbbbbbcccccccccc".to_string(), "abc".to_string()), "abbbbbbbbbbbc".to_string());
}

#[test]
fn test_48() {
    assert_eq!(Solution::min_window("xyzzzyxzyxzyxzyxzy".to_string(), "zyxzyxz".to_string()), "xyzzzyx".to_string());
}

#[test]
fn test_49() {
    assert_eq!(Solution::min_window("ababababababab".to_string(), "abba".to_string()), "abab".to_string());
}

#[test]
fn test_50() {
    assert_eq!(Solution::min_window("abcdabcdeabcdf".to_string(), "abcfed".to_string()), "eabcdf".to_string());
}

#[test]
fn test_51() {
    assert_eq!(Solution::min_window("abababababababababab".to_string(), "aabbcc".to_string()), "".to_string());
}

#[test]
fn test_52() {
    assert_eq!(Solution::min_window("abcdefg".to_string(), "xyz".to_string()), "".to_string());
}

#[test]
fn test_53() {
    assert_eq!(Solution::min_window("aabbccddeeffgghhii".to_string(), "aabbcc".to_string()), "aabbcc".to_string());
}

#[test]
fn test_54() {
    assert_eq!(Solution::min_window("ababcabcabcabcabcabcabcabcabcabcabcabc".to_string(), "abcabcabc".to_string()), "abcabcabc".to_string());
}

#[test]
fn test_55() {
    assert_eq!(Solution::min_window("hellohellohello".to_string(), "lle".to_string()), "ell".to_string());
}

#[test]
fn test_56() {
    assert_eq!(Solution::min_window("abcdefghijklmnopqrstuvwxyz".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), "abcdefghijklmnopqrstuvwxyz".to_string());
}

#[test]
fn test_57() {
    assert_eq!(Solution::min_window("sadjhasjhdjahsjdhasjhadsjhsahjdahjdsjahjdhasjdhajsdhasjdhajsdjasdhasjdhsa".to_string(), "hasjdh".to_string()), "hasjhd".to_string());
}

#[test]
fn test_58() {
    assert_eq!(Solution::min_window("bancbbancbbanc".to_string(), "abc".to_string()), "banc".to_string());
}

#[test]
fn test_59() {
    assert_eq!(Solution::min_window("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzzzzzzzzzzzzzzzzzzzz".to_string(), "abcdefghijklmnopqrstuvwxyz".to_string()), "abbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyz".to_string());
}

#[test]
fn test_60() {
    assert_eq!(Solution::min_window("thisisaverylongstringthatweneedtolookinto".to_string(), "tin".to_string()), "int".to_string());
}

#[test]
fn test_61() {
    assert_eq!(Solution::min_window("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string(), "zzzzzzzz".to_string()), "zzzzzzzz".to_string());
}

#[test]
fn test_62() {
    assert_eq!(Solution::min_window("bbaaaaaaabbbbcccc".to_string(), "aabbbccc".to_string()), "aabbbbccc".to_string());
}

#[test]
fn test_63() {
    assert_eq!(Solution::min_window("abababababababab".to_string(), "abab".to_string()), "abab".to_string());
}

#[test]
fn test_64() {
    assert_eq!(Solution::min_window("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string(), "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string()), "".to_string());
}

#[test]
fn test_65() {
    assert_eq!(Solution::min_window("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "mnopqrstuvwxyz".to_string()), "mnnooppqqrrssttuuvvwwxxyyz".to_string());
}

#[test]
fn test_66() {
    assert_eq!(Solution::min_window("qwertyuiopasdfghjklzxcvbnmqwertyuiopasdfghjklzxcvbnm".to_string(), "qwertyuiopzxcvbnm".to_string()), "zxcvbnmqwertyuiop".to_string());
}

#[test]
fn test_67() {
    assert_eq!(Solution::min_window("xyzzyxzyzyxzyzxzyzxzyxzyzyxzyx".to_string(), "xyz".to_string()), "xyz".to_string());
}

#[test]
fn test_68() {
    assert_eq!(Solution::min_window("abcbacbacbacbacbacbacbacbacbacbacbacbacbacbac".to_string(), "acbcba".to_string()), "abcbac".to_string());
}

#[test]
fn test_69() {
    assert_eq!(Solution::min_window("abccbaacz".to_string(), "abc".to_string()), "abc".to_string());
}

#[test]
fn test_70() {
    assert_eq!(Solution::min_window("abracadabra".to_string(), "rac".to_string()), "rac".to_string());
}

#[test]
fn test_71() {
    assert_eq!(Solution::min_window("aaaaaaaaaabbbbbbbbbbcccccccccc".to_string(), "abc".to_string()), "abbbbbbbbbbc".to_string());
}

#[test]
fn test_72() {
    assert_eq!(Solution::min_window("bbaaacccaaaabbbccc".to_string(), "aabbbccc".to_string()), "aabbbccc".to_string());
}

#[test]
fn test_73() {
    assert_eq!(Solution::min_window("bancancode".to_string(), "abc".to_string()), "banc".to_string());
}

#[test]
fn test_74() {
    assert_eq!(Solution::min_window("abcdefgabcdefg".to_string(), "abcd".to_string()), "abcd".to_string());
}

#[test]
fn test_75() {
    assert_eq!(Solution::min_window("qwertyuiopasdfghjklzxcvbnm".to_string(), "qwertyuiop".to_string()), "qwertyuiop".to_string());
}

#[test]
fn test_76() {
    assert_eq!(Solution::min_window("aaaaaaaaaa".to_string(), "aaa".to_string()), "aaa".to_string());
}

#[test]
fn test_77() {
    assert_eq!(Solution::min_window("ababababababababababababababababababab".to_string(), "aba".to_string()), "aba".to_string());
}

#[test]
fn test_78() {
    assert_eq!(Solution::min_window("aaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcccccccccccccccccccccccccccccccccc".to_string(), "abc".to_string()), "abbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbc".to_string());
}

#[test]
fn test_79() {
    assert_eq!(Solution::min_window("abcdeffeghijk".to_string(), "efg".to_string()), "feg".to_string());
}

#[test]
fn test_80() {
    assert_eq!(Solution::min_window("abcabcabcabcabc".to_string(), "cba".to_string()), "abc".to_string());
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "abc".to_string(), "aabbcc".to_string()), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::is_interleave("".to_string(), "a".to_string(), "a".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::is_interleave("".to_string(), "b".to_string(), "b".to_string()), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::is_interleave("a".to_string(), "".to_string(), "a".to_string()), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "".to_string(), "abc".to_string()), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::is_interleave("aab".to_string(), "dbbc".to_string(), "aadbbcbc".to_string()), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbbaccc".to_string()), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "def".to_string(), "defabc".to_string()), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::is_interleave("a".to_string(), "a".to_string(), "aa".to_string()), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::is_interleave("aaaa".to_string(), "bbbb".to_string(), "abababab".to_string()), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "def".to_string(), "abcdef".to_string()), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "def".to_string(), "abcfde".to_string()), false);
}

#[test]
fn test_13() {
    assert_eq!(Solution::is_interleave("".to_string(), "abc".to_string(), "abc".to_string()), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::is_interleave("a".to_string(), "b".to_string(), "ba".to_string()), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::is_interleave("a".to_string(), "a".to_string(), "ab".to_string()), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::is_interleave("ab".to_string(), "cd".to_string(), "abcd".to_string()), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "def".to_string(), "adbcef".to_string()), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::is_interleave("aabbcc".to_string(), "ddeeff".to_string(), "aaddbbccceedeff".to_string()), false);
}

#[test]
fn test_20() {
    assert_eq!(Solution::is_interleave("aabbcc".to_string(), "ddeeff".to_string(), "aabbccddeeff".to_string()), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::is_interleave("a".to_string(), "b".to_string(), "ab".to_string()), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::is_interleave("".to_string(), "def".to_string(), "def".to_string()), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::is_interleave("".to_string(), "".to_string(), "".to_string()), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "def".to_string(), "cabdef".to_string()), false);
}

#[test]
fn test_25() {
    assert_eq!(Solution::is_interleave("aabbcc".to_string(), "ddeeff".to_string(), "aadddbbbeeffcc".to_string()), false);
}

#[test]
fn test_26() {
    assert_eq!(Solution::is_interleave("".to_string(), "abcdef".to_string(), "abcdef".to_string()), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::is_interleave("abcdefghij".to_string(), "klmnopqrstu".to_string(), "ackbgdenfphiojqmrstnu".to_string()), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "defghijkl".to_string(), "adbecfghijkl".to_string()), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::is_interleave("aaa".to_string(), "bb".to_string(), "aaabb".to_string()), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::is_interleave("xy".to_string(), "xxyy".to_string(), "xxxyyy".to_string()), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::is_interleave("abcde".to_string(), "xyz".to_string(), "axbyczde".to_string()), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::is_interleave("abcdef".to_string(), "ghijkl".to_string(), "aghbciidfjkel".to_string()), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::is_interleave("abcdefgh".to_string(), "hgfedcba".to_string(), "ahbgcfeddaehbgcfecba".to_string()), false);
}

#[test]
fn test_34() {
    assert_eq!(Solution::is_interleave("abcd".to_string(), "efgh".to_string(), "aebfcgdh".to_string()), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::is_interleave("aaa".to_string(), "bbb".to_string(), "ababab".to_string()), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::is_interleave("mississippi".to_string(), "pwwkew".to_string(), "mpiswimppwisikew".to_string()), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::is_interleave("abcd".to_string(), "efgh".to_string(), "aebcfdgh".to_string()), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::is_interleave("aaa".to_string(), "bbb".to_string(), "aababbab".to_string()), false);
}

#[test]
fn test_39() {
    assert_eq!(Solution::is_interleave("a".to_string(), "ababababab".to_string(), "abababababa".to_string()), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::is_interleave("abcabc".to_string(), "defdef".to_string(), "abcdefabcdef".to_string()), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::is_interleave("zzzz".to_string(), "zzzz".to_string(), "zzzzzzzz".to_string()), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::is_interleave("abcdefghijklmnopqrstuvwxyz".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string(), "azbycxdwevfugthvisjrkqlpmqonnpmojniklhgfeidchbegaf".to_string()), false);
}

#[test]
fn test_43() {
    assert_eq!(Solution::is_interleave("aaabbb".to_string(), "aaabbb".to_string(), "aaabbaabbb".to_string()), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::is_interleave("abcdefghij".to_string(), "klmnopqrst".to_string(), "akblcfdmengoqhpristj".to_string()), false);
}

#[test]
fn test_45() {
    assert_eq!(Solution::is_interleave("abcabcabc".to_string(), "bcdbcd".to_string(), "abcbcadbcabc".to_string()), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::is_interleave("abcdefghij".to_string(), "klmnopqrst".to_string(), "akblcmfdgnheijopqrst".to_string()), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::is_interleave("aabb".to_string(), "ccdd".to_string(), "accbbd".to_string()), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::is_interleave("abcdefghij".to_string(), "klm".to_string(), "akbclmdefghij".to_string()), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::is_interleave("abcdefg".to_string(), "hijklmn".to_string(), "haijbckldemfnfg".to_string()), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::is_interleave("aabb".to_string(), "bbcc".to_string(), "aabbccbb".to_string()), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::is_interleave("aabbaa".to_string(), "bbccbb".to_string(), "aabbabbbccba".to_string()), false);
}

#[test]
fn test_52() {
    assert_eq!(Solution::is_interleave("aabbccddeeff".to_string(), "gghhiijjkkllmm".to_string(), "agbhchdijejfkflglhlimkmjmmnnoopp".to_string()), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::is_interleave("aabbcc".to_string(), "ddeeff".to_string(), "aaddbbeeffcc".to_string()), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::is_interleave("abcde".to_string(), "fghij".to_string(), "afbgchidiej".to_string()), false);
}

#[test]
fn test_55() {
    assert_eq!(Solution::is_interleave("xxxx".to_string(), "yyyy".to_string(), "xxyxyyxxyy".to_string()), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::is_interleave("abcdefghij".to_string(), "klmnopqrst".to_string(), "akbldmconepfqgrhtisj".to_string()), false);
}

#[test]
fn test_57() {
    assert_eq!(Solution::is_interleave("".to_string(), "abcd".to_string(), "abcd".to_string()), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::is_interleave("abcd".to_string(), "".to_string(), "abcd".to_string()), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::is_interleave("abcde".to_string(), "fghij".to_string(), "afbgchdije".to_string()), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::is_interleave("aab".to_string(), "bc".to_string(), "aabbcc".to_string()), false);
}

#[test]
fn test_61() {
    assert_eq!(Solution::is_interleave("xyz".to_string(), "uvw".to_string(), "xuzyvw".to_string()), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::is_interleave("aabb".to_string(), "ccdd".to_string(), "accdbdb".to_string()), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "abc".to_string(), "aabbbc".to_string()), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::is_interleave("abcd".to_string(), "efgh".to_string(), "aebfcdgh".to_string()), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::is_interleave("aaaaaa".to_string(), "bbbbbb".to_string(), "abababababab".to_string()), true);
}

#[test]
fn test_66() {
    assert_eq!(Solution::is_interleave("aabb".to_string(), "ccdd".to_string(), "acbdcbad".to_string()), false);
}

#[test]
fn test_67() {
    assert_eq!(Solution::is_interleave("xyz".to_string(), "abc".to_string(), "xyzabc".to_string()), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::is_interleave("aaa".to_string(), "bbb".to_string(), "aaabbb".to_string()), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::is_interleave("abcdefg".to_string(), "hijklmn".to_string(), "ahbicdjekflgmn".to_string()), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "defg".to_string(), "adbcefeg".to_string()), false);
}

#[test]
fn test_71() {
    assert_eq!(Solution::is_interleave("zzz".to_string(), "zzz".to_string(), "zzzzzzz".to_string()), false);
}

#[test]
fn test_72() {
    assert_eq!(Solution::is_interleave("".to_string(), "aabbccddeeff".to_string(), "aabbccddeeff".to_string()), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::is_interleave("abcabcabc".to_string(), "abcabcabc".to_string(), "aabbaabbaabbaabcabc".to_string()), false);
}

#[test]
fn test_74() {
    assert_eq!(Solution::is_interleave("".to_string(), "abcde".to_string(), "abcde".to_string()), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::is_interleave("abcd".to_string(), "aabb".to_string(), "aabaabdc".to_string()), false);
}

#[test]
fn test_76() {
    assert_eq!(Solution::is_interleave("ababab".to_string(), "bababa".to_string(), "babababababa".to_string()), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "".to_string(), "acb".to_string()), false);
}

#[test]
fn test_78() {
    assert_eq!(Solution::is_interleave("abcdefgh".to_string(), "ijklmnop".to_string(), "aicfbjdhekgmlonp".to_string()), false);
}

#[test]
fn test_79() {
    assert_eq!(Solution::is_interleave("abcd".to_string(), "ef".to_string(), "aebcfed".to_string()), false);
}

#[test]
fn test_80() {
    assert_eq!(Solution::is_interleave("abcdef".to_string(), "ghijkl".to_string(), "agbhicjkldfe".to_string()), false);
}

#[test]
fn test_81() {
    assert_eq!(Solution::is_interleave("aabbccddeeff".to_string(), "zzzxxx".to_string(), "azzbzxcxzddeeff".to_string()), false);
}

#[test]
fn test_82() {
    assert_eq!(Solution::is_interleave("abcdexyz".to_string(), "mnopqr".to_string(), "ambonpdqcrxezy".to_string()), false);
}

#[test]
fn test_83() {
    assert_eq!(Solution::is_interleave("abcabc".to_string(), "xyzxyz".to_string(), "axbyczaxbycz".to_string()), true);
}

#[test]
fn test_84() {
    assert_eq!(Solution::is_interleave("aab".to_string(), "dbb".to_string(), "aadbb".to_string()), false);
}

#[test]
fn test_85() {
    assert_eq!(Solution::is_interleave("aabbcc".to_string(), "ddeeff".to_string(), "addbeeffcc".to_string()), false);
}

#[test]
fn test_86() {
    assert_eq!(Solution::is_interleave("abcd".to_string(), "abdc".to_string(), "aabbbdcc".to_string()), false);
}

#[test]
fn test_87() {
    assert_eq!(Solution::is_interleave("".to_string(), "aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), true);
}

#[test]
fn test_88() {
    assert_eq!(Solution::is_interleave("abcd".to_string(), "efgh".to_string(), "abcdghfe".to_string()), false);
}

#[test]
fn test_89() {
    assert_eq!(Solution::is_interleave("aab".to_string(), "cdd".to_string(), "aadbb".to_string()), false);
}

#[test]
fn test_90() {
    assert_eq!(Solution::is_interleave("abababab".to_string(), "babababa".to_string(), "abbabababaabab".to_string()), false);
}

#[test]
fn test_91() {
    assert_eq!(Solution::is_interleave("abacabadabacaba".to_string(), "cdcdcdc".to_string(), "acbacabaacbadabacaba".to_string()), false);
}

#[test]
fn test_92() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "def".to_string(), "abcdefg".to_string()), false);
}

#[test]
fn test_93() {
    assert_eq!(Solution::is_interleave("".to_string(), "z".to_string(), "z".to_string()), true);
}

#[test]
fn test_94() {
    assert_eq!(Solution::is_interleave("ab".to_string(), "cd".to_string(), "cabd".to_string()), true);
}

#[test]
fn test_95() {
    assert_eq!(Solution::is_interleave("abcdef".to_string(), "".to_string(), "abcdef".to_string()), true);
}

#[test]
fn test_96() {
    assert_eq!(Solution::is_interleave("aaabbb".to_string(), "ccdddd".to_string(), "aaacbbbddddd".to_string()), false);
}

#[test]
fn test_97() {
    assert_eq!(Solution::is_interleave("xxyy".to_string(), "aabb".to_string(), "xaayybb".to_string()), false);
}

#[test]
fn test_98() {
    assert_eq!(Solution::is_interleave("ab".to_string(), "cd".to_string(), "cadb".to_string()), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "def".to_string(), "abdecf".to_string()), true);
}

#[test]
fn test_100() {
    assert_eq!(Solution::is_interleave("aabbcc".to_string(), "ddeeff".to_string(), "adabdbcecefef".to_string()), false);
}

#[test]
fn test_101() {
    assert_eq!(Solution::is_interleave("aa".to_string(), "aa".to_string(), "aaaa".to_string()), true);
}

#[test]
fn test_102() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "abc".to_string(), "abacbc".to_string()), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::is_interleave("abcdefghij".to_string(), "klmnopqrst".to_string(), "akblcmndfoegphiqjrst".to_string()), false);
}

#[test]
fn test_104() {
    assert_eq!(Solution::is_interleave("abcde".to_string(), "fghij".to_string(), "afbgchdeij".to_string()), true);
}

#[test]
fn test_105() {
    assert_eq!(Solution::is_interleave("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "".to_string(), "aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), true);
}

#[test]
fn test_106() {
    assert_eq!(Solution::is_interleave("zzzz".to_string(), "zzz".to_string(), "zzzzzzz".to_string()), true);
}

#[test]
fn test_107() {
    assert_eq!(Solution::is_interleave("abcd".to_string(), "efgh".to_string(), "abcdefghe".to_string()), false);
}

#[test]
fn test_108() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "abcd".to_string(), "abcabcd".to_string()), true);
}

#[test]
fn test_109() {
    assert_eq!(Solution::is_interleave("aabbccddeeff".to_string(), "".to_string(), "aabbccddeeff".to_string()), true);
}

#[test]
fn test_110() {
    assert_eq!(Solution::is_interleave("abcdefghijk".to_string(), "lmnopqrstuvwxyz".to_string(), "abcdefghijklmnoqrstuvwxyz".to_string()), false);
}

#[test]
fn test_111() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "abc".to_string(), "abccba".to_string()), false);
}

#[test]
fn test_112() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "def".to_string(), "dabcef".to_string()), true);
}

#[test]
fn test_113() {
    assert_eq!(Solution::is_interleave("abcde".to_string(), "".to_string(), "abcde".to_string()), true);
}

#[test]
fn test_114() {
    assert_eq!(Solution::is_interleave("abcdef".to_string(), "ghijkl".to_string(), "aghbidejfkfl".to_string()), false);
}

#[test]
fn test_115() {
    assert_eq!(Solution::is_interleave("z".to_string(), "".to_string(), "z".to_string()), true);
}

#[test]
fn test_116() {
    assert_eq!(Solution::is_interleave("zxy".to_string(), "xzy".to_string(), "zxzyxy".to_string()), true);
}

#[test]
fn test_117() {
    assert_eq!(Solution::is_interleave("abcdefghij".to_string(), "klmnopqrst".to_string(), "akblcmnodpefqgrstihj".to_string()), false);
}

#[test]
fn test_118() {
    assert_eq!(Solution::is_interleave("aab".to_string(), "acc".to_string(), "aaabcac".to_string()), false);
}

#[test]
fn test_119() {
    assert_eq!(Solution::is_interleave("aabbaabbaabb".to_string(), "bbccbbccbbcc".to_string(), "aabbaabbccbaabbccbaabbccbb".to_string()), false);
}

#[test]
fn test_120() {
    assert_eq!(Solution::is_interleave("xxxx".to_string(), "yyyy".to_string(), "xyxxyxyyxyyx".to_string()), false);
}

#[test]
fn test_121() {
    assert_eq!(Solution::is_interleave("aabbcc".to_string(), "ddeeff".to_string(), "aadbbccddeeff".to_string()), false);
}

#[test]
fn test_122() {
    assert_eq!(Solution::is_interleave("ababab".to_string(), "bababa".to_string(), "abababababab".to_string()), true);
}

#[test]
fn test_123() {
    assert_eq!(Solution::is_interleave("aabb".to_string(), "ccdd".to_string(), "acabcd".to_string()), false);
}

#[test]
fn test_124() {
    assert_eq!(Solution::is_interleave("aabb".to_string(), "ccdd".to_string(), "acbdad".to_string()), false);
}

#[test]
fn test_125() {
    assert_eq!(Solution::is_interleave("aaaaa".to_string(), "bbbbb".to_string(), "ababababab".to_string()), true);
}

#[test]
fn test_126() {
    assert_eq!(Solution::is_interleave("aabbccddeeff".to_string(), "gg hh ii jj kk ll".to_string(), "aaggbbccddhhffeeggiijjkkl".to_string()), false);
}

#[test]
fn test_127() {
    assert_eq!(Solution::is_interleave("abcabcabc".to_string(), "xyzxyzxyz".to_string(), "axbyczaxbyczaxbycz".to_string()), true);
}

#[test]
fn test_128() {
    assert_eq!(Solution::is_interleave("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcacd".to_string()), false);
}

#[test]
fn test_129() {
    assert_eq!(Solution::is_interleave("aabb".to_string(), "ccdd".to_string(), "acabcdbd".to_string()), true);
}

#[test]
fn test_130() {
    assert_eq!(Solution::is_interleave("abc".to_string(), "def".to_string(), "dabecf".to_string()), true);
}

#[test]
fn test_131() {
    assert_eq!(Solution::is_interleave("abcde".to_string(), "fghij".to_string(), "abcdefghij".to_string()), true);
}

#[test]
fn test_132() {
    assert_eq!(Solution::is_interleave("abcabcabc".to_string(), "xyzxyzxyz".to_string(), "axbxcyzaybzcyzabc".to_string()), false);
}

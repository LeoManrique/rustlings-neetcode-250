include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::longest_palindrome("abba".to_string()), "abba".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::longest_palindrome("aaaa".to_string()), "aaaa".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::longest_palindrome("abacdfgdcaba".to_string()), "aba".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::longest_palindrome("babad".to_string()), "aba".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::longest_palindrome("noon".to_string()), "noon".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::longest_palindrome("abcba".to_string()), "abcba".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::longest_palindrome("bcbabcbabcba".to_string()), "bcbabcbabcb".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::longest_palindrome("noonhighnoon".to_string()), "noon".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::longest_palindrome("forgeeksskeegfor".to_string()), "geeksskeeg".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::longest_palindrome("aaabaaaa".to_string()), "aaabaaa".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::longest_palindrome("abcdedcba".to_string()), "abcdedcba".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::longest_palindrome("aaa".to_string()), "aaa".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::longest_palindrome("aaaaa".to_string()), "aaaaa".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::longest_palindrome("racecar".to_string()), "racecar".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::longest_palindrome("abcdefg".to_string()), "a".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::longest_palindrome("aabbccddeeeffgg".to_string()), "eee".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::longest_palindrome("abcdedcba12321".to_string()), "abcdedcba".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::longest_palindrome("xxyyyxyxyxyxyxyxxyyxyxyxyxyxyx".to_string()), "xyxyxyxyxyx".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::longest_palindrome("thisisanexamplewithlongestpalindromeonyxdxyxdx".to_string()), "xdxyxdx".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::longest_palindrome("12345678987654321".to_string()), 12345678987654321);
}

#[test]
fn test_24() {
    assert_eq!(Solution::longest_palindrome("xyzaaazyxzyzyxyz".to_string()), "xyzaaazyx".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::longest_palindrome("12321abcdcba45654".to_string()), "abcdcba".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::longest_palindrome("012210".to_string()), "012210".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::longest_palindrome("tattarrattat".to_string()), "tattarrattat".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::longest_palindrome("aabbabbaa".to_string()), "aabbabbaa".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::longest_palindrome("abacdfgdcaba12321".to_string()), 12321);
}

#[test]
fn test_30() {
    assert_eq!(Solution::longest_palindrome("xyxxyxyxyxyxyxyx".to_string()), "xyxyxyxyxyxyx".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::longest_palindrome("1234321abcdefghgfedcba".to_string()), "abcdefghgfedcba".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::longest_palindrome("abababababababababababababababababababababababababababababababab".to_string()), "bababababababababababababababababababababababababababababababab".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::longest_palindrome("abacdfgdcabaxxxabcdcba".to_string()), "abcdcba".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::longest_palindrome("12321abccba45654".to_string()), "abccba".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::longest_palindrome("12321abcdedcbavcvcv".to_string()), "abcdedcba".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::longest_palindrome("abcbaekayakecivic".to_string()), "ekayake".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::longest_palindrome("noonmoonnoon".to_string()), "oonnoo".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::longest_palindrome("abcbaxxxxxabcdcba".to_string()), "cbaxxxxxabc".to_string());
}

#[test]
fn test_39() {
    assert_eq!(Solution::longest_palindrome("noonhighnoonnoon".to_string()), "noonnoon".to_string());
}

#[test]
fn test_40() {
    assert_eq!(Solution::longest_palindrome("noonmidnightnoon".to_string()), "noon".to_string());
}

#[test]
fn test_41() {
    assert_eq!(Solution::longest_palindrome("abcba12321defedcba".to_string()), "defed".to_string());
}

#[test]
fn test_42() {
    assert_eq!(Solution::longest_palindrome("aabbabaaaabbaaabaaabbbbbaaaaaabbbaaaabbbbaaabbaabbbaaaabbbaaabbbbaaabbaabbaabbab".to_string()), "bbaaabbbbaaabb".to_string());
}

#[test]
fn test_43() {
    assert_eq!(Solution::longest_palindrome("ababababababababa".to_string()), "ababababababababa".to_string());
}

#[test]
fn test_44() {
    assert_eq!(Solution::longest_palindrome("noonnoonnoonnoonnoonnoon".to_string()), "noonnoonnoonnoonnoonnoon".to_string());
}

#[test]
fn test_45() {
    assert_eq!(Solution::longest_palindrome("abccbaabacdfgdcaba".to_string()), "abccba".to_string());
}

#[test]
fn test_46() {
    assert_eq!(Solution::longest_palindrome("racecarxracecar".to_string()), "racecarxracecar".to_string());
}

#[test]
fn test_47() {
    assert_eq!(Solution::longest_palindrome("madamracecarlevel".to_string()), "racecar".to_string());
}

#[test]
fn test_48() {
    assert_eq!(Solution::longest_palindrome("babcbabcbabcba".to_string()), "abcbabcbabcba".to_string());
}

#[test]
fn test_49() {
    assert_eq!(Solution::longest_palindrome("abacdfgdcabaabacdfgdcaba".to_string()), "dcabaabacd".to_string());
}

#[test]
fn test_50() {
    assert_eq!(Solution::longest_palindrome("madamintanimadaminabba".to_string()), "animadamina".to_string());
}

#[test]
fn test_51() {
    assert_eq!(Solution::longest_palindrome("noonracecarracecar".to_string()), "racecarracecar".to_string());
}

#[test]
fn test_52() {
    assert_eq!(Solution::longest_palindrome("zzzzzzzzzzzz".to_string()), "zzzzzzzzzzzz".to_string());
}

#[test]
fn test_53() {
    assert_eq!(Solution::longest_palindrome("racecar2racecar".to_string()), "racecar2racecar".to_string());
}

#[test]
fn test_54() {
    assert_eq!(Solution::longest_palindrome("zxyabcddcbaabczyx".to_string()), "abcddcba".to_string());
}

#[test]
fn test_55() {
    assert_eq!(Solution::longest_palindrome("deeee".to_string()), "eeee".to_string());
}

#[test]
fn test_56() {
    assert_eq!(Solution::longest_palindrome("abacdfgdcabacdfgdcaba".to_string()), "dcabacd".to_string());
}

#[test]
fn test_57() {
    assert_eq!(Solution::longest_palindrome("1234543216789876".to_string()), 123454321);
}

#[test]
fn test_58() {
    assert_eq!(Solution::longest_palindrome("abcbaaabcba".to_string()), "abcbaaabcba".to_string());
}

#[test]
fn test_59() {
    assert_eq!(Solution::longest_palindrome("abcdedcbaefghihgfexyzzyx".to_string()), "efghihgfe".to_string());
}

#[test]
fn test_60() {
    assert_eq!(Solution::longest_palindrome("abcdefgfebac".to_string()), "efgfe".to_string());
}

#[test]
fn test_61() {
    assert_eq!(Solution::longest_palindrome("levelhannahlevel".to_string()), "levelhannahlevel".to_string());
}

#[test]
fn test_62() {
    assert_eq!(Solution::longest_palindrome("xxyyzzzyyxx".to_string()), "xxyyzzzyyxx".to_string());
}

#[test]
fn test_63() {
    assert_eq!(Solution::longest_palindrome("abcddcbaabcddcbaxyzzyx".to_string()), "abcddcbaabcddcba".to_string());
}

#[test]
fn test_64() {
    assert_eq!(Solution::longest_palindrome("racecar12321racecar".to_string()), "racecar12321racecar".to_string());
}

#[test]
fn test_65() {
    assert_eq!(Solution::longest_palindrome("abcdeffedcba".to_string()), "abcdeffedcba".to_string());
}

#[test]
fn test_66() {
    assert_eq!(Solution::longest_palindrome("civicracecar".to_string()), "racecar".to_string());
}

#[test]
fn test_67() {
    assert_eq!(Solution::longest_palindrome("levelmadammadam".to_string()), "madammadam".to_string());
}

#[test]
fn test_68() {
    assert_eq!(Solution::longest_palindrome("zxyaxzyaz".to_string()), "z".to_string());
}

#[test]
fn test_69() {
    assert_eq!(Solution::longest_palindrome("abcdefedcba".to_string()), "abcdefedcba".to_string());
}

#[test]
fn test_70() {
    assert_eq!(Solution::longest_palindrome("12321321321321321".to_string()), 12321);
}

#[test]
fn test_71() {
    assert_eq!(Solution::longest_palindrome("xyzzyxcbaapqrqpabczyzyx".to_string()), "apqrqpa".to_string());
}

#[test]
fn test_72() {
    assert_eq!(Solution::longest_palindrome("abacdfgdcaba123321".to_string()), 123321);
}

#[test]
fn test_73() {
    assert_eq!(Solution::longest_palindrome("abacdfgdcabaxxxxxabcdcba".to_string()), "baxxxxxab".to_string());
}

#[test]
fn test_74() {
    assert_eq!(Solution::longest_palindrome("aabcdcbadefedcbaa".to_string()), "abcdcba".to_string());
}

#[test]
fn test_75() {
    assert_eq!(Solution::longest_palindrome("abcdefghijiklmnopqrstuvwxyzzyxwvutsrqponmlkjihgfedcba".to_string()), "klmnopqrstuvwxyzzyxwvutsrqponmlk".to_string());
}

#[test]
fn test_76() {
    assert_eq!(Solution::longest_palindrome("bananaananab".to_string()), "bananaananab".to_string());
}

#[test]
fn test_77() {
    assert_eq!(Solution::longest_palindrome("aabbccddeedcba".to_string()), "deed".to_string());
}

#[test]
fn test_78() {
    assert_eq!(Solution::longest_palindrome("noonhighnoonnoonhighnoon".to_string()), "hnoonnoonh".to_string());
}

#[test]
fn test_79() {
    assert_eq!(Solution::longest_palindrome("babaddabba".to_string()), "baddab".to_string());
}

#[test]
fn test_80() {
    assert_eq!(Solution::longest_palindrome("abababababababababababababababababababababababababababababababbababa".to_string()), "babababababababababababababababababababababababababababababab".to_string());
}

#[test]
fn test_81() {
    assert_eq!(Solution::longest_palindrome("abcdeedcba1234321xyzzyx".to_string()), "abcdeedcba".to_string());
}

#[test]
fn test_82() {
    assert_eq!(Solution::longest_palindrome("aabb".to_string()), "bb".to_string());
}

#[test]
fn test_83() {
    assert_eq!(Solution::longest_palindrome("mamamamamamamamama".to_string()), "amamamamamamamama".to_string());
}

#[test]
fn test_84() {
    assert_eq!(Solution::longest_palindrome("abcdefgfedcba".to_string()), "abcdefgfedcba".to_string());
}

#[test]
fn test_85() {
    assert_eq!(Solution::longest_palindrome("abcbabcba".to_string()), "abcbabcba".to_string());
}

#[test]
fn test_86() {
    assert_eq!(Solution::longest_palindrome("xyzzzzyxabcdefedcba".to_string()), "abcdefedcba".to_string());
}

#[test]
fn test_87() {
    assert_eq!(Solution::longest_palindrome("banana".to_string()), "anana".to_string());
}

#[test]
fn test_88() {
    assert_eq!(Solution::longest_palindrome("abcbcbcbcbcbcbcbcbcbcbcbcb".to_string()), "bcbcbcbcbcbcbcbcbcbcbcbcb".to_string());
}

#[test]
fn test_89() {
    assert_eq!(Solution::longest_palindrome("anana".to_string()), "anana".to_string());
}

#[test]
fn test_90() {
    assert_eq!(Solution::longest_palindrome("aabbccddeeeeddccbbbaa".to_string()), "bbccddeeeeddccbb".to_string());
}

#[test]
fn test_91() {
    assert_eq!(Solution::longest_palindrome("12321abcdedcba45654".to_string()), "abcdedcba".to_string());
}

#[test]
fn test_92() {
    assert_eq!(Solution::longest_palindrome("aabbccddeeffgg".to_string()), "gg".to_string());
}

#[test]
fn test_93() {
    assert_eq!(Solution::longest_palindrome("levelracecardeifiedracecar".to_string()), "racecardeifiedracecar".to_string());
}

#[test]
fn test_94() {
    assert_eq!(Solution::longest_palindrome("aaaaabbbbbaaaa".to_string()), "aaaabbbbbaaaa".to_string());
}

#[test]
fn test_95() {
    assert_eq!(Solution::longest_palindrome("abccba".to_string()), "abccba".to_string());
}

#[test]
fn test_96() {
    assert_eq!(Solution::longest_palindrome("abcdcba12321xyzzyx".to_string()), "abcdcba".to_string());
}

#[test]
fn test_97() {
    assert_eq!(Solution::longest_palindrome("12321abcba21321".to_string()), "abcba".to_string());
}

#[test]
fn test_98() {
    assert_eq!(Solution::longest_palindrome("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), "zz".to_string());
}

#[test]
fn test_99() {
    assert_eq!(Solution::longest_palindrome("abcdcbaxxxabcdcbaabcdcbaxxxabcdcba".to_string()), "abcdcbaxxxabcdcbaabcdcbaxxxabcdcba".to_string());
}

#[test]
fn test_100() {
    assert_eq!(Solution::longest_palindrome("xyzabcbaxyz".to_string()), "abcba".to_string());
}

#[test]
fn test_101() {
    assert_eq!(Solution::longest_palindrome("racecarannakayak".to_string()), "racecar".to_string());
}

#[test]
fn test_102() {
    assert_eq!(Solution::longest_palindrome("abacdfgdcab".to_string()), "aba".to_string());
}

#[test]
fn test_103() {
    assert_eq!(Solution::longest_palindrome("abcdeedcbafedcbe".to_string()), "abcdeedcba".to_string());
}

#[test]
fn test_104() {
    assert_eq!(Solution::longest_palindrome("a1b2c3d4c3b2a".to_string()), "a".to_string());
}

#[test]
fn test_105() {
    assert_eq!(Solution::longest_palindrome("abccccba".to_string()), "abccccba".to_string());
}

#[test]
fn test_106() {
    assert_eq!(Solution::longest_palindrome("noonnoonnoon".to_string()), "noonnoonnoon".to_string());
}

#[test]
fn test_107() {
    assert_eq!(Solution::longest_palindrome("aabbccddeeeedddccbaa".to_string()), "ddeeeedd".to_string());
}

#[test]
fn test_108() {
    assert_eq!(Solution::longest_palindrome("AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz".to_string()), "A".to_string());
}

#[test]
fn test_109() {
    assert_eq!(Solution::longest_palindrome("acbbac".to_string()), "bb".to_string());
}

#[test]
fn test_110() {
    assert_eq!(Solution::longest_palindrome("noonlevelnoon".to_string()), "noonlevelnoon".to_string());
}

#[test]
fn test_111() {
    assert_eq!(Solution::longest_palindrome("abbaabba".to_string()), "abbaabba".to_string());
}

#[test]
fn test_112() {
    assert_eq!(Solution::longest_palindrome("rotor1234321rotor".to_string()), "rotor1234321rotor".to_string());
}

#[test]
fn test_113() {
    assert_eq!(Solution::longest_palindrome("aaaaabaaa".to_string()), "aaabaaa".to_string());
}

#[test]
fn test_114() {
    assert_eq!(Solution::longest_palindrome("abcdefghihgfedcba".to_string()), "abcdefghihgfedcba".to_string());
}

#[test]
fn test_115() {
    assert_eq!(Solution::longest_palindrome("civicdeifiedrotorlevel".to_string()), "deified".to_string());
}

#[test]
fn test_116() {
    assert_eq!(Solution::longest_palindrome("aquickbrownfoxjumpsoverthelazydog".to_string()), "a".to_string());
}

#[test]
fn test_117() {
    assert_eq!(Solution::longest_palindrome("zyxwvutsrqponmlkjihgfedcbaedcba".to_string()), "z".to_string());
}

#[test]
fn test_118() {
    assert_eq!(Solution::longest_palindrome("aabcddeffedcba".to_string()), "deffed".to_string());
}

#[test]
fn test_119() {
    assert_eq!(Solution::longest_palindrome("pppppppppppppppppppppppppppppppp".to_string()), "pppppppppppppppppppppppppppppppp".to_string());
}

#[test]
fn test_120() {
    assert_eq!(Solution::longest_palindrome("aabbccddeeeedddccbbaa".to_string()), "ddeeeedd".to_string());
}

#[test]
fn test_121() {
    assert_eq!(Solution::longest_palindrome("a1b2c3d4e5f6g7h8i9j0j9i8h7g6f5e4d3c2b1a".to_string()), "a1b2c3d4e5f6g7h8i9j0j9i8h7g6f5e4d3c2b1a".to_string());
}

#[test]
fn test_122() {
    assert_eq!(Solution::longest_palindrome("mississippi".to_string()), "ississi".to_string());
}

#[test]
fn test_123() {
    assert_eq!(Solution::longest_palindrome("zxcvbnmlkjhgfdsapoiuytrewqpoiuytrewqpoiuytrewqpoiuytrewq".to_string()), "z".to_string());
}

#[test]
fn test_124() {
    assert_eq!(Solution::longest_palindrome("deifiedrotorlevel".to_string()), "deified".to_string());
}

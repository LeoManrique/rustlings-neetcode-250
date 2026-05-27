include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::count_substrings("abba".to_string()), 6);
}

#[test]
fn test_2() {
    assert_eq!(Solution::count_substrings("aaaaa".to_string()), 15);
}

#[test]
fn test_3() {
    assert_eq!(Solution::count_substrings("babad".to_string()), 7);
}

#[test]
fn test_4() {
    assert_eq!(Solution::count_substrings("noon".to_string()), 6);
}

#[test]
fn test_5() {
    assert_eq!(Solution::count_substrings("banana".to_string()), 10);
}

#[test]
fn test_6() {
    assert_eq!(Solution::count_substrings("aabbbaa".to_string()), 14);
}

#[test]
fn test_7() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
}

#[test]
fn test_8() {
    assert_eq!(Solution::count_substrings("level".to_string()), 7);
}

#[test]
fn test_9() {
    assert_eq!(Solution::count_substrings("".to_string()), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
}

#[test]
fn test_11() {
    assert_eq!(Solution::count_substrings("civic".to_string()), 7);
}

#[test]
fn test_12() {
    assert_eq!(Solution::count_substrings("rotor".to_string()), 7);
}

#[test]
fn test_13() {
    assert_eq!(Solution::count_substrings("racecar".to_string()), 10);
}

#[test]
fn test_14() {
    assert_eq!(Solution::count_substrings("a".to_string()), 1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::count_substrings("abcdefg".to_string()), 7);
}

#[test]
fn test_16() {
    assert_eq!(Solution::count_substrings("zxcvbnmmlkjhgfdsapoiuytrewq".to_string()), 28);
}

#[test]
fn test_17() {
    assert_eq!(Solution::count_substrings("abcbaabccbaabcba".to_string()), 34);
}

#[test]
fn test_18() {
    assert_eq!(Solution::count_substrings("amoreinimorac".to_string()), 14);
}

#[test]
fn test_19() {
    assert_eq!(Solution::count_substrings("atoyota".to_string()), 10);
}

#[test]
fn test_20() {
    assert_eq!(Solution::count_substrings("aabbcccbbbaa".to_string()), 23);
}

#[test]
fn test_21() {
    assert_eq!(Solution::count_substrings("abccbaabcbaabc".to_string()), 28);
}

#[test]
fn test_22() {
    assert_eq!(Solution::count_substrings("rotorrotor".to_string()), 19);
}

#[test]
fn test_23() {
    assert_eq!(Solution::count_substrings("abracadabra".to_string()), 13);
}

#[test]
fn test_24() {
    assert_eq!(Solution::count_substrings("levelmadam".to_string()), 14);
}

#[test]
fn test_25() {
    assert_eq!(Solution::count_substrings("ab".to_string()), 2);
}

#[test]
fn test_26() {
    assert_eq!(Solution::count_substrings("steponnopets".to_string()), 18);
}

#[test]
fn test_27() {
    assert_eq!(Solution::count_substrings("noonnoon".to_string()), 16);
}

#[test]
fn test_28() {
    assert_eq!(Solution::count_substrings("aaaaabbbb".to_string()), 25);
}

#[test]
fn test_29() {
    assert_eq!(Solution::count_substrings("aaaaaaaaaa".to_string()), 55);
}

#[test]
fn test_30() {
    assert_eq!(Solution::count_substrings("abcddcbaabcddcba".to_string()), 32);
}

#[test]
fn test_31() {
    assert_eq!(Solution::count_substrings("racecarracecarracecarracecar".to_string()), 82);
}

#[test]
fn test_32() {
    assert_eq!(Solution::count_substrings("madaminnadammadam".to_string()), 27);
}

#[test]
fn test_33() {
    assert_eq!(Solution::count_substrings("xyxzyzyzyxzyzyxzyzyxzyx".to_string()), 34);
}

#[test]
fn test_34() {
    assert_eq!(Solution::count_substrings("noonabnoon".to_string()), 14);
}

#[test]
fn test_35() {
    assert_eq!(Solution::count_substrings("bbaabbccddeebbaabbccddee".to_string()), 40);
}

#[test]
fn test_36() {
    assert_eq!(Solution::count_substrings("qwertyuiopoiuytrewq".to_string()), 28);
}

#[test]
fn test_37() {
    assert_eq!(Solution::count_substrings("abccbaabccba".to_string()), 24);
}

#[test]
fn test_38() {
    assert_eq!(Solution::count_substrings("aba".to_string()), 4);
}

#[test]
fn test_39() {
    assert_eq!(Solution::count_substrings("xyzyzyzyzyzyzyzy".to_string()), 65);
}

#[test]
fn test_40() {
    assert_eq!(Solution::count_substrings("abacbacbacbacba".to_string()), 16);
}

#[test]
fn test_41() {
    assert_eq!(Solution::count_substrings("noonnoonnoonnoon".to_string()), 48);
}

#[test]
fn test_42() {
    assert_eq!(Solution::count_substrings("aabbccddeee".to_string()), 18);
}

#[test]
fn test_43() {
    assert_eq!(Solution::count_substrings("abacaba".to_string()), 12);
}

#[test]
fn test_44() {
    assert_eq!(Solution::count_substrings("madamracecar".to_string()), 17);
}

#[test]
fn test_45() {
    assert_eq!(Solution::count_substrings("rotorcarrot".to_string()), 14);
}

#[test]
fn test_46() {
    assert_eq!(Solution::count_substrings("abca".to_string()), 4);
}

#[test]
fn test_47() {
    assert_eq!(Solution::count_substrings("aabbcccbbbbaaa".to_string()), 30);
}

#[test]
fn test_48() {
    assert_eq!(Solution::count_substrings("aabbccddeeeeeddccbbaa".to_string()), 47);
}

#[test]
fn test_49() {
    assert_eq!(Solution::count_substrings("aaaaaabbbb".to_string()), 31);
}

#[test]
fn test_50() {
    assert_eq!(Solution::count_substrings("aabbabaabbaa".to_string()), 23);
}

#[test]
fn test_51() {
    assert_eq!(Solution::count_substrings("abcba".to_string()), 7);
}

#[test]
fn test_52() {
    assert_eq!(Solution::count_substrings("abcdcba".to_string()), 10);
}

#[test]
fn test_53() {
    assert_eq!(Solution::count_substrings("xyzzyxzyxzyzyxzyz".to_string()), 23);
}

#[test]
fn test_54() {
    assert_eq!(Solution::count_substrings("madaminnakayak".to_string()), 20);
}

#[test]
fn test_55() {
    assert_eq!(Solution::count_substrings("abccbaabccbaabccba".to_string()), 45);
}

#[test]
fn test_56() {
    assert_eq!(Solution::count_substrings("aabbccccbbaa".to_string()), 26);
}

#[test]
fn test_57() {
    assert_eq!(Solution::count_substrings("rotorcentralrotor".to_string()), 21);
}

#[test]
fn test_58() {
    assert_eq!(Solution::count_substrings("racecarracecarracecar".to_string()), 51);
}

#[test]
fn test_59() {
    assert_eq!(Solution::count_substrings("aabbbaaabbbaaa".to_string()), 37);
}

#[test]
fn test_60() {
    assert_eq!(Solution::count_substrings("xxyyyxyxx".to_string()), 17);
}

#[test]
fn test_61() {
    assert_eq!(Solution::count_substrings("aabbccddeeefffggghhhh".to_string()), 40);
}

#[test]
fn test_62() {
    assert_eq!(Solution::count_substrings("abacdfgdcabaabacdfgdcaba".to_string()), 33);
}

#[test]
fn test_63() {
    assert_eq!(Solution::count_substrings("racecarbanana".to_string()), 20);
}

#[test]
fn test_64() {
    assert_eq!(Solution::count_substrings("levelup".to_string()), 9);
}

#[test]
fn test_65() {
    assert_eq!(Solution::count_substrings("abcdeedcba".to_string()), 15);
}

#[test]
fn test_66() {
    assert_eq!(Solution::count_substrings("deeee".to_string()), 11);
}

#[test]
fn test_67() {
    assert_eq!(Solution::count_substrings("abaaabaaab".to_string()), 24);
}

#[test]
fn test_68() {
    assert_eq!(Solution::count_substrings("abbaabbaba".to_string()), 20);
}

#[test]
fn test_69() {
    assert_eq!(Solution::count_substrings("abcbaabcbaabcbaabcba".to_string()), 58);
}

#[test]
fn test_70() {
    assert_eq!(Solution::count_substrings("bcbcbcbcbcbcbc".to_string()), 56);
}

#[test]
fn test_71() {
    assert_eq!(Solution::count_substrings("abaaabbaaabaaa".to_string()), 35);
}

#[test]
fn test_72() {
    assert_eq!(Solution::count_substrings("abcababcababc".to_string()), 17);
}

#[test]
fn test_73() {
    assert_eq!(Solution::count_substrings("levellevel".to_string()), 19);
}

#[test]
fn test_74() {
    assert_eq!(Solution::count_substrings("abcdefedcba".to_string()), 16);
}

#[test]
fn test_75() {
    assert_eq!(Solution::count_substrings("abcdefgfedcbafedcbagfedcbafedcbagfedcba".to_string()), 45);
}

#[test]
fn test_76() {
    assert_eq!(Solution::count_substrings("acdcacdcacdc".to_string()), 27);
}

#[test]
fn test_77() {
    assert_eq!(Solution::count_substrings("zxcvbnmmnbvcxz".to_string()), 21);
}

#[test]
fn test_78() {
    assert_eq!(Solution::count_substrings("thisisnotapalindrome".to_string()), 23);
}

#[test]
fn test_79() {
    assert_eq!(Solution::count_substrings("abbaeae".to_string()), 11);
}

#[test]
fn test_80() {
    assert_eq!(Solution::count_substrings("aaaaaaa".to_string()), 28);
}

#[test]
fn test_81() {
    assert_eq!(Solution::count_substrings("levellevellevel".to_string()), 36);
}

#[test]
fn test_82() {
    assert_eq!(Solution::count_substrings("abcdcbaabcdcba".to_string()), 27);
}

#[test]
fn test_83() {
    assert_eq!(Solution::count_substrings("abccbaabc".to_string()), 15);
}

#[test]
fn test_84() {
    assert_eq!(Solution::count_substrings("xyzyzyxzyzyzyx".to_string()), 25);
}

#[test]
fn test_85() {
    assert_eq!(Solution::count_substrings("abacdfgdcabaabacdfgdcabaabacdfgdcaba".to_string()), 52);
}

#[test]
fn test_86() {
    assert_eq!(Solution::count_substrings("leveloneeleven".to_string()), 19);
}

#[test]
fn test_87() {
    assert_eq!(Solution::count_substrings("babcbabcba".to_string()), 20);
}

#[test]
fn test_88() {
    assert_eq!(Solution::count_substrings("zyzzyzzyzyzy".to_string()), 29);
}

#[test]
fn test_89() {
    assert_eq!(Solution::count_substrings("abababa".to_string()), 16);
}

#[test]
fn test_90() {
    assert_eq!(Solution::count_substrings("xyxzyxyzyx".to_string()), 15);
}

#[test]
fn test_91() {
    assert_eq!(Solution::count_substrings("abacabadabacaba".to_string()), 32);
}

#[test]
fn test_92() {
    assert_eq!(Solution::count_substrings("civicciviccivicciviccivic".to_string()), 85);
}

#[test]
fn test_93() {
    assert_eq!(Solution::count_substrings("zzzzzzzzzzzzzzzzzzzz".to_string()), 210);
}

#[test]
fn test_94() {
    assert_eq!(Solution::count_substrings("abcbaba".to_string()), 11);
}

#[test]
fn test_95() {
    assert_eq!(Solution::count_substrings("abcbabcba".to_string()), 17);
}

#[test]
fn test_96() {
    assert_eq!(Solution::count_substrings("deifiedrotor".to_string()), 17);
}

#[test]
fn test_97() {
    assert_eq!(Solution::count_substrings("noonracecarrace".to_string()), 24);
}

#[test]
fn test_98() {
    assert_eq!(Solution::count_substrings("civiccivic".to_string()), 19);
}

#[test]
fn test_99() {
    assert_eq!(Solution::count_substrings("aabbccddeeffgg".to_string()), 21);
}

#[test]
fn test_100() {
    assert_eq!(Solution::count_substrings("abccba".to_string()), 9);
}

#[test]
fn test_101() {
    assert_eq!(Solution::count_substrings("aabbccddeeff".to_string()), 18);
}

#[test]
fn test_102() {
    assert_eq!(Solution::count_substrings("madamimadam".to_string()), 20);
}

#[test]
fn test_103() {
    assert_eq!(Solution::count_substrings("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), 78);
}

#[test]
fn test_104() {
    assert_eq!(Solution::count_substrings("racecarannakayak".to_string()), 25);
}

#[test]
fn test_105() {
    assert_eq!(Solution::count_substrings("amanaplanacanalpanama".to_string()), 37);
}

#[test]
fn test_106() {
    assert_eq!(Solution::count_substrings("abbabababa".to_string()), 24);
}

#[test]
fn test_107() {
    assert_eq!(Solution::count_substrings("abcdefgfeijklmmlkjihgf".to_string()), 29);
}

#[test]
fn test_108() {
    assert_eq!(Solution::count_substrings("racecarracecar".to_string()), 27);
}

#[test]
fn test_109() {
    assert_eq!(Solution::count_substrings("noonnoonnoon".to_string()), 30);
}

#[test]
fn test_110() {
    assert_eq!(Solution::count_substrings("deified".to_string()), 10);
}

#[test]
fn test_111() {
    assert_eq!(Solution::count_substrings("bcbcbcbcb".to_string()), 25);
}

#[test]
fn test_112() {
    assert_eq!(Solution::count_substrings("abcdef".to_string()), 6);
}

#[test]
fn test_113() {
    assert_eq!(Solution::count_substrings("rotorrotorrotor".to_string()), 36);
}

#[test]
fn test_114() {
    assert_eq!(Solution::count_substrings("abcdedcba".to_string()), 13);
}

#[test]
fn test_115() {
    assert_eq!(Solution::count_substrings("noonhighnoon".to_string()), 16);
}

#[test]
fn test_116() {
    assert_eq!(Solution::count_substrings("abcbaabcbaabcba".to_string()), 36);
}

#[test]
fn test_117() {
    assert_eq!(Solution::count_substrings("rotorrotorrotorrotor".to_string()), 58);
}

#[test]
fn test_118() {
    assert_eq!(Solution::count_substrings("abacdfgdcaba".to_string()), 14);
}

#[test]
fn test_119() {
    assert_eq!(Solution::count_substrings("abbaabba".to_string()), 16);
}

#[test]
fn test_120() {
    assert_eq!(Solution::count_substrings("ababababab".to_string()), 30);
}

#[test]
fn test_121() {
    assert_eq!(Solution::count_substrings("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzzzzzzzzzzz".to_string()), 141);
}

#[test]
fn test_122() {
    assert_eq!(Solution::count_substrings("abcddcba".to_string()), 12);
}

#[test]
fn test_123() {
    assert_eq!(Solution::count_substrings("aaaaabaaa".to_string()), 25);
}

#[test]
fn test_124() {
    assert_eq!(Solution::count_substrings("abcdefghihgfedcba".to_string()), 25);
}

#[test]
fn test_125() {
    assert_eq!(Solution::count_substrings("xyzyxzyzyx".to_string()), 14);
}

#[test]
fn test_126() {
    assert_eq!(Solution::count_substrings("xyxxyxyxxyxyxyx".to_string()), 40);
}

#[test]
fn test_127() {
    assert_eq!(Solution::count_substrings("xyzzzyzx".to_string()), 13);
}

#[test]
fn test_128() {
    assert_eq!(Solution::count_substrings("abcdefgfedcbafedcbagfedcba".to_string()), 32);
}

#[test]
fn test_129() {
    assert_eq!(Solution::count_substrings("aabbccddeeaabbccddee".to_string()), 30);
}

#[test]
fn test_130() {
    assert_eq!(Solution::count_substrings("abacdfgdcababa".to_string()), 19);
}

#[test]
fn test_131() {
    assert_eq!(Solution::count_substrings("civicciviccivic".to_string()), 36);
}

#[test]
fn test_132() {
    assert_eq!(Solution::count_substrings("aabbcc".to_string()), 9);
}

#[test]
fn test_133() {
    assert_eq!(Solution::count_substrings("mississippi".to_string()), 20);
}

#[test]
fn test_134() {
    assert_eq!(Solution::count_substrings("xyzyxyzyx".to_string()), 17);
}

#[test]
fn test_135() {
    assert_eq!(Solution::count_substrings("abbababaabbaba".to_string()), 28);
}

#[test]
fn test_136() {
    assert_eq!(Solution::count_substrings("xyxzyzyxzyzyzyxzyxzyzyzyxzyzyzyzyx".to_string()), 61);
}

#[test]
fn test_137() {
    assert_eq!(Solution::count_substrings("forgeeksskeegfor".to_string()), 23);
}

#[test]
fn test_138() {
    assert_eq!(Solution::count_substrings("bananaabababa".to_string()), 27);
}

#[test]
fn test_139() {
    assert_eq!(Solution::count_substrings("abcbba".to_string()), 8);
}

#[test]
fn test_140() {
    assert_eq!(Solution::count_substrings("zzzzzzzzzzz".to_string()), 66);
}

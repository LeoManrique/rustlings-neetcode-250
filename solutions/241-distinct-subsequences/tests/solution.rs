include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::num_distinct("abc".to_string(), "abcd".to_string()), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::num_distinct("abcd".to_string(), "abcd".to_string()), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
}

#[test]
fn test_4() {
    assert_eq!(Solution::num_distinct("aaa".to_string(), "a".to_string()), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::num_distinct("".to_string(), "abc".to_string()), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::num_distinct("mississippi".to_string(), "isip".to_string()), 16);
}

#[test]
fn test_7() {
    assert_eq!(Solution::num_distinct("aaaaa".to_string(), "aa".to_string()), 10);
}

#[test]
fn test_8() {
    assert_eq!(Solution::num_distinct("abcde".to_string(), "ae".to_string()), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::num_distinct("abcde".to_string(), "".to_string()), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::num_distinct("abcd".to_string(), "ab".to_string()), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::num_distinct("a".to_string(), "a".to_string()), 1);
}

#[test]
fn test_12() {
    assert_eq!(Solution::num_distinct("abcdabcabc".to_string(), "abc".to_string()), 10);
}

#[test]
fn test_13() {
    assert_eq!(Solution::num_distinct("babgbag".to_string(), "bag".to_string()), 5);
}

#[test]
fn test_14() {
    assert_eq!(Solution::num_distinct("abc".to_string(), "abc".to_string()), 1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::num_distinct("pppp".to_string(), "p".to_string()), 4);
}

#[test]
fn test_16() {
    assert_eq!(Solution::num_distinct("abab".to_string(), "aba".to_string()), 1);
}

#[test]
fn test_17() {
    assert_eq!(Solution::num_distinct("".to_string(), "".to_string()), 1);
}

#[test]
fn test_18() {
    assert_eq!(Solution::num_distinct("leetcodeisgood".to_string(), "code".to_string()), 1);
}

#[test]
fn test_19() {
    assert_eq!(Solution::num_distinct("abcdefghijklmnopqrstuvwxyz".to_string(), "acegikmoqsuwy".to_string()), 1);
}

#[test]
fn test_20() {
    assert_eq!(Solution::num_distinct("zzzzzzzzzzzzzzzzzzzz".to_string(), "zzz".to_string()), 1140);
}

#[test]
fn test_21() {
    assert_eq!(Solution::num_distinct("dynamicprogramming".to_string(), "dpm".to_string()), 2);
}

#[test]
fn test_22() {
    assert_eq!(Solution::num_distinct("ababababab".to_string(), "abab".to_string()), 35);
}

#[test]
fn test_23() {
    assert_eq!(Solution::num_distinct("abcdabcdabcd".to_string(), "abcd".to_string()), 15);
}

#[test]
fn test_24() {
    assert_eq!(Solution::num_distinct("abcdabcdabcdabcd".to_string(), "abcd".to_string()), 35);
}

#[test]
fn test_25() {
    assert_eq!(Solution::num_distinct("zzzzzzzzzz".to_string(), "zzzz".to_string()), 210);
}

#[test]
fn test_26() {
    assert_eq!(Solution::num_distinct("aaaabbbbccccdddd".to_string(), "abcd".to_string()), 256);
}

#[test]
fn test_27() {
    assert_eq!(Solution::num_distinct("abracadabra".to_string(), "abrac".to_string()), 1);
}

#[test]
fn test_28() {
    assert_eq!(Solution::num_distinct("abcdefgabcdefgabcdefgabcdefgabcdefg".to_string(), "abcdefg".to_string()), 330);
}

#[test]
fn test_29() {
    assert_eq!(Solution::num_distinct("abcabcabcabcabcabcabcabc".to_string(), "abcabc".to_string()), 924);
}

#[test]
fn test_30() {
    assert_eq!(Solution::num_distinct("ababcabcabc".to_string(), "abc".to_string()), 19);
}

#[test]
fn test_31() {
    assert_eq!(Solution::num_distinct("abcdefghikjlmnopqrstuvwxyz".to_string(), "xyz".to_string()), 1);
}

#[test]
fn test_32() {
    assert_eq!(Solution::num_distinct("abcabcabcabc".to_string(), "abc".to_string()), 20);
}

#[test]
fn test_33() {
    assert_eq!(Solution::num_distinct("abcbabc".to_string(), "abc".to_string()), 5);
}

#[test]
fn test_34() {
    assert_eq!(Solution::num_distinct("xyzxyzxyz".to_string(), "xyzyx".to_string()), 1);
}

#[test]
fn test_35() {
    assert_eq!(Solution::num_distinct("aabbbccc".to_string(), "abc".to_string()), 18);
}

#[test]
fn test_36() {
    assert_eq!(Solution::num_distinct("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string(), "zzzzzzzzzz".to_string()), 64512240);
}

#[test]
fn test_37() {
    assert_eq!(Solution::num_distinct("xyzxyzxyzxyz".to_string(), "xyz".to_string()), 20);
}

#[test]
fn test_38() {
    assert_eq!(Solution::num_distinct("aabbccddeeffgg".to_string(), "aabbcc".to_string()), 1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::num_distinct("abcdeabcdeabcde".to_string(), "abc".to_string()), 10);
}

#[test]
fn test_40() {
    assert_eq!(Solution::num_distinct("xyzzxyzzxyzzxyzz".to_string(), "xyz".to_string()), 40);
}

#[test]
fn test_41() {
    assert_eq!(Solution::num_distinct("xxyxyxyxyxyxyxyx".to_string(), "xyx".to_string()), 112);
}

#[test]
fn test_42() {
    assert_eq!(Solution::num_distinct("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "abc".to_string()), 8);
}

#[test]
fn test_43() {
    assert_eq!(Solution::num_distinct("xyxxyxyxyx".to_string(), "xyx".to_string()), 27);
}

#[test]
fn test_44() {
    assert_eq!(Solution::num_distinct("mississippi".to_string(), "issi".to_string()), 15);
}

#[test]
fn test_45() {
    assert_eq!(Solution::num_distinct("pneumonoultramicroscopicsilicovolcanoconiosis".to_string(), "pneumo".to_string()), 23);
}

#[test]
fn test_46() {
    assert_eq!(Solution::num_distinct("pppppppppppp".to_string(), "pppp".to_string()), 495);
}

#[test]
fn test_47() {
    assert_eq!(Solution::num_distinct("aaaaabaaaaabaabaaabaababab".to_string(), "aab".to_string()), 686);
}

#[test]
fn test_48() {
    assert_eq!(Solution::num_distinct("abcdefg".to_string(), "xyz".to_string()), 0);
}

#[test]
fn test_49() {
    assert_eq!(Solution::num_distinct("rabbbitrabbbitrabbbit".to_string(), "rabbit".to_string()), 126);
}

#[test]
fn test_50() {
    assert_eq!(Solution::num_distinct("leetcodeleetcode".to_string(), "leet".to_string()), 12);
}

#[test]
fn test_51() {
    assert_eq!(Solution::num_distinct("xyzxyzxyzxyz".to_string(), "xyzxyz".to_string()), 28);
}

#[test]
fn test_52() {
    assert_eq!(Solution::num_distinct("abcdefghijk".to_string(), "acegi".to_string()), 1);
}

#[test]
fn test_53() {
    assert_eq!(Solution::num_distinct("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "abcdefghijklmnopqrstuvwxyz".to_string()), 67108864);
}

#[test]
fn test_54() {
    assert_eq!(Solution::num_distinct("aabbccddeeffgg".to_string(), "abcde".to_string()), 32);
}

#[test]
fn test_55() {
    assert_eq!(Solution::num_distinct("longerstringexample".to_string(), "long".to_string()), 3);
}

#[test]
fn test_56() {
    assert_eq!(Solution::num_distinct("abracadabra".to_string(), "abra".to_string()), 9);
}

#[test]
fn test_57() {
    assert_eq!(Solution::num_distinct("xxyyzzxxxyyzz".to_string(), "xyxz".to_string()), 24);
}

#[test]
fn test_58() {
    assert_eq!(Solution::num_distinct("pppppppp".to_string(), "pp".to_string()), 28);
}

#[test]
fn test_59() {
    assert_eq!(Solution::num_distinct("attatchmentattach".to_string(), "attach".to_string()), 32);
}

#[test]
fn test_60() {
    assert_eq!(Solution::num_distinct("abcdefghijabcdefghij".to_string(), "abcde".to_string()), 6);
}

#[test]
fn test_61() {
    assert_eq!(Solution::num_distinct("abcdabcdeabcdeabcd".to_string(), "abcd".to_string()), 35);
}

#[test]
fn test_62() {
    assert_eq!(Solution::num_distinct("abcdefghij".to_string(), "abcdefghijk".to_string()), 0);
}

#[test]
fn test_63() {
    assert_eq!(Solution::num_distinct("zzzzzzzzzz".to_string(), "zz".to_string()), 45);
}

#[test]
fn test_64() {
    assert_eq!(Solution::num_distinct("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string(), "zzzzzzzzzz".to_string()), 14782231840815648);
}

#[test]
fn test_65() {
    assert_eq!(Solution::num_distinct("ppppppppp".to_string(), "ppp".to_string()), 84);
}

#[test]
fn test_66() {
    assert_eq!(Solution::num_distinct("abcdefghij".to_string(), "j".to_string()), 1);
}

#[test]
fn test_67() {
    assert_eq!(Solution::num_distinct("abcdabcdabcdabcdabcd".to_string(), "abcd".to_string()), 70);
}

#[test]
fn test_68() {
    assert_eq!(Solution::num_distinct("repeatedsequence".to_string(), "seq".to_string()), 1);
}

#[test]
fn test_69() {
    assert_eq!(Solution::num_distinct("aaaaaaaaaaaaaab".to_string(), "aaab".to_string()), 364);
}

#[test]
fn test_70() {
    assert_eq!(Solution::num_distinct("aabbaabbaabbaabbaabb".to_string(), "aabb".to_string()), 415);
}

#[test]
fn test_71() {
    assert_eq!(Solution::num_distinct("mixedcharacters".to_string(), "mix".to_string()), 1);
}

#[test]
fn test_72() {
    assert_eq!(Solution::num_distinct("abababababab".to_string(), "ababab".to_string()), 84);
}

#[test]
fn test_73() {
    assert_eq!(Solution::num_distinct("abcabcabcabc".to_string(), "abcabc".to_string()), 28);
}

#[test]
fn test_74() {
    assert_eq!(Solution::num_distinct("aaaaaaabbaaaaa".to_string(), "aaaaaab".to_string()), 14);
}

#[test]
fn test_75() {
    assert_eq!(Solution::num_distinct("abcabcabc".to_string(), "abc".to_string()), 10);
}

#[test]
fn test_76() {
    assert_eq!(Solution::num_distinct("abcdabcdabcd".to_string(), "abca".to_string()), 5);
}

#[test]
fn test_77() {
    assert_eq!(Solution::num_distinct("babgbagbabgbagbabgbag".to_string(), "bagbag".to_string()), 329);
}

#[test]
fn test_78() {
    assert_eq!(Solution::num_distinct("anappleperap".to_string(), "ape".to_string()), 10);
}

#[test]
fn test_79() {
    assert_eq!(Solution::num_distinct("zzzzzzzzzz".to_string(), "zzz".to_string()), 120);
}

#[test]
fn test_80() {
    assert_eq!(Solution::num_distinct("aaaaaaaaaa".to_string(), "aaaaa".to_string()), 252);
}

#[test]
fn test_81() {
    assert_eq!(Solution::num_distinct("abcdefghij".to_string(), "af".to_string()), 1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::num_distinct("abcabcabcabcabcabcabcabcabcabc".to_string(), "abcabc".to_string()), 3003);
}

#[test]
fn test_83() {
    assert_eq!(Solution::num_distinct("complexpattern".to_string(), "comp".to_string()), 2);
}

#[test]
fn test_84() {
    assert_eq!(Solution::num_distinct("abcdefghij".to_string(), "a".to_string()), 1);
}

#[test]
fn test_85() {
    assert_eq!(Solution::num_distinct("aabbaabbaabbaabbaabb".to_string(), "aab".to_string()), 190);
}

#[test]
fn test_86() {
    assert_eq!(Solution::num_distinct("abracadabraabracadabra".to_string(), "abra".to_string()), 103);
}

#[test]
fn test_87() {
    assert_eq!(Solution::num_distinct("leetcodeleetcodeleetcode".to_string(), "leet".to_string()), 51);
}

#[test]
fn test_88() {
    assert_eq!(Solution::num_distinct("aabbccddeeffgghhiijjkkllmnoonnpooqqrrssttuuvvwwxxyyzz".to_string(), "abcxyz".to_string()), 64);
}

#[test]
fn test_89() {
    assert_eq!(Solution::num_distinct("abcdefabcdefabcdef".to_string(), "abcdef".to_string()), 28);
}

#[test]
fn test_90() {
    assert_eq!(Solution::num_distinct("abcdefghij".to_string(), "acegi".to_string()), 1);
}

#[test]
fn test_91() {
    assert_eq!(Solution::num_distinct("pppppppppp".to_string(), "pp".to_string()), 45);
}

#[test]
fn test_92() {
    assert_eq!(Solution::num_distinct("hellohellohello".to_string(), "hellohello".to_string()), 17);
}

#[test]
fn test_93() {
    assert_eq!(Solution::num_distinct("abcdefghij".to_string(), "aceg".to_string()), 1);
}

#[test]
fn test_94() {
    assert_eq!(Solution::num_distinct("aabbccddeeffgg".to_string(), "abcdefg".to_string()), 128);
}

#[test]
fn test_95() {
    assert_eq!(Solution::num_distinct("hellohellohello".to_string(), "he".to_string()), 6);
}

#[test]
fn test_96() {
    assert_eq!(Solution::num_distinct("thisisaverylongstringthisisaverylongstring".to_string(), "thisisaverylongstring".to_string()), 73);
}

#[test]
fn test_97() {
    assert_eq!(Solution::num_distinct("subsequenceexample".to_string(), "subseq".to_string()), 1);
}

#[test]
fn test_98() {
    assert_eq!(Solution::num_distinct("abracadabra".to_string(), "aca".to_string()), 6);
}

#[test]
fn test_99() {
    assert_eq!(Solution::num_distinct("abcdeabcde".to_string(), "abc".to_string()), 4);
}

#[test]
fn test_100() {
    assert_eq!(Solution::num_distinct("aaaaabaaaabaaaab".to_string(), "aaab".to_string()), 380);
}

#[test]
fn test_101() {
    assert_eq!(Solution::num_distinct("abcdefghijabcdefghijabcdefghij".to_string(), "abcde".to_string()), 21);
}

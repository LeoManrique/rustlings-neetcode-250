include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::longest_common_subsequence("a".to_string(), "b".to_string()), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::longest_common_subsequence("xyz".to_string(), "zyx".to_string()), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::longest_common_subsequence("ezupkr".to_string(), "ubmrapg".to_string()), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::longest_common_subsequence("aabcc".to_string(), "dbbca".to_string()), 2);
}

#[test]
fn test_7() {
    assert_eq!(Solution::longest_common_subsequence("programming".to_string(), "gaming".to_string()), 6);
}

#[test]
fn test_8() {
    assert_eq!(Solution::longest_common_subsequence("ylqpfxxqy".to_string(), "wxqxalnnsow".to_string()), 2);
}

#[test]
fn test_9() {
    assert_eq!(Solution::longest_common_subsequence("pqr".to_string(), "stu".to_string()), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::longest_common_subsequence("abcdgh".to_string(), "aedfhr".to_string()), 3);
}

#[test]
fn test_11() {
    assert_eq!(Solution::longest_common_subsequence("hofubmnyg".to_string(), "hvmpywxck".to_string()), 3);
}

#[test]
fn test_12() {
    assert_eq!(Solution::longest_common_subsequence("abcdef".to_string(), "z".to_string()), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::longest_common_subsequence("mhunuzqrkz".to_string(), "dhlmfpnqjk".to_string()), 4);
}

#[test]
fn test_14() {
    assert_eq!(Solution::longest_common_subsequence("abcdefg".to_string(), "abdfg".to_string()), 5);
}

#[test]
fn test_15() {
    assert_eq!(Solution::longest_common_subsequence("a".to_string(), "a".to_string()), 1);
}

#[test]
fn test_16() {
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "".to_string()), 0);
}

#[test]
fn test_17() {
    assert_eq!(Solution::longest_common_subsequence("xyz".to_string(), "abc".to_string()), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::longest_common_subsequence("".to_string(), "abc".to_string()), 0);
}

#[test]
fn test_19() {
    assert_eq!(Solution::longest_common_subsequence("psnw".to_string(), "vozbbmloqjbpbprqs".to_string()), 2);
}

#[test]
fn test_20() {
    assert_eq!(Solution::longest_common_subsequence("abcxyz".to_string(), "xyzabc".to_string()), 3);
}

#[test]
fn test_21() {
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
}

#[test]
fn test_22() {
    assert_eq!(Solution::longest_common_subsequence("abcdef".to_string(), "zabcf".to_string()), 4);
}

#[test]
fn test_23() {
    assert_eq!(Solution::longest_common_subsequence("oxcpqrsvwf".to_string(), "shmtulqrypy".to_string()), 2);
}

#[test]
fn test_24() {
    assert_eq!(Solution::longest_common_subsequence("".to_string(), "".to_string()), 0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::longest_common_subsequence("abcdxyz".to_string(), "xyzabcd".to_string()), 4);
}

#[test]
fn test_26() {
    assert_eq!(Solution::longest_common_subsequence("aaaa".to_string(), "aa".to_string()), 2);
}

#[test]
fn test_27() {
    assert_eq!(Solution::longest_common_subsequence("aggtab".to_string(), "gxtxayb".to_string()), 4);
}

#[test]
fn test_28() {
    assert_eq!(Solution::longest_common_subsequence("abcxyzabc".to_string(), "xyzabcxyz".to_string()), 6);
}

#[test]
fn test_29() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaa".to_string(), "aaaaaa".to_string()), 6);
}

#[test]
fn test_30() {
    assert_eq!(Solution::longest_common_subsequence("abacdfgdcaba".to_string(), "abacdgfdcaba".to_string()), 11);
}

#[test]
fn test_31() {
    assert_eq!(Solution::longest_common_subsequence("aabccbb".to_string(), "dbbcbab".to_string()), 4);
}

#[test]
fn test_32() {
    assert_eq!(Solution::longest_common_subsequence("longerstringone".to_string(), "short".to_string()), 3);
}

#[test]
fn test_33() {
    assert_eq!(Solution::longest_common_subsequence("mississippi".to_string(), "pississippi".to_string()), 10);
}

#[test]
fn test_34() {
    assert_eq!(Solution::longest_common_subsequence("abracadabra".to_string(), "avadakedavra".to_string()), 7);
}

#[test]
fn test_35() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(), "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string()), 0);
}

#[test]
fn test_36() {
    assert_eq!(Solution::longest_common_subsequence("aquickbrownfoxjumpsoverthelazydog".to_string(), "lazydog".to_string()), 7);
}

#[test]
fn test_37() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghij".to_string(), "acegik".to_string()), 5);
}

#[test]
fn test_38() {
    assert_eq!(Solution::longest_common_subsequence("abcdpqrs".to_string(), "efghijkl".to_string()), 0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::longest_common_subsequence("aaaabbbbccccddddeeeeffffgggghhhhiiii".to_string(), "abcdefghi".to_string()), 9);
}

#[test]
fn test_40() {
    assert_eq!(Solution::longest_common_subsequence("xyzxyzxyz".to_string(), "zyxzyxzyx".to_string()), 5);
}

#[test]
fn test_41() {
    assert_eq!(Solution::longest_common_subsequence("zyxwvutsrqponmlkjihgfedcba".to_string(), "abcdefghijklmnopqrstuvwxyz".to_string()), 1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::longest_common_subsequence("abcdabcdabcdabcdabcdabcdabcdabcd".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), 4);
}

#[test]
fn test_43() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghijklmnopqrstuvwxyz".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), 1);
}

#[test]
fn test_44() {
    assert_eq!(Solution::longest_common_subsequence("longestcommonsubsequence".to_string(), "shortestsub".to_string()), 7);
}

#[test]
fn test_45() {
    assert_eq!(Solution::longest_common_subsequence("mississippi".to_string(), "ississippi".to_string()), 10);
}

#[test]
fn test_46() {
    assert_eq!(Solution::longest_common_subsequence("aaaaabaaaabaaaabaaaa".to_string(), "bbbaabbaabbaabbaab".to_string()), 11);
}

#[test]
fn test_47() {
    assert_eq!(Solution::longest_common_subsequence("abcdeabcdeabcde".to_string(), "deabcdea".to_string()), 8);
}

#[test]
fn test_48() {
    assert_eq!(Solution::longest_common_subsequence("interpersonal".to_string(), "personality".to_string()), 8);
}

#[test]
fn test_49() {
    assert_eq!(Solution::longest_common_subsequence("aaaaabbbb".to_string(), "ababababab".to_string()), 6);
}

#[test]
fn test_50() {
    assert_eq!(Solution::longest_common_subsequence("abcdefg".to_string(), "hijklmn".to_string()), 0);
}

#[test]
fn test_51() {
    assert_eq!(Solution::longest_common_subsequence("pqrstuvwxyzabcdefghijklmno".to_string(), "mnopqrstuvwxyzabcdefghijkl".to_string()), 23);
}

#[test]
fn test_52() {
    assert_eq!(Solution::longest_common_subsequence("pwwkew".to_string(), "wkepwkew".to_string()), 5);
}

#[test]
fn test_53() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaaab".to_string(), "bbbbbaaa".to_string()), 3);
}

#[test]
fn test_54() {
    assert_eq!(Solution::longest_common_subsequence("abcdexyz".to_string(), "zyxeabcd".to_string()), 4);
}

#[test]
fn test_55() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghij".to_string(), "0123456789abcdefghij".to_string()), 10);
}

#[test]
fn test_56() {
    assert_eq!(Solution::longest_common_subsequence("thisisaverylongstring".to_string(), "shisivaverylongstring".to_string()), 19);
}

#[test]
fn test_57() {
    assert_eq!(Solution::longest_common_subsequence("acbdefgh".to_string(), "bdfg".to_string()), 4);
}

#[test]
fn test_58() {
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "cba".to_string()), 1);
}

#[test]
fn test_59() {
    assert_eq!(Solution::longest_common_subsequence("abacabadabacaba".to_string(), "abacabadabacab".to_string()), 14);
}

#[test]
fn test_60() {
    assert_eq!(Solution::longest_common_subsequence("abcabcabcabcabcabcabcabcabcabcabc".to_string(), "abcabcabcabc".to_string()), 12);
}

#[test]
fn test_61() {
    assert_eq!(Solution::longest_common_subsequence("aaaabbbbcccc".to_string(), "bbcccaabbbccc".to_string()), 8);
}

#[test]
fn test_62() {
    assert_eq!(Solution::longest_common_subsequence("aaaaabbbbbbccccccdddddeeeeeeeeeffffff".to_string(), "dddddeeeeeeeeeccccccbbbbaaaaaa".to_string()), 14);
}

#[test]
fn test_63() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaaaaaa".to_string(), "aaaaaaaaaa".to_string()), 10);
}

#[test]
fn test_64() {
    assert_eq!(Solution::longest_common_subsequence("ab".to_string(), "ba".to_string()), 1);
}

#[test]
fn test_65() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaaabbbbbbbccccccdddddd".to_string(), "abcde".to_string()), 4);
}

#[test]
fn test_66() {
    assert_eq!(Solution::longest_common_subsequence("ababcabcabc".to_string(), "abcabcabc".to_string()), 9);
}

#[test]
fn test_67() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghijk".to_string(), "lmnopqrstuvwxyz".to_string()), 0);
}

#[test]
fn test_68() {
    assert_eq!(Solution::longest_common_subsequence("aabccba".to_string(), "abcabcabc".to_string()), 5);
}

#[test]
fn test_69() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghijlkmnopqrstuvwxyz".to_string(), "zxywvutsrqponmlkjihgfedcba".to_string()), 2);
}

#[test]
fn test_70() {
    assert_eq!(Solution::longest_common_subsequence("repeatedrepeated".to_string(), "repeated".to_string()), 8);
}

#[test]
fn test_71() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghij".to_string(), "acegikmopq".to_string()), 5);
}

#[test]
fn test_72() {
    assert_eq!(Solution::longest_common_subsequence("aabbccddeeffgghhiijj".to_string(), "jjiihhggffeeddccbbaa".to_string()), 2);
}

#[test]
fn test_73() {
    assert_eq!(Solution::longest_common_subsequence("supercalifragilisticexpialidocious".to_string(), "superfragilistic".to_string()), 16);
}

#[test]
fn test_74() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaaabbbbbbbcccccc".to_string(), "bbbbbbccccccdddddd".to_string()), 12);
}

#[test]
fn test_75() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghigklmnopqrstuvwxyz".to_string(), "zzzyyxxwwvvuuttrrqqppoonnmmllkkjjiihhggeeffddeeccbbbaa".to_string()), 2);
}

#[test]
fn test_76() {
    assert_eq!(Solution::longest_common_subsequence("abracadabraabracadabra".to_string(), "alacazamalacazam".to_string()), 10);
}

#[test]
fn test_77() {
    assert_eq!(Solution::longest_common_subsequence("abcabcabcabcabcabcabcabcabcabc".to_string(), "abcabcabc".to_string()), 9);
}

#[test]
fn test_78() {
    assert_eq!(Solution::longest_common_subsequence("abababababababababababababababab".to_string(), "babababababababababababababababa".to_string()), 31);
}

#[test]
fn test_79() {
    assert_eq!(Solution::longest_common_subsequence("randomsequence".to_string(), "sequence".to_string()), 8);
}

#[test]
fn test_80() {
    assert_eq!(Solution::longest_common_subsequence("abcdeabcdeabcde".to_string(), "cdecdecdecde".to_string()), 9);
}

#[test]
fn test_81() {
    assert_eq!(Solution::longest_common_subsequence("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "mnopqrstuvwxyz".to_string()), 14);
}

#[test]
fn test_82() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaaabaaaaaa".to_string(), "bbbbbbbbaaaaaa".to_string()), 7);
}

#[test]
fn test_83() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(), "aaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()), 26);
}

#[test]
fn test_84() {
    assert_eq!(Solution::longest_common_subsequence("abcdeabcdeabcde".to_string(), "deabcdeabcdeabcdeab".to_string()), 15);
}

#[test]
fn test_85() {
    assert_eq!(Solution::longest_common_subsequence("abcdexyz".to_string(), "xyzabcd".to_string()), 4);
}

#[test]
fn test_86() {
    assert_eq!(Solution::longest_common_subsequence("aaaabbbb".to_string(), "bbaaaa".to_string()), 4);
}

#[test]
fn test_87() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()), 29);
}

#[test]
fn test_88() {
    assert_eq!(Solution::longest_common_subsequence("pneumonoultramicroscopicsilicovolcanoconiosis".to_string(), "ultramicroscopically".to_string()), 18);
}

#[test]
fn test_89() {
    assert_eq!(Solution::longest_common_subsequence("dynamicprogramming".to_string(), "rhythmprogramming".to_string()), 13);
}

#[test]
fn test_90() {
    assert_eq!(Solution::longest_common_subsequence("aaaa".to_string(), "bbbb".to_string()), 0);
}

#[test]
fn test_91() {
    assert_eq!(Solution::longest_common_subsequence("abcdpqrsabcd".to_string(), "dcbapqrscba".to_string()), 6);
}

#[test]
fn test_92() {
    assert_eq!(Solution::longest_common_subsequence("longestcommonsubsequence".to_string(), "shortestsubsequence".to_string()), 15);
}

#[test]
fn test_93() {
    assert_eq!(Solution::longest_common_subsequence("abacabadabacaba".to_string(), "abacabadabacaba".to_string()), 15);
}

#[test]
fn test_94() {
    assert_eq!(Solution::longest_common_subsequence("findlongestcommonsubsequence".to_string(), "longestsubsequence".to_string()), 18);
}

#[test]
fn test_95() {
    assert_eq!(Solution::longest_common_subsequence("xyxzyzyxyzyx".to_string(), "zyzyxzyxzyzyxzyzyx".to_string()), 12);
}

#[test]
fn test_96() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghijklnmopqrstuvwxyz".to_string(), "aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), 25);
}

#[test]
fn test_97() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghigklmnopqrstuvwxyz".to_string(), "aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), 25);
}

#[test]
fn test_98() {
    assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "".to_string()), 0);
}

#[test]
fn test_99() {
    assert_eq!(Solution::longest_common_subsequence("pneumonoultramicroscopicsilicovolcanoconiosis".to_string(), "supercalifragilisticexpialidocious".to_string()), 17);
}

#[test]
fn test_100() {
    assert_eq!(Solution::longest_common_subsequence("papiermuh".to_string(), "leidermuz".to_string()), 5);
}

#[test]
fn test_101() {
    assert_eq!(Solution::longest_common_subsequence("abcdabcdabcdabcdabcdabcdabcdabcd".to_string(), "cdabcdabcdabcdabcdabcdabcdabcdabc".to_string()), 31);
}

#[test]
fn test_102() {
    assert_eq!(Solution::longest_common_subsequence("pwwkew".to_string(), "wkep".to_string()), 3);
}

#[test]
fn test_103() {
    assert_eq!(Solution::longest_common_subsequence("xyxzyxyxzyx".to_string(), "zyxzyxzyxzyx".to_string()), 10);
}

#[test]
fn test_104() {
    assert_eq!(Solution::longest_common_subsequence("xyzzaz".to_string(), "yza".to_string()), 3);
}

#[test]
fn test_105() {
    assert_eq!(Solution::longest_common_subsequence("commonsubstring".to_string(), "substring".to_string()), 9);
}

#[test]
fn test_106() {
    assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "edcba".to_string()), 1);
}

#[test]
fn test_107() {
    assert_eq!(Solution::longest_common_subsequence("aabbaabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "ababababcabcacacacaddaddaeaeaeafaafagagahaiai".to_string()), 20);
}

#[test]
fn test_108() {
    assert_eq!(Solution::longest_common_subsequence("abcdabcdabcd".to_string(), "dcbaabcd".to_string()), 6);
}

#[test]
fn test_109() {
    assert_eq!(Solution::longest_common_subsequence("".to_string(), "abcde".to_string()), 0);
}

#[test]
fn test_110() {
    assert_eq!(Solution::longest_common_subsequence("aaaaabaaaacaaaaadaaaaa".to_string(), "aaabaaaaaaacaaaaaa".to_string()), 17);
}

#[test]
fn test_111() {
    assert_eq!(Solution::longest_common_subsequence("thisisaverylongstringthatshouldtestthelimits".to_string(), "stringthatshouldtest".to_string()), 20);
}

#[test]
fn test_112() {
    assert_eq!(Solution::longest_common_subsequence("mississippi".to_string(), "issipi".to_string()), 6);
}

#[test]
fn test_113() {
    assert_eq!(Solution::longest_common_subsequence("longestcommonsubsequence".to_string(), "shortcommonsequence".to_string()), 16);
}

#[test]
fn test_114() {
    assert_eq!(Solution::longest_common_subsequence("abcdabcdabcd".to_string(), "ddd".to_string()), 3);
}

#[test]
fn test_115() {
    assert_eq!(Solution::longest_common_subsequence("longestcommonsubsequence".to_string(), "commonsubsequences".to_string()), 17);
}

#[test]
fn test_116() {
    assert_eq!(Solution::longest_common_subsequence("longstringwithnocommonsubsequence".to_string(), "anotherlongstringwithoutcommonsubsequence".to_string()), 32);
}

#[test]
fn test_117() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghijabcdefghijabcdefghij".to_string(), "abcdefghijabcdefghij".to_string()), 20);
}

#[test]
fn test_118() {
    assert_eq!(Solution::longest_common_subsequence("lclcl".to_string(), "clcl".to_string()), 4);
}

#[test]
fn test_119() {
    assert_eq!(Solution::longest_common_subsequence("thequickbrownfoxjumpsoverthelazydog".to_string(), "quickbrownfoxjumpsoverthelazydo".to_string()), 31);
}

#[test]
fn test_120() {
    assert_eq!(Solution::longest_common_subsequence("longestcommonsubsequenceproblem".to_string(), "programmingproblemsolving".to_string()), 12);
}

#[test]
fn test_121() {
    assert_eq!(Solution::longest_common_subsequence("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "azbycxdwevfugthsirjqkplohnmgpfnoreqmtpslknjihgfedcba".to_string()), 17);
}

#[test]
fn test_122() {
    assert_eq!(Solution::longest_common_subsequence("abcabcabcabc".to_string(), "cbacbacbacba".to_string()), 7);
}

#[test]
fn test_123() {
    assert_eq!(Solution::longest_common_subsequence("abcdeabcdeabcdeabcdeabcdeabcdeabcde".to_string(), "abcdeabcde".to_string()), 10);
}

#[test]
fn test_124() {
    assert_eq!(Solution::longest_common_subsequence("xyzxyzxyzxyz".to_string(), "zyxzyxzyxzyx".to_string()), 7);
}

#[test]
fn test_125() {
    assert_eq!(Solution::longest_common_subsequence("aabaaaabaaaaabaaaabaaabaaa".to_string(), "abaababaabaaaaabaaabaaaaabaaaab".to_string()), 24);
}

#[test]
fn test_126() {
    assert_eq!(Solution::longest_common_subsequence("qwertypasdfghjklzxcvbnm".to_string(), "qwertyuiopasdfghjklzxcvbnm".to_string()), 23);
}

#[test]
fn test_127() {
    assert_eq!(Solution::longest_common_subsequence("aabbccddeeff".to_string(), "abcdefabcdef".to_string()), 7);
}

#[test]
fn test_128() {
    assert_eq!(Solution::longest_common_subsequence("abcxyzdefghijklmnopqrstuvwxyz".to_string(), "xyzabcghijklmnopqrstuvwxyzdef".to_string()), 23);
}

#[test]
fn test_129() {
    assert_eq!(Solution::longest_common_subsequence("abcdabcdabcdabcd".to_string(), "abcabcabcabc".to_string()), 12);
}

#[test]
fn test_130() {
    assert_eq!(Solution::longest_common_subsequence("thisisatestcase".to_string(), "testingthestcase".to_string()), 11);
}

#[test]
fn test_131() {
    assert_eq!(Solution::longest_common_subsequence("abacabadabacaba".to_string(), "cabacabacabaca".to_string()), 12);
}

#[test]
fn test_132() {
    assert_eq!(Solution::longest_common_subsequence("dynamicprogramming".to_string(), "longestincreasingsubsequence".to_string()), 8);
}

#[test]
fn test_133() {
    assert_eq!(Solution::longest_common_subsequence("abcdefg".to_string(), "xyz".to_string()), 0);
}

#[test]
fn test_134() {
    assert_eq!(Solution::longest_common_subsequence("1234567890".to_string(), "0987654321".to_string()), 1);
}

#[test]
fn test_135() {
    assert_eq!(Solution::longest_common_subsequence("thisisatest".to_string(), "testingthest".to_string()), 7);
}

#[test]
fn test_136() {
    assert_eq!(Solution::longest_common_subsequence("aaaaaaa".to_string(), "aaaaaaaaa".to_string()), 7);
}

#[test]
fn test_137() {
    assert_eq!(Solution::longest_common_subsequence("longestcommonsubsequence".to_string(), "commonsequences".to_string()), 14);
}

#[test]
fn test_138() {
    assert_eq!(Solution::longest_common_subsequence("abracadabra".to_string(), "alacazam".to_string()), 5);
}

#[test]
fn test_139() {
    assert_eq!(Solution::longest_common_subsequence("mississippi".to_string(), "issip".to_string()), 5);
}

#[test]
fn test_140() {
    assert_eq!(Solution::longest_common_subsequence("1234567890".to_string(), "9876543210".to_string()), 2);
}

#[test]
fn test_141() {
    assert_eq!(Solution::longest_common_subsequence("thisisareallylongstringthatweneedtocheck".to_string(), "thisstringislong".to_string()), 11);
}

#[test]
fn test_142() {
    assert_eq!(Solution::longest_common_subsequence("abcdabcde".to_string(), "abcde".to_string()), 5);
}

#[test]
fn test_143() {
    assert_eq!(Solution::longest_common_subsequence("abcdefghij".to_string(), "jihgfedcba".to_string()), 1);
}

#[test]
fn test_144() {
    assert_eq!(Solution::longest_common_subsequence("abababababababab".to_string(), "babababababababa".to_string()), 15);
}

#[test]
fn test_145() {
    assert_eq!(Solution::longest_common_subsequence("mississippi".to_string(), "mississsippississippi".to_string()), 11);
}

#[test]
fn test_146() {
    assert_eq!(Solution::longest_common_subsequence("aaaaabbbbbcccc".to_string(), "cccbbaaaa".to_string()), 4);
}

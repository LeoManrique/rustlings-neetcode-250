include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::valid_palindrome("raceecar".to_string()), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::valid_palindrome("abcba".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::valid_palindrome("deeee".to_string()), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::valid_palindrome("ab".to_string()), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
}

#[test]
fn test_6() {
    assert_eq!(Solution::valid_palindrome("racecarx".to_string()), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::valid_palindrome("abcdedcba".to_string()), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::valid_palindrome("abcdefgihgfedcba".to_string()), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::valid_palindrome("abcdefdba".to_string()), false);
}

#[test]
fn test_11() {
    assert_eq!(Solution::valid_palindrome("aabaa".to_string()), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::valid_palindrome("racecar".to_string()), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::valid_palindrome("a".to_string()), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::valid_palindrome("abcdefg".to_string()), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::valid_palindrome("aabbbbaaabbb".to_string()), false);
}

#[test]
fn test_17() {
    assert_eq!(Solution::valid_palindrome("abcdefghijklnkjihgfedcba".to_string()), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::valid_palindrome("abcdefghikjlmnopqrstuvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_19() {
    assert_eq!(Solution::valid_palindrome("aabbaa".to_string()), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::valid_palindrome("noonappa".to_string()), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::valid_palindrome("zxcvbnmlkjihgfedcbazyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_22() {
    assert_eq!(Solution::valid_palindrome("abcdexyzzyxedcba".to_string()), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::valid_palindrome("aaaabbbbccccddddcccccccddddbbbbbbaaaaa".to_string()), false);
}

#[test]
fn test_24() {
    assert_eq!(Solution::valid_palindrome("aabbcccbbbaa".to_string()), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::valid_palindrome("flgfgldad".to_string()), false);
}

#[test]
fn test_26() {
    assert_eq!(Solution::valid_palindrome("tattarrattat".to_string()), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeeedcba".to_string()), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::valid_palindrome("aabbacdfgfddcabbaa".to_string()), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::valid_palindrome("zzzzzzzzzz".to_string()), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeeffgggghhhiiiijjjjkkoollmmnnnnooppqqrrsstttuuuvvvvwwxxyyyzzzzyyyyxwvvvvuuutttsrrrqpnoonmlkkkiiihhhhggggfffeeedddccbbaa".to_string()), false);
}

#[test]
fn test_31() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeeffgggihhii".to_string()), false);
}

#[test]
fn test_32() {
    assert_eq!(Solution::valid_palindrome("qwertyuiopoiuytrewq".to_string()), true);
}

#[test]
fn test_33() {
    assert_eq!(Solution::valid_palindrome("zxcvbnmasdfghjklqwertyuiopasdfghjklzxcvbnm".to_string()), false);
}

#[test]
fn test_34() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeffgghhiaahhgffeeddccbbaa".to_string()), false);
}

#[test]
fn test_35() {
    assert_eq!(Solution::valid_palindrome("abcdefghihgfeddcba".to_string()), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::valid_palindrome("abcdabc".to_string()), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::valid_palindrome("aabbcddcbbbaa".to_string()), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::valid_palindrome("aabbbbbaaabbbb".to_string()), false);
}

#[test]
fn test_39() {
    assert_eq!(Solution::valid_palindrome("aabbaabbaa".to_string()), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::valid_palindrome("aabcaaba".to_string()), false);
}

#[test]
fn test_41() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeffgghhijjihhgffeeddccbbaa".to_string()), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupucmlmgqfvnbgtapekouga".to_string()), false);
}

#[test]
fn test_43() {
    assert_eq!(Solution::valid_palindrome("abcdcba".to_string()), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::valid_palindrome("abcdexyzxedcba".to_string()), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::valid_palindrome("aabbccddee".to_string()), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::valid_palindrome("abbaabbaabbaabba".to_string()), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::valid_palindrome("racecarwitharacecarinitt".to_string()), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::valid_palindrome("racecarxracecar".to_string()), true);
}

#[test]
fn test_49() {
    assert_eq!(Solution::valid_palindrome("aabbacdfgdcabbaa".to_string()), true);
}

#[test]
fn test_50() {
    assert_eq!(Solution::valid_palindrome("abbaaaabba".to_string()), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::valid_palindrome("aabbccddeedccbbaa".to_string()), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeeffgggghhhhiiiijjjjkkkkllmmnnnnooppqqrrsstttuuuvvvvvwwxxyyyzzzzzzzyyyyxwvvvvuuutttsrrrqpnoonmlkkkiiihhhhggggfffeeedddccbbaa".to_string()), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::valid_palindrome("abcaabcba".to_string()), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::valid_palindrome("raceacar".to_string()), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::valid_palindrome("noon".to_string()), true);
}

#[test]
fn test_56() {
    assert_eq!(Solution::valid_palindrome("abcdeffedcba".to_string()), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::valid_palindrome("level".to_string()), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::valid_palindrome("aabbbbbbbaaabbbbbb".to_string()), false);
}

#[test]
fn test_59() {
    assert_eq!(Solution::valid_palindrome("aaaaabbaaaaa".to_string()), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::valid_palindrome("abcdefghhgfedcba".to_string()), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeeffggghhiiii".to_string()), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::valid_palindrome("ababababababa".to_string()), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::valid_palindrome("zxcvbnmasdfghjklqwertyuiopplkjhgfdsazxcvbnm".to_string()), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::valid_palindrome("abcdefghijkjihgfedcba".to_string()), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::valid_palindrome("aabbccddeedcba".to_string()), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeefffggg".to_string()), false);
}

#[test]
fn test_67() {
    assert_eq!(Solution::valid_palindrome("levelwithoneleveldropped".to_string()), false);
}

#[test]
fn test_68() {
    assert_eq!(Solution::valid_palindrome("noonwithanothernoon".to_string()), false);
}

#[test]
fn test_69() {
    assert_eq!(Solution::valid_palindrome("abzcdedcba".to_string()), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::valid_palindrome("acbbbca".to_string()), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::valid_palindrome("lcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuc".to_string()), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::valid_palindrome("abcbad".to_string()), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::valid_palindrome("abcdefghijkmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::valid_palindrome("abcdexyzzyxdecba".to_string()), false);
}

#[test]
fn test_75() {
    assert_eq!(Solution::valid_palindrome("abccba".to_string()), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::valid_palindrome("abcdxzyxcba".to_string()), false);
}

#[test]
fn test_77() {
    assert_eq!(Solution::valid_palindrome("madamimadam".to_string()), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::valid_palindrome("aabbabba".to_string()), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::valid_palindrome("abcdefghihgfedcbai".to_string()), true);
}

#[test]
fn test_80() {
    assert_eq!(Solution::valid_palindrome("amanaplanacanalpanama".to_string()), true);
}

#[test]
fn test_81() {
    assert_eq!(Solution::valid_palindrome("aabbccddeedccbaa".to_string()), false);
}

#[test]
fn test_82() {
    assert_eq!(Solution::valid_palindrome("aebcbda".to_string()), false);
}

#[test]
fn test_83() {
    assert_eq!(Solution::valid_palindrome("deified".to_string()), true);
}

#[test]
fn test_84() {
    assert_eq!(Solution::valid_palindrome("repaper".to_string()), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::valid_palindrome("eedede".to_string()), true);
}

#[test]
fn test_86() {
    assert_eq!(Solution::valid_palindrome("abcdeffgedcba".to_string()), true);
}

#[test]
fn test_87() {
    assert_eq!(Solution::valid_palindrome("acbbcdcba".to_string()), false);
}

#[test]
fn test_88() {
    assert_eq!(Solution::valid_palindrome("aabbaaabbaa".to_string()), true);
}

#[test]
fn test_89() {
    assert_eq!(Solution::valid_palindrome("abcdefghijkllkjihgfedcba".to_string()), true);
}

#[test]
fn test_90() {
    assert_eq!(Solution::valid_palindrome("abacdfgdcaba".to_string()), true);
}

#[test]
fn test_91() {
    assert_eq!(Solution::valid_palindrome("abcdefghijklmnopqrrponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_92() {
    assert_eq!(Solution::valid_palindrome("abacdfgdfcba".to_string()), false);
}

#[test]
fn test_93() {
    assert_eq!(Solution::valid_palindrome("aabbccdddccbaa".to_string()), true);
}

#[test]
fn test_94() {
    assert_eq!(Solution::valid_palindrome("abcddcba".to_string()), true);
}

#[test]
fn test_95() {
    assert_eq!(Solution::valid_palindrome("aaaaabaaa".to_string()), true);
}

#[test]
fn test_96() {
    assert_eq!(Solution::valid_palindrome("rotor".to_string()), true);
}

#[test]
fn test_97() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyz".to_string()), false);
}

#[test]
fn test_98() {
    assert_eq!(Solution::valid_palindrome("abcdefghihgfedcba".to_string()), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupucmlgqfvnbgtapekouga".to_string()), false);
}

#[test]
fn test_100() {
    assert_eq!(Solution::valid_palindrome("detartratedwithanotherdetartrated".to_string()), false);
}

#[test]
fn test_101() {
    assert_eq!(Solution::valid_palindrome("aabbccddeeffggahhigghhffeeddccbbaa".to_string()), false);
}

#[test]
fn test_102() {
    assert_eq!(Solution::valid_palindrome("zzazz".to_string()), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::valid_palindrome("rotorwithatinyrotor".to_string()), false);
}

#[test]
fn test_104() {
    assert_eq!(Solution::valid_palindrome("mississippi".to_string()), false);
}

#[test]
fn test_105() {
    assert_eq!(Solution::valid_palindrome("e race car e".to_string()), true);
}

#[test]
fn test_106() {
    assert_eq!(Solution::valid_palindrome("abcdefghihgfedcbaj".to_string()), true);
}

#[test]
fn test_107() {
    assert_eq!(Solution::valid_palindrome("ebcbbececabbacecbbcbe".to_string()), true);
}

#[test]
fn test_108() {
    assert_eq!(Solution::valid_palindrome("acbbba".to_string()), true);
}

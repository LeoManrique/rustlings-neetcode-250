include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::check_inclusion("abc".to_string(), "bbbccca".to_string()), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::check_inclusion("adc".to_string(), "dcda".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::check_inclusion("abc".to_string(), "bbbccba".to_string()), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::check_inclusion("abcde".to_string(), "adecb".to_string()), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::check_inclusion("test".to_string(), "ttewest".to_string()), false);
}

#[test]
fn test_6() {
    assert_eq!(Solution::check_inclusion("abc".to_string(), "cbadef".to_string()), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::check_inclusion("abc".to_string(), "defabc".to_string()), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::check_inclusion("xyz".to_string(), "zyxwvut".to_string()), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::check_inclusion("hello".to_string(), "ooollehed".to_string()), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()), false);
}

#[test]
fn test_11() {
    assert_eq!(Solution::check_inclusion("aabbcc".to_string(), "abcabc".to_string()), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::check_inclusion("hello".to_string(), "ooolleoooleh".to_string()), false);
}

#[test]
fn test_13() {
    assert_eq!(Solution::check_inclusion("xyz".to_string(), "ayzxbcd".to_string()), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::check_inclusion("hello".to_string(), "ooollehdl".to_string()), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::check_inclusion("a".to_string(), "ab".to_string()), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::check_inclusion("z".to_string(), "abcz".to_string()), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::check_inclusion("a".to_string(), "b".to_string()), false);
}

#[test]
fn test_18() {
    assert_eq!(Solution::check_inclusion("abcd".to_string(), "dcba".to_string()), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::check_inclusion("abcd".to_string(), "dcbaefg".to_string()), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::check_inclusion("abc".to_string(), "cccccbabb".to_string()), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::check_inclusion("aaaa".to_string(), "aaabaaaa".to_string()), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::check_inclusion("abcd".to_string(), "abcdxzyw".to_string()), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::check_inclusion("a".to_string(), "a".to_string()), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::check_inclusion("abracadabra".to_string(), "cadabraabra".to_string()), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::check_inclusion("unique".to_string(), "enquci".to_string()), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::check_inclusion("complexity".to_string(), "itpelxcmoytz".to_string()), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::check_inclusion("python".to_string(), "nothpy".to_string()), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::check_inclusion("permutation".to_string(), "aeronpmutitno".to_string()), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::check_inclusion("pqrstuvw".to_string(), "stuvwpqrxyz".to_string()), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::check_inclusion("zzzzz".to_string(), "zzzzzzzzzzzzzzzzzzzzzz".to_string()), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::check_inclusion("aabbccddeeff".to_string(), "bbccddeeffaabb".to_string()), true);
}

#[test]
fn test_33() {
    assert_eq!(Solution::check_inclusion("xyz".to_string(), "zyxabcdef".to_string()), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::check_inclusion("permutation".to_string(), "terumtnipxo".to_string()), false);
}

#[test]
fn test_35() {
    assert_eq!(Solution::check_inclusion("characters".to_string(), "trchaesrhc".to_string()), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::check_inclusion("aabbcc".to_string(), "baccabdefg".to_string()), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::check_inclusion("algorithm".to_string(), "logarithma".to_string()), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::check_inclusion("abcdef".to_string(), "fedcba".to_string()), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::check_inclusion("substring".to_string(), "tstringsub".to_string()), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::check_inclusion("mnopqr".to_string(), "qrstuvwxyzmnopqr".to_string()), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::check_inclusion("aabbccddeeff".to_string(), "fedcbazyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_42() {
    assert_eq!(Solution::check_inclusion("unique".to_string(), "eniquu".to_string()), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::check_inclusion("abcdefghij".to_string(), "abcdefghijabcdefghij".to_string()), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::check_inclusion("xyz".to_string(), "axbyczd".to_string()), false);
}

#[test]
fn test_45() {
    assert_eq!(Solution::check_inclusion("abcdefghij".to_string(), "jihgfedcbaefghijkl".to_string()), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::check_inclusion("hello".to_string(), "ohellonow".to_string()), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::check_inclusion("abcdefghijklmnopqrstuvwxyz".to_string(), "zyxwvutsrqponmlkjihgfedcbaabcde".to_string()), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::check_inclusion("substring".to_string(), "stringgnusbs".to_string()), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::check_inclusion("permutation".to_string(), "tnuatipremot".to_string()), true);
}

#[test]
fn test_50() {
    assert_eq!(Solution::check_inclusion("testcase".to_string(), "stceatcases".to_string()), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::check_inclusion("unique".to_string(), "euqnieabcd".to_string()), false);
}

#[test]
fn test_52() {
    assert_eq!(Solution::check_inclusion("longstring".to_string(), "gnirtsolongstring".to_string()), true);
}

#[test]
fn test_53() {
    assert_eq!(Solution::check_inclusion("abcdabcd".to_string(), "dcbaabcd".to_string()), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::check_inclusion("mississippi".to_string(), "isppiimsss".to_string()), false);
}

#[test]
fn test_55() {
    assert_eq!(Solution::check_inclusion("abcdef".to_string(), "ghfedcbijklm".to_string()), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::check_inclusion("substringpermutation".to_string(), "permutationsubstring".to_string()), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::check_inclusion("variation".to_string(), "atinoriva".to_string()), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::check_inclusion("abcdefghijk".to_string(), "jihgfedcbaklmnopqrs".to_string()), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::check_inclusion("abcd".to_string(), "dcbaefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::check_inclusion("permutation".to_string(), "tporemutani".to_string()), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::check_inclusion("complexity".to_string(), "xxlpeicmtostiy".to_string()), false);
}

#[test]
fn test_62() {
    assert_eq!(Solution::check_inclusion("unique".to_string(), "ueiqnunc".to_string()), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::check_inclusion("aabbcc".to_string(), "cbacbacbacbacbacbacbacbacb".to_string()), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::check_inclusion("abcdefg".to_string(), "ghijklmnopabcdefg".to_string()), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::check_inclusion("abcdefghiklmnopqrstuvwxyz".to_string(), "zyxwvutsrqponmlkjihgfedcbaabcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::check_inclusion("aabbccddeeff".to_string(), "fedcbafedcbafedcba".to_string()), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::check_inclusion("challenge".to_string(), "hgelangecllon".to_string()), false);
}

#[test]
fn test_68() {
    assert_eq!(Solution::check_inclusion("abcdefg".to_string(), "gfedcbahijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::check_inclusion("permutation".to_string(), "tpremnoiuat".to_string()), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::check_inclusion("hello".to_string(), "oellhworld".to_string()), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::check_inclusion("test".to_string(), "tsetabcd".to_string()), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::check_inclusion("abcdefghijk".to_string(), "kljihgfedcbazyxwvutsrqponml".to_string()), false);
}

#[test]
fn test_73() {
    assert_eq!(Solution::check_inclusion("abcdef".to_string(), "fedcbaxyzabcdef".to_string()), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::check_inclusion("example".to_string(), "melpaxe".to_string()), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::check_inclusion("interview".to_string(), "wterevinirt".to_string()), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::check_inclusion("zyxw".to_string(), "wxyzabcd".to_string()), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::check_inclusion("permutation".to_string(), "ttnremuapoi".to_string()), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::check_inclusion("xyzz".to_string(), "zzzyxzzzzzyx".to_string()), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::check_inclusion("hello".to_string(), "ohellworld".to_string()), true);
}

#[test]
fn test_80() {
    assert_eq!(Solution::check_inclusion("longstring".to_string(), "tgnirlongs".to_string()), true);
}

#[test]
fn test_81() {
    assert_eq!(Solution::check_inclusion("aabbcc".to_string(), "bbccaaabcdef".to_string()), true);
}

#[test]
fn test_82() {
    assert_eq!(Solution::check_inclusion("aaaaabbbbb".to_string(), "ababababab".to_string()), true);
}

#[test]
fn test_83() {
    assert_eq!(Solution::check_inclusion("complexpermutation".to_string(), "xmplxcmpotrenuati".to_string()), false);
}

#[test]
fn test_84() {
    assert_eq!(Solution::check_inclusion("xyz".to_string(), "zyxzyxzyxzyxzyx".to_string()), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::check_inclusion("mississippi".to_string(), "ssippiimis".to_string()), false);
}

#[test]
fn test_86() {
    assert_eq!(Solution::check_inclusion("aabbbccc".to_string(), "cccbbbaaabbbcccaabb".to_string()), true);
}

#[test]
fn test_87() {
    assert_eq!(Solution::check_inclusion("xyz".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_88() {
    assert_eq!(Solution::check_inclusion("abcdefg".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_89() {
    assert_eq!(Solution::check_inclusion("permutationexample".to_string(), "xamplepermutation".to_string()), false);
}

#[test]
fn test_90() {
    assert_eq!(Solution::check_inclusion("substring".to_string(), "ggnirtsabcd".to_string()), false);
}

#[test]
fn test_91() {
    assert_eq!(Solution::check_inclusion("abcdefgh".to_string(), "hgfedcbaijkl".to_string()), true);
}

#[test]
fn test_92() {
    assert_eq!(Solution::check_inclusion("abacabadabacaba".to_string(), "badacababacabadaba".to_string()), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::check_inclusion("characters".to_string(), "tcsrhaec".to_string()), false);
}

#[test]
fn test_94() {
    assert_eq!(Solution::check_inclusion("longerstring".to_string(), "stringlongeron".to_string()), true);
}

#[test]
fn test_95() {
    assert_eq!(Solution::check_inclusion("aabbcc".to_string(), "baccab".to_string()), true);
}

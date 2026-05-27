include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::is_alien_sorted(vec!["kuvp".to_string(), "q".to_string()], "ngxlkthsjuoqcpavbfdermiyzw".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::is_alien_sorted(vec!["zzz".to_string(), "zzzz".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "ab".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_5() {
    assert_eq!(Solution::is_alien_sorted(vec!["hello".to_string(), "hello".to_string(), "hello".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "app".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::is_alien_sorted(vec!["app".to_string(), "apple".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::is_alien_sorted(vec!["hello".to_string(), "leetcode".to_string()], "hlabcdefgijkmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::is_alien_sorted(vec!["zoo".to_string(), "zoop".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_12() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "acb".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::is_alien_sorted(vec!["kz".to_string(), "kzc".to_string()], "zabklmncopqrstuvwxy".to_string()), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string()], "cba".to_string()), false);
}

#[test]
fn test_15() {
    assert_eq!(Solution::is_alien_sorted(vec!["kuvp".to_string(), "q".to_string()], "ngxlkthsjuoqcpavbfderimyzw".to_string()), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::is_alien_sorted(vec!["zzz".to_string(), "zaz".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::is_alien_sorted(vec!["z".to_string(), "z".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string(), "aa".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_19() {
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(), "world".to_string(), "row".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
}

#[test]
fn test_20() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "acb".to_string(), "bac".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::is_alien_sorted(vec!["wrt".to_string(), "wrf".to_string(), "er".to_string(), "ett".to_string(), "rftt".to_string()], "wertfabcghijklmnopqsudvyxz".to_string()), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::is_alien_sorted(vec!["banana".to_string(), "apple".to_string(), "orange".to_string()], "zyxcbaedfghijklmnopqrstuvw".to_string()), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::is_alien_sorted(vec!["abcd".to_string(), "abcde".to_string(), "abcd".to_string()], "fedcbazyxwvutsrqponmlkjhig".to_string()), false);
}

#[test]
fn test_24() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string(), "aab".to_string(), "aac".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::is_alien_sorted(vec!["cat".to_string(), "dog".to_string(), "bird".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_26() {
    assert_eq!(Solution::is_alien_sorted(vec!["same".to_string(), "sake".to_string(), "saki".to_string()], "askdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_27() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string(), "aaab".to_string(), "aaac".to_string()], "abcdefghijkmnopqrstuvwxyzl".to_string()), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::is_alien_sorted(vec!["banana".to_string(), "bandana".to_string(), "bandwidth".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_29() {
    assert_eq!(Solution::is_alien_sorted(vec!["xyz".to_string(), "xyzz".to_string(), "xyza".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()], "edcba".to_string()), false);
}

#[test]
fn test_31() {
    assert_eq!(Solution::is_alien_sorted(vec!["short".to_string(), "shorter".to_string(), "shortest".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::is_alien_sorted(vec!["banana".to_string(), "bandana".to_string(), "bandit".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_33() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()], "cbaedfghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_34() {
    assert_eq!(Solution::is_alien_sorted(vec!["xyz".to_string(), "xyy".to_string(), "xyx".to_string()], "xyzabcdefghijklmnopqrstuvw".to_string()), false);
}

#[test]
fn test_35() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::is_alien_sorted(vec!["mismatch".to_string(), "mis".to_string(), "misleading".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::is_alien_sorted(vec!["dog".to_string(), "cat".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::is_alien_sorted(vec!["zebra".to_string(), "dog".to_string(), "cat".to_string()], "dogcatzebraabcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_39() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "abz".to_string(), "abcde".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_40() {
    assert_eq!(Solution::is_alien_sorted(vec!["long".to_string(), "longer".to_string(), "longest".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_41() {
    assert_eq!(Solution::is_alien_sorted(vec!["app".to_string(), "apple".to_string(), "apples".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(), "world".to_string(), "wor".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
}

#[test]
fn test_43() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "apples".to_string(), "app".to_string(), "appl".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_44() {
    assert_eq!(Solution::is_alien_sorted(vec!["dog".to_string(), "cat".to_string(), "bird".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_45() {
    assert_eq!(Solution::is_alien_sorted(vec!["alien".to_string(), "algorithm".to_string(), "all".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::is_alien_sorted(vec!["aa".to_string(), "a".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::is_alien_sorted(vec!["zebra".to_string(), "zebraa".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::is_alien_sorted(vec!["banana".to_string(), "bandana".to_string(), "apple".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::is_alien_sorted(vec!["zebra".to_string(), "zoo".to_string(), "zoom".to_string(), "zoology".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::is_alien_sorted(vec!["same".to_string(), "same".to_string(), "same".to_string()], "fedcbazyxwvutsrqponmlkjhig".to_string()), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::is_alien_sorted(vec!["abcd".to_string(), "abce".to_string(), "abcf".to_string()], "acegbdfhijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_52() {
    assert_eq!(Solution::is_alien_sorted(vec!["one".to_string(), "only".to_string(), "on".to_string(), "once".to_string()], "onabcdefghijkmplqstuvxyzwer".to_string()), false);
}

#[test]
fn test_53() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string(), "aa".to_string(), "aaaa".to_string(), "aaaaa".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_54() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "acb".to_string(), "bac".to_string()], "cbafehidgjklmponqrstuvwxzy".to_string()), false);
}

#[test]
fn test_55() {
    assert_eq!(Solution::is_alien_sorted(vec!["zzz".to_string(), "zz".to_string(), "z".to_string(), "".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_56() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string(), "aab".to_string(), "aac".to_string()], "cbafehidgjklmponqrstuvwxzy".to_string()), false);
}

#[test]
fn test_57() {
    assert_eq!(Solution::is_alien_sorted(vec!["xylophone".to_string(), "xylometer".to_string(), "xylography".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::is_alien_sorted(vec!["abcd".to_string(), "abcde".to_string(), "abcdeg".to_string(), "abcdef".to_string(), "abcdefg".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_59() {
    assert_eq!(Solution::is_alien_sorted(vec!["same".to_string(), "same".to_string(), "same".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::is_alien_sorted(vec!["zebra".to_string(), "dog".to_string(), "duck".to_string(), "dove".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_61() {
    assert_eq!(Solution::is_alien_sorted(vec!["zz".to_string(), "zzz".to_string(), "zzzz".to_string(), "zzzzz".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_62() {
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(), "wor".to_string(), "worl".to_string(), "world".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::is_alien_sorted(vec!["abcd".to_string(), "abce".to_string(), "abcf".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::is_alien_sorted(vec!["short".to_string(), "shorter".to_string(), "shortest".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::is_alien_sorted(vec!["cat".to_string(), "bat".to_string(), "rat".to_string()], "abcdefghirjklmnopqrstuvwxyzct".to_string()), false);
}

#[test]
fn test_66() {
    assert_eq!(Solution::is_alien_sorted(vec!["zebra".to_string(), "zealot".to_string(), "zest".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_67() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string()], "cbaz".to_string()), false);
}

#[test]
fn test_68() {
    assert_eq!(Solution::is_alien_sorted(vec!["zebra".to_string(), "zebr".to_string(), "zebraa".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_69() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "apples".to_string(), "app".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_70() {
    assert_eq!(Solution::is_alien_sorted(vec!["xww".to_string(), "xwx".to_string(), "xw".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_71() {
    assert_eq!(Solution::is_alien_sorted(vec!["xyz".to_string(), "xy".to_string(), "x".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_72() {
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(), "world".to_string(), "wor".to_string(), "worl".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
}

#[test]
fn test_73() {
    assert_eq!(Solution::is_alien_sorted(vec!["zyx".to_string(), "zy".to_string(), "zxy".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_74() {
    assert_eq!(Solution::is_alien_sorted(vec!["xyz".to_string(), "xya".to_string(), "xyb".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_75() {
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(), "world".to_string(), "worlds".to_string(), "worldwide".to_string()], "worldabcdefghijklnmpqstuvxyz".to_string()), false);
}

#[test]
fn test_76() {
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(), "world".to_string(), "wordly".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
}

#[test]
fn test_77() {
    assert_eq!(Solution::is_alien_sorted(vec!["long".to_string(), "longer".to_string(), "longest".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_78() {
    assert_eq!(Solution::is_alien_sorted(vec!["aa".to_string(), "a".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_79() {
    assert_eq!(Solution::is_alien_sorted(vec!["longest".to_string(), "longer".to_string(), "long".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_80() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "a".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_81() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()], "edcba".to_string()), false);
}

#[test]
fn test_82() {
    assert_eq!(Solution::is_alien_sorted(vec!["same".to_string(), "same".to_string(), "same".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_83() {
    assert_eq!(Solution::is_alien_sorted(vec!["special".to_string(), "spectacular".to_string(), "spectacularly".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_84() {
    assert_eq!(Solution::is_alien_sorted(vec!["special".to_string(), "spectacular".to_string(), "spectacularly".to_string()], "abcdefghijklmnopqrstuvwxyze".to_string()), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "acb".to_string(), "bac".to_string(), "bca".to_string(), "cab".to_string(), "cba".to_string()], "abcdefghijkmnopqrstuvwxyzl".to_string()), true);
}

#[test]
fn test_86() {
    assert_eq!(Solution::is_alien_sorted(vec!["zz".to_string(), "z".to_string(), "zzz".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_87() {
    assert_eq!(Solution::is_alien_sorted(vec!["z".to_string(), "y".to_string(), "x".to_string(), "w".to_string(), "v".to_string(), "u".to_string(), "t".to_string(), "s".to_string(), "r".to_string(), "q".to_string(), "p".to_string(), "o".to_string(), "n".to_string(), "m".to_string(), "l".to_string(), "k".to_string(), "j".to_string(), "i".to_string(), "h".to_string(), "g".to_string(), "f".to_string(), "e".to_string(), "d".to_string(), "c".to_string(), "b".to_string(), "a".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_88() {
    assert_eq!(Solution::is_alien_sorted(vec!["banana".to_string(), "bandana".to_string(), "band".to_string(), "bad".to_string()], "badnecghijklmopqrstuvwxyzf".to_string()), false);
}

#[test]
fn test_89() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_90() {
    assert_eq!(Solution::is_alien_sorted(vec!["one".to_string(), "onetwo".to_string(), "onefour".to_string()], "onetwofourmnbvcxzlkjhgfdsapq".to_string()), true);
}

#[test]
fn test_91() {
    assert_eq!(Solution::is_alien_sorted(vec!["zyx".to_string(), "zy".to_string(), "z".to_string(), "".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_92() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string(), "l".to_string(), "m".to_string(), "n".to_string(), "o".to_string(), "p".to_string(), "q".to_string(), "r".to_string(), "s".to_string(), "t".to_string(), "u".to_string(), "v".to_string(), "w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::is_alien_sorted(vec!["this".to_string(), "thisis".to_string(), "thisisatest".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_94() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "ab".to_string(), "abcd".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_95() {
    assert_eq!(Solution::is_alien_sorted(vec!["jjg".to_string(), "jja".to_string(), "jj".to_string(), "jjb".to_string()], "jgfedcbaopqrstuvwxyznhimkl".to_string()), false);
}

#[test]
fn test_96() {
    assert_eq!(Solution::is_alien_sorted(vec!["banana".to_string(), "bandana".to_string(), "band".to_string()], "banxyzklmnopqrstuvwedcfghij".to_string()), false);
}

#[test]
fn test_97() {
    assert_eq!(Solution::is_alien_sorted(vec!["racecar".to_string(), "racecars".to_string(), "racecarx".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_98() {
    assert_eq!(Solution::is_alien_sorted(vec!["abcd".to_string(), "abce".to_string(), "abcf".to_string()], "mnopqrstuvwxyzabcdefghijkl".to_string()), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "app".to_string(), "application".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_100() {
    assert_eq!(Solution::is_alien_sorted(vec!["zz".to_string(), "zzz".to_string(), "zzzz".to_string(), "zzzzz".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_101() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string(), "aa".to_string(), "a".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_102() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "apples".to_string(), "banana".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "app".to_string(), "application".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_104() {
    assert_eq!(Solution::is_alien_sorted(vec!["abcd".to_string(), "dcba".to_string(), "abcd".to_string()], "abcdexyz".to_string()), false);
}

#[test]
fn test_105() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "ab".to_string(), "a".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_106() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "abcd".to_string(), "ab".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_107() {
    assert_eq!(Solution::is_alien_sorted(vec!["abcd".to_string(), "abc".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_108() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string(), "aa".to_string(), "a".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_109() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "abb".to_string(), "aaa".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_110() {
    assert_eq!(Solution::is_alien_sorted(vec!["abcd".to_string(), "abc".to_string(), "ab".to_string(), "a".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_111() {
    assert_eq!(Solution::is_alien_sorted(vec!["mismatch".to_string(), "mis".to_string(), "misleading".to_string()], "abcdefghijklmnopqrstuvwxyze".to_string()), false);
}

#[test]
fn test_112() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string(), "aab".to_string(), "aac".to_string(), "aad".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_113() {
    assert_eq!(Solution::is_alien_sorted(vec!["prefix".to_string(), "pre".to_string(), "prelude".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_114() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "apples".to_string(), "app".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_115() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "app".to_string(), "application".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_116() {
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(), "world".to_string(), "wordplay".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
}

#[test]
fn test_117() {
    assert_eq!(Solution::is_alien_sorted(vec!["zz".to_string(), "za".to_string(), "zb".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_118() {
    assert_eq!(Solution::is_alien_sorted(vec!["z".to_string(), "za".to_string(), "zb".to_string(), "zab".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_119() {
    assert_eq!(Solution::is_alien_sorted(vec!["dog".to_string(), "cat".to_string(), "bird".to_string()], "dogcatbirdwxyzefghijklmnpqrstuvwxyz".to_string()), false);
}

#[test]
fn test_120() {
    assert_eq!(Solution::is_alien_sorted(vec!["ab".to_string(), "abc".to_string(), "abcd".to_string(), "abcde".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_121() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "apples".to_string(), "app".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_122() {
    assert_eq!(Solution::is_alien_sorted(vec!["zzzz".to_string(), "zzz".to_string(), "zzzzz".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_123() {
    assert_eq!(Solution::is_alien_sorted(vec!["longer".to_string(), "long".to_string(), "lon".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_124() {
    assert_eq!(Solution::is_alien_sorted(vec!["hello".to_string(), "hallo".to_string(), "halloween".to_string()], "hlabcdefgijkmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_125() {
    assert_eq!(Solution::is_alien_sorted(vec!["hello".to_string(), "helloo".to_string(), "hell".to_string(), "hellp".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_126() {
    assert_eq!(Solution::is_alien_sorted(vec!["interstellar".to_string(), "inter".to_string(), "intergalactic".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_127() {
    assert_eq!(Solution::is_alien_sorted(vec!["banana".to_string(), "apple".to_string(), "orange".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_128() {
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(), "world".to_string(), "wordld".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
}

#[test]
fn test_129() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "ab".to_string(), "abcd".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_130() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaaa".to_string(), "bbbb".to_string(), "cccc".to_string(), "ddd".to_string()], "abcdefgihjklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_131() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "a".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_132() {
    assert_eq!(Solution::is_alien_sorted(vec!["cba".to_string(), "cbcd".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_133() {
    assert_eq!(Solution::is_alien_sorted(vec!["same".to_string(), "sam".to_string(), "samesame".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_134() {
    assert_eq!(Solution::is_alien_sorted(vec!["ab".to_string(), "abc".to_string(), "abcd".to_string(), "abcde".to_string()], "edcba".to_string()), true);
}

#[test]
fn test_135() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "apples".to_string(), "application".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_136() {
    assert_eq!(Solution::is_alien_sorted(vec!["xx".to_string(), "xy".to_string(), "xz".to_string(), "ya".to_string(), "yb".to_string(), "yc".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_137() {
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(), "world".to_string(), "row".to_string(), "wordz".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
}

#[test]
fn test_138() {
    assert_eq!(Solution::is_alien_sorted(vec!["hello".to_string(), "hell".to_string(), "he".to_string(), "h".to_string()], "hlabcdefgijkmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_139() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaa".to_string(), "aa".to_string(), "a".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_140() {
    assert_eq!(Solution::is_alien_sorted(vec!["hello".to_string(), "hell".to_string(), "hel".to_string(), "he".to_string(), "h".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_141() {
    assert_eq!(Solution::is_alien_sorted(vec!["same".to_string(), "same".to_string(), "same".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), true);
}

#[test]
fn test_142() {
    assert_eq!(Solution::is_alien_sorted(vec!["zyx".to_string(), "zy".to_string(), "z".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_143() {
    assert_eq!(Solution::is_alien_sorted(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string(), "l".to_string(), "m".to_string(), "n".to_string(), "o".to_string(), "p".to_string(), "q".to_string(), "r".to_string(), "s".to_string(), "t".to_string(), "u".to_string(), "v".to_string(), "w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_144() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "acb".to_string(), "bac".to_string(), "bca".to_string(), "cab".to_string(), "cba".to_string()], "bacdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_145() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "abd".to_string(), "abcd".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_146() {
    assert_eq!(Solution::is_alien_sorted(vec!["hello".to_string(), "hallo".to_string(), "hella".to_string()], "hlabcdefgijkmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_147() {
    assert_eq!(Solution::is_alien_sorted(vec!["alien".to_string(), "algorithm".to_string(), "alliance".to_string()], "abcdefghijklmonpqrsuvwxyztefgh".to_string()), false);
}

#[test]
fn test_148() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "abcd".to_string(), "ab".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_149() {
    assert_eq!(Solution::is_alien_sorted(vec!["same".to_string(), "some".to_string(), "sand".to_string()], "samponedcbjklfgtqrhxvziwuy".to_string()), false);
}

#[test]
fn test_150() {
    assert_eq!(Solution::is_alien_sorted(vec!["aaaaa".to_string(), "aaaab".to_string(), "aaaba".to_string()], "abcdefghijklmnopqrstuvwzyx".to_string()), true);
}

#[test]
fn test_151() {
    assert_eq!(Solution::is_alien_sorted(vec!["same".to_string(), "samee".to_string(), "sameee".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_152() {
    assert_eq!(Solution::is_alien_sorted(vec!["abc".to_string(), "abb".to_string(), "ab".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_153() {
    assert_eq!(Solution::is_alien_sorted(vec!["xylophone".to_string(), "xylo".to_string(), "xylophones".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_154() {
    assert_eq!(Solution::is_alien_sorted(vec!["fgh".to_string(), "fg".to_string(), "f".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_155() {
    assert_eq!(Solution::is_alien_sorted(vec!["prefix".to_string(), "pre".to_string(), "prelude".to_string()], "abcdefghijklmnopqrstuvwxyze".to_string()), false);
}

#[test]
fn test_156() {
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(), "apply".to_string(), "app".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

#[test]
fn test_157() {
    assert_eq!(Solution::is_alien_sorted(vec!["banana".to_string(), "bandana".to_string(), "bandanaa".to_string()], "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_158() {
    assert_eq!(Solution::is_alien_sorted(vec!["aab".to_string(), "aac".to_string(), "aaa".to_string(), "aad".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

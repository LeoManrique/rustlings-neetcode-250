include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_extra_char("abc".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_extra_char("sayhelloworld".to_string(), vec!["hello".to_string(), "world".to_string()]), 3);
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_extra_char("abc".to_string(), vec!["d".to_string(), "e".to_string()]), 3);
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_extra_char("abcabcabc".to_string(), vec!["abcabc".to_string(), "abc".to_string()]), 0);
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_extra_char("abracadabra".to_string(), vec!["abra".to_string(), "cadabra".to_string(), "abc".to_string()]), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_extra_char("abc".to_string(), vec!["d".to_string(), "e".to_string(), "f".to_string()]), 3);
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_extra_char("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_extra_char("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), 2);
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_extra_char("ab".to_string(), vec!["abc".to_string(), "def".to_string()]), 2);
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_extra_char("abcde".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::min_extra_char("programming".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string()]), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::min_extra_char("a".to_string(), vec!["b".to_string()]), 1);
}

#[test]
fn test_13() {
    assert_eq!(Solution::min_extra_char("mississippi".to_string(), vec!["mis".to_string(), "is".to_string(), "ip".to_string(), "i".to_string(), "p".to_string()]), 2);
}

#[test]
fn test_14() {
    assert_eq!(Solution::min_extra_char("leetscode".to_string(), vec!["leet".to_string(), "code".to_string(), "leetcode".to_string()]), 1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::min_extra_char("aabbcc".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]), 0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::min_extra_char("programmingisfun".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "is".to_string(), "fun".to_string(), "prog".to_string(), "am".to_string(), "ing".to_string(), "pr".to_string(), "gramm".to_string(), "ingis".to_string(), "funs".to_string()]), 0);
}

#[test]
fn test_17() {
    assert_eq!(Solution::min_extra_char("dynamicprogramming".to_string(), vec!["dyn".to_string(), "ami".to_string(), "cpro".to_string(), "gram".to_string(), "ming".to_string(), "pro".to_string(), "gramming".to_string()]), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::min_extra_char("algorithm".to_string(), vec!["algo".to_string(), "rith".to_string(), "m".to_string(), "alg".to_string(), "ith".to_string(), "o".to_string(), "g".to_string(), "a".to_string(), "l".to_string(), "th".to_string(), "or".to_string(), "thm".to_string(), "al".to_string()]), 0);
}

#[test]
fn test_19() {
    assert_eq!(Solution::min_extra_char("abcdefghijklmnopqrstuvwxyz".to_string(), vec!["a".to_string(), "z".to_string(), "abc".to_string(), "xyz".to_string(), "mnopqr".to_string(), "uvw".to_string(), "def".to_string(), "jkl".to_string()]), 5);
}

#[test]
fn test_20() {
    assert_eq!(Solution::min_extra_char("substringsearch".to_string(), vec!["sub".to_string(), "string".to_string(), "search".to_string(), "substri".to_string(), "ngsea".to_string(), "rch".to_string(), "str".to_string(), "sea".to_string(), "subsearc".to_string(), "ubstringse".to_string()]), 0);
}

#[test]
fn test_21() {
    assert_eq!(Solution::min_extra_char("encyclopedia".to_string(), vec!["ency".to_string(), "clop".to_string(), "edia".to_string(), "lo".to_string(), "pedia".to_string(), "cycle".to_string(), "en".to_string(), "dec".to_string(), "clo".to_string(), "opaedia".to_string()]), 0);
}

#[test]
fn test_22() {
    assert_eq!(Solution::min_extra_char("thisisateststring".to_string(), vec!["this".to_string(), "is".to_string(), "a".to_string(), "test".to_string(), "string".to_string(), "isatest".to_string(), "teststr".to_string(), "str".to_string(), "ring".to_string(), "thi".to_string(), "sta".to_string(), "tes".to_string(), "ing".to_string(), "est".to_string(), "at".to_string()]), 0);
}

#[test]
fn test_23() {
    assert_eq!(Solution::min_extra_char("abcdefghijklmnopqrstuvwxyz".to_string(), vec!["abc".to_string(), "def".to_string(), "ghi".to_string(), "jkl".to_string(), "mno".to_string(), "pqr".to_string(), "stu".to_string(), "vwx".to_string(), "yz".to_string(), "a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string(), "l".to_string(), "m".to_string(), "n".to_string(), "o".to_string(), "p".to_string(), "q".to_string(), "r".to_string(), "s".to_string(), "t".to_string(), "u".to_string(), "v".to_string(), "w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()]), 0);
}

#[test]
fn test_24() {
    assert_eq!(Solution::min_extra_char("optimization".to_string(), vec!["opt".to_string(), "imiz".to_string(), "tio".to_string(), "n".to_string(), "zat".to_string(), "ion".to_string(), "optim".to_string()]), 1);
}

#[test]
fn test_25() {
    assert_eq!(Solution::min_extra_char("programmingisfun".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "is".to_string(), "fun".to_string(), "gramm".to_string(), "ing".to_string(), "program".to_string(), "mingis".to_string(), "funn".to_string()]), 0);
}

#[test]
fn test_26() {
    assert_eq!(Solution::min_extra_char("thisisatest".to_string(), vec!["this".to_string(), "is".to_string(), "a".to_string(), "test".to_string(), "ate".to_string(), "st".to_string()]), 0);
}

#[test]
fn test_27() {
    assert_eq!(Solution::min_extra_char("programmingchallenge".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "chall".to_string(), "lenge".to_string(), "ge".to_string()]), 2);
}

#[test]
fn test_28() {
    assert_eq!(Solution::min_extra_char("abcdefghijklmnopqrstuvwxyz".to_string(), vec!["abc".to_string(), "def".to_string(), "ghi".to_string(), "jkl".to_string(), "mno".to_string(), "pqr".to_string(), "stu".to_string(), "vwx".to_string(), "yz".to_string()]), 0);
}

#[test]
fn test_29() {
    assert_eq!(Solution::min_extra_char("encyclopedia".to_string(), vec!["ency".to_string(), "clopedia".to_string(), "encyclo".to_string(), "pedia".to_string(), "encyclope".to_string(), "dopia".to_string(), "edia".to_string(), "a".to_string(), "ei".to_string(), "lo".to_string(), "ope".to_string(), "dic".to_string(), "lope".to_string(), "dia".to_string(), "edia".to_string()]), 0);
}

#[test]
fn test_30() {
    assert_eq!(Solution::min_extra_char("minimizingextra".to_string(), vec!["mini".to_string(), "min".to_string(), "zing".to_string(), "extra".to_string(), "ze".to_string(), "tra".to_string(), "minimize".to_string()]), 2);
}

#[test]
fn test_31() {
    assert_eq!(Solution::min_extra_char("repeatedcharacters".to_string(), vec!["re".to_string(), "pe".to_string(), "at".to_string(), "ed".to_string(), "cha".to_string(), "rac".to_string(), "ters".to_string(), "ter".to_string(), "char".to_string(), "acters".to_string()]), 0);
}

#[test]
fn test_32() {
    assert_eq!(Solution::min_extra_char("findingthesolution".to_string(), vec!["find".to_string(), "ing".to_string(), "the".to_string(), "solu".to_string(), "tion".to_string(), "solution".to_string(), "thesolu".to_string(), "tingthe".to_string(), "thesolu".to_string(), "solu".to_string(), "tionfind".to_string(), "ingthe".to_string(), "thesolution".to_string(), "ingthes".to_string()]), 0);
}

#[test]
fn test_33() {
    assert_eq!(Solution::min_extra_char("mississippi".to_string(), vec!["mis".to_string(), "sis".to_string(), "ip".to_string(), "i".to_string(), "ss".to_string(), "p".to_string()]), 1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::min_extra_char("aaaaabbbbb".to_string(), vec!["a".to_string(), "b".to_string(), "aa".to_string(), "bb".to_string(), "ab".to_string(), "ba".to_string(), "aaa".to_string(), "bbb".to_string(), "aab".to_string(), "bba".to_string(), "aba".to_string(), "bab".to_string()]), 0);
}

#[test]
fn test_35() {
    assert_eq!(Solution::min_extra_char("mississippi".to_string(), vec!["issi".to_string(), "ssippi".to_string(), "miss".to_string(), "ippi".to_string(), "is".to_string(), "s".to_string(), "pi".to_string(), "p".to_string()]), 0);
}

#[test]
fn test_36() {
    assert_eq!(Solution::min_extra_char("subsequences".to_string(), vec!["sub".to_string(), "seq".to_string(), "uen".to_string(), "ce".to_string(), "s".to_string(), "quence".to_string(), "quencece".to_string()]), 0);
}

#[test]
fn test_37() {
    assert_eq!(Solution::min_extra_char("backtracking".to_string(), vec!["back".to_string(), "track".to_string(), "ing".to_string(), "backtr".to_string(), "acking".to_string(), "tracki".to_string(), "ing".to_string()]), 0);
}

#[test]
fn test_38() {
    assert_eq!(Solution::min_extra_char("abcdefghijk".to_string(), vec!["abc".to_string(), "def".to_string(), "gh".to_string(), "ijk".to_string(), "a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string()]), 0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::min_extra_char("abcdefghijabcdefghijabcdefghij".to_string(), vec!["abc".to_string(), "def".to_string(), "ghi".to_string(), "j".to_string(), "abcdefghij".to_string(), "abcdefgh".to_string(), "abcde".to_string(), "fghij".to_string(), "ab".to_string(), "cd".to_string(), "ef".to_string(), "gh".to_string(), "ij".to_string()]), 0);
}

#[test]
fn test_40() {
    assert_eq!(Solution::min_extra_char("thisisaverylongstring".to_string(), vec!["this".to_string(), "is".to_string(), "a".to_string(), "very".to_string(), "long".to_string(), "string".to_string(), "thi".to_string(), "sis".to_string(), "verylong".to_string()]), 0);
}

#[test]
fn test_41() {
    assert_eq!(Solution::min_extra_char("optimallybreakingstrings".to_string(), vec!["opti".to_string(), "mally".to_string(), "breaking".to_string(), "strings".to_string(), "opt".to_string(), "im".to_string(), "ally".to_string(), "break".to_string(), "ing".to_string(), "str".to_string(), "ing".to_string(), "s".to_string(), "optimal".to_string(), "breaks".to_string(), "allybreak".to_string()]), 0);
}

#[test]
fn test_42() {
    assert_eq!(Solution::min_extra_char("abracadabra".to_string(), vec!["abra".to_string(), "cad".to_string(), "bra".to_string(), "abc".to_string(), "rac".to_string()]), 0);
}

#[test]
fn test_43() {
    assert_eq!(Solution::min_extra_char("mississippi".to_string(), vec!["mis".to_string(), "si".to_string(), "issi".to_string(), "pi".to_string(), "ppi".to_string(), "pis".to_string(), "missis".to_string(), "sippi".to_string(), "ippi".to_string(), "issipp".to_string()]), 0);
}

#[test]
fn test_44() {
    assert_eq!(Solution::min_extra_char("xylophone".to_string(), vec!["xy".to_string(), "lo".to_string(), "phone".to_string(), "ph".to_string(), "one".to_string(), "xo".to_string(), "xyl".to_string(), "ylo".to_string(), "ho".to_string(), "ne".to_string(), "pho".to_string()]), 0);
}

#[test]
fn test_45() {
    assert_eq!(Solution::min_extra_char("subsequences".to_string(), vec!["sub".to_string(), "seq".to_string(), "en".to_string(), "ce".to_string(), "qu".to_string(), "subseq".to_string(), "quence".to_string()]), 2);
}

#[test]
fn test_46() {
    assert_eq!(Solution::min_extra_char("thecodinginterview".to_string(), vec!["the".to_string(), "coding".to_string(), "in".to_string(), "terview".to_string(), "inter".to_string(), "view".to_string(), "codingin".to_string(), "erview".to_string(), "viewint".to_string(), "nterview".to_string(), "inerview".to_string()]), 0);
}

#[test]
fn test_47() {
    assert_eq!(Solution::min_extra_char("abcdefghijklmnopqrstuvwxyz".to_string(), vec!["a".to_string(), "z".to_string(), "abc".to_string(), "def".to_string(), "ghi".to_string(), "jkl".to_string(), "mno".to_string(), "pqr".to_string(), "stu".to_string(), "vwx".to_string(), "yz".to_string()]), 0);
}

#[test]
fn test_48() {
    assert_eq!(Solution::min_extra_char("abracadabra".to_string(), vec!["abra".to_string(), "cad".to_string(), "bra".to_string(), "rac".to_string(), "ab".to_string(), "ad".to_string(), "a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "r".to_string()]), 0);
}

#[test]
fn test_49() {
    assert_eq!(Solution::min_extra_char("optimization".to_string(), vec!["opti".to_string(), "miz".to_string(), "ation".to_string(), "iza".to_string(), "ti".to_string(), "on".to_string()]), 0);
}

#[test]
fn test_50() {
    assert_eq!(Solution::min_extra_char("mississippi".to_string(), vec!["mis".to_string(), "sis".to_string(), "ip".to_string(), "i".to_string(), "ss".to_string(), "is".to_string(), "pi".to_string(), "pp".to_string()]), 1);
}

#[test]
fn test_51() {
    assert_eq!(Solution::min_extra_char("dynamicprogramming".to_string(), vec!["dynamic".to_string(), "program".to_string(), "gram".to_string(), "ming".to_string(), "pro".to_string(), "gramming".to_string()]), 0);
}

#[test]
fn test_52() {
    assert_eq!(Solution::min_extra_char("constraints".to_string(), vec!["con".to_string(), "straint".to_string(), "stra".to_string(), "ints".to_string(), "t".to_string(), "res".to_string(), "trai".to_string(), "ns".to_string(), "constra".to_string()]), 0);
}

#[test]
fn test_53() {
    assert_eq!(Solution::min_extra_char("abcdabcdabcd".to_string(), vec!["ab".to_string(), "cd".to_string(), "abcd".to_string(), "abc".to_string(), "bcd".to_string(), "dabc".to_string()]), 0);
}

#[test]
fn test_54() {
    assert_eq!(Solution::min_extra_char("watermelon".to_string(), vec!["water".to_string(), "melon".to_string(), "wa".to_string(), "ter".to_string(), "melon".to_string(), "wa".to_string(), "me".to_string(), "lo".to_string(), "on".to_string(), "terme".to_string(), "lono".to_string(), "wate".to_string(), "rme".to_string(), "lo".to_string(), "non".to_string(), "w".to_string(), "a".to_string(), "t".to_string(), "e".to_string(), "r".to_string(), "m".to_string(), "e".to_string(), "l".to_string(), "o".to_string(), "n".to_string()]), 0);
}

#[test]
fn test_55() {
    assert_eq!(Solution::min_extra_char("breakfast".to_string(), vec!["break".to_string(), "fast".to_string(), "bre".to_string(), "ak".to_string(), "fastbreak".to_string(), "breakfa".to_string(), "st".to_string()]), 0);
}

#[test]
fn test_56() {
    assert_eq!(Solution::min_extra_char("subsequence".to_string(), vec!["sub".to_string(), "sequence".to_string(), "seq".to_string(), "uen".to_string(), "ce".to_string(), "subse".to_string(), "quen".to_string()]), 0);
}

#[test]
fn test_57() {
    assert_eq!(Solution::min_extra_char("optimization".to_string(), vec!["opti".to_string(), "mize".to_string(), "optim".to_string(), "on".to_string(), "ize".to_string(), "miz".to_string(), "ization".to_string()]), 0);
}

#[test]
fn test_58() {
    assert_eq!(Solution::min_extra_char("programming".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "ing".to_string(), "progr".to_string(), "amming".to_string()]), 0);
}

#[test]
fn test_59() {
    assert_eq!(Solution::min_extra_char("alibabacloud".to_string(), vec!["ali".to_string(), "ba".to_string(), "bacloud".to_string(), "cloud".to_string(), "ib".to_string(), "baclou".to_string(), "bacla".to_string(), "baba".to_string(), "clouds".to_string(), "alibaba".to_string()]), 0);
}

#[test]
fn test_60() {
    assert_eq!(Solution::min_extra_char("segmentation".to_string(), vec!["seg".to_string(), "men".to_string(), "tation".to_string(), "ment".to_string(), "entation".to_string(), "segmen".to_string(), "t".to_string(), "ation".to_string()]), 0);
}

#[test]
fn test_61() {
    assert_eq!(Solution::min_extra_char("algorithm".to_string(), vec!["algo".to_string(), "rithm".to_string(), "log".to_string(), "ith".to_string(), "m".to_string(), "a".to_string(), "thm".to_string(), "gor".to_string(), "orithm".to_string(), "ithm".to_string(), "gori".to_string(), "thmi".to_string()]), 0);
}

#[test]
fn test_62() {
    assert_eq!(Solution::min_extra_char("beautiful".to_string(), vec!["be".to_string(), "autiful".to_string(), "aut".to_string(), "ful".to_string(), "at".to_string(), "beau".to_string(), "ti".to_string(), "ful".to_string(), "b".to_string(), "e".to_string(), "a".to_string(), "u".to_string(), "t".to_string(), "i".to_string(), "f".to_string(), "u".to_string(), "l".to_string(), "beauti".to_string(), "ful".to_string(), "auti".to_string(), "ful".to_string(), "beaut".to_string(), "iful".to_string(), "bea".to_string(), "utiful".to_string(), "be".to_string(), "autiful".to_string(), "ti".to_string(), "ful".to_string(), "bea".to_string(), "ut".to_string(), "iful".to_string(), "beau".to_string(), "tiful".to_string(), "beaut".to_string(), "ifu".to_string(), "l".to_string()]), 0);
}

#[test]
fn test_63() {
    assert_eq!(Solution::min_extra_char("quickbrownfoxjumpsoverthelazydog".to_string(), vec!["quick".to_string(), "brown".to_string(), "fox".to_string(), "jumps".to_string(), "over".to_string(), "the".to_string(), "lazy".to_string(), "dog".to_string(), "qu".to_string(), "ick".to_string(), "br".to_string(), "own".to_string(), "fo".to_string(), "x".to_string(), "ump".to_string(), "so".to_string(), "ver".to_string(), "el".to_string(), "az".to_string(), "y".to_string()]), 0);
}

#[test]
fn test_64() {
    assert_eq!(Solution::min_extra_char("example".to_string(), vec!["ex".to_string(), "ample".to_string(), "am".to_string(), "ple".to_string(), "e".to_string(), "xample".to_string()]), 0);
}

#[test]
fn test_65() {
    assert_eq!(Solution::min_extra_char("dynamicprogramming".to_string(), vec!["dyn".to_string(), "amic".to_string(), "prog".to_string(), "ram".to_string(), "ming".to_string(), "gramming".to_string(), "pro".to_string(), "dynamic".to_string(), "grampro".to_string(), "mingprog".to_string(), "rammi".to_string()]), 0);
}

#[test]
fn test_66() {
    assert_eq!(Solution::min_extra_char("mississippi".to_string(), vec!["mis".to_string(), "sis".to_string(), "sip".to_string(), "issi".to_string(), "ippi".to_string()]), 1);
}

#[test]
fn test_67() {
    assert_eq!(Solution::min_extra_char("subsequence".to_string(), vec!["sub".to_string(), "seq".to_string(), "en".to_string(), "ce".to_string(), "subse".to_string(), "quence".to_string()]), 0);
}

#[test]
fn test_68() {
    assert_eq!(Solution::min_extra_char("overlapping".to_string(), vec!["over".to_string(), "lap".to_string(), "ping".to_string(), "lap".to_string(), "pingo".to_string(), "ver".to_string(), "lap".to_string(), "laplap".to_string()]), 0);
}

#[test]
fn test_69() {
    assert_eq!(Solution::min_extra_char("dynamicprogramming".to_string(), vec!["dyn".to_string(), "amic".to_string(), "pro".to_string(), "gram".to_string(), "ming".to_string(), "prog".to_string(), "dyna".to_string(), "micpro".to_string(), "grammi".to_string(), "amming".to_string()]), 0);
}

#[test]
fn test_70() {
    assert_eq!(Solution::min_extra_char("programming".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "gramming".to_string(), "prog".to_string(), "program".to_string(), "mming".to_string(), "progr".to_string(), "gram".to_string(), "ming".to_string(), "gramm".to_string(), "pro".to_string(), "gram".to_string(), "ming".to_string(), "programming".to_string(), "gram".to_string(), "ming".to_string(), "pro".to_string(), "gram".to_string(), "ming".to_string(), "gramming".to_string(), "pro".to_string(), "gram".to_string(), "ming".to_string(), "gramm".to_string(), "pro".to_string(), "gram".to_string(), "ming".to_string(), "program".to_string(), "ming".to_string(), "pro".to_string(), "gramming".to_string(), "gram".to_string(), "ming".to_string(), "gramm".to_string(), "prog".to_string(), "ming".to_string(), "pro".to_string(), "gramming".to_string(), "pro".to_string(), "gram".to_string(), "ming".to_string(), "gramm".to_string(), "pro".to_string(), "gram".to_string(), "ming".to_string(), "gramming".to_string(), "prog".to_string(), "gram".to_string(), "ming".to_string(), "pro".to_string(), "gramming".to_string()]), 0);
}

#[test]
fn test_71() {
    assert_eq!(Solution::min_extra_char("partitioning".to_string(), vec!["parti".to_string(), "tion".to_string(), "ion".to_string(), "part".to_string(), "itioning".to_string(), "parti".to_string(), "tion".to_string(), "ing".to_string()]), 0);
}

#[test]
fn test_72() {
    assert_eq!(Solution::min_extra_char("abcdefghijklmnopqrstuvwxyz".to_string(), vec!["abc".to_string(), "def".to_string(), "ghi".to_string(), "jkl".to_string(), "mno".to_string(), "pqr".to_string(), "stu".to_string(), "vwx".to_string(), "yz".to_string()]), 0);
}

#[test]
fn test_73() {
    assert_eq!(Solution::min_extra_char("optimization".to_string(), vec!["opt".to_string(), "im".to_string(), "iz".to_string(), "ation".to_string(), "iza".to_string(), "tio".to_string()]), 0);
}

#[test]
fn test_74() {
    assert_eq!(Solution::min_extra_char("breakintothepieces".to_string(), vec!["break".to_string(), "into".to_string(), "the".to_string(), "pie".to_string(), "ces".to_string(), "iece".to_string(), "to".to_string(), "pieces".to_string(), "breakin".to_string(), "intothep".to_string()]), 0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::min_extra_char("fuzzywuzzy".to_string(), vec!["fuz".to_string(), "zy".to_string(), "wuz".to_string(), "z".to_string(), "wuzz".to_string(), "fuzzy".to_string(), "wuzzywuzz".to_string()]), 0);
}

#[test]
fn test_76() {
    assert_eq!(Solution::min_extra_char("programmingcontest".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "con".to_string(), "test".to_string(), "gramming".to_string(), "est".to_string()]), 0);
}

#[test]
fn test_77() {
    assert_eq!(Solution::min_extra_char("abcdefgabcdefg".to_string(), vec!["abc".to_string(), "def".to_string(), "gh".to_string(), "abcdef".to_string(), "bcde".to_string(), "fgh".to_string(), "abcd".to_string(), "efg".to_string(), "fg".to_string(), "abcdab".to_string(), "bcdefg".to_string()]), 0);
}

#[test]
fn test_78() {
    assert_eq!(Solution::min_extra_char("pythonprogramming".to_string(), vec!["py".to_string(), "thon".to_string(), "pro".to_string(), "gram".to_string(), "ming".to_string(), "prog".to_string(), "python".to_string(), "gramming".to_string(), "ron".to_string(), "th".to_string(), "ing".to_string()]), 0);
}

#[test]
fn test_79() {
    assert_eq!(Solution::min_extra_char("helloworld".to_string(), vec!["hello".to_string(), "wor".to_string(), "ld".to_string(), "hell".to_string(), "world".to_string(), "owor".to_string(), "orl".to_string(), "rld".to_string(), "hel".to_string(), "wo".to_string(), "rld".to_string(), "worl".to_string(), "hello".to_string(), "world".to_string(), "worldly".to_string(), "or".to_string(), "wor".to_string(), "ld".to_string(), "hell".to_string(), "world".to_string(), "hellworld".to_string(), "hell".to_string(), "wo".to_string(), "rld".to_string(), "hello".to_string(), "world".to_string()]), 0);
}

#[test]
fn test_80() {
    assert_eq!(Solution::min_extra_char("programmingcontest".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "con".to_string(), "test".to_string(), "gramming".to_string(), "est".to_string(), "on".to_string(), "gramcon".to_string(), "estcontest".to_string()]), 0);
}

#[test]
fn test_81() {
    assert_eq!(Solution::min_extra_char("onomatopoeia".to_string(), vec!["ono".to_string(), "mato".to_string(), "poei".to_string(), "a".to_string(), "ia".to_string(), "oeia".to_string(), "opoe".to_string(), "oe".to_string(), "onom".to_string(), "atopoeia".to_string()]), 0);
}

#[test]
fn test_82() {
    assert_eq!(Solution::min_extra_char("mississippi".to_string(), vec!["mis".to_string(), "issi".to_string(), "ippi".to_string(), "sip".to_string(), "iss".to_string(), "is".to_string(), "i".to_string(), "pi".to_string(), "missis".to_string(), "sippi".to_string()]), 0);
}

#[test]
fn test_83() {
    assert_eq!(Solution::min_extra_char("loremipsum".to_string(), vec!["lor".to_string(), "em".to_string(), "ip".to_string(), "sum".to_string(), "lorem".to_string(), "rem".to_string(), "ems".to_string(), "mip".to_string(), "ips".to_string(), "pum".to_string(), "ipsum".to_string()]), 0);
}

#[test]
fn test_84() {
    assert_eq!(Solution::min_extra_char("interview".to_string(), vec!["inter".to_string(), "view".to_string(), "ter".to_string(), "iew".to_string(), "in".to_string(), "terview".to_string(), "nterv".to_string(), "vieww".to_string()]), 0);
}

#[test]
fn test_85() {
    assert_eq!(Solution::min_extra_char("abracadabra".to_string(), vec!["ab".to_string(), "ra".to_string(), "cad".to_string(), "abra".to_string(), "brac".to_string()]), 0);
}

#[test]
fn test_86() {
    assert_eq!(Solution::min_extra_char("programmingcontest".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "con".to_string(), "test".to_string(), "est".to_string(), "contest".to_string()]), 0);
}

#[test]
fn test_87() {
    assert_eq!(Solution::min_extra_char("mississippi".to_string(), vec!["mis".to_string(), "sis".to_string(), "sip".to_string(), "i".to_string(), "ss".to_string(), "pi".to_string(), "pp".to_string()]), 0);
}

#[test]
fn test_88() {
    assert_eq!(Solution::min_extra_char("dynamicprogramming".to_string(), vec!["dyn".to_string(), "amic".to_string(), "pro".to_string(), "gram".to_string(), "ming".to_string(), "gramm".to_string(), "prog".to_string(), "dyna".to_string(), "progr".to_string()]), 0);
}

#[test]
fn test_89() {
    assert_eq!(Solution::min_extra_char("supercalifragilisticexpialidocious".to_string(), vec!["super".to_string(), "cali".to_string(), "fragilistic".to_string(), "expiali".to_string(), "docious".to_string(), "ex".to_string(), "pi".to_string(), "ali".to_string(), "ous".to_string()]), 0);
}

#[test]
fn test_90() {
    assert_eq!(Solution::min_extra_char("supercalifragilisticexpialidocious".to_string(), vec!["super".to_string(), "cali".to_string(), "fragilistic".to_string(), "expiali".to_string(), "docious".to_string(), "superca".to_string(), "lifrag".to_string(), "istic".to_string(), "expialido".to_string(), "cious".to_string(), "superca".to_string(), "li".to_string(), "frag".to_string(), "ilistic".to_string(), "ex".to_string(), "pialido".to_string(), "california".to_string(), "super".to_string(), "superdocus".to_string()]), 0);
}

#[test]
fn test_91() {
    assert_eq!(Solution::min_extra_char("supercalifragilisticexpialidocious".to_string(), vec!["super".to_string(), "cali".to_string(), "fragilistic".to_string(), "expiali".to_string(), "docious".to_string(), "frag".to_string(), "listic".to_string(), "exp".to_string(), "ali".to_string(), "do".to_string(), "cious".to_string(), "superca".to_string(), "li".to_string(), "fragi".to_string(), "list".to_string(), "expialidoc".to_string(), "supercalifrag".to_string(), "supercalifragilisticexpi".to_string(), "alidocious".to_string(), "istic".to_string(), "expialidoci".to_string(), "superca".to_string(), "lifragilisti".to_string(), "cexpialid".to_string(), "alidoc".to_string(), "expialido".to_string(), "fragilisticexpialidocious".to_string()]), 0);
}

#[test]
fn test_92() {
    assert_eq!(Solution::min_extra_char("optimizationproblems".to_string(), vec!["opt".to_string(), "im".to_string(), "ization".to_string(), "prob".to_string(), "lems".to_string(), "ob".to_string(), "lem".to_string(), "pro".to_string(), "blem".to_string(), "blempro".to_string()]), 0);
}

#[test]
fn test_93() {
    assert_eq!(Solution::min_extra_char("breakfast".to_string(), vec!["break".to_string(), "fast".to_string(), "bre".to_string(), "ak".to_string(), "fast".to_string(), "reak".to_string()]), 0);
}

#[test]
fn test_94() {
    assert_eq!(Solution::min_extra_char("dictionary".to_string(), vec!["dic".to_string(), "tion".to_string(), "tio".to_string(), "nary".to_string(), "dict".to_string(), "ionar".to_string(), "ictionary".to_string(), "dictio".to_string(), "n".to_string()]), 0);
}

#[test]
fn test_95() {
    assert_eq!(Solution::min_extra_char("backtracking".to_string(), vec!["back".to_string(), "track".to_string(), "ing".to_string(), "bac".to_string(), "ktrack".to_string(), "king".to_string()]), 0);
}

#[test]
fn test_96() {
    assert_eq!(Solution::min_extra_char("algorithms".to_string(), vec!["algo".to_string(), "rithm".to_string(), "thms".to_string(), "log".to_string(), "rith".to_string(), "ms".to_string(), "algothm".to_string(), "rithms".to_string()]), 0);
}

#[test]
fn test_97() {
    assert_eq!(Solution::min_extra_char("optimization".to_string(), vec!["opt".to_string(), "im".to_string(), "ization".to_string(), "iz".to_string(), "ation".to_string(), "optim".to_string(), "ize".to_string()]), 0);
}

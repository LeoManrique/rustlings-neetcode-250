include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::word_break("cars".to_string(), vec!["car".to_string(), "ca".to_string(), "rs".to_string()]), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::word_break("goals".to_string(), vec!["go".to_string(), "goal".to_string(), "goals".to_string()]), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::word_break("pineapplepenapple".to_string(), vec!["apple".to_string(), "pen".to_string(), "applepen".to_string(), "pine".to_string(), "pineapple".to_string()]), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::word_break("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::word_break("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::word_break("abcd".to_string(), vec!["a".to_string(), "abc".to_string(), "b".to_string(), "cd".to_string()]), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::word_break("supercalifragilisticexpialidocious".to_string(), vec!["super".to_string(), "cali".to_string(), "fragi".to_string(), "listic".to_string(), "expi".to_string(), "ali".to_string(), "docious".to_string()]), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::word_break("skyscraper".to_string(), vec!["sky".to_string(), "sc".to_string(), "raper".to_string(), "scra".to_string(), "per".to_string()]), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::word_break("wordbreakproblem".to_string(), vec!["word".to_string(), "break".to_string(), "problem".to_string(), "wordbreak".to_string(), "breakproblem".to_string(), "wordbreakproblem".to_string(), "wordbreakpro".to_string(), "blem".to_string(), "wordprob".to_string(), "lem".to_string(), "wordb".to_string(), "reak".to_string(), "breakpro".to_string(), "brea".to_string(), "kprob".to_string(), "wordbre".to_string(), "akprob".to_string(), "wordbreakp".to_string(), "rob".to_string(), "reakp".to_string(), "reakpro".to_string(), "wordbre".to_string(), "reakproblem".to_string()]), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::word_break("cascadingsubstrings".to_string(), vec!["cascade".to_string(), "sub".to_string(), "strings".to_string(), "cascade".to_string(), "ing".to_string(), "substring".to_string()]), false);
}

#[test]
fn test_12() {
    assert_eq!(Solution::word_break("breaktheglassceiling".to_string(), vec!["break".to_string(), "the".to_string(), "glass".to_string(), "ceiling".to_string(), "breaks".to_string(), "theglass".to_string(), "ceiling".to_string()]), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::word_break("aquickbrownfoxjumpsoverthelazydog".to_string(), vec!["a".to_string(), "quick".to_string(), "brown".to_string(), "fox".to_string(), "jumps".to_string(), "over".to_string(), "the".to_string(), "lazy".to_string(), "dog".to_string()]), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::word_break("abcdabcdabcd".to_string(), vec!["a".to_string(), "ab".to_string(), "abc".to_string(), "abcd".to_string()]), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::word_break("abcd".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "abc".to_string(), "bc".to_string(), "abcd".to_string(), "cd".to_string()]), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::word_break("wordbreakproblem".to_string(), vec!["word".to_string(), "break".to_string(), "problem".to_string(), "pro".to_string(), "gram".to_string(), "wordbreak".to_string(), "breakprob".to_string()]), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::word_break("aaaaaaaaaaaaaab".to_string(), vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string(), "aaaaa".to_string(), "aaaaaa".to_string(), "aaaaaaa".to_string(), "aaaaaaaa".to_string(), "aaaaaaaaa".to_string(), "aaaaaaaaaa".to_string()]), false);
}

#[test]
fn test_18() {
    assert_eq!(Solution::word_break("interviewquestionsarehard".to_string(), vec!["interview".to_string(), "questions".to_string(), "are".to_string(), "hard".to_string(), "inter".to_string(), "view".to_string(), "quest".to_string(), "ions".to_string(), "ques".to_string(), "tionsare".to_string(), "arehard".to_string()]), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::word_break("mississippi".to_string(), vec!["mis".to_string(), "is".to_string(), "sip".to_string(), "i".to_string(), "pi".to_string(), "p".to_string(), "mississipp".to_string(), "missis".to_string(), "miss".to_string(), "issi".to_string(), "ippi".to_string(), "ssippi".to_string(), "ssipp".to_string(), "ssip".to_string(), "ssips".to_string(), "ssip".to_string(), "issipi".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string(), "issip".to_string(), "issipp".to_string(), "issip".to_string(), "issips".to_string()]), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::word_break("complexwordbreakproblem".to_string(), vec!["complex".to_string(), "word".to_string(), "break".to_string(), "problem".to_string(), "wordbreak".to_string(), "breakprob".to_string(), "lem".to_string(), "prob".to_string(), "lemp".to_string(), "complexword".to_string()]), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::word_break("mississippi".to_string(), vec!["mis".to_string(), "issi".to_string(), "ppi".to_string(), "issip".to_string(), "ippi".to_string(), "missis".to_string(), "sip".to_string(), "pi".to_string(), "ssippi".to_string(), "is".to_string(), "ip".to_string(), "sipi".to_string(), "issipp".to_string(), "ippi".to_string(), "ippi".to_string(), "mississi".to_string(), "ppis".to_string(), "ippii".to_string(), "missi".to_string(), "mississipp".to_string(), "i".to_string(), "p".to_string(), "issippip".to_string(), "issiippi".to_string(), "mississippi".to_string()]), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::word_break("abracadabra".to_string(), vec!["abra".to_string(), "cad".to_string(), "abra".to_string(), "cadabra".to_string(), "ra".to_string(), "dab".to_string(), "ra".to_string(), "cadabra".to_string()]), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::word_break("supercalifragilisticexpialidocious".to_string(), vec!["super".to_string(), "cali".to_string(), "fragilistic".to_string(), "expiali".to_string(), "docious".to_string(), "superca".to_string(), "li".to_string(), "fragi".to_string(), "listicex".to_string(), "piali".to_string(), "do".to_string()]), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::word_break("abrakadabra".to_string(), vec!["abra".to_string(), "kadabra".to_string(), "abra".to_string(), "ka".to_string(), "da".to_string(), "bra".to_string()]), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::word_break("aabbccddeeff".to_string(), vec!["aa".to_string(), "bb".to_string(), "cc".to_string(), "dd".to_string(), "ee".to_string(), "ff".to_string(), "abc".to_string(), "def".to_string(), "ef".to_string(), "fe".to_string(), "efg".to_string(), "gh".to_string(), "abcdefgh".to_string()]), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::word_break("helplineshorthelpline".to_string(), vec!["help".to_string(), "line".to_string(), "short".to_string(), "helpline".to_string()]), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::word_break("leetleetcodeleet".to_string(), vec!["leet".to_string(), "code".to_string(), "leetcode".to_string()]), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::word_break("racecar".to_string(), vec!["race".to_string(), "car".to_string(), "racec".to_string(), "arc".to_string(), "cec".to_string(), "er".to_string(), "c".to_string(), "ra".to_string(), "ec".to_string(), "ce".to_string()]), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::word_break("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(), vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string(), "aaaaa".to_string(), "ba".to_string()]), false);
}

#[test]
fn test_30() {
    assert_eq!(Solution::word_break("mississippi".to_string(), vec!["mis".to_string(), "sis".to_string(), "ip".to_string(), "is".to_string()]), false);
}

#[test]
fn test_31() {
    assert_eq!(Solution::word_break("aaaaaaa".to_string(), vec!["aa".to_string(), "aaa".to_string()]), true);
}

#[test]
fn test_32() {
    assert_eq!(Solution::word_break("mississippi".to_string(), vec!["mis".to_string(), "is".to_string(), "ip".to_string(), "i".to_string(), "ssip".to_string()]), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::word_break("supercalifragilisticexpialidocious".to_string(), vec!["super".to_string(), "cali".to_string(), "fragilistic".to_string(), "expiali".to_string(), "docious".to_string(), "superfragilisticexpialidocious".to_string()]), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::word_break("banana".to_string(), vec!["ba".to_string(), "na".to_string(), "nan".to_string(), "ban".to_string(), "nana".to_string(), "an".to_string()]), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::word_break("thisisatest".to_string(), vec!["this".to_string(), "is".to_string(), "a".to_string(), "test".to_string(), "thisis".to_string(), "isatest".to_string()]), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::word_break("abcdefgh".to_string(), vec!["a".to_string(), "abc".to_string(), "abcd".to_string(), "efg".to_string(), "h".to_string()]), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::word_break("abcdabcdeabcdabcde".to_string(), vec!["abc".to_string(), "abcd".to_string(), "abcde".to_string(), "de".to_string(), "abcdabcde".to_string()]), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::word_break("ababcabcabcabab".to_string(), vec!["ab".to_string(), "abc".to_string(), "ababc".to_string(), "ababcabc".to_string(), "ababcabcabc".to_string(), "abab".to_string()]), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::word_break("ababababab".to_string(), vec!["ab".to_string(), "aba".to_string(), "bab".to_string(), "baba".to_string()]), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::word_break("aaaaaaa".to_string(), vec!["aa".to_string(), "aaa".to_string(), "aaaa".to_string(), "aaaaa".to_string(), "aaaaaa".to_string(), "aaaaaaa".to_string()]), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::word_break("dynamicprogramming".to_string(), vec!["dynamic".to_string(), "programming".to_string(), "dyna".to_string(), "mic".to_string(), "prog".to_string(), "gram".to_string(), "ming".to_string()]), true);
}

#[test]
fn test_42() {
    assert_eq!(Solution::word_break("aquickbrownfoxjumpsoverthelazydog".to_string(), vec!["a".to_string(), "quick".to_string(), "brown".to_string(), "fox".to_string(), "jumps".to_string(), "over".to_string(), "the".to_string(), "lazy".to_string(), "dog".to_string(), "aquick".to_string(), "brownfox".to_string(), "jump".to_string(), "sover".to_string(), "thelazy".to_string(), "zydog".to_string(), "quickbr".to_string(), "ownfo".to_string(), "xjump".to_string(), "soverth".to_string()]), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::word_break("thisisaverylongstringthatisdifficulttobreak".to_string(), vec!["this".to_string(), "is".to_string(), "a".to_string(), "very".to_string(), "long".to_string(), "string".to_string(), "that".to_string(), "is".to_string(), "difficult".to_string(), "to".to_string(), "break".to_string(), "avery".to_string(), "longstring".to_string(), "isdifficult".to_string()]), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::word_break("aabbccddeeff".to_string(), vec!["aa".to_string(), "bb".to_string(), "cc".to_string(), "dd".to_string(), "ee".to_string(), "ff".to_string(), "abc".to_string(), "def".to_string(), "ef".to_string(), "cd".to_string(), "ab".to_string(), "de".to_string(), "fe".to_string(), "bc".to_string(), "dc".to_string(), "ad".to_string(), "da".to_string(), "fb".to_string(), "bf".to_string(), "ba".to_string(), "ac".to_string(), "ca".to_string(), "ea".to_string(), "ae".to_string(), "be".to_string(), "eb".to_string()]), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::word_break("programmingisfun".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "is".to_string(), "fun".to_string(), "grammi".to_string(), "ngis".to_string(), "funny".to_string(), "progr".to_string(), "amming".to_string(), "isfun".to_string(), "ogramming".to_string(), "grammi".to_string(), "ngisfu".to_string(), "n".to_string()]), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::word_break("mississippi".to_string(), vec!["missi".to_string(), "pi".to_string(), "ssippi".to_string(), "issipi".to_string(), "ippi".to_string()]), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::word_break("abcdefghij".to_string(), vec!["a".to_string(), "ab".to_string(), "abc".to_string(), "abcd".to_string(), "abcde".to_string(), "abcdef".to_string(), "abcdefg".to_string(), "abcdefgh".to_string(), "abcdefghi".to_string(), "abcdefghij".to_string()]), true);
}

#[test]
fn test_48() {
    assert_eq!(Solution::word_break("abcdefghijk".to_string(), vec!["abc".to_string(), "def".to_string(), "ghi".to_string(), "jkl".to_string()]), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::word_break("thefastbrownfoxjumpedoverthelazydog".to_string(), vec!["the".to_string(), "fast".to_string(), "brown".to_string(), "fox".to_string(), "jumped".to_string(), "over".to_string(), "lazy".to_string(), "dog".to_string()]), true);
}

#[test]
fn test_50() {
    assert_eq!(Solution::word_break("solutionsarefun".to_string(), vec!["solution".to_string(), "solutions".to_string(), "are".to_string(), "fun".to_string(), "solu".to_string(), "tions".to_string(), "sol".to_string(), "u".to_string(), "n".to_string()]), true);
}

#[test]
fn test_51() {
    assert_eq!(Solution::word_break("thelongestword".to_string(), vec!["the".to_string(), "long".to_string(), "longest".to_string(), "word".to_string(), "lon".to_string(), "gest".to_string(), "longes".to_string(), "estword".to_string(), "longestwo".to_string(), "longestwo".to_string(), "longes".to_string(), "gestwo".to_string(), "est".to_string(), "wordwo".to_string(), "rdwo".to_string(), "rdo".to_string(), "wo".to_string(), "o".to_string()]), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::word_break("bcbcbc".to_string(), vec!["b".to_string(), "c".to_string(), "bc".to_string(), "cb".to_string()]), true);
}

#[test]
fn test_53() {
    assert_eq!(Solution::word_break("programming".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "gramm".to_string(), "ing".to_string(), "program".to_string()]), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::word_break("xyzzyzyxzyzyzyzyxzyz".to_string(), vec!["xyz".to_string(), "zyz".to_string(), "zx".to_string(), "zyxzy".to_string(), "zyzyz".to_string(), "zyzyzy".to_string()]), false);
}

#[test]
fn test_55() {
    assert_eq!(Solution::word_break("theskyisblue".to_string(), vec!["the".to_string(), "sky".to_string(), "is".to_string(), "blue".to_string(), "thesky".to_string(), "isblue".to_string(), "theskyisblue".to_string(), "theblue".to_string(), "theskyblue".to_string(), "blueis".to_string(), "isblue".to_string(), "blueisblue".to_string(), "theskyis".to_string()]), true);
}

#[test]
fn test_56() {
    assert_eq!(Solution::word_break("mississippi".to_string(), vec!["miss".to_string(), "issi".to_string(), "ssippi".to_string(), "ppi".to_string(), "ipi".to_string(), "i".to_string()]), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::word_break("sheisbeautiful".to_string(), vec!["she".to_string(), "is".to_string(), "beau".to_string(), "tiful".to_string(), "beauti".to_string(), "ful".to_string(), "be".to_string(), "auti".to_string(), "ful".to_string(), "ti".to_string(), "ful".to_string(), "shi".to_string(), "bea".to_string(), "autiful".to_string(), "sheis".to_string()]), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::word_break("exampleexampleexampleexample".to_string(), vec!["ex".to_string(), "ample".to_string(), "example".to_string(), "pleexam".to_string(), "ampleex".to_string()]), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::word_break("abcdefgh".to_string(), vec!["ab".to_string(), "abc".to_string(), "cd".to_string(), "efgh".to_string(), "abcd".to_string()]), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::word_break("abcdefghijklmnopqrstuvwxyz".to_string(), vec!["abcdefgh".to_string(), "ijklmnop".to_string(), "qrstuvwx".to_string(), "yz".to_string(), "mnopqr".to_string(), "stuvwx".to_string(), "yzab".to_string(), "cdefghijklmnop".to_string(), "qrstuvwxyza".to_string(), "bcdefghijklmnopq".to_string(), "rstuvwxyzabcde".to_string()]), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::word_break("supercalifragilisticexpialidocious".to_string(), vec!["super".to_string(), "cali".to_string(), "fragilistic".to_string(), "expiali".to_string(), "docious".to_string(), "superfragilistic".to_string()]), true);
}

#[test]
fn test_62() {
    assert_eq!(Solution::word_break("ababababab".to_string(), vec!["a".to_string(), "ab".to_string(), "aba".to_string(), "ababa".to_string()]), true);
}

#[test]
fn test_63() {
    assert_eq!(Solution::word_break("thisisaverylongwordthatcanbebrokenintomultiplesubwords".to_string(), vec!["this".to_string(), "is".to_string(), "a".to_string(), "very".to_string(), "long".to_string(), "word".to_string(), "that".to_string(), "can".to_string(), "be".to_string(), "broken".to_string(), "into".to_string(), "multiple".to_string(), "sub".to_string(), "words".to_string(), "averylong".to_string(), "bebroken".to_string()]), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::word_break("interview".to_string(), vec!["in".to_string(), "ter".to_string(), "view".to_string(), "int".to_string(), "ent".to_string(), "rview".to_string(), "terview".to_string(), "erview".to_string()]), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::word_break("abcdefghij".to_string(), vec!["ab".to_string(), "abc".to_string(), "cd".to_string(), "efg".to_string(), "hij".to_string(), "abcdefghij".to_string()]), true);
}

#[test]
fn test_66() {
    assert_eq!(Solution::word_break("thisisatutorial".to_string(), vec!["this".to_string(), "is".to_string(), "a".to_string(), "tu".to_string(), "torial".to_string(), "tuto".to_string(), "rial".to_string(), "ial".to_string(), "al".to_string()]), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::word_break("abcdabcde".to_string(), vec!["ab".to_string(), "cd".to_string(), "ef".to_string(), "de".to_string(), "abcde".to_string(), "abcd".to_string()]), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::word_break("mississippi".to_string(), vec!["mis".to_string(), "is".to_string(), "is".to_string(), "ppi".to_string(), "issi".to_string(), "ippi".to_string(), "pp".to_string(), "miss".to_string(), "mis".to_string(), "ippi".to_string()]), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::word_break("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(), vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string(), "aaaaa".to_string(), "aaaaaa".to_string(), "aaaaaaa".to_string()]), false);
}

#[test]
fn test_70() {
    assert_eq!(Solution::word_break("findingthesolutionisfun".to_string(), vec!["find".to_string(), "finding".to_string(), "the".to_string(), "solution".to_string(), "is".to_string(), "fun".to_string(), "solutions".to_string(), "funny".to_string()]), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::word_break("thelongestword".to_string(), vec!["the".to_string(), "long".to_string(), "longest".to_string(), "word".to_string(), "longestword".to_string(), "oneword".to_string(), "endword".to_string(), "word".to_string(), "one".to_string()]), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::word_break("abcd".to_string(), vec!["a".to_string(), "ab".to_string(), "abc".to_string(), "abcd".to_string(), "abcde".to_string()]), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::word_break("xxyzzyyx".to_string(), vec!["xy".to_string(), "xyz".to_string(), "zy".to_string(), "zyx".to_string(), "yx".to_string(), "xx".to_string(), "zz".to_string(), "zyxzyx".to_string()]), false);
}

#[test]
fn test_74() {
    assert_eq!(Solution::word_break("supercalifragilisticexpialidocious".to_string(), vec!["super".to_string(), "cali".to_string(), "fragi".to_string(), "listic".to_string(), "expi".to_string(), "ali".to_string(), "docious".to_string(), "supercalifragilisticexpialidoci".to_string()]), true);
}

#[test]
fn test_75() {
    assert_eq!(Solution::word_break("thelongestword".to_string(), vec!["the".to_string(), "long".to_string(), "est".to_string(), "word".to_string(), "longest".to_string(), "estword".to_string(), "ongestwor".to_string(), "ngestwo".to_string(), "gestw".to_string(), "estwo".to_string(), "stwo".to_string(), "two".to_string(), "thelo".to_string(), "hello".to_string(), "world".to_string()]), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::word_break("programmingisawesome".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "is".to_string(), "awe".to_string(), "some".to_string(), "awesome".to_string(), "awe".to_string(), "so".to_string(), "me".to_string()]), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::word_break("thecodecanbeanything".to_string(), vec!["the".to_string(), "code".to_string(), "can".to_string(), "be".to_string(), "anything".to_string(), "any".to_string(), "thing".to_string()]), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::word_break("a".to_string(), vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string(), "aaaaa".to_string(), "aaaaaa".to_string(), "aaaaaaa".to_string(), "aaaaaaaa".to_string(), "aaaaaaaaa".to_string(), "aaaaaaaaaa".to_string()]), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::word_break("abcdabcd".to_string(), vec!["ab".to_string(), "abcd".to_string(), "cd".to_string()]), true);
}

#[test]
fn test_80() {
    assert_eq!(Solution::word_break("thisisaverylongstringthatshouldnotmatch".to_string(), vec!["this".to_string(), "is".to_string(), "very".to_string(), "long".to_string(), "string".to_string(), "that".to_string(), "should".to_string(), "not".to_string(), "match".to_string()]), false);
}

#[test]
fn test_81() {
    assert_eq!(Solution::word_break("programmingproblemsolver".to_string(), vec!["pro".to_string(), "gram".to_string(), "ming".to_string(), "problem".to_string(), "problemsolver".to_string(), "solver".to_string()]), true);
}

#[test]
fn test_82() {
    assert_eq!(Solution::word_break("aaaaaaa".to_string(), vec!["aaa".to_string(), "aaaa".to_string(), "aa".to_string()]), true);
}

#[test]
fn test_83() {
    assert_eq!(Solution::word_break("skilfullyskilled".to_string(), vec!["skill".to_string(), "ful".to_string(), "ly".to_string(), "ski".to_string(), "lly".to_string(), "ed".to_string()]), false);
}

#[test]
fn test_84() {
    assert_eq!(Solution::word_break("internationalization".to_string(), vec!["inter".to_string(), "national".to_string(), "ization".to_string(), "inter".to_string(), "nationalization".to_string()]), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::word_break("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(), vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string(), "aaaaa".to_string(), "aaaaaa".to_string(), "aaaaaaa".to_string(), "aaaaaaaa".to_string(), "aaaaaaaaa".to_string(), "aaaaaaaaaa".to_string()]), false);
}

#[test]
fn test_86() {
    assert_eq!(Solution::word_break("abracadabra".to_string(), vec!["abra".to_string(), "cad".to_string(), "bra".to_string(), "a".to_string(), "ab".to_string(), "rac".to_string()]), true);
}

#[test]
fn test_87() {
    assert_eq!(Solution::word_break("complexstringwithwords".to_string(), vec!["complex".to_string(), "string".to_string(), "with".to_string(), "words".to_string(), "complexstring".to_string(), "stringwith".to_string(), "withwords".to_string()]), true);
}

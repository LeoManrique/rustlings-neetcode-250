include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::is_anagram("abcdefghijklmnopqrstuvwxyz".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::is_anagram("abcde".to_string(), "edcba".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::is_anagram("abc".to_string(), "abcd".to_string()), false);
}

#[test]
fn test_4() {
    assert_eq!(Solution::is_anagram("apple".to_string(), "pale".to_string()), false);
}

#[test]
fn test_5() {
    assert_eq!(Solution::is_anagram("hello".to_string(), "bello".to_string()), false);
}

#[test]
fn test_6() {
    assert_eq!(Solution::is_anagram("aacc".to_string(), "ccac".to_string()), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::is_anagram("abc".to_string(), "def".to_string()), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::is_anagram("abc".to_string(), "cba".to_string()), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::is_anagram("abcd".to_string(), "abce".to_string()), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::is_anagram("rat".to_string(), "car".to_string()), false);
}

#[test]
fn test_12() {
    assert_eq!(Solution::is_anagram("a".to_string(), "a".to_string()), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::is_anagram("ab".to_string(), "ba".to_string()), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::is_anagram("listen".to_string(), "silent".to_string()), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::is_anagram("abcd".to_string(), "dcba".to_string()), true);
}

#[test]
fn test_16() {
    assert_eq!(Solution::is_anagram("triangle".to_string(), "integral".to_string()), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::is_anagram("aabbcc".to_string(), "abcabc".to_string()), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::is_anagram("".to_string(), "".to_string()), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::is_anagram("ababababababababab".to_string(), "bababababababababa".to_string()), true);
}

#[test]
fn test_20() {
    assert_eq!(Solution::is_anagram("hello world".to_string(), "worldhello".to_string()), false);
}

#[test]
fn test_21() {
    assert_eq!(Solution::is_anagram("theeyes".to_string(), "theysee".to_string()), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::is_anagram("repeatedcharactershere".to_string(), "repeatedcharactershere".to_string()), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::is_anagram("ababababab".to_string(), "bababababa".to_string()), true);
}

#[test]
fn test_24() {
    assert_eq!(Solution::is_anagram("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "zzxxyywwvvuuttssrrqqppoonnmmllkkjjiihhggffeeddccbbaa".to_string()), true);
}

#[test]
fn test_25() {
    assert_eq!(Solution::is_anagram("the quick brown fox".to_string(), "xof nworb kciuq eht".to_string()), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::is_anagram("astronomer".to_string(), "moonstarer".to_string()), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::is_anagram("thisisananagram".to_string(), "isanagramthis".to_string()), false);
}

#[test]
fn test_28() {
    assert_eq!(Solution::is_anagram("spaces should be ignored".to_string(), "should be ignored spaces".to_string()), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::is_anagram("aabbcc".to_string(), "ccbbaa".to_string()), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::is_anagram("special!@#$%^&*()characters".to_string(), "characters)!@#$%^&*()special".to_string()), false);
}

#[test]
fn test_31() {
    assert_eq!(Solution::is_anagram("aaaabbbbccccddddeeeeffffgggghhhhiiiijjjjkkkkllllmmmmnnnnooooppppqqqqrrrrssssttttuuuuvvvvwwwwxxxxyyyyzzzz".to_string(), "zzzzyyyxxxwwwwvvvvuuuuttttssssrrrrqqqqppppooooonnnnmmmmllllkkkkjjjjiiiigggghhhhffffffeeeeeeeeddccccbbbbaaaa".to_string()), false);
}

#[test]
fn test_32() {
    assert_eq!(Solution::is_anagram("anagram".to_string(), "nagarams".to_string()), false);
}

#[test]
fn test_33() {
    assert_eq!(Solution::is_anagram("dormitory".to_string(), "dirtyroom".to_string()), true);
}

#[test]
fn test_34() {
    assert_eq!(Solution::is_anagram("1234567890".to_string(), "0987654321".to_string()), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::is_anagram("anananana".to_string(), "naanaanna".to_string()), true);
}

#[test]
fn test_36() {
    assert_eq!(Solution::is_anagram("".to_string(), "a".to_string()), false);
}

#[test]
fn test_37() {
    assert_eq!(Solution::is_anagram("thisisanagramtest".to_string(), "agamnartisiesttst".to_string()), false);
}

#[test]
fn test_38() {
    assert_eq!(Solution::is_anagram("hello world".to_string(), "world hello".to_string()), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::is_anagram("a gentleman".to_string(), "elegant man".to_string()), true);
}

#[test]
fn test_40() {
    assert_eq!(Solution::is_anagram("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "zzyyxxwwvvuuttrrssqqppoonnmmllkkjjiihhggffeeddccbbaa".to_string()), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::is_anagram("aabbcc".to_string(), "aabbc".to_string()), false);
}

#[test]
fn test_42() {
    assert_eq!(Solution::is_anagram("school master".to_string(), "the classroom".to_string()), true);
}

#[test]
fn test_43() {
    assert_eq!(Solution::is_anagram("thisisaverylongstringthatwearetesting".to_string(), "averylongstringthatwearetestingthisis".to_string()), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::is_anagram("thequickbrownfoxjumpsoverthelazydog".to_string(), "godzylathedelvropmusfixnworbquickthe".to_string()), false);
}

#[test]
fn test_45() {
    assert_eq!(Solution::is_anagram("thisisananagram".to_string(), "isananagramthis".to_string()), true);
}

#[test]
fn test_46() {
    assert_eq!(Solution::is_anagram("funeral".to_string(), "real fun".to_string()), false);
}

#[test]
fn test_47() {
    assert_eq!(Solution::is_anagram("unitedstates".to_string(), "adtenisestatesu".to_string()), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::is_anagram("mississippi".to_string(), "mississipi".to_string()), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::is_anagram("elevenplus".to_string(), "twelvestop".to_string()), false);
}

#[test]
fn test_50() {
    assert_eq!(Solution::is_anagram("a".to_string(), "b".to_string()), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::is_anagram("anagramanagramanagram".to_string(), "nagaramnagaramnagaram".to_string()), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::is_anagram("aabbccdd".to_string(), "ddbbaacc".to_string()), true);
}

#[test]
fn test_53() {
    assert_eq!(Solution::is_anagram("aquickbrownfoxjumpsoverthelazydog".to_string(), "quickbrownfoxjumpsoverthelazydoga".to_string()), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::is_anagram("abcdeabced".to_string(), "abcedabcde".to_string()), true);
}

#[test]
fn test_55() {
    assert_eq!(Solution::is_anagram("aquickbrownfoxjumpsoverthelazydog".to_string(), "thelazydogjumpsoveraquickbrownfox".to_string()), true);
}

#[test]
fn test_56() {
    assert_eq!(Solution::is_anagram("pythonprogramming".to_string(), "nohtypgnimmargorp".to_string()), true);
}

#[test]
fn test_57() {
    assert_eq!(Solution::is_anagram("forty five".to_string(), "over fifty".to_string()), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::is_anagram("a!@#b$%^c&*()".to_string(), "c&*()b$%^a!@#".to_string()), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::is_anagram("aquickbrownfoxjumpsoverthelazydog".to_string(), "quickbrownfoxjumpsoverthelazygod".to_string()), false);
}

#[test]
fn test_60() {
    assert_eq!(Solution::is_anagram("noon".to_string(), "noon".to_string()), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::is_anagram("anagrammatic".to_string(), "icnagarammat".to_string()), true);
}

#[test]
fn test_62() {
    assert_eq!(Solution::is_anagram("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::is_anagram("abacabadabacaba".to_string(), "bacabacabacaba".to_string()), false);
}

#[test]
fn test_64() {
    assert_eq!(Solution::is_anagram("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "zzyyxxwwvvuuttssrrqqppoonnmmllkkjjiihhggeeffddeebbaa".to_string()), false);
}

#[test]
fn test_65() {
    assert_eq!(Solution::is_anagram("the quick brown fox jumps over the lazy dog".to_string(), "dog lazy the over jumps fox brown quick the".to_string()), true);
}

#[test]
fn test_66() {
    assert_eq!(Solution::is_anagram("conversation".to_string(), "voices rant on".to_string()), false);
}

#[test]
fn test_67() {
    assert_eq!(Solution::is_anagram("eleven plus two".to_string(), "twelve plus one".to_string()), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::is_anagram("the eyes".to_string(), "they see".to_string()), true);
}

#[test]
fn test_69() {
    assert_eq!(Solution::is_anagram("conversation".to_string(), "voicesranton".to_string()), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::is_anagram("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "zzyyxxwwvvuuttssrrqqppoonnmmllkkjjiihhggeeffddccbbbaa".to_string()), false);
}

#[test]
fn test_71() {
    assert_eq!(Solution::is_anagram("anagramanagram".to_string(), "nagaramnagaram".to_string()), true);
}

#[test]
fn test_72() {
    assert_eq!(Solution::is_anagram("this is a test".to_string(), "test a is this".to_string()), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::is_anagram("night".to_string(), "thing".to_string()), true);
}

#[test]
fn test_74() {
    assert_eq!(Solution::is_anagram("aaabbbccc".to_string(), "bbbaaacc".to_string()), false);
}

#[test]
fn test_75() {
    assert_eq!(Solution::is_anagram("samecharacters".to_string(), "charactersames".to_string()), true);
}

#[test]
fn test_76() {
    assert_eq!(Solution::is_anagram("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "zzyxwvuttsrqponmlkjihgfeddcbaabbccddeeffgghhiijjkkllmmnnooppqqrrss".to_string()), false);
}

#[test]
fn test_77() {
    assert_eq!(Solution::is_anagram("mississippi".to_string(), "ssmissippii".to_string()), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::is_anagram("abcabcabcabcabcabc".to_string(), "abcabcabcabcabcabc".to_string()), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::is_anagram("xxyyzz".to_string(), "zzxxyy".to_string()), true);
}

#[test]
fn test_80() {
    assert_eq!(Solution::is_anagram("thequickbrownfoxjumpsoverthelazydog".to_string(), "thequickbrownfoxjumpsoverthelazygod".to_string()), true);
}

#[test]
fn test_81() {
    assert_eq!(Solution::is_anagram("elevenpluszwo".to_string(), "twelvezillion".to_string()), false);
}

#[test]
fn test_82() {
    assert_eq!(Solution::is_anagram("notanagramhere".to_string(), "anagramherenot".to_string()), true);
}

#[test]
fn test_83() {
    assert_eq!(Solution::is_anagram("uniqueanagram".to_string(), "nagramuniqueanagram".to_string()), false);
}

#[test]
fn test_84() {
    assert_eq!(Solution::is_anagram("fluster".to_string(), "restful".to_string()), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::is_anagram("dormitory".to_string(), "dirty room".to_string()), false);
}

#[test]
fn test_86() {
    assert_eq!(Solution::is_anagram("aaaaaa".to_string(), "aaaaa".to_string()), false);
}

#[test]
fn test_87() {
    assert_eq!(Solution::is_anagram("punishments".to_string(), "ninepunishment".to_string()), false);
}

#[test]
fn test_88() {
    assert_eq!(Solution::is_anagram("thirty".to_string(), "typhrirt".to_string()), false);
}

#[test]
fn test_89() {
    assert_eq!(Solution::is_anagram("racecar".to_string(), "carrace".to_string()), true);
}

#[test]
fn test_90() {
    assert_eq!(Solution::is_anagram("ab".to_string(), "aabb".to_string()), false);
}

#[test]
fn test_91() {
    assert_eq!(Solution::is_anagram("a".to_string(), "".to_string()), false);
}

#[test]
fn test_92() {
    assert_eq!(Solution::is_anagram("qwertyuiopasdfghjklzxcvbnm".to_string(), "mnbvcxzlkjhgfdsapoiuytrewq".to_string()), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::is_anagram("abacax".to_string(), "aacxab".to_string()), true);
}

#[test]
fn test_94() {
    assert_eq!(Solution::is_anagram("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_string(), "zyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcba".to_string()), true);
}

#[test]
fn test_95() {
    assert_eq!(Solution::is_anagram("questionnaire".to_string(), "reinquirequest".to_string()), false);
}

#[test]
fn test_96() {
    assert_eq!(Solution::is_anagram("anagramatically".to_string(), "gramaticallyanagram".to_string()), false);
}

#[test]
fn test_97() {
    assert_eq!(Solution::is_anagram("uniquecharacters".to_string(), "uniquecharactersx".to_string()), false);
}

#[test]
fn test_98() {
    assert_eq!(Solution::is_anagram("abcdabcdabcd".to_string(), "dcbaabcdabcd".to_string()), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::is_anagram("adobe".to_string(), "abode".to_string()), true);
}

#[test]
fn test_100() {
    assert_eq!(Solution::is_anagram("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "zzzyyxwwvvuuttssrrqqppoonnmlkkjjiihhggffeeeeddccbaab".to_string()), false);
}

#[test]
fn test_101() {
    assert_eq!(Solution::is_anagram("clint eastwood".to_string(), "old west action".to_string()), false);
}

#[test]
fn test_102() {
    assert_eq!(Solution::is_anagram("abcabcabcabc".to_string(), "cbacbacbacba".to_string()), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::is_anagram("schoolmaster".to_string(), "theclassroom".to_string()), true);
}

#[test]
fn test_104() {
    assert_eq!(Solution::is_anagram("embarrassing".to_string(), "backgroundsere".to_string()), false);
}

#[test]
fn test_105() {
    assert_eq!(Solution::is_anagram("racecar".to_string(), "racecar".to_string()), true);
}

#[test]
fn test_106() {
    assert_eq!(Solution::is_anagram("nematocysts".to_string(), "cytoplasmnets".to_string()), false);
}

#[test]
fn test_107() {
    assert_eq!(Solution::is_anagram("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), "zzyyxxwwvvuuttsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcbaa".to_string()), false);
}

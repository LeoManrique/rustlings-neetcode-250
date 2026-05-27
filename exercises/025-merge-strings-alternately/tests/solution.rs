include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::merge_alternately("a".to_string(), "b".to_string()), "ab".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::merge_alternately("abcd".to_string(), "pq".to_string()), "apbqcd".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::merge_alternately("ab".to_string(), "pqrs".to_string()), "apbqrs".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::merge_alternately("".to_string(), "uvw".to_string()), "uvw".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::merge_alternately("x".to_string(), "y".to_string()), "xy".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::merge_alternately("hello".to_string(), "world".to_string()), "hweolrllod".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::merge_alternately("a".to_string(), "bcd".to_string()), "abcd".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::merge_alternately("abcde".to_string(), "fgh".to_string()), "afbgchde".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::merge_alternately("abc".to_string(), "pqr".to_string()), "apbqcr".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::merge_alternately("abcd".to_string(), "e".to_string()), "aebcd".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::merge_alternately("xyz".to_string(), "".to_string()), "xyz".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::merge_alternately("abcdefghijklmnopqrstuvwxyz".to_string(), "".to_string()), "abcdefghijklmnopqrstuvwxyz".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::merge_alternately("a".to_string(), "bcdefghijklmnopqrstuvwxyz".to_string()), "abcdefghijklmnopqrstuvwxyz".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::merge_alternately("thisisateststring".to_string(), "anotheronehere".to_string()), "tahniostihseartoensethsetrreing".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::merge_alternately("aabbccdd".to_string(), "eeffgg".to_string()), "aeaebfbfcgcgdd".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::merge_alternately("complex".to_string(), "strings".to_string()), "csotmrpilnegxs".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::merge_alternately("xyz".to_string(), "abcde".to_string()), "xaybzcde".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::merge_alternately("abcdefgh".to_string(), "ijkl".to_string()), "aibjckdlefgh".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::merge_alternately("abcdefghijk".to_string(), "lmnopqrstuvwxyz".to_string()), "albmcndoepfqgrhsitjukvwxyz".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::merge_alternately("abcdefghij".to_string(), "klmnopqrstuvwxyz".to_string()), "akblcmdneofpgqhrisjtuvwxyz".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::merge_alternately("abcdefghijklmnopqrstuvwxy".to_string(), "z".to_string()), "azbcdefghijklmnopqrstuvwxy".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::merge_alternately("quick".to_string(), "brownfox".to_string()), "qburiocwknfox".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::merge_alternately("onetwothreefour".to_string(), "five".to_string()), "ofnievtewothreefour".to_string());
}

#[test]
fn test_24() {
    assert_eq!(Solution::merge_alternately("overlap".to_string(), "lapping".to_string()), "olvaeprplianpg".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::merge_alternately("".to_string(), "".to_string()), "".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::merge_alternately("python".to_string(), "java".to_string()), "pjyatvhaon".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::merge_alternately("abcdefgh".to_string(), "ijklmnopqrstuv".to_string()), "aibjckdlemfngohpqrstuv".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::merge_alternately("different".to_string(), "length".to_string()), "dliefnfgetrhent".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::merge_alternately("abcdefghij".to_string(), "klmnopqrst".to_string()), "akblcmdneofpgqhrisjt".to_string());
}

#[test]
fn test_30() {
    assert_eq!(Solution::merge_alternately("abcdabcdabcd".to_string(), "zyxwzyxwzyxw".to_string()), "azbycxdwazbycxdwazbycxdw".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::merge_alternately("longerfirstword".to_string(), "short".to_string()), "lsohnogretrfirstword".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::merge_alternately("abcdefg".to_string(), "hijklmnop".to_string()), "ahbicjdkelfmgnop".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::merge_alternately("abcdef".to_string(), "ghijklmnopqrstuvwxyz".to_string()), "agbhcidjekflmnopqrstuvwxyz".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::merge_alternately("xyz".to_string(), "abcdefg".to_string()), "xaybzcdefg".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::merge_alternately("abcdabcdabcd".to_string(), "efghefghefghefgh".to_string()), "aebfcgdhaebfcgdhaebfcgdhefgh".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::merge_alternately("short".to_string(), "longersecondword".to_string()), "slhoonrgtersecondword".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::merge_alternately("".to_string(), "notempty".to_string()), "notempty".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::merge_alternately("notempty".to_string(), "".to_string()), "notempty".to_string());
}

#[test]
fn test_39() {
    assert_eq!(Solution::merge_alternately("".to_string(), "abcdefghijklmnopqrstuvwxyz".to_string()), "abcdefghijklmnopqrstuvwxyz".to_string());
}

#[test]
fn test_40() {
    assert_eq!(Solution::merge_alternately("aabbccddeeff".to_string(), "gggghhhhiiii".to_string()), "agagbgbgchchdhdheieififi".to_string());
}

#[test]
fn test_41() {
    assert_eq!(Solution::merge_alternately("".to_string(), "abcde".to_string()), "abcde".to_string());
}

#[test]
fn test_42() {
    assert_eq!(Solution::merge_alternately("a1b2c3d4".to_string(), "e5f6g7".to_string()), "ae15bf26cg37d4".to_string());
}

#[test]
fn test_43() {
    assert_eq!(Solution::merge_alternately("oddlength".to_string(), "evenlengths".to_string()), "oedvdelnelnegntghths".to_string());
}

#[test]
fn test_44() {
    assert_eq!(Solution::merge_alternately("same".to_string(), "size".to_string()), "ssaimzee".to_string());
}

#[test]
fn test_45() {
    assert_eq!(Solution::merge_alternately("abcdefg".to_string(), "hijklmnopqrstuvwxyz".to_string()), "ahbicjdkelfmgnopqrstuvwxyz".to_string());
}

#[test]
fn test_46() {
    assert_eq!(Solution::merge_alternately("aabbccddeeffgghhiijj".to_string(), "kkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string()), "akakblblcmcmdndneoeofpfpgqgqhrhrisisjtjtuuvvwwxxyyzz".to_string());
}

#[test]
fn test_47() {
    assert_eq!(Solution::merge_alternately("hello".to_string(), "worldwide".to_string()), "hweolrllodwide".to_string());
}

#[test]
fn test_48() {
    assert_eq!(Solution::merge_alternately("xyzxyzxyz".to_string(), "abcabcabc".to_string()), "xaybzcxaybzcxaybzc".to_string());
}

#[test]
fn test_49() {
    assert_eq!(Solution::merge_alternately("abcdxyz".to_string(), "efghwvu".to_string()), "aebfcgdhxwyvzu".to_string());
}

#[test]
fn test_50() {
    assert_eq!(Solution::merge_alternately("short".to_string(), "averylongstring".to_string()), "sahvoerrtylongstring".to_string());
}

#[test]
fn test_51() {
    assert_eq!(Solution::merge_alternately("hello".to_string(), "worldthisisaverylongstring".to_string()), "hweolrllodthisisaverylongstring".to_string());
}

#[test]
fn test_52() {
    assert_eq!(Solution::merge_alternately("abcdefg".to_string(), "hijkl".to_string()), "ahbicjdkelfg".to_string());
}

#[test]
fn test_53() {
    assert_eq!(Solution::merge_alternately("abcdef".to_string(), "ghijklmnop".to_string()), "agbhcidjekflmnop".to_string());
}

#[test]
fn test_54() {
    assert_eq!(Solution::merge_alternately("abcdabcdabcd".to_string(), "xyzxyzxyzxyz".to_string()), "axbyczdxaybzcxdyazbxcydz".to_string());
}

#[test]
fn test_55() {
    assert_eq!(Solution::merge_alternately("averyveryverylongword".to_string(), "short".to_string()), "asvheorrytveryverylongword".to_string());
}

#[test]
fn test_56() {
    assert_eq!(Solution::merge_alternately("longerword".to_string(), "short".to_string()), "lsohnogretrword".to_string());
}

#[test]
fn test_57() {
    assert_eq!(Solution::merge_alternately("hello".to_string(), "world!".to_string()), "hweolrllod!".to_string());
}

#[test]
fn test_58() {
    assert_eq!(Solution::merge_alternately("nonempty".to_string(), "".to_string()), "nonempty".to_string());
}

#[test]
fn test_59() {
    assert_eq!(Solution::merge_alternately("longstringwithmorecharacters".to_string(), "shortstr".to_string()), "lsohnogrsttsrtirngwithmorecharacters".to_string());
}

#[test]
fn test_60() {
    assert_eq!(Solution::merge_alternately("abcdef".to_string(), "zyxwvu".to_string()), "azbycxdwevfu".to_string());
}

#[test]
fn test_61() {
    assert_eq!(Solution::merge_alternately("onlyone".to_string(), "".to_string()), "onlyone".to_string());
}

#[test]
fn test_62() {
    assert_eq!(Solution::merge_alternately("xyz".to_string(), "uvw".to_string()), "xuyvzw".to_string());
}

#[test]
fn test_63() {
    assert_eq!(Solution::merge_alternately("aabbccddeeff".to_string(), "zzzzyyyxxww".to_string()), "azazbzbzcycydydxexewfwf".to_string());
}

#[test]
fn test_64() {
    assert_eq!(Solution::merge_alternately("abcdefgh".to_string(), "ijklmnop".to_string()), "aibjckdlemfngohp".to_string());
}

#[test]
fn test_65() {
    assert_eq!(Solution::merge_alternately("onetwothree".to_string(), "four".to_string()), "ofnoeutrwothree".to_string());
}

#[test]
fn test_66() {
    assert_eq!(Solution::merge_alternately("aabbcc".to_string(), "ddeeff".to_string()), "adadbebecfcf".to_string());
}

#[test]
fn test_67() {
    assert_eq!(Solution::merge_alternately("aabbccddeeffgghhiijj".to_string(), "zzzzzzzzzz".to_string()), "azazbzbzczczdzdzezezffgghhiijj".to_string());
}

#[test]
fn test_68() {
    assert_eq!(Solution::merge_alternately("abacaxi".to_string(), "manga".to_string()), "ambaancgaaxi".to_string());
}

#[test]
fn test_69() {
    assert_eq!(Solution::merge_alternately("onetwothreefour".to_string(), "fivesix".to_string()), "ofnievtewsoitxhreefour".to_string());
}

#[test]
fn test_70() {
    assert_eq!(Solution::merge_alternately("one".to_string(), "twothreefour".to_string()), "otnweothreefour".to_string());
}

#[test]
fn test_71() {
    assert_eq!(Solution::merge_alternately("alphanumeric123".to_string(), "characters!@#".to_string()), "aclhpahraancutmeerrsi!c@1#23".to_string());
}

#[test]
fn test_72() {
    assert_eq!(Solution::merge_alternately("".to_string(), "nonempty".to_string()), "nonempty".to_string());
}

#[test]
fn test_73() {
    assert_eq!(Solution::merge_alternately("xyz".to_string(), "wvutsrqponmlkjihgfedcba".to_string()), "xwyvzutsrqponmlkjihgfedcba".to_string());
}

#[test]
fn test_74() {
    assert_eq!(Solution::merge_alternately("abcde".to_string(), "".to_string()), "abcde".to_string());
}

#[test]
fn test_75() {
    assert_eq!(Solution::merge_alternately("short".to_string(), "averyverylongwordindeed".to_string()), "sahvoerrtyverylongwordindeed".to_string());
}

#[test]
fn test_76() {
    assert_eq!(Solution::merge_alternately("xy".to_string(), "abcdefghijk".to_string()), "xaybcdefghijk".to_string());
}

#[test]
fn test_77() {
    assert_eq!(Solution::merge_alternately("onetwothree".to_string(), "fourfivesix".to_string()), "ofnoeutrwfoitvherseiex".to_string());
}

#[test]
fn test_78() {
    assert_eq!(Solution::merge_alternately("python".to_string(), "programming".to_string()), "ppyrtohgornamming".to_string());
}

#[test]
fn test_79() {
    assert_eq!(Solution::merge_alternately("verylongwordthatgoesonandone".to_string(), "short".to_string()), "vsehroyrltongwordthatgoesonandone".to_string());
}

#[test]
fn test_80() {
    assert_eq!(Solution::merge_alternately("xyz".to_string(), "abcdef".to_string()), "xaybzcdef".to_string());
}

#[test]
fn test_81() {
    assert_eq!(Solution::merge_alternately("merge".to_string(), "these".to_string()), "mtehregsee".to_string());
}

#[test]
fn test_82() {
    assert_eq!(Solution::merge_alternately("abcdefghijklmnopqrstuvwxyz".to_string(), "z".to_string()), "azbcdefghijklmnopqrstuvwxyz".to_string());
}

#[test]
fn test_83() {
    assert_eq!(Solution::merge_alternately("xyzlmnop".to_string(), "qrstuvwx".to_string()), "xqyrzsltmunvowpx".to_string());
}

#[test]
fn test_84() {
    assert_eq!(Solution::merge_alternately("abcdefgh".to_string(), "ijklmno".to_string()), "aibjckdlemfngoh".to_string());
}

#[test]
fn test_85() {
    assert_eq!(Solution::merge_alternately("short".to_string(), "averyveryverylongword".to_string()), "sahvoerrtyveryverylongword".to_string());
}

#[test]
fn test_86() {
    assert_eq!(Solution::merge_alternately("averyverylongwordindeed".to_string(), "short".to_string()), "asvheorrytverylongwordindeed".to_string());
}

#[test]
fn test_87() {
    assert_eq!(Solution::merge_alternately("equal".to_string(), "equal".to_string()), "eeqquuaall".to_string());
}

#[test]
fn test_88() {
    assert_eq!(Solution::merge_alternately("ab".to_string(), "cd".to_string()), "acbd".to_string());
}

#[test]
fn test_89() {
    assert_eq!(Solution::merge_alternately("short".to_string(), "averylongstringthatweneedtocheck".to_string()), "sahvoerrtylongstringthatweneedtocheck".to_string());
}

#[test]
fn test_90() {
    assert_eq!(Solution::merge_alternately("same".to_string(), "same".to_string()), "ssaammee".to_string());
}

#[test]
fn test_91() {
    assert_eq!(Solution::merge_alternately("abcd".to_string(), "".to_string()), "abcd".to_string());
}

#[test]
fn test_92() {
    assert_eq!(Solution::merge_alternately("thisisaverylongstring".to_string(), "hello".to_string()), "thheilsliosaverylongstring".to_string());
}

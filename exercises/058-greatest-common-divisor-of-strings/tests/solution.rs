include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::gcd_of_strings("ABAB".to_string(), "ABA".to_string()), "".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::gcd_of_strings("ABCDEFGH".to_string(), "XYZ".to_string()), "".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()), "ABC".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::gcd_of_strings("ABABAB".to_string(), "ABA".to_string()), "".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()), "AB".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::gcd_of_strings("ABCABCABC".to_string(), "ABCABC".to_string()), "ABC".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::gcd_of_strings("ABCD".to_string(), "EFGH".to_string()), "".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::gcd_of_strings("XYZXYZXYZ".to_string(), "XYZXYZ".to_string()), "XYZ".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::gcd_of_strings("AAAAAAAAAA".to_string(), "AAAAAAAAAA".to_string()), "AAAAAAAAAA".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::gcd_of_strings("GCDGCDGCD".to_string(), "GCD".to_string()), "GCD".to_string());
}

#[test]
fn test_11() {
    assert_eq!(Solution::gcd_of_strings("ABCD".to_string(), "ABCD".to_string()), "ABCD".to_string());
}

#[test]
fn test_12() {
    assert_eq!(Solution::gcd_of_strings("HELLOHELLO".to_string(), "HELLO".to_string()), "HELLO".to_string());
}

#[test]
fn test_13() {
    assert_eq!(Solution::gcd_of_strings("A".to_string(), "A".to_string()), "A".to_string());
}

#[test]
fn test_14() {
    assert_eq!(Solution::gcd_of_strings("PQRS".to_string(), "PQRS".to_string()), "PQRS".to_string());
}

#[test]
fn test_15() {
    assert_eq!(Solution::gcd_of_strings("AAAA".to_string(), "AA".to_string()), "AA".to_string());
}

#[test]
fn test_16() {
    assert_eq!(Solution::gcd_of_strings("ABCDEFG".to_string(), "BCD".to_string()), "".to_string());
}

#[test]
fn test_17() {
    assert_eq!(Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()), "".to_string());
}

#[test]
fn test_18() {
    assert_eq!(Solution::gcd_of_strings("HELLOHELLOHELLO".to_string(), "HELLO".to_string()), "HELLO".to_string());
}

#[test]
fn test_19() {
    assert_eq!(Solution::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string()), "".to_string());
}

#[test]
fn test_20() {
    assert_eq!(Solution::gcd_of_strings("ABABABAB".to_string(), "BABA".to_string()), "".to_string());
}

#[test]
fn test_21() {
    assert_eq!(Solution::gcd_of_strings("AAAAAAAAA".to_string(), "AAAAA".to_string()), "A".to_string());
}

#[test]
fn test_22() {
    assert_eq!(Solution::gcd_of_strings("TATATATA".to_string(), "TAT".to_string()), "".to_string());
}

#[test]
fn test_23() {
    assert_eq!(Solution::gcd_of_strings("ABCABCABC".to_string(), "ABC".to_string()), "ABC".to_string());
}

#[test]
fn test_24() {
    assert_eq!(Solution::gcd_of_strings("ABABABAB".to_string(), "ABAB".to_string()), "ABAB".to_string());
}

#[test]
fn test_25() {
    assert_eq!(Solution::gcd_of_strings("BANANABANANABANANABANANA".to_string(), "BANANABANANA".to_string()), "BANANABANANA".to_string());
}

#[test]
fn test_26() {
    assert_eq!(Solution::gcd_of_strings("XYXYXYXY".to_string(), "XYXY".to_string()), "XYXY".to_string());
}

#[test]
fn test_27() {
    assert_eq!(Solution::gcd_of_strings("DOUBLEDOUBLEDOUBLE".to_string(), "DOUBLEDOUBLE".to_string()), "DOUBLE".to_string());
}

#[test]
fn test_28() {
    assert_eq!(Solution::gcd_of_strings("SIMILARITYSIMILARITY".to_string(), "SIMILARITY".to_string()), "SIMILARITY".to_string());
}

#[test]
fn test_29() {
    assert_eq!(Solution::gcd_of_strings("REPREPREP".to_string(), "REPRE".to_string()), "".to_string());
}

#[test]
fn test_30() {
    assert_eq!(Solution::gcd_of_strings("ABACABACABAC".to_string(), "ABAC".to_string()), "ABAC".to_string());
}

#[test]
fn test_31() {
    assert_eq!(Solution::gcd_of_strings("DOGDOGDOGDOGDOGDOG".to_string(), "DOGDOGDOG".to_string()), "DOGDOGDOG".to_string());
}

#[test]
fn test_32() {
    assert_eq!(Solution::gcd_of_strings("XYZXYZXYZXYZXYZXYZ".to_string(), "XYZXYZXYZXYZ".to_string()), "XYZXYZ".to_string());
}

#[test]
fn test_33() {
    assert_eq!(Solution::gcd_of_strings("PPPPPPPPPPPPPPPP".to_string(), "PPPPPP".to_string()), "PP".to_string());
}

#[test]
fn test_34() {
    assert_eq!(Solution::gcd_of_strings("MULTIPLEMULTIPLEMULTIPLE".to_string(), "MULTIPLEMULTIPLE".to_string()), "MULTIPLE".to_string());
}

#[test]
fn test_35() {
    assert_eq!(Solution::gcd_of_strings("MIXEDCASEMIXEDCASE".to_string(), "MIXEDCASE".to_string()), "MIXEDCASE".to_string());
}

#[test]
fn test_36() {
    assert_eq!(Solution::gcd_of_strings("XYXYXYXYXYXYXYXY".to_string(), "XYXYXY".to_string()), "XY".to_string());
}

#[test]
fn test_37() {
    assert_eq!(Solution::gcd_of_strings("ANTEANTEANTEANTEANTEANTEANTEANTEANTE".to_string(), "ANTEANTEANTEANTE".to_string()), "ANTE".to_string());
}

#[test]
fn test_38() {
    assert_eq!(Solution::gcd_of_strings("QWQWQWQWQWQW".to_string(), "QWQW".to_string()), "QWQW".to_string());
}

#[test]
fn test_39() {
    assert_eq!(Solution::gcd_of_strings("PINEAPPLEPINEAPPLEPINEAPPLEPINEAPPLE".to_string(), "PINEAPPLEPINEAPPLE".to_string()), "PINEAPPLEPINEAPPLE".to_string());
}

#[test]
fn test_40() {
    assert_eq!(Solution::gcd_of_strings("MNMNMNMN".to_string(), "MNMN".to_string()), "MNMN".to_string());
}

#[test]
fn test_41() {
    assert_eq!(Solution::gcd_of_strings("XYZXYZXYZXYZXYZ".to_string(), "XYZXYZXYZ".to_string()), "XYZ".to_string());
}

#[test]
fn test_42() {
    assert_eq!(Solution::gcd_of_strings("REPORTEPORTEPORTE".to_string(), "REPORTEPORTE".to_string()), "".to_string());
}

#[test]
fn test_43() {
    assert_eq!(Solution::gcd_of_strings("RRRRRRRRRRRR".to_string(), "RRRRRR".to_string()), "RRRRRR".to_string());
}

#[test]
fn test_44() {
    assert_eq!(Solution::gcd_of_strings("DIFFERENTLENGTHDIFFERENTLENGTH".to_string(), "DIFFERENTLENGTH".to_string()), "DIFFERENTLENGTH".to_string());
}

#[test]
fn test_45() {
    assert_eq!(Solution::gcd_of_strings("SMALLSAME".to_string(), "SAME".to_string()), "".to_string());
}

#[test]
fn test_46() {
    assert_eq!(Solution::gcd_of_strings("MIXEDUPMIXEDUP".to_string(), "MIXEDUP".to_string()), "MIXEDUP".to_string());
}

#[test]
fn test_47() {
    assert_eq!(Solution::gcd_of_strings("UPPERCASEUPPERCASEUPPERCASE".to_string(), "UPPERCASEUPPERCASE".to_string()), "UPPERCASE".to_string());
}

#[test]
fn test_48() {
    assert_eq!(Solution::gcd_of_strings("LONGSTRINGLONGSTRINGLONGSTRINGLONGSTRING".to_string(), "LONGSTRINGLONGSTRING".to_string()), "LONGSTRINGLONGSTRING".to_string());
}

#[test]
fn test_49() {
    assert_eq!(Solution::gcd_of_strings("LONGSTRINGLONGSTRINGLONGSTRING".to_string(), "LONGSTRINGLONG".to_string()), "".to_string());
}

#[test]
fn test_50() {
    assert_eq!(Solution::gcd_of_strings("SIMPLESIMPLESIMPLE".to_string(), "SIMPLESIMPLE".to_string()), "SIMPLE".to_string());
}

#[test]
fn test_51() {
    assert_eq!(Solution::gcd_of_strings("AAAAABBBB".to_string(), "AAAAA".to_string()), "".to_string());
}

#[test]
fn test_52() {
    assert_eq!(Solution::gcd_of_strings("SHORT".to_string(), "VERYLONGSTRINGTHATISNOTAREPEAT".to_string()), "".to_string());
}

#[test]
fn test_53() {
    assert_eq!(Solution::gcd_of_strings("AB".to_string(), "BA".to_string()), "".to_string());
}

#[test]
fn test_54() {
    assert_eq!(Solution::gcd_of_strings("PRIMEPRIMEPRIME".to_string(), "PRIMEPRIME".to_string()), "PRIME".to_string());
}

#[test]
fn test_55() {
    assert_eq!(Solution::gcd_of_strings("MNMNMNMNMNMN".to_string(), "MNMN".to_string()), "MNMN".to_string());
}

#[test]
fn test_56() {
    assert_eq!(Solution::gcd_of_strings("BEEBEEBEEBEEBEEBEEBEEBEE".to_string(), "BEEBEEBEEBEE".to_string()), "BEEBEEBEEBEE".to_string());
}

#[test]
fn test_57() {
    assert_eq!(Solution::gcd_of_strings("GRAPEGRAPEGRAPEGRAPEGRAPEGRAPE".to_string(), "GRAPEGRAPEGRAPE".to_string()), "GRAPEGRAPEGRAPE".to_string());
}

#[test]
fn test_58() {
    assert_eq!(Solution::gcd_of_strings("TIGERTIGERTIGERTIGERTIGERTIGER".to_string(), "TIGERTIGERTIGER".to_string()), "TIGERTIGERTIGER".to_string());
}

#[test]
fn test_59() {
    assert_eq!(Solution::gcd_of_strings("SCIENCE".to_string(), "SCIENCE".to_string()), "SCIENCE".to_string());
}

#[test]
fn test_60() {
    assert_eq!(Solution::gcd_of_strings("ABCDEFGABCDEFG".to_string(), "ABCDEFG".to_string()), "ABCDEFG".to_string());
}

#[test]
fn test_61() {
    assert_eq!(Solution::gcd_of_strings("XYZXYZXYZXYZXYZXYZXYZXYZ".to_string(), "XYZXYZXYZ".to_string()), "XYZ".to_string());
}

#[test]
fn test_62() {
    assert_eq!(Solution::gcd_of_strings("MIXEDMIXEDMIXEDMIXED".to_string(), "MIXEDMIXED".to_string()), "MIXEDMIXED".to_string());
}

#[test]
fn test_63() {
    assert_eq!(Solution::gcd_of_strings("ALMOSTSAMEALMOSTSAME".to_string(), "ALMOSTSAME".to_string()), "ALMOSTSAME".to_string());
}

#[test]
fn test_64() {
    assert_eq!(Solution::gcd_of_strings("APPLEAPPLEAPPLEAPPLEAPPLE".to_string(), "APPLEAPPLEAPPLE".to_string()), "APPLE".to_string());
}

#[test]
fn test_65() {
    assert_eq!(Solution::gcd_of_strings("MIXMIXMIXMIX".to_string(), "MIXMIX".to_string()), "MIXMIX".to_string());
}

#[test]
fn test_66() {
    assert_eq!(Solution::gcd_of_strings("MATHMATHMATHMATH".to_string(), "MATH".to_string()), "MATH".to_string());
}

#[test]
fn test_67() {
    assert_eq!(Solution::gcd_of_strings("SIMPLESIMPLESIMPLESIMPLE".to_string(), "SIMPLE".to_string()), "SIMPLE".to_string());
}

#[test]
fn test_68() {
    assert_eq!(Solution::gcd_of_strings("DUCKDUCKDUCKDUCKDUCKDUCKDUCKDUCK".to_string(), "DUCKDUCKDUCKDUCK".to_string()), "DUCKDUCKDUCKDUCK".to_string());
}

#[test]
fn test_69() {
    assert_eq!(Solution::gcd_of_strings("LONGLONGLONGLONGLONGLONG".to_string(), "LONGLONGLONG".to_string()), "LONGLONGLONG".to_string());
}

#[test]
fn test_70() {
    assert_eq!(Solution::gcd_of_strings("ZAZAZAZAZA".to_string(), "ZAZA".to_string()), "ZA".to_string());
}

#[test]
fn test_71() {
    assert_eq!(Solution::gcd_of_strings("AABAAABAAB".to_string(), "AAB".to_string()), "".to_string());
}

#[test]
fn test_72() {
    assert_eq!(Solution::gcd_of_strings("ABCDEFABCDEF".to_string(), "ABCDEF".to_string()), "ABCDEF".to_string());
}

#[test]
fn test_73() {
    assert_eq!(Solution::gcd_of_strings("PATTERNPATTERNPATTERN".to_string(), "PATTERNPATTERN".to_string()), "PATTERN".to_string());
}

#[test]
fn test_74() {
    assert_eq!(Solution::gcd_of_strings("UNIQUEUNIQUEUNIQUEUNIQUEUNIQUE".to_string(), "UNIQUEUNIQUE".to_string()), "UNIQUE".to_string());
}

#[test]
fn test_75() {
    assert_eq!(Solution::gcd_of_strings("A".to_string(), "".to_string()), "".to_string());
}

#[test]
fn test_76() {
    assert_eq!(Solution::gcd_of_strings("WOWOWOWOWOWO".to_string(), "WOWOWO".to_string()), "WOWOWO".to_string());
}

#[test]
fn test_77() {
    assert_eq!(Solution::gcd_of_strings("HELLOHELLOHELLOHELLO".to_string(), "HELLOHELLOHELLO".to_string()), "HELLO".to_string());
}

#[test]
fn test_78() {
    assert_eq!(Solution::gcd_of_strings("REPEATREPEATREPEATREPEATREPEAT".to_string(), "REPEATREPEAT".to_string()), "REPEAT".to_string());
}

#[test]
fn test_79() {
    assert_eq!(Solution::gcd_of_strings("IDENTICALIDENTICALIDENTICAL".to_string(), "IDENTICALIDENTICAL".to_string()), "IDENTICAL".to_string());
}

#[test]
fn test_80() {
    assert_eq!(Solution::gcd_of_strings("ABCDEFGABCDEFG".to_string(), "DEFGABCD".to_string()), "".to_string());
}

#[test]
fn test_81() {
    assert_eq!(Solution::gcd_of_strings("SAMESTRINGSAME".to_string(), "SAME".to_string()), "".to_string());
}

#[test]
fn test_82() {
    assert_eq!(Solution::gcd_of_strings("REPEATREPEATREPEAT".to_string(), "REPEATREPEAT".to_string()), "REPEAT".to_string());
}

#[test]
fn test_83() {
    assert_eq!(Solution::gcd_of_strings("QQQQQQQQQQ".to_string(), "QQQQ".to_string()), "QQ".to_string());
}

#[test]
fn test_84() {
    assert_eq!(Solution::gcd_of_strings("FAMILYFAMILYFAMILY".to_string(), "FAMILYFAMILY".to_string()), "FAMILY".to_string());
}

#[test]
fn test_85() {
    assert_eq!(Solution::gcd_of_strings("ABCABCABCABCABCABC".to_string(), "ABCABCABC".to_string()), "ABCABCABC".to_string());
}

#[test]
fn test_86() {
    assert_eq!(Solution::gcd_of_strings("".to_string(), "A".to_string()), "".to_string());
}

#[test]
fn test_87() {
    assert_eq!(Solution::gcd_of_strings("UNIQUEUNIQUEUNIQUEUNIQUE".to_string(), "UNIQUEUNIQUE".to_string()), "UNIQUEUNIQUE".to_string());
}

#[test]
fn test_88() {
    assert_eq!(Solution::gcd_of_strings("LONGSTRINGLONGSTRING".to_string(), "LONGSTRING".to_string()), "LONGSTRING".to_string());
}

#[test]
fn test_89() {
    assert_eq!(Solution::gcd_of_strings("UNIQUEUNIQUE".to_string(), "UNIQUEUNIQUEUNIQUE".to_string()), "UNIQUE".to_string());
}

#[test]
fn test_90() {
    assert_eq!(Solution::gcd_of_strings("MOUSEMOUSEMOUSEMOUSEMOUSEMOUSE".to_string(), "MOUSEMOUSE".to_string()), "MOUSEMOUSE".to_string());
}

#[test]
fn test_91() {
    assert_eq!(Solution::gcd_of_strings("ABABABABABAB".to_string(), "ABAB".to_string()), "ABAB".to_string());
}

#[test]
fn test_92() {
    assert_eq!(Solution::gcd_of_strings("HELLOWORLDHELLOWORLD".to_string(), "HELLOWORLD".to_string()), "HELLOWORLD".to_string());
}

#[test]
fn test_93() {
    assert_eq!(Solution::gcd_of_strings("ABCDABCD".to_string(), "ABCDABCDABCD".to_string()), "ABCD".to_string());
}

#[test]
fn test_94() {
    assert_eq!(Solution::gcd_of_strings("SAMESAMESAME".to_string(), "SAME".to_string()), "SAME".to_string());
}

#[test]
fn test_95() {
    assert_eq!(Solution::gcd_of_strings("QRSTQRSTQRST".to_string(), "QRSTQRST".to_string()), "QRST".to_string());
}

#[test]
fn test_96() {
    assert_eq!(Solution::gcd_of_strings("KARMAKARMAKARMAKARMA".to_string(), "KARMAKARMA".to_string()), "KARMAKARMA".to_string());
}

#[test]
fn test_97() {
    assert_eq!(Solution::gcd_of_strings("AAAAABAAAAAB".to_string(), "AAAAAB".to_string()), "AAAAAB".to_string());
}

#[test]
fn test_98() {
    assert_eq!(Solution::gcd_of_strings("TTT".to_string(), "TTTTTTT".to_string()), "T".to_string());
}

#[test]
fn test_99() {
    assert_eq!(Solution::gcd_of_strings("QQQQQQQQQQQQQQQQ".to_string(), "QQQQ".to_string()), "QQQQ".to_string());
}

#[test]
fn test_100() {
    assert_eq!(Solution::gcd_of_strings("AAAABBBBAAAABBBB".to_string(), "AAAABBBB".to_string()), "AAAABBBB".to_string());
}

#[test]
fn test_101() {
    assert_eq!(Solution::gcd_of_strings("QWQWQWQWQWQW".to_string(), "QWQWQW".to_string()), "QWQWQW".to_string());
}

#[test]
fn test_102() {
    assert_eq!(Solution::gcd_of_strings("PPPPPPPP".to_string(), "PPPP".to_string()), "PPPP".to_string());
}

#[test]
fn test_103() {
    assert_eq!(Solution::gcd_of_strings("SPAMSPAMSPAMSPAM".to_string(), "SPAMSPAM".to_string()), "SPAMSPAM".to_string());
}

#[test]
fn test_104() {
    assert_eq!(Solution::gcd_of_strings("PATTERNPATTERN".to_string(), "PATTERN".to_string()), "PATTERN".to_string());
}

#[test]
fn test_105() {
    assert_eq!(Solution::gcd_of_strings("XYZXYZXYZXYZXYZXYZ".to_string(), "XYZXYZ".to_string()), "XYZXYZ".to_string());
}

#[test]
fn test_106() {
    assert_eq!(Solution::gcd_of_strings("SCREENSCREENSCREENSCREEN".to_string(), "SCREEN".to_string()), "SCREEN".to_string());
}

#[test]
fn test_107() {
    assert_eq!(Solution::gcd_of_strings("PIZZAPIZZAPIZZAPIZZA".to_string(), "PIZZAPIZZA".to_string()), "PIZZAPIZZA".to_string());
}

#[test]
fn test_108() {
    assert_eq!(Solution::gcd_of_strings("ORANGEORANGEORANGEORANGEORANGEORANGEORANGEORANGE".to_string(), "ORANGEORANGEORANGE".to_string()), "ORANGE".to_string());
}

#[test]
fn test_109() {
    assert_eq!(Solution::gcd_of_strings("PRIMEPRIMEPRIMEPRIME".to_string(), "PRIMEPRIME".to_string()), "PRIMEPRIME".to_string());
}

#[test]
fn test_110() {
    assert_eq!(Solution::gcd_of_strings("REPEATREPEATREPEAT".to_string(), "REPEAT".to_string()), "REPEAT".to_string());
}

#[test]
fn test_111() {
    assert_eq!(Solution::gcd_of_strings("CODECODECODECODECODECODE".to_string(), "CODECODECODE".to_string()), "CODECODECODE".to_string());
}

#[test]
fn test_112() {
    assert_eq!(Solution::gcd_of_strings("PROJECTORPROJECTORPROJECTORPROJECTOR".to_string(), "PROJECTORPROJECTOR".to_string()), "PROJECTORPROJECTOR".to_string());
}

#[test]
fn test_113() {
    assert_eq!(Solution::gcd_of_strings("PYTHONPYTHONPYTHON".to_string(), "PYTHONPYTHON".to_string()), "PYTHON".to_string());
}

#[test]
fn test_114() {
    assert_eq!(Solution::gcd_of_strings("CCCCCCCCCCCCCC".to_string(), "CCCCCCCCC".to_string()), "C".to_string());
}

#[test]
fn test_115() {
    assert_eq!(Solution::gcd_of_strings("PQPQPQ".to_string(), "PQPQPQPQPQ".to_string()), "PQ".to_string());
}

#[test]
fn test_116() {
    assert_eq!(Solution::gcd_of_strings("DESKDESKDESKDESK".to_string(), "DESK".to_string()), "DESK".to_string());
}

#[test]
fn test_117() {
    assert_eq!(Solution::gcd_of_strings("GUITARGUITARGUITAR".to_string(), "GUITAR".to_string()), "GUITAR".to_string());
}

#[test]
fn test_118() {
    assert_eq!(Solution::gcd_of_strings("QWQWQWQWQW".to_string(), "QWQWQW".to_string()), "QW".to_string());
}

#[test]
fn test_119() {
    assert_eq!(Solution::gcd_of_strings("LONGLONGLONGLONGLONG".to_string(), "LONGLONGLONG".to_string()), "LONG".to_string());
}

#[test]
fn test_120() {
    assert_eq!(Solution::gcd_of_strings("DEVDEVDEVDEVDEV".to_string(), "DEVDEV".to_string()), "DEV".to_string());
}

#[test]
fn test_121() {
    assert_eq!(Solution::gcd_of_strings("UNIQUEUNIQUEUNIQUEUNIQUEUNIQUE".to_string(), "UNIQUEUNIQUEUNIQUE".to_string()), "UNIQUE".to_string());
}

#[test]
fn test_122() {
    assert_eq!(Solution::gcd_of_strings("TREETREETREETREE".to_string(), "TREETREE".to_string()), "TREETREE".to_string());
}

#[test]
fn test_123() {
    assert_eq!(Solution::gcd_of_strings("XYZXYZXYZXYZXYZXYZ".to_string(), "XYZXYZXYZ".to_string()), "XYZXYZXYZ".to_string());
}

#[test]
fn test_124() {
    assert_eq!(Solution::gcd_of_strings("HELLOHELLOHELLOHELLOHELLO".to_string(), "HELLOHELLOHELLO".to_string()), "HELLO".to_string());
}

#[test]
fn test_125() {
    assert_eq!(Solution::gcd_of_strings("SMALL".to_string(), "SMALLSMALLSMALL".to_string()), "SMALL".to_string());
}

#[test]
fn test_126() {
    assert_eq!(Solution::gcd_of_strings("MNMNMNMNMN".to_string(), "MNMN".to_string()), "MN".to_string());
}

#[test]
fn test_127() {
    assert_eq!(Solution::gcd_of_strings("ABCABCABCABC".to_string(), "ABCABC".to_string()), "ABCABC".to_string());
}

#[test]
fn test_128() {
    assert_eq!(Solution::gcd_of_strings("LONGERSTRINGLONGERSTRINGLONGERSTRING".to_string(), "LONGERSTRINGLONGERSTRING".to_string()), "LONGERSTRING".to_string());
}

#[test]
fn test_129() {
    assert_eq!(Solution::gcd_of_strings("PATTERNPATTERNPATTERNPATTERN".to_string(), "PATTERNPATTERN".to_string()), "PATTERNPATTERN".to_string());
}

#[test]
fn test_130() {
    assert_eq!(Solution::gcd_of_strings("HELLOHELLOHELLOHELLOHELLOHELLO".to_string(), "HELLOHELLOHELLOHELLOHELLO".to_string()), "HELLO".to_string());
}

#[test]
fn test_131() {
    assert_eq!(Solution::gcd_of_strings("DIFFERENTLENGTHS".to_string(), "SOMEOTHERSTRING".to_string()), "".to_string());
}

#[test]
fn test_132() {
    assert_eq!(Solution::gcd_of_strings("MIXEDUP".to_string(), "MIXEDUPMIXEDUP".to_string()), "MIXEDUP".to_string());
}

#[test]
fn test_133() {
    assert_eq!(Solution::gcd_of_strings("KEYBOARDKEYBOARDKEYBOARDKEYBOARD".to_string(), "KEYBOARDKEYBOARD".to_string()), "KEYBOARDKEYBOARD".to_string());
}

#[test]
fn test_134() {
    assert_eq!(Solution::gcd_of_strings("FLOWERFLOWERFLOWERFLOWER".to_string(), "FLOWERFLOWER".to_string()), "FLOWERFLOWER".to_string());
}

#[test]
fn test_135() {
    assert_eq!(Solution::gcd_of_strings("LONGSTRING".to_string(), "LONGSTRINGLONGSTRING".to_string()), "LONGSTRING".to_string());
}

#[test]
fn test_136() {
    assert_eq!(Solution::gcd_of_strings("SAMESTRINGSAMESTRINGSAMESTRING".to_string(), "SAMESTRINGSAMESTRING".to_string()), "SAMESTRING".to_string());
}

#[test]
fn test_137() {
    assert_eq!(Solution::gcd_of_strings("NONONONONONO".to_string(), "NONO".to_string()), "NONO".to_string());
}

#[test]
fn test_138() {
    assert_eq!(Solution::gcd_of_strings("ABCDABCDABCDABCD".to_string(), "ABCDABCD".to_string()), "ABCDABCD".to_string());
}

#[test]
fn test_139() {
    assert_eq!(Solution::gcd_of_strings("TESTTESTTESTTEST".to_string(), "TESTTEST".to_string()), "TESTTEST".to_string());
}

#[test]
fn test_140() {
    assert_eq!(Solution::gcd_of_strings("SAMELENGTH".to_string(), "DIFFERENT".to_string()), "".to_string());
}

#[test]
fn test_141() {
    assert_eq!(Solution::gcd_of_strings("HELLOHELLOHELLO".to_string(), "HELLOHELLO".to_string()), "HELLO".to_string());
}

#[test]
fn test_142() {
    assert_eq!(Solution::gcd_of_strings("DOGDOGDOGDOGDOG".to_string(), "DOGDOG".to_string()), "DOG".to_string());
}

#[test]
fn test_143() {
    assert_eq!(Solution::gcd_of_strings("CATCATCATCATCAT".to_string(), "CATCAT".to_string()), "CAT".to_string());
}

#[test]
fn test_144() {
    assert_eq!(Solution::gcd_of_strings("ABABABABAB".to_string(), "ABAB".to_string()), "AB".to_string());
}

#[test]
fn test_145() {
    assert_eq!(Solution::gcd_of_strings("LONGLONGLONGLONG".to_string(), "LONG".to_string()), "LONG".to_string());
}

#[test]
fn test_146() {
    assert_eq!(Solution::gcd_of_strings("COMPUTERCOMPUTERCOMPUTER".to_string(), "COMPUTER".to_string()), "COMPUTER".to_string());
}

#[test]
fn test_147() {
    assert_eq!(Solution::gcd_of_strings("RUBYRUBYRUBYRUBY".to_string(), "RUBYRUBY".to_string()), "RUBYRUBY".to_string());
}

#[test]
fn test_148() {
    assert_eq!(Solution::gcd_of_strings("BEEBEEBEEBEEBEE".to_string(), "BEEBEE".to_string()), "BEE".to_string());
}

#[test]
fn test_149() {
    assert_eq!(Solution::gcd_of_strings("ABABABABABAB".to_string(), "ABABAB".to_string()), "ABABAB".to_string());
}

#[test]
fn test_150() {
    assert_eq!(Solution::gcd_of_strings("BIGBIGBIGBIG".to_string(), "BIG".to_string()), "BIG".to_string());
}

#[test]
fn test_151() {
    assert_eq!(Solution::gcd_of_strings("APPLEAPPLEAPPLEAPPLEAPPLEAPPLE".to_string(), "APPLEAPPLEAPPLE".to_string()), "APPLEAPPLEAPPLE".to_string());
}

#[test]
fn test_152() {
    assert_eq!(Solution::gcd_of_strings("AABBCCDDEEFF".to_string(), "AABBCCDD".to_string()), "".to_string());
}

#[test]
fn test_153() {
    assert_eq!(Solution::gcd_of_strings("UVUVUVUVUVUV".to_string(), "UVUVUV".to_string()), "UVUVUV".to_string());
}

#[test]
fn test_154() {
    assert_eq!(Solution::gcd_of_strings("PPPPPPPPPP".to_string(), "PPPP".to_string()), "PP".to_string());
}

#[test]
fn test_155() {
    assert_eq!(Solution::gcd_of_strings("TESTTESTTESTTEST".to_string(), "TESTTESTTEST".to_string()), "TEST".to_string());
}

#[test]
fn test_156() {
    assert_eq!(Solution::gcd_of_strings("AABBCCDDEE".to_string(), "AABBCC".to_string()), "".to_string());
}

#[test]
fn test_157() {
    assert_eq!(Solution::gcd_of_strings("MIXMIXMIXMIXMIX".to_string(), "MIXMIX".to_string()), "MIX".to_string());
}

#[test]
fn test_158() {
    assert_eq!(Solution::gcd_of_strings("PYTHONPYTHONPYTHONPYTHON".to_string(), "PYTHONPYTHON".to_string()), "PYTHONPYTHON".to_string());
}

#[test]
fn test_159() {
    assert_eq!(Solution::gcd_of_strings("".to_string(), "".to_string()), "".to_string());
}

#[test]
fn test_160() {
    assert_eq!(Solution::gcd_of_strings("PYTHONPYTHONPYTHONPYTHONPYTHON".to_string(), "PYTHONPYTHON".to_string()), "PYTHON".to_string());
}

#[test]
fn test_161() {
    assert_eq!(Solution::gcd_of_strings("AABBCCAABBCC".to_string(), "AABBCC".to_string()), "AABBCC".to_string());
}

#[test]
fn test_162() {
    assert_eq!(Solution::gcd_of_strings("LIONLIONLIONLIONLIONLION".to_string(), "LIONLIONLION".to_string()), "LIONLIONLION".to_string());
}

#[test]
fn test_163() {
    assert_eq!(Solution::gcd_of_strings("ABCDEFGABCDEFGABCDEFG".to_string(), "ABCDEFGABCDEFG".to_string()), "ABCDEFG".to_string());
}

#[test]
fn test_164() {
    assert_eq!(Solution::gcd_of_strings("MICROPHONEMICROPHONEMICROPHONE".to_string(), "MICROPHONE".to_string()), "MICROPHONE".to_string());
}

#[test]
fn test_165() {
    assert_eq!(Solution::gcd_of_strings("ABABABABABABABAB".to_string(), "ABABAB".to_string()), "AB".to_string());
}

#[test]
fn test_166() {
    assert_eq!(Solution::gcd_of_strings("XYZXYZXYZXYZ".to_string(), "XYZXYZ".to_string()), "XYZXYZ".to_string());
}

#[test]
fn test_167() {
    assert_eq!(Solution::gcd_of_strings("HEADPHONESHEADPHONES".to_string(), "HEADPHONESHEADPHONES".to_string()), "HEADPHONESHEADPHONES".to_string());
}

#[test]
fn test_168() {
    assert_eq!(Solution::gcd_of_strings("ABCDEFGHABCDEFGHABCDEFGH".to_string(), "ABCDEFGHABCDEFGH".to_string()), "ABCDEFGH".to_string());
}

#[test]
fn test_169() {
    assert_eq!(Solution::gcd_of_strings("SPEAKERSPEAKERSPEAKERSPEAKERSPEAKER".to_string(), "SPEAKERSPEAKER".to_string()), "SPEAKER".to_string());
}

#[test]
fn test_170() {
    assert_eq!(Solution::gcd_of_strings("DIVIDEDIVIDE".to_string(), "DIVIDE".to_string()), "DIVIDE".to_string());
}

#[test]
fn test_171() {
    assert_eq!(Solution::gcd_of_strings("JSJSJSJSJSJSJS".to_string(), "JSJSJSJS".to_string()), "JS".to_string());
}

#[test]
fn test_172() {
    assert_eq!(Solution::gcd_of_strings("CATCATCATCATCATCAT".to_string(), "CATCATCAT".to_string()), "CATCATCAT".to_string());
}

#[test]
fn test_173() {
    assert_eq!(Solution::gcd_of_strings("NOTCOMMONNOTCOMMONNOTCOMMON".to_string(), "NOTCOMMONNOT".to_string()), "".to_string());
}

#[test]
fn test_174() {
    assert_eq!(Solution::gcd_of_strings("XZXZXZXZXZ".to_string(), "XZXZ".to_string()), "XZ".to_string());
}

#[test]
fn test_175() {
    assert_eq!(Solution::gcd_of_strings("DOGDOGDOGDOGDOGDOGDOGDOGDOGDOG".to_string(), "DOGDOGDOGDOG".to_string()), "DOGDOG".to_string());
}

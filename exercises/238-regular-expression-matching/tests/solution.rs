include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
}

#[test]
fn test_5() {
    assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
}

#[test]
fn test_6() {
    assert_eq!(Solution::is_match("aabbccddeeff".to_string(), "a*b*c*d*e*f*f*".to_string()), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::is_match("abababa".to_string(), "(ab)*a".to_string()), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a*.*e".to_string()), true);
}

#[test]
fn test_9() {
    assert_eq!(Solution::is_match("ababcd".to_string(), "a.*a.*d".to_string()), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::is_match("aabbbbc".to_string(), "a*b*c".to_string()), true);
}

#[test]
fn test_11() {
    assert_eq!(Solution::is_match("aaaaaa".to_string(), "a*a*a*a*a*a".to_string()), true);
}

#[test]
fn test_12() {
    assert_eq!(Solution::is_match("aabb".to_string(), "ab*a*b*".to_string()), true);
}

#[test]
fn test_13() {
    assert_eq!(Solution::is_match("abcccccaaaa".to_string(), "ab*c*a*.*".to_string()), true);
}

#[test]
fn test_14() {
    assert_eq!(Solution::is_match("abcdef".to_string(), "abc.*f".to_string()), true);
}

#[test]
fn test_15() {
    assert_eq!(Solution::is_match("ababab".to_string(), "(ab)*".to_string()), false);
}

#[test]
fn test_16() {
    assert_eq!(Solution::is_match("ab".to_string(), "a*b*c*d*.*e*".to_string()), true);
}

#[test]
fn test_17() {
    assert_eq!(Solution::is_match("xaymz".to_string(), "x.*z".to_string()), true);
}

#[test]
fn test_18() {
    assert_eq!(Solution::is_match("xaybz".to_string(), "xa*y*b*z".to_string()), true);
}

#[test]
fn test_19() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "a*d*fh".to_string()), false);
}

#[test]
fn test_20() {
    assert_eq!(Solution::is_match("aaa".to_string(), "a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*a".to_string()), true);
}

#[test]
fn test_21() {
    assert_eq!(Solution::is_match("mississippi".to_string(), "mi*ss*is*si*p*i*".to_string()), true);
}

#[test]
fn test_22() {
    assert_eq!(Solution::is_match("zzzz".to_string(), "z*".to_string()), true);
}

#[test]
fn test_23() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "a.h".to_string()), false);
}

#[test]
fn test_24() {
    assert_eq!(Solution::is_match("ababab".to_string(), "(ab)*b*".to_string()), false);
}

#[test]
fn test_25() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "a.*h".to_string()), true);
}

#[test]
fn test_26() {
    assert_eq!(Solution::is_match("mississippi".to_string(), "mi.*is.*p*i".to_string()), true);
}

#[test]
fn test_27() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "abcdefgh".to_string()), true);
}

#[test]
fn test_28() {
    assert_eq!(Solution::is_match("abcdefg".to_string(), "a*bc.d*efg".to_string()), true);
}

#[test]
fn test_29() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "a*b*c*d*e*f*g*h".to_string()), true);
}

#[test]
fn test_30() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "a*b*c*d*e*f*g*h*".to_string()), true);
}

#[test]
fn test_31() {
    assert_eq!(Solution::is_match("abcd".to_string(), "a.b.c.d".to_string()), false);
}

#[test]
fn test_32() {
    assert_eq!(Solution::is_match("aabbcc".to_string(), "a*b*c*c".to_string()), true);
}

#[test]
fn test_33() {
    assert_eq!(Solution::is_match("abc".to_string(), "abc.".to_string()), false);
}

#[test]
fn test_34() {
    assert_eq!(Solution::is_match("mississippi".to_string(), "m*is*i*s*i*p*i".to_string()), true);
}

#[test]
fn test_35() {
    assert_eq!(Solution::is_match("mississippi".to_string(), "m*i*ss*i*p*i*".to_string()), false);
}

#[test]
fn test_36() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a.*de".to_string()), true);
}

#[test]
fn test_37() {
    assert_eq!(Solution::is_match("abcdeabcde".to_string(), "abc*de*abc*de*".to_string()), true);
}

#[test]
fn test_38() {
    assert_eq!(Solution::is_match("abcd".to_string(), "a*b*c*d*".to_string()), true);
}

#[test]
fn test_39() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a*.b*c*e*".to_string()), false);
}

#[test]
fn test_40() {
    assert_eq!(Solution::is_match("aabbbcccddd".to_string(), "a*b*c*d*".to_string()), true);
}

#[test]
fn test_41() {
    assert_eq!(Solution::is_match("abc".to_string(), "a.b.c".to_string()), false);
}

#[test]
fn test_42() {
    assert_eq!(Solution::is_match("abababab".to_string(), "(ab)*".to_string()), false);
}

#[test]
fn test_43() {
    assert_eq!(Solution::is_match("hello".to_string(), "he*llo*".to_string()), true);
}

#[test]
fn test_44() {
    assert_eq!(Solution::is_match("hello".to_string(), "he.*o".to_string()), true);
}

#[test]
fn test_45() {
    assert_eq!(Solution::is_match("abcd".to_string(), "d*".to_string()), false);
}

#[test]
fn test_46() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a..de".to_string()), true);
}

#[test]
fn test_47() {
    assert_eq!(Solution::is_match("abcabcabcabc".to_string(), "(abc)*d".to_string()), false);
}

#[test]
fn test_48() {
    assert_eq!(Solution::is_match("abcdabcd".to_string(), "abcd*".to_string()), false);
}

#[test]
fn test_49() {
    assert_eq!(Solution::is_match("aabb".to_string(), "aab*b*".to_string()), true);
}

#[test]
fn test_50() {
    assert_eq!(Solution::is_match("xyzzy".to_string(), "x*zy".to_string()), false);
}

#[test]
fn test_51() {
    assert_eq!(Solution::is_match("abcd".to_string(), "a.*d".to_string()), true);
}

#[test]
fn test_52() {
    assert_eq!(Solution::is_match("xxyyzz".to_string(), "x*y*z*".to_string()), true);
}

#[test]
fn test_53() {
    assert_eq!(Solution::is_match("abc".to_string(), "a.c".to_string()), true);
}

#[test]
fn test_54() {
    assert_eq!(Solution::is_match("xyxxyxyx".to_string(), "(xy)*x".to_string()), false);
}

#[test]
fn test_55() {
    assert_eq!(Solution::is_match("aabbbccc".to_string(), "a*b*c*".to_string()), true);
}

#[test]
fn test_56() {
    assert_eq!(Solution::is_match("abcabcabcabc".to_string(), "(abc)*".to_string()), false);
}

#[test]
fn test_57() {
    assert_eq!(Solution::is_match("abcdef".to_string(), "a*bcdef".to_string()), true);
}

#[test]
fn test_58() {
    assert_eq!(Solution::is_match("aaaab".to_string(), "a*b*".to_string()), true);
}

#[test]
fn test_59() {
    assert_eq!(Solution::is_match("complex".to_string(), "c*o*m*p*l*e*x*".to_string()), true);
}

#[test]
fn test_60() {
    assert_eq!(Solution::is_match("aaa".to_string(), "a*a".to_string()), true);
}

#[test]
fn test_61() {
    assert_eq!(Solution::is_match("xyz".to_string(), "x*y*z*".to_string()), true);
}

#[test]
fn test_62() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "a.*g".to_string()), false);
}

#[test]
fn test_63() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a*b.c*d*e*".to_string()), true);
}

#[test]
fn test_64() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a*b*c*d*e*".to_string()), true);
}

#[test]
fn test_65() {
    assert_eq!(Solution::is_match("zzzzzzzzzzzz".to_string(), "z*z*z*z*z*z*z*z*z*z*z*z*z*z*z*z*z*".to_string()), true);
}

#[test]
fn test_66() {
    assert_eq!(Solution::is_match("aabbccddeeff".to_string(), "a*b*c*d*e*f*".to_string()), true);
}

#[test]
fn test_67() {
    assert_eq!(Solution::is_match("hello".to_string(), "he.*".to_string()), true);
}

#[test]
fn test_68() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a*b*c*d.e".to_string()), false);
}

#[test]
fn test_69() {
    assert_eq!(Solution::is_match("aaaaabbb".to_string(), "a*b*.*".to_string()), true);
}

#[test]
fn test_70() {
    assert_eq!(Solution::is_match("aabbccddeeffgghh".to_string(), "a*b*c*d*e*f*g*h*".to_string()), true);
}

#[test]
fn test_71() {
    assert_eq!(Solution::is_match("abbabb".to_string(), "a*b*b*".to_string()), false);
}

#[test]
fn test_72() {
    assert_eq!(Solution::is_match("sequence".to_string(), "s.e*q*u*e*n*c*e*".to_string()), true);
}

#[test]
fn test_73() {
    assert_eq!(Solution::is_match("abcdabcd".to_string(), "a*b*c*d*".to_string()), false);
}

#[test]
fn test_74() {
    assert_eq!(Solution::is_match("abcdedef".to_string(), "abcd*e*f*".to_string()), false);
}

#[test]
fn test_75() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "a.b.c.d.e.f.g.h".to_string()), false);
}

#[test]
fn test_76() {
    assert_eq!(Solution::is_match("mississippi".to_string(), "mi.*.pi.*".to_string()), true);
}

#[test]
fn test_77() {
    assert_eq!(Solution::is_match("hello".to_string(), "he*ll*o".to_string()), true);
}

#[test]
fn test_78() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a*bc.e*".to_string()), true);
}

#[test]
fn test_79() {
    assert_eq!(Solution::is_match("aaaab".to_string(), "a*a*a*a".to_string()), false);
}

#[test]
fn test_80() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a.*e".to_string()), true);
}

#[test]
fn test_81() {
    assert_eq!(Solution::is_match("aabbb".to_string(), "a*b*b".to_string()), true);
}

#[test]
fn test_82() {
    assert_eq!(Solution::is_match("regex".to_string(), "r.e*g*e*x*".to_string()), true);
}

#[test]
fn test_83() {
    assert_eq!(Solution::is_match("abcdef".to_string(), "a*b*c*d*e*f*".to_string()), true);
}

#[test]
fn test_84() {
    assert_eq!(Solution::is_match("aaaa".to_string(), "a*a*a*a".to_string()), true);
}

#[test]
fn test_85() {
    assert_eq!(Solution::is_match("teststring".to_string(), "te*t*st*ring".to_string()), false);
}

#[test]
fn test_86() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a*b*c*de".to_string()), true);
}

#[test]
fn test_87() {
    assert_eq!(Solution::is_match("bbbac".to_string(), "ba*ac".to_string()), false);
}

#[test]
fn test_88() {
    assert_eq!(Solution::is_match("aabbccddeeffgg".to_string(), "a*b*c*d*e*f*g*".to_string()), true);
}

#[test]
fn test_89() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a*bc*de".to_string()), true);
}

#[test]
fn test_90() {
    assert_eq!(Solution::is_match("xyxyxyxyxyx".to_string(), "x.y.x.y.x.y.x.y.x.y.x".to_string()), false);
}

#[test]
fn test_91() {
    assert_eq!(Solution::is_match("a".to_string(), "a*a*a*a*".to_string()), true);
}

#[test]
fn test_92() {
    assert_eq!(Solution::is_match("abababab".to_string(), "a*b*a*b*a*b*a*b".to_string()), true);
}

#[test]
fn test_93() {
    assert_eq!(Solution::is_match("abcdeabcdeabcde".to_string(), "abc*de*abc*de*abc*de*f*".to_string()), true);
}

#[test]
fn test_94() {
    assert_eq!(Solution::is_match("abbbba".to_string(), "ab*ba".to_string()), true);
}

#[test]
fn test_95() {
    assert_eq!(Solution::is_match("teststring".to_string(), "t.*st.*r.*ing".to_string()), true);
}

#[test]
fn test_96() {
    assert_eq!(Solution::is_match("abcabc".to_string(), "abc*".to_string()), false);
}

#[test]
fn test_97() {
    assert_eq!(Solution::is_match("abcd".to_string(), "a*b*c*d*.*".to_string()), true);
}

#[test]
fn test_98() {
    assert_eq!(Solution::is_match("zzzzzzzzzzzz".to_string(), "z*".to_string()), true);
}

#[test]
fn test_99() {
    assert_eq!(Solution::is_match("mississippi".to_string(), "m.*s*is*p*i*.*".to_string()), true);
}

#[test]
fn test_100() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), ".*".to_string()), true);
}

#[test]
fn test_101() {
    assert_eq!(Solution::is_match("abcdefg".to_string(), ".*f.*".to_string()), true);
}

#[test]
fn test_102() {
    assert_eq!(Solution::is_match("abccde".to_string(), "abc*d*e".to_string()), true);
}

#[test]
fn test_103() {
    assert_eq!(Solution::is_match("aabbbccdd".to_string(), "aa*bbb*cc*dd*".to_string()), true);
}

#[test]
fn test_104() {
    assert_eq!(Solution::is_match("foobar".to_string(), "fo*oba*r".to_string()), true);
}

#[test]
fn test_105() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a.*f".to_string()), false);
}

#[test]
fn test_106() {
    assert_eq!(Solution::is_match("aabb".to_string(), "a*b*b*a*".to_string()), true);
}

#[test]
fn test_107() {
    assert_eq!(Solution::is_match("zabczabcz".to_string(), "z*abc*z*".to_string()), false);
}

#[test]
fn test_108() {
    assert_eq!(Solution::is_match("aaaabbbb".to_string(), "a*b*b*".to_string()), true);
}

#[test]
fn test_109() {
    assert_eq!(Solution::is_match("aabbcc".to_string(), "a*b*b*c*c*".to_string()), true);
}

#[test]
fn test_110() {
    assert_eq!(Solution::is_match("abc".to_string(), "a*b*c*".to_string()), true);
}

#[test]
fn test_111() {
    assert_eq!(Solution::is_match("abcdeabcdeabcde".to_string(), "abc*de*abc*de*abc*de*".to_string()), true);
}

#[test]
fn test_112() {
    assert_eq!(Solution::is_match("abcde".to_string(), "a*c*e".to_string()), false);
}

#[test]
fn test_113() {
    assert_eq!(Solution::is_match("xyzzaz".to_string(), "x*y*.*z*".to_string()), true);
}

#[test]
fn test_114() {
    assert_eq!(Solution::is_match("ababab".to_string(), "(ab)*b".to_string()), false);
}

#[test]
fn test_115() {
    assert_eq!(Solution::is_match("abcdef".to_string(), "abc.def".to_string()), false);
}

#[test]
fn test_116() {
    assert_eq!(Solution::is_match("xyx".to_string(), "x*y*x*".to_string()), true);
}

#[test]
fn test_117() {
    assert_eq!(Solution::is_match("aaaaaa".to_string(), "a*a*a*a*".to_string()), true);
}

#[test]
fn test_118() {
    assert_eq!(Solution::is_match("abbb".to_string(), "ab*".to_string()), true);
}

#[test]
fn test_119() {
    assert_eq!(Solution::is_match("a".to_string(), ".".to_string()), true);
}

#[test]
fn test_120() {
    assert_eq!(Solution::is_match("abcdexyz".to_string(), "abc.*xyz".to_string()), true);
}

#[test]
fn test_121() {
    assert_eq!(Solution::is_match("aabbcc".to_string(), "a*b*c*".to_string()), true);
}

#[test]
fn test_122() {
    assert_eq!(Solution::is_match("leetcode".to_string(), "le.*e.*tcode".to_string()), true);
}

#[test]
fn test_123() {
    assert_eq!(Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string()), true);
}

#[test]
fn test_124() {
    assert_eq!(Solution::is_match("abcd".to_string(), "a.d".to_string()), false);
}

#[test]
fn test_125() {
    assert_eq!(Solution::is_match("xylophone".to_string(), "x.l*o.h.p*ne".to_string()), true);
}

#[test]
fn test_126() {
    assert_eq!(Solution::is_match("abcde".to_string(), ".*".to_string()), true);
}

#[test]
fn test_127() {
    assert_eq!(Solution::is_match("abxyzbcd".to_string(), "ab.*bc*d".to_string()), true);
}

#[test]
fn test_128() {
    assert_eq!(Solution::is_match("zzzzz".to_string(), "z*".to_string()), true);
}

#[test]
fn test_129() {
    assert_eq!(Solution::is_match("aaa".to_string(), "a*a*".to_string()), true);
}

#[test]
fn test_130() {
    assert_eq!(Solution::is_match("aaaaaaab".to_string(), "a*a*a*a*a*a*a*b".to_string()), true);
}

#[test]
fn test_131() {
    assert_eq!(Solution::is_match("hello world".to_string(), "h.*o w*r*d".to_string()), false);
}

#[test]
fn test_132() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "a.*b.*c.*d.*e.*f.*g.*h".to_string()), true);
}

#[test]
fn test_133() {
    assert_eq!(Solution::is_match("hello".to_string(), "h.l.o".to_string()), true);
}

#[test]
fn test_134() {
    assert_eq!(Solution::is_match("abcd".to_string(), "a.b*c.d".to_string()), false);
}

#[test]
fn test_135() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "abcdefgh.".to_string()), false);
}

#[test]
fn test_136() {
    assert_eq!(Solution::is_match("a".to_string(), "ab*a".to_string()), false);
}

#[test]
fn test_137() {
    assert_eq!(Solution::is_match("patternmatching".to_string(), "pat*tern*m*atching*".to_string()), true);
}

#[test]
fn test_138() {
    assert_eq!(Solution::is_match("abcabcabc".to_string(), "(abc)*".to_string()), false);
}

#[test]
fn test_139() {
    assert_eq!(Solution::is_match("ababab".to_string(), "(ab)*ab*".to_string()), false);
}

#[test]
fn test_140() {
    assert_eq!(Solution::is_match("abcd".to_string(), "a.c*d*".to_string()), true);
}

#[test]
fn test_141() {
    assert_eq!(Solution::is_match("a".to_string(), ".*".to_string()), true);
}

#[test]
fn test_142() {
    assert_eq!(Solution::is_match("abcdef".to_string(), "abcd.e*f".to_string()), true);
}

#[test]
fn test_143() {
    assert_eq!(Solution::is_match("aabb".to_string(), "a*bb".to_string()), true);
}

#[test]
fn test_144() {
    assert_eq!(Solution::is_match("abcdefgh".to_string(), "abcdefgh*".to_string()), true);
}

#[test]
fn test_145() {
    assert_eq!(Solution::is_match("aaa".to_string(), "a*a*a*a*a*a*".to_string()), true);
}

#[test]
fn test_146() {
    assert_eq!(Solution::is_match("aaaaaa".to_string(), "a*a*a*a*a*a*".to_string()), true);
}

#[test]
fn test_147() {
    assert_eq!(Solution::is_match("abab".to_string(), "(ab)*".to_string()), false);
}

#[test]
fn test_148() {
    assert_eq!(Solution::is_match("mississippi".to_string(), "mi*s*is*ip*pi*s*".to_string()), true);
}

#[test]
fn test_149() {
    assert_eq!(Solution::is_match("aaaaab".to_string(), "a*ba*".to_string()), true);
}

#[test]
fn test_150() {
    assert_eq!(Solution::is_match("abc".to_string(), ".b.".to_string()), true);
}

#[test]
fn test_151() {
    assert_eq!(Solution::is_match("abcd".to_string(), "a*b.c*d*".to_string()), true);
}

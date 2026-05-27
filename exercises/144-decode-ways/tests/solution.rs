include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::num_decodings("100100100".to_string()), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::num_decodings("101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010".to_string()), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::num_decodings("1111001111001111001111001111001111001111001111001111001111".to_string()), 0);
}

#[test]
fn test_4() {
    assert_eq!(Solution::num_decodings("2611055971756562".to_string()), 4);
}

#[test]
fn test_5() {
    assert_eq!(Solution::num_decodings("10011".to_string()), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::num_decodings("11106".to_string()), 2);
}

#[test]
fn test_7() {
    assert_eq!(Solution::num_decodings("30".to_string()), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::num_decodings("10101010101010101010101010101010".to_string()), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::num_decodings("260260260260260260260260260260260260260260260260260260260".to_string()), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::num_decodings("50633191395000636099666".to_string()), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::num_decodings("12212212212212212212212212212212".to_string()), 3524578);
}

#[test]
fn test_12() {
    assert_eq!(Solution::num_decodings("120123".to_string()), 3);
}

#[test]
fn test_13() {
    assert_eq!(Solution::num_decodings("1201234".to_string()), 3);
}

#[test]
fn test_14() {
    assert_eq!(Solution::num_decodings("999999999999999999999999999999999999999999999999999999999".to_string()), 1);
}

#[test]
fn test_15() {
    assert_eq!(Solution::num_decodings("1230123".to_string()), 0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::num_decodings("1234567890".to_string()), 0);
}

#[test]
fn test_17() {
    assert_eq!(Solution::num_decodings("123456789".to_string()), 3);
}

#[test]
fn test_18() {
    assert_eq!(Solution::num_decodings("2626262626262626262626262626262626262626262626262626262626".to_string()), 536870912);
}

#[test]
fn test_19() {
    assert_eq!(Solution::num_decodings("1234567891011121314151617181920212223242526".to_string()), 259584);
}

#[test]
fn test_20() {
    assert_eq!(Solution::num_decodings("200".to_string()), 0);
}

#[test]
fn test_21() {
    assert_eq!(Solution::num_decodings("0".to_string()), 0);
}

#[test]
fn test_22() {
    assert_eq!(Solution::num_decodings("100".to_string()), 0);
}

#[test]
fn test_23() {
    assert_eq!(Solution::num_decodings("2121212121212121212121212121212121212121212121212121212121".to_string()), 956722026041);
}

#[test]
fn test_24() {
    assert_eq!(Solution::num_decodings("1231231231231231231231231231231231231231231231231231231231".to_string()), 1162261467);
}

#[test]
fn test_25() {
    assert_eq!(Solution::num_decodings("47575625458446174945557469461".to_string()), 4);
}

#[test]
fn test_26() {
    assert_eq!(Solution::num_decodings("2200".to_string()), 0);
}

#[test]
fn test_27() {
    assert_eq!(Solution::num_decodings("010".to_string()), 0);
}

#[test]
fn test_28() {
    assert_eq!(Solution::num_decodings("99999999999999999999999999999999".to_string()), 1);
}

#[test]
fn test_29() {
    assert_eq!(Solution::num_decodings("230".to_string()), 0);
}

#[test]
fn test_30() {
    assert_eq!(Solution::num_decodings("2020202020202020202020202020202020202020202020202020202020".to_string()), 1);
}

#[test]
fn test_31() {
    assert_eq!(Solution::num_decodings("99999".to_string()), 1);
}

#[test]
fn test_32() {
    assert_eq!(Solution::num_decodings("111122221111222211112222111122221111222211112222111122221111".to_string()), 2504730781961);
}

#[test]
fn test_33() {
    assert_eq!(Solution::num_decodings("3".to_string()), 1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::num_decodings("11111111111111111111111111111111".to_string()), 3524578);
}

#[test]
fn test_35() {
    assert_eq!(Solution::num_decodings("27".to_string()), 1);
}

#[test]
fn test_36() {
    assert_eq!(Solution::num_decodings("1000000000000000000000000000000000000000000000000000000000".to_string()), 0);
}

#[test]
fn test_37() {
    assert_eq!(Solution::num_decodings("301".to_string()), 0);
}

#[test]
fn test_38() {
    assert_eq!(Solution::num_decodings("000".to_string()), 0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::num_decodings("61105709526116709637291338570167016761".to_string()), 0);
}

#[test]
fn test_40() {
    assert_eq!(Solution::num_decodings("1010101010101010101010101010101010101010101010101010101010".to_string()), 1);
}

#[test]
fn test_41() {
    assert_eq!(Solution::num_decodings("10".to_string()), 1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::num_decodings("1101101101101101101101101101101101101101101101101101101101".to_string()), 1);
}

#[test]
fn test_43() {
    assert_eq!(Solution::num_decodings("101010101010101010101010101010101010101010101010101010".to_string()), 1);
}

#[test]
fn test_44() {
    assert_eq!(Solution::num_decodings("110".to_string()), 1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::num_decodings("12".to_string()), 2);
}

#[test]
fn test_46() {
    assert_eq!(Solution::num_decodings("226".to_string()), 3);
}

#[test]
fn test_47() {
    assert_eq!(Solution::num_decodings("1".to_string()), 1);
}

#[test]
fn test_48() {
    assert_eq!(Solution::num_decodings("2222002222002222002222002222002222002222002222002222002222".to_string()), 0);
}

#[test]
fn test_49() {
    assert_eq!(Solution::num_decodings("2323232323232323232323232323232323232323232323232323232323".to_string()), 536870912);
}

#[test]
fn test_50() {
    assert_eq!(Solution::num_decodings("1010101010101010101010101010101010101010101010101010101010101010".to_string()), 1);
}

#[test]
fn test_51() {
    assert_eq!(Solution::num_decodings("1000".to_string()), 0);
}

#[test]
fn test_52() {
    assert_eq!(Solution::num_decodings("1001001001001001001001001001001001".to_string()), 0);
}

#[test]
fn test_53() {
    assert_eq!(Solution::num_decodings("11111111111111111111111111111111111111111111111111".to_string()), 20365011074);
}

#[test]
fn test_54() {
    assert_eq!(Solution::num_decodings("111111111111111111111111111111111111111111111111111111".to_string()), 139583862445);
}

#[test]
fn test_55() {
    assert_eq!(Solution::num_decodings("2101".to_string()), 1);
}

#[test]
fn test_56() {
    assert_eq!(Solution::num_decodings("20".to_string()), 1);
}

#[test]
fn test_57() {
    assert_eq!(Solution::num_decodings("1212121212121212121212121212121212121212121212121212121212".to_string()), 956722026041);
}

#[test]
fn test_58() {
    assert_eq!(Solution::num_decodings("1111111111111111111111111111111111111111111111111111111111".to_string()), 956722026041);
}

#[test]
fn test_59() {
    assert_eq!(Solution::num_decodings("1001001001001001001001001001001001001001001001001001001001".to_string()), 0);
}

#[test]
fn test_60() {
    assert_eq!(Solution::num_decodings("06".to_string()), 0);
}

#[test]
fn test_61() {
    assert_eq!(Solution::num_decodings("1234567890123456789012345678901234567890123456789012345679".to_string()), 0);
}

#[test]
fn test_62() {
    assert_eq!(Solution::num_decodings("12321213213213213213213213213213213213213213213213".to_string()), 114791256);
}

#[test]
fn test_63() {
    assert_eq!(Solution::num_decodings("1111111111111111111111111111111111111111111111111111111111111".to_string()), 4052739537881);
}

#[test]
fn test_64() {
    assert_eq!(Solution::num_decodings("12211221122112211221122112211221122112211221122112211221122112".to_string()), 6557470319842);
}

#[test]
fn test_65() {
    assert_eq!(Solution::num_decodings("12345678987654321".to_string()), 6);
}

#[test]
fn test_66() {
    assert_eq!(Solution::num_decodings("22222222222222222222222222222222222222222222222222222222222".to_string()), 1548008755920);
}

#[test]
fn test_67() {
    assert_eq!(Solution::num_decodings("2525252525252525252525252525252525252525252525252525252525".to_string()), 536870912);
}

#[test]
fn test_68() {
    assert_eq!(Solution::num_decodings("12345678901234567890123456789012345678901234567890".to_string()), 0);
}

#[test]
fn test_69() {
    assert_eq!(Solution::num_decodings("1010101010101010101010101010101010101010101010101010101010101".to_string()), 1);
}

#[test]
fn test_70() {
    assert_eq!(Solution::num_decodings("10101010101010101010101010101010101010101010101010".to_string()), 1);
}

#[test]
fn test_71() {
    assert_eq!(Solution::num_decodings("121212121212121212121212121212121212121212121212121212121212121".to_string()), 10610209857723);
}

#[test]
fn test_72() {
    assert_eq!(Solution::num_decodings("1201201201201201201201201201201201201201201201201201201201".to_string()), 1);
}

#[test]
fn test_73() {
    assert_eq!(Solution::num_decodings("122522201".to_string()), 10);
}

#[test]
fn test_74() {
    assert_eq!(Solution::num_decodings("20202020202020202020202020202020202020202020202020202020202".to_string()), 1);
}

#[test]
fn test_75() {
    assert_eq!(Solution::num_decodings("2222222222222222222222222222222222222222222222222222222222".to_string()), 956722026041);
}

#[test]
fn test_76() {
    assert_eq!(Solution::num_decodings("200200200200200200200200200200200200200200200200200".to_string()), 0);
}

#[test]
fn test_77() {
    assert_eq!(Solution::num_decodings("25252525252525252525252525252525252525252525252525252525250".to_string()), 0);
}

#[test]
fn test_78() {
    assert_eq!(Solution::num_decodings("120120120120120".to_string()), 1);
}

#[test]
fn test_79() {
    assert_eq!(Solution::num_decodings("191919191919191919191919191919191919191919191919191919191919".to_string()), 1073741824);
}

#[test]
fn test_80() {
    assert_eq!(Solution::num_decodings("9999999999999999999999999999999999999999999999999999999999".to_string()), 1);
}

#[test]
fn test_81() {
    assert_eq!(Solution::num_decodings("112112112112112112112112112112112112112112112112112112112112".to_string()), 2504730781961);
}

#[test]
fn test_82() {
    assert_eq!(Solution::num_decodings("1234567890123456789012345678901234567890123456789012345678".to_string()), 0);
}

#[test]
fn test_83() {
    assert_eq!(Solution::num_decodings("112233445566778899".to_string()), 8);
}

#[test]
fn test_84() {
    assert_eq!(Solution::num_decodings("99999999999999999999999999999999999999".to_string()), 1);
}

#[test]
fn test_85() {
    assert_eq!(Solution::num_decodings("11111111111111111111111111111111111110".to_string()), 24157817);
}

#[test]
fn test_86() {
    assert_eq!(Solution::num_decodings("192939495969798991011121314151617181920212223242526272829".to_string()), 173056);
}

#[test]
fn test_87() {
    assert_eq!(Solution::num_decodings("20202020202020202020202020202020202020202020202020".to_string()), 1);
}

#[test]
fn test_88() {
    assert_eq!(Solution::num_decodings("1234567890987654321012345678909876543210".to_string()), 0);
}

#[test]
fn test_89() {
    assert_eq!(Solution::num_decodings("272727".to_string()), 1);
}

#[test]
fn test_90() {
    assert_eq!(Solution::num_decodings("21212121212121212121212121212121212121212121212121212121212121".to_string()), 6557470319842);
}

#[test]
fn test_91() {
    assert_eq!(Solution::num_decodings("3030303030303030303030303030303030303030303030303030303030303".to_string()), 0);
}

#[test]
fn test_92() {
    assert_eq!(Solution::num_decodings("22122212122212122212122212122212122212122212122212122212122".to_string()), 1548008755920);
}

#[test]
fn test_93() {
    assert_eq!(Solution::num_decodings("001".to_string()), 0);
}

#[test]
fn test_94() {
    assert_eq!(Solution::num_decodings("1122221122221122221122221122221122221122221122221122221122221".to_string()), 4052739537881);
}

#[test]
fn test_95() {
    assert_eq!(Solution::num_decodings("11110011110011110011110011110011110011110011110011".to_string()), 0);
}

#[test]
fn test_96() {
    assert_eq!(Solution::num_decodings("1111111111111111111111111111111111111111111111111111111112".to_string()), 956722026041);
}

#[test]
fn test_97() {
    assert_eq!(Solution::num_decodings("2511452311222112511452311222112511452311222112511".to_string()), 143748000);
}

#[test]
fn test_98() {
    assert_eq!(Solution::num_decodings("112112112112112112112112112112112112112112112112112112112112112".to_string()), 10610209857723);
}

#[test]
fn test_99() {
    assert_eq!(Solution::num_decodings("222222222222222222222222222222222222222222222222222222222222".to_string()), 2504730781961);
}

#[test]
fn test_100() {
    assert_eq!(Solution::num_decodings("1234567898765432123456789876543212345678987654321234567898".to_string()), 375);
}

#[test]
fn test_101() {
    assert_eq!(Solution::num_decodings("30515015150150150150150150150150150150150150150150".to_string()), 0);
}

#[test]
fn test_102() {
    assert_eq!(Solution::num_decodings("11213141516171819202122232425262728293031323334353637383940".to_string()), 0);
}

#[test]
fn test_103() {
    assert_eq!(Solution::num_decodings("212121212121212121212121212121212121212121212121212121212".to_string()), 591286729879);
}

#[test]
fn test_104() {
    assert_eq!(Solution::num_decodings("10203040506070809010111213141516171819202122232425262728293031".to_string()), 0);
}

#[test]
fn test_105() {
    assert_eq!(Solution::num_decodings("22111122111122111122111122111122111122111122111122111122111122".to_string()), 6557470319842);
}

#[test]
fn test_106() {
    assert_eq!(Solution::num_decodings("0102030405060708091011121314151617181920212223242526".to_string()), 0);
}

#[test]
fn test_107() {
    assert_eq!(Solution::num_decodings("11111111111111111111111111111111111111111111111111111111112".to_string()), 1548008755920);
}

#[test]
fn test_108() {
    assert_eq!(Solution::num_decodings("212221222122212221222122212221222122212221222122212221222122".to_string()), 2504730781961);
}

#[test]
fn test_109() {
    assert_eq!(Solution::num_decodings("11122211122211122211122211122211122211122211122211122211122211".to_string()), 6557470319842);
}

#[test]
fn test_110() {
    assert_eq!(Solution::num_decodings("01010101010101010101010101010101010101010101010101010101010".to_string()), 0);
}

#[test]
fn test_111() {
    assert_eq!(Solution::num_decodings("111222333444555666777888999000111222333444555666777888999000".to_string()), 0);
}

#[test]
fn test_112() {
    assert_eq!(Solution::num_decodings("1234567890123456789012345678901234567890123456789012345670".to_string()), 0);
}

#[test]
fn test_113() {
    assert_eq!(Solution::num_decodings("0000000000000000000000000000000000000000000000000000000000".to_string()), 0);
}

#[test]
fn test_114() {
    assert_eq!(Solution::num_decodings("1234056789".to_string()), 0);
}

#[test]
fn test_115() {
    assert_eq!(Solution::num_decodings("19019019019019019019019019019019019019019019019019".to_string()), 0);
}

#[test]
fn test_116() {
    assert_eq!(Solution::num_decodings("12223242526272829303132333435363738394041424344454647484950".to_string()), 0);
}

#[test]
fn test_117() {
    assert_eq!(Solution::num_decodings("11111111111111111111111111111111111111".to_string()), 63245986);
}

#[test]
fn test_118() {
    assert_eq!(Solution::num_decodings("30313233343536373839404142434445464748495051525354555657585960".to_string()), 0);
}

#[test]
fn test_119() {
    assert_eq!(Solution::num_decodings("21212121212121212121212121212121212121212121212121212121212".to_string()), 1548008755920);
}

#[test]
fn test_120() {
    assert_eq!(Solution::num_decodings("270029".to_string()), 0);
}

#[test]
fn test_121() {
    assert_eq!(Solution::num_decodings("11001100110011001100".to_string()), 0);
}

#[test]
fn test_122() {
    assert_eq!(Solution::num_decodings("12345678987654321234567898765432123456789".to_string()), 75);
}

#[test]
fn test_123() {
    assert_eq!(Solution::num_decodings("1122334455667788991011121314151617181920212223242526".to_string()), 692224);
}

#[test]
fn test_124() {
    assert_eq!(Solution::num_decodings("10101010101010101010101010101010101010101010101010101010101".to_string()), 1);
}

#[test]
fn test_125() {
    assert_eq!(Solution::num_decodings("273747576787980".to_string()), 0);
}

#[test]
fn test_126() {
    assert_eq!(Solution::num_decodings("129129129".to_string()), 8);
}

#[test]
fn test_127() {
    assert_eq!(Solution::num_decodings("112211221122112211221122112211221122112211221122112211221122112".to_string()), 10610209857723);
}

#[test]
fn test_128() {
    assert_eq!(Solution::num_decodings("301010".to_string()), 0);
}

#[test]
fn test_129() {
    assert_eq!(Solution::num_decodings("121212121212121212121212121212121212121212121212121212121212".to_string()), 2504730781961);
}

#[test]
fn test_130() {
    assert_eq!(Solution::num_decodings("1101111011111011111011111011111011111011111011111".to_string()), 375000);
}

#[test]
fn test_131() {
    assert_eq!(Solution::num_decodings("123456789012345678901234567890123456789012345678901234567890".to_string()), 0);
}

#[test]
fn test_132() {
    assert_eq!(Solution::num_decodings("1101010101010101010101010101010101010101010101010101010101".to_string()), 1);
}

#[test]
fn test_133() {
    assert_eq!(Solution::num_decodings("111000111000111000111000111000111000111000111000111000".to_string()), 0);
}

#[test]
fn test_134() {
    assert_eq!(Solution::num_decodings("1225232125".to_string()), 50);
}

#[test]
fn test_135() {
    assert_eq!(Solution::num_decodings("212223242526212223242526212223242526212223242526".to_string()), 116985856);
}

#[test]
fn test_136() {
    assert_eq!(Solution::num_decodings("262626262626262626262626262626262626262626262626262626262".to_string()), 268435456);
}

#[test]
fn test_137() {
    assert_eq!(Solution::num_decodings("0123456789".to_string()), 0);
}

#[test]
fn test_138() {
    assert_eq!(Solution::num_decodings("11223344556677889911223344556677889911223344556677889911".to_string()), 1024);
}

#[test]
fn test_139() {
    assert_eq!(Solution::num_decodings("11223344556677889910111213141516171819202122232425262728293031".to_string()), 0);
}

#[test]
fn test_140() {
    assert_eq!(Solution::num_decodings("26262626262626262626262626262626262626".to_string()), 524288);
}

#[test]
fn test_141() {
    assert_eq!(Solution::num_decodings("101010101010101010101010101010101010101010101010101010101010".to_string()), 1);
}

#[test]
fn test_142() {
    assert_eq!(Solution::num_decodings("111111111111111111111111111111111111111111111111111111111111".to_string()), 2504730781961);
}

#[test]
fn test_143() {
    assert_eq!(Solution::num_decodings("12222222222222222222222222222222222222222222222222222222222".to_string()), 1548008755920);
}

#[test]
fn test_144() {
    assert_eq!(Solution::num_decodings("22322322322322322322322322322322322322322322322322322322322".to_string()), 2324522934);
}

#[test]
fn test_145() {
    assert_eq!(Solution::num_decodings("12121212121212121212121212121212121212".to_string()), 63245986);
}

#[test]
fn test_146() {
    assert_eq!(Solution::num_decodings("1222222222222222222222222222222222222222222222222222222222".to_string()), 956722026041);
}

#[test]
fn test_147() {
    assert_eq!(Solution::num_decodings("25242322212019181716151413121110987654321".to_string()), 51200);
}

#[test]
fn test_148() {
    assert_eq!(Solution::num_decodings("101010101010101010101010101010101010101010101010101010101010100".to_string()), 0);
}

#[test]
fn test_149() {
    assert_eq!(Solution::num_decodings("101112131415161718192021222324252620212223242526".to_string()), 8998912);
}

#[test]
fn test_150() {
    assert_eq!(Solution::num_decodings("11111111111111111111111111111111111111111111111111111111110".to_string()), 591286729879);
}

#[test]
fn test_151() {
    assert_eq!(Solution::num_decodings("11121314151617181920212223242526".to_string()), 86528);
}

#[test]
fn test_152() {
    assert_eq!(Solution::num_decodings("22222222222222222222222222222222222222".to_string()), 63245986);
}

#[test]
fn test_153() {
    assert_eq!(Solution::num_decodings("1230011010101010101010101010101010101010101010101010101010".to_string()), 0);
}

#[test]
fn test_154() {
    assert_eq!(Solution::num_decodings("2727272727272727272727272727272727272727272727272727272727".to_string()), 1);
}

#[test]
fn test_155() {
    assert_eq!(Solution::num_decodings("1230450678901234567890".to_string()), 0);
}

#[test]
fn test_156() {
    assert_eq!(Solution::num_decodings("11223344556677889911223344556677889911223344556677889911222".to_string()), 4096);
}

#[test]
fn test_157() {
    assert_eq!(Solution::num_decodings("33333333333333333333333333333333333333333333333333333333333".to_string()), 1);
}

#[test]
fn test_158() {
    assert_eq!(Solution::num_decodings("21012101210121012101210121012101210121012101210121".to_string()), 6144);
}

#[test]
fn test_159() {
    assert_eq!(Solution::num_decodings("11111111111111111111111111111111111111111111111111111111111101".to_string()), 1548008755920);
}

#[test]
fn test_160() {
    assert_eq!(Solution::num_decodings("212121212121212121212121212121212121212121212121212121212121".to_string()), 2504730781961);
}

#[test]
fn test_161() {
    assert_eq!(Solution::num_decodings("12121212121212121212121212121212121212121212121212121212121212".to_string()), 6557470319842);
}

#[test]
fn test_162() {
    assert_eq!(Solution::num_decodings("112358132134558914423337761098765534211".to_string()), 360);
}

#[test]
fn test_163() {
    assert_eq!(Solution::num_decodings("262626262626262626262626262626262626262626262626262626262626".to_string()), 1073741824);
}

#[test]
fn test_164() {
    assert_eq!(Solution::num_decodings("102030405060708090".to_string()), 0);
}

#[test]
fn test_165() {
    assert_eq!(Solution::num_decodings("9999999999999999999999999999999999999999999999999".to_string()), 1);
}

#[test]
fn test_166() {
    assert_eq!(Solution::num_decodings("25252525252525252525252525252525252525252525252525".to_string()), 33554432);
}

#[test]
fn test_167() {
    assert_eq!(Solution::num_decodings("222333444555666777888999".to_string()), 5);
}

#[test]
fn test_168() {
    assert_eq!(Solution::num_decodings("272727272727272727272727272727272727272727272727272727272".to_string()), 1);
}

#[test]
fn test_169() {
    assert_eq!(Solution::num_decodings("101010".to_string()), 1);
}

#[test]
fn test_170() {
    assert_eq!(Solution::num_decodings("102233445566778899112233445566778899112233445566".to_string()), 192);
}

#[test]
fn test_171() {
    assert_eq!(Solution::num_decodings("110110110".to_string()), 1);
}

#[test]
fn test_172() {
    assert_eq!(Solution::num_decodings("110110110110110110110110110110110110110110110110110110110110".to_string()), 1);
}

#[test]
fn test_173() {
    assert_eq!(Solution::num_decodings("21211212211212122112121221121212211212122112121221121212211212".to_string()), 6557470319842);
}

#[test]
fn test_174() {
    assert_eq!(Solution::num_decodings("19191919191919191919191919191919191919191919191919191919191919".to_string()), 2147483648);
}

#[test]
fn test_175() {
    assert_eq!(Solution::num_decodings("26262626262626262626262626262626262626262626262626".to_string()), 33554432);
}

#[test]
fn test_176() {
    assert_eq!(Solution::num_decodings("112233445566778899112233445566778899112233445566".to_string()), 512);
}

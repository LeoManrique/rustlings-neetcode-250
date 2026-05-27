include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::roman_to_int("XCIX".to_string()), 99);
}

#[test]
fn test_2() {
    assert_eq!(Solution::roman_to_int("MMCMXCIX".to_string()), 2999);
}

#[test]
fn test_3() {
    assert_eq!(Solution::roman_to_int("MMMCMXCIX".to_string()), 3999);
}

#[test]
fn test_4() {
    assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
}

#[test]
fn test_5() {
    assert_eq!(Solution::roman_to_int("XC".to_string()), 90);
}

#[test]
fn test_6() {
    assert_eq!(Solution::roman_to_int("VIII".to_string()), 8);
}

#[test]
fn test_7() {
    assert_eq!(Solution::roman_to_int("XV".to_string()), 15);
}

#[test]
fn test_8() {
    assert_eq!(Solution::roman_to_int("XXVII".to_string()), 27);
}

#[test]
fn test_9() {
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
}

#[test]
fn test_10() {
    assert_eq!(Solution::roman_to_int("DCCLXXIX".to_string()), 779);
}

#[test]
fn test_11() {
    assert_eq!(Solution::roman_to_int("XX".to_string()), 20);
}

#[test]
fn test_12() {
    assert_eq!(Solution::roman_to_int("CDXLIV".to_string()), 444);
}

#[test]
fn test_13() {
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
}

#[test]
fn test_14() {
    assert_eq!(Solution::roman_to_int("CM".to_string()), 900);
}

#[test]
fn test_15() {
    assert_eq!(Solution::roman_to_int("D".to_string()), 500);
}

#[test]
fn test_16() {
    assert_eq!(Solution::roman_to_int("X".to_string()), 10);
}

#[test]
fn test_17() {
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
}

#[test]
fn test_18() {
    assert_eq!(Solution::roman_to_int("XXV".to_string()), 25);
}

#[test]
fn test_19() {
    assert_eq!(Solution::roman_to_int("XXX".to_string()), 30);
}

#[test]
fn test_20() {
    assert_eq!(Solution::roman_to_int("XL".to_string()), 40);
}

#[test]
fn test_21() {
    assert_eq!(Solution::roman_to_int("MMMDCCCLXXXVIII".to_string()), 3888);
}

#[test]
fn test_22() {
    assert_eq!(Solution::roman_to_int("XXXIX".to_string()), 39);
}

#[test]
fn test_23() {
    assert_eq!(Solution::roman_to_int("XLIV".to_string()), 44);
}

#[test]
fn test_24() {
    assert_eq!(Solution::roman_to_int("CCCXCIX".to_string()), 399);
}

#[test]
fn test_25() {
    assert_eq!(Solution::roman_to_int("CD".to_string()), 400);
}

#[test]
fn test_26() {
    assert_eq!(Solution::roman_to_int("LXX".to_string()), 70);
}

#[test]
fn test_27() {
    assert_eq!(Solution::roman_to_int("CCC".to_string()), 300);
}

#[test]
fn test_28() {
    assert_eq!(Solution::roman_to_int("MMM".to_string()), 3000);
}

#[test]
fn test_29() {
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}

#[test]
fn test_30() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
}

#[test]
fn test_31() {
    assert_eq!(Solution::roman_to_int("MDCCCLXXIV".to_string()), 1874);
}

#[test]
fn test_32() {
    assert_eq!(Solution::roman_to_int("CMXCIX".to_string()), 999);
}

#[test]
fn test_33() {
    assert_eq!(Solution::roman_to_int("MMCDXCIX".to_string()), 2499);
}

#[test]
fn test_34() {
    assert_eq!(Solution::roman_to_int("M".to_string()), 1000);
}

#[test]
fn test_35() {
    assert_eq!(Solution::roman_to_int("CCXLVI".to_string()), 246);
}

#[test]
fn test_36() {
    assert_eq!(Solution::roman_to_int("CC".to_string()), 200);
}

#[test]
fn test_37() {
    assert_eq!(Solution::roman_to_int("DCCLXXIV".to_string()), 774);
}

#[test]
fn test_38() {
    assert_eq!(Solution::roman_to_int("MCCCLXXXIX".to_string()), 1389);
}

#[test]
fn test_39() {
    assert_eq!(Solution::roman_to_int("CMLXXXVII".to_string()), 987);
}

#[test]
fn test_40() {
    assert_eq!(Solution::roman_to_int("XCIV".to_string()), 94);
}

#[test]
fn test_41() {
    assert_eq!(Solution::roman_to_int("MDCCLXXVI".to_string()), 1776);
}

#[test]
fn test_42() {
    assert_eq!(Solution::roman_to_int("DCCLXXVI".to_string()), 776);
}

#[test]
fn test_43() {
    assert_eq!(Solution::roman_to_int("CMXLVII".to_string()), 947);
}

#[test]
fn test_44() {
    assert_eq!(Solution::roman_to_int("MMMCMLXXIV".to_string()), 3974);
}

#[test]
fn test_45() {
    assert_eq!(Solution::roman_to_int("MDCCCLXXI".to_string()), 1871);
}

#[test]
fn test_46() {
    assert_eq!(Solution::roman_to_int("MMMDCCCLXXVII".to_string()), 3877);
}

#[test]
fn test_47() {
    assert_eq!(Solution::roman_to_int("MMMCMXCXCIX".to_string()), 4089);
}

#[test]
fn test_48() {
    assert_eq!(Solution::roman_to_int("MMMLXXVIII".to_string()), 3078);
}

#[test]
fn test_49() {
    assert_eq!(Solution::roman_to_int("CCCLXXIV".to_string()), 374);
}

#[test]
fn test_50() {
    assert_eq!(Solution::roman_to_int("MCMXLIV".to_string()), 1944);
}

#[test]
fn test_51() {
    assert_eq!(Solution::roman_to_int("MMCDLXXI".to_string()), 2471);
}

#[test]
fn test_52() {
    assert_eq!(Solution::roman_to_int("DCCCLXXXVIII".to_string()), 888);
}

#[test]
fn test_53() {
    assert_eq!(Solution::roman_to_int("MMDCCCLXXIV".to_string()), 2874);
}

#[test]
fn test_54() {
    assert_eq!(Solution::roman_to_int("MMCDXLIV".to_string()), 2444);
}

#[test]
fn test_55() {
    assert_eq!(Solution::roman_to_int("MMDCCCLXXVII".to_string()), 2877);
}

#[test]
fn test_56() {
    assert_eq!(Solution::roman_to_int("MMMDCCCXCIX".to_string()), 3899);
}

#[test]
fn test_57() {
    assert_eq!(Solution::roman_to_int("LXXXIX".to_string()), 89);
}

#[test]
fn test_58() {
    assert_eq!(Solution::roman_to_int("DCCCLXXVIII".to_string()), 878);
}

#[test]
fn test_59() {
    assert_eq!(Solution::roman_to_int("MMXXIII".to_string()), 2023);
}

#[test]
fn test_60() {
    assert_eq!(Solution::roman_to_int("LXXXVII".to_string()), 87);
}

#[test]
fn test_61() {
    assert_eq!(Solution::roman_to_int("MMMCMXCXC".to_string()), 4080);
}

#[test]
fn test_62() {
    assert_eq!(Solution::roman_to_int("DCCCXC".to_string()), 890);
}

#[test]
fn test_63() {
    assert_eq!(Solution::roman_to_int("MMCMCCXCIX".to_string()), 3199);
}

#[test]
fn test_64() {
    assert_eq!(Solution::roman_to_int("MMMDCCCLXXX".to_string()), 3880);
}

#[test]
fn test_65() {
    assert_eq!(Solution::roman_to_int("MDCCCCLXXV".to_string()), 1975);
}

#[test]
fn test_66() {
    assert_eq!(Solution::roman_to_int("MCMXCMLXXIX".to_string()), 2869);
}

#[test]
fn test_67() {
    assert_eq!(Solution::roman_to_int("MMMDCCCLXXIX".to_string()), 3879);
}

#[test]
fn test_68() {
    assert_eq!(Solution::roman_to_int("CDXC".to_string()), 490);
}

#[test]
fn test_69() {
    assert_eq!(Solution::roman_to_int("MCMLXXI".to_string()), 1971);
}

#[test]
fn test_70() {
    assert_eq!(Solution::roman_to_int("MCMLIV".to_string()), 1954);
}

#[test]
fn test_71() {
    assert_eq!(Solution::roman_to_int("MMDCCCXCIX".to_string()), 2899);
}

#[test]
fn test_72() {
    assert_eq!(Solution::roman_to_int("CCXCIX".to_string()), 299);
}

#[test]
fn test_73() {
    assert_eq!(Solution::roman_to_int("MMMCMXCCLXXVIII".to_string()), 4168);
}

#[test]
fn test_74() {
    assert_eq!(Solution::roman_to_int("CDXCIX".to_string()), 499);
}

#[test]
fn test_75() {
    assert_eq!(Solution::roman_to_int("MMMCMLXXIX".to_string()), 3979);
}

#[test]
fn test_76() {
    assert_eq!(Solution::roman_to_int("DCCLXXVIII".to_string()), 778);
}

#[test]
fn test_77() {
    assert_eq!(Solution::roman_to_int("MDCCCLXXVIII".to_string()), 1878);
}

#[test]
fn test_78() {
    assert_eq!(Solution::roman_to_int("MMDCCCLXXXVIII".to_string()), 2888);
}

#[test]
fn test_79() {
    assert_eq!(Solution::roman_to_int("MCMXLVII".to_string()), 1947);
}

#[test]
fn test_80() {
    assert_eq!(Solution::roman_to_int("DCXXVIII".to_string()), 628);
}

#[test]
fn test_81() {
    assert_eq!(Solution::roman_to_int("CCXLVIII".to_string()), 248);
}

#[test]
fn test_82() {
    assert_eq!(Solution::roman_to_int("MMMCDXLIV".to_string()), 3444);
}

#[test]
fn test_83() {
    assert_eq!(Solution::roman_to_int("DCCCXCIX".to_string()), 899);
}

#[test]
fn test_84() {
    assert_eq!(Solution::roman_to_int("DCCCXCIV".to_string()), 894);
}

#[test]
fn test_85() {
    assert_eq!(Solution::roman_to_int("DCCCLXXIV".to_string()), 874);
}

#[test]
fn test_86() {
    assert_eq!(Solution::roman_to_int("MCMLXXIII".to_string()), 1973);
}

#[test]
fn test_87() {
    assert_eq!(Solution::roman_to_int("MMMCDXCIX".to_string()), 3499);
}

#[test]
fn test_88() {
    assert_eq!(Solution::roman_to_int("MMCDLXXVIII".to_string()), 2478);
}

#[test]
fn test_89() {
    assert_eq!(Solution::roman_to_int("LVIV".to_string()), 59);
}

#[test]
fn test_90() {
    assert_eq!(Solution::roman_to_int("MMCDXXI".to_string()), 2421);
}

#[test]
fn test_91() {
    assert_eq!(Solution::roman_to_int("MDCCCLXXVII".to_string()), 1877);
}

#[test]
fn test_92() {
    assert_eq!(Solution::roman_to_int("LXXXIV".to_string()), 84);
}

#[test]
fn test_93() {
    assert_eq!(Solution::roman_to_int("CMXLIV".to_string()), 944);
}

#[test]
fn test_94() {
    assert_eq!(Solution::roman_to_int("MCMLXXXIV".to_string()), 1984);
}

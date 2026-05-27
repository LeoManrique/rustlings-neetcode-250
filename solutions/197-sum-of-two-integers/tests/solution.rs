include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::get_sum(1000, -1000), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::get_sum(-2, -3), -5);
}

#[test]
fn test_3() {
    assert_eq!(Solution::get_sum(0, 5), 5);
}

#[test]
fn test_4() {
    assert_eq!(Solution::get_sum(500, 500), 1000);
}

#[test]
fn test_5() {
    assert_eq!(Solution::get_sum(-5, -5), -10);
}

#[test]
fn test_6() {
    assert_eq!(Solution::get_sum(1, 2), 3);
}

#[test]
fn test_7() {
    assert_eq!(Solution::get_sum(999, 1), 1000);
}

#[test]
fn test_8() {
    assert_eq!(Solution::get_sum(-5, 5), 0);
}

#[test]
fn test_9() {
    assert_eq!(Solution::get_sum(-1000, 1000), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::get_sum(-500, 500), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::get_sum(-500, -500), -1000);
}

#[test]
fn test_12() {
    assert_eq!(Solution::get_sum(0, 0), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::get_sum(-1, 1), 0);
}

#[test]
fn test_14() {
    assert_eq!(Solution::get_sum(2, 3), 5);
}

#[test]
fn test_15() {
    assert_eq!(Solution::get_sum(-999, -1), -1000);
}

#[test]
fn test_16() {
    assert_eq!(Solution::get_sum(-7, -13), -20);
}

#[test]
fn test_17() {
    assert_eq!(Solution::get_sum(-999, 1), -998);
}

#[test]
fn test_18() {
    assert_eq!(Solution::get_sum(-128, 256), 128);
}

#[test]
fn test_19() {
    assert_eq!(Solution::get_sum(-999, 999), 0);
}

#[test]
fn test_20() {
    assert_eq!(Solution::get_sum(1234, -5678), -4444);
}

#[test]
fn test_21() {
    assert_eq!(Solution::get_sum(7, 13), 20);
}

#[test]
fn test_22() {
    assert_eq!(Solution::get_sum(7, -7), 0);
}

#[test]
fn test_23() {
    assert_eq!(Solution::get_sum(256, 255), 511);
}

#[test]
fn test_24() {
    assert_eq!(Solution::get_sum(0, -1), -1);
}

#[test]
fn test_25() {
    assert_eq!(Solution::get_sum(-223, -777), -1000);
}

#[test]
fn test_26() {
    assert_eq!(Solution::get_sum(-1000, -999), -1999);
}

#[test]
fn test_27() {
    assert_eq!(Solution::get_sum(-1234, 5678), 4444);
}

#[test]
fn test_28() {
    assert_eq!(Solution::get_sum(-123, 456), 333);
}

#[test]
fn test_29() {
    assert_eq!(Solution::get_sum(1, 0), 1);
}

#[test]
fn test_30() {
    assert_eq!(Solution::get_sum(7, -3), 4);
}

#[test]
fn test_31() {
    assert_eq!(Solution::get_sum(678, 322), 1000);
}

#[test]
fn test_32() {
    assert_eq!(Solution::get_sum(500, 501), 1001);
}

#[test]
fn test_33() {
    assert_eq!(Solution::get_sum(-789, 321), -468);
}

#[test]
fn test_34() {
    assert_eq!(Solution::get_sum(1, 999), 1000);
}

#[test]
fn test_35() {
    assert_eq!(Solution::get_sum(456, 123), 579);
}

#[test]
fn test_36() {
    assert_eq!(Solution::get_sum(1, -1), 0);
}

#[test]
fn test_37() {
    assert_eq!(Solution::get_sum(7, -13), -6);
}

#[test]
fn test_38() {
    assert_eq!(Solution::get_sum(512, -512), 0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::get_sum(-314, 157), -157);
}

#[test]
fn test_40() {
    assert_eq!(Solution::get_sum(499, 500), 999);
}

#[test]
fn test_41() {
    assert_eq!(Solution::get_sum(256, 256), 512);
}

#[test]
fn test_42() {
    assert_eq!(Solution::get_sum(314, -157), 157);
}

#[test]
fn test_43() {
    assert_eq!(Solution::get_sum(223, -777), -554);
}

#[test]
fn test_44() {
    assert_eq!(Solution::get_sum(1234, -1234), 0);
}

#[test]
fn test_45() {
    assert_eq!(Solution::get_sum(-1, -999), -1000);
}

#[test]
fn test_46() {
    assert_eq!(Solution::get_sum(456, -789), -333);
}

#[test]
fn test_47() {
    assert_eq!(Solution::get_sum(456, -234), 222);
}

#[test]
fn test_48() {
    assert_eq!(Solution::get_sum(-777, -223), -1000);
}

#[test]
fn test_49() {
    assert_eq!(Solution::get_sum(-678, -322), -1000);
}

#[test]
fn test_50() {
    assert_eq!(Solution::get_sum(789, -321), 468);
}

#[test]
fn test_51() {
    assert_eq!(Solution::get_sum(-333, -667), -1000);
}

#[test]
fn test_52() {
    assert_eq!(Solution::get_sum(0, -1000), -1000);
}

#[test]
fn test_53() {
    assert_eq!(Solution::get_sum(-7, 3), -4);
}

#[test]
fn test_54() {
    assert_eq!(Solution::get_sum(-456, -123), -579);
}

#[test]
fn test_55() {
    assert_eq!(Solution::get_sum(-500, 250), -250);
}

#[test]
fn test_56() {
    assert_eq!(Solution::get_sum(345, 678), 1023);
}

#[test]
fn test_57() {
    assert_eq!(Solution::get_sum(1000, 0), 1000);
}

#[test]
fn test_58() {
    assert_eq!(Solution::get_sum(-321, 654), 333);
}

#[test]
fn test_59() {
    assert_eq!(Solution::get_sum(333, 667), 1000);
}

#[test]
fn test_60() {
    assert_eq!(Solution::get_sum(0, 1000), 1000);
}

#[test]
fn test_61() {
    assert_eq!(Solution::get_sum(-256, -256), -512);
}

#[test]
fn test_62() {
    assert_eq!(Solution::get_sum(-777, 223), -554);
}

#[test]
fn test_63() {
    assert_eq!(Solution::get_sum(-1000, 0), -1000);
}

#[test]
fn test_64() {
    assert_eq!(Solution::get_sum(-123, -456), -579);
}

#[test]
fn test_65() {
    assert_eq!(Solution::get_sum(-456, 123), -333);
}

#[test]
fn test_66() {
    assert_eq!(Solution::get_sum(777, 223), 1000);
}

#[test]
fn test_67() {
    assert_eq!(Solution::get_sum(-789, 456), -333);
}

#[test]
fn test_68() {
    assert_eq!(Solution::get_sum(63, 127), 190);
}

#[test]
fn test_69() {
    assert_eq!(Solution::get_sum(-500, -499), -999);
}

#[test]
fn test_70() {
    assert_eq!(Solution::get_sum(100, -100), 0);
}

#[test]
fn test_71() {
    assert_eq!(Solution::get_sum(-1, 0), -1);
}

#[test]
fn test_72() {
    assert_eq!(Solution::get_sum(999, -999), 0);
}

#[test]
fn test_73() {
    assert_eq!(Solution::get_sum(1000, -999), 1);
}

#[test]
fn test_74() {
    assert_eq!(Solution::get_sum(123, 456), 579);
}

#[test]
fn test_75() {
    assert_eq!(Solution::get_sum(1000, 999), 1999);
}

#[test]
fn test_76() {
    assert_eq!(Solution::get_sum(123, -456), -333);
}

#[test]
fn test_77() {
    assert_eq!(Solution::get_sum(-256, -255), -511);
}

#[test]
fn test_78() {
    assert_eq!(Solution::get_sum(500, -250), 250);
}

#[test]
fn test_79() {
    assert_eq!(Solution::get_sum(1024, -1024), 0);
}

#[test]
fn test_80() {
    assert_eq!(Solution::get_sum(-7, 13), 6);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::my_pow(3.0, 0), 1.0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::my_pow(10.0, -3), 0.001);
}

#[test]
fn test_3() {
    assert_eq!(Solution::my_pow(5.0, 1), 5.0);
}

#[test]
fn test_4() {
    assert_eq!(Solution::my_pow(2.0, -2), 0.25);
}

#[test]
fn test_5() {
    assert_eq!(Solution::my_pow(0.1, 2), 0.010000000000000002);
}

#[test]
fn test_6() {
    assert_eq!(Solution::my_pow(1.5, 5), 7.59375);
}

#[test]
fn test_7() {
    assert_eq!(Solution::my_pow(2.1, 3), 9.261000000000001);
}

#[test]
fn test_8() {
    assert_eq!(Solution::my_pow(2.0, -1), 0.5);
}

#[test]
fn test_9() {
    assert_eq!(Solution::my_pow(0.5, 4), 0.0625);
}

#[test]
fn test_10() {
    assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::my_pow(0.99, 100), 0.3660323412732289);
}

#[test]
fn test_12() {
    assert_eq!(Solution::my_pow(1.0, -1000000), 1.0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::my_pow(2.5, -5), 0.01024);
}

#[test]
fn test_14() {
    assert_eq!(Solution::my_pow(10.0, 5), 100000.0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::my_pow(-1.0, 2147483647), -1.0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::my_pow(-2.0, 12), 4096.0);
}

#[test]
fn test_17() {
    assert_eq!(Solution::my_pow(1.23456, 789), 1.5963462056225718e+72);
}

#[test]
fn test_18() {
    assert_eq!(Solution::my_pow(3.14, 7), 3009.5913952479914);
}

#[test]
fn test_19() {
    assert_eq!(Solution::my_pow(-2.0, -3), -0.125);
}

#[test]
fn test_20() {
    assert_eq!(Solution::my_pow(-1.0, 1000001), -1.0);
}

#[test]
fn test_21() {
    assert_eq!(Solution::my_pow(0.1, 10), 1.0000000000000011e-10);
}

#[test]
fn test_22() {
    assert_eq!(Solution::my_pow(0.1, 20), 1.0000000000000022e-20);
}

#[test]
fn test_23() {
    assert_eq!(Solution::my_pow(0.99999, -10000), 1.1051714706643663);
}

#[test]
fn test_24() {
    assert_eq!(Solution::my_pow(-1.5, 2), 2.25);
}

#[test]
fn test_25() {
    assert_eq!(Solution::my_pow(-0.5, 4), 0.0625);
}

#[test]
fn test_26() {
    assert_eq!(Solution::my_pow(-0.5, 6), 0.015625);
}

#[test]
fn test_27() {
    assert_eq!(Solution::my_pow(-1.0, -2147483648), 1.0);
}

#[test]
fn test_28() {
    assert_eq!(Solution::my_pow(9.87654, -321), 0.0);
}

#[test]
fn test_29() {
    assert_eq!(Solution::my_pow(2.5, 100), 6.223015277861143e+39);
}

#[test]
fn test_30() {
    assert_eq!(Solution::my_pow(10.0, 10), 10000000000.0);
}

#[test]
fn test_31() {
    assert_eq!(Solution::my_pow(1.0, 1000000), 1.0);
}

#[test]
fn test_32() {
    assert_eq!(Solution::my_pow(5.0, -3), 0.008);
}

#[test]
fn test_33() {
    assert_eq!(Solution::my_pow(1.0, -2147483648), 1.0);
}

#[test]
fn test_34() {
    assert_eq!(Solution::my_pow(1.0, 0), 1.0);
}

#[test]
fn test_35() {
    assert_eq!(Solution::my_pow(10.0, 2147483647), inf.0);
}

#[test]
fn test_36() {
    assert_eq!(Solution::my_pow(99.99999, 10), 9.999990000004497e+19);
}

#[test]
fn test_37() {
    assert_eq!(Solution::my_pow(-0.5, 8), 0.00390625);
}

#[test]
fn test_38() {
    assert_eq!(Solution::my_pow(2.0, -1000), 9.332636185032189e-302);
}

#[test]
fn test_39() {
    assert_eq!(Solution::my_pow(2.0, 0), 1.0);
}

#[test]
fn test_40() {
    assert_eq!(Solution::my_pow(2.5, -3), 0.064);
}

#[test]
fn test_41() {
    assert_eq!(Solution::my_pow(10.0, 1000), inf.0);
}

#[test]
fn test_42() {
    assert_eq!(Solution::my_pow(0.99999, 1000), 0.990049784246398);
}

#[test]
fn test_43() {
    assert_eq!(Solution::my_pow(-3.0, 4), 81.0);
}

#[test]
fn test_44() {
    assert_eq!(Solution::my_pow(-3.0, -3), -0.037037037037037035);
}

#[test]
fn test_45() {
    assert_eq!(Solution::my_pow(0.1, -5), 99999.99999999994);
}

#[test]
fn test_46() {
    assert_eq!(Solution::my_pow(2.5, 20), 90949470.17729282);
}

#[test]
fn test_47() {
    assert_eq!(Solution::my_pow(1.00001, -1000000), 4.5402199796741926e-05);
}

#[test]
fn test_48() {
    assert_eq!(Solution::my_pow(0.1, -3), 999.9999999999998);
}

#[test]
fn test_49() {
    assert_eq!(Solution::my_pow(-0.5, 3), -0.125);
}

#[test]
fn test_50() {
    assert_eq!(Solution::my_pow(1.73205, 12), 728.9959212545092);
}

#[test]
fn test_51() {
    assert_eq!(Solution::my_pow(10.0, 1), 10.0);
}

#[test]
fn test_52() {
    assert_eq!(Solution::my_pow(-2.0, 4), 16.0);
}

#[test]
fn test_53() {
    assert_eq!(Solution::my_pow(0.1, 100), 1.0000000000000108e-100);
}

#[test]
fn test_54() {
    assert_eq!(Solution::my_pow(2.0, 2147483647), inf.0);
}

#[test]
fn test_55() {
    assert_eq!(Solution::my_pow(1e-05, 10000), 0.0);
}

#[test]
fn test_56() {
    assert_eq!(Solution::my_pow(0.0, 0), 1.0);
}

#[test]
fn test_57() {
    assert_eq!(Solution::my_pow(0.5, -5), 32.0);
}

#[test]
fn test_58() {
    assert_eq!(Solution::my_pow(3.0, 20), 3486784401.0);
}

#[test]
fn test_59() {
    assert_eq!(Solution::my_pow(3.0, 15), 14348907.0);
}

#[test]
fn test_60() {
    assert_eq!(Solution::my_pow(1.00001, 1000), 1.0100501165820832);
}

#[test]
fn test_61() {
    assert_eq!(Solution::my_pow(1.5, 2147483646), inf.0);
}

#[test]
fn test_62() {
    assert_eq!(Solution::my_pow(10.0, 100), 1.0000000000000002e+100);
}

#[test]
fn test_63() {
    assert_eq!(Solution::my_pow(2.0, -2147483648), 0.0);
}

#[test]
fn test_64() {
    assert_eq!(Solution::my_pow(1e-05, 1000000), 0.0);
}

#[test]
fn test_65() {
    assert_eq!(Solution::my_pow(-1.0, 2147483646), 1.0);
}

#[test]
fn test_66() {
    assert_eq!(Solution::my_pow(1.5, 20), 3325.256730079651);
}

#[test]
fn test_67() {
    assert_eq!(Solution::my_pow(-2.0, 3), -8.0);
}

#[test]
fn test_68() {
    assert_eq!(Solution::my_pow(1.2, -5), 0.4018775720164609);
}

#[test]
fn test_69() {
    assert_eq!(Solution::my_pow(0.1, -10), 9999999999.999989);
}

#[test]
fn test_70() {
    assert_eq!(Solution::my_pow(1.0, -1000), 1.0);
}

#[test]
fn test_71() {
    assert_eq!(Solution::my_pow(2.0, 100000000), inf.0);
}

#[test]
fn test_72() {
    assert_eq!(Solution::my_pow(-2.0, 11), -2048.0);
}

#[test]
fn test_73() {
    assert_eq!(Solution::my_pow(3.5, 15), 144884079.28292847);
}

#[test]
fn test_74() {
    assert_eq!(Solution::my_pow(0.5, -10), 1024.0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::my_pow(1.41421, 50), 33550206.11671562);
}

#[test]
fn test_76() {
    assert_eq!(Solution::my_pow(3.0, 13), 1594323.0);
}

#[test]
fn test_77() {
    assert_eq!(Solution::my_pow(0.99999, 1000000), 4.539765980992338e-05);
}

#[test]
fn test_78() {
    assert_eq!(Solution::my_pow(5.0, 0), 1.0);
}

#[test]
fn test_79() {
    assert_eq!(Solution::my_pow(2.0, -10), 0.0009765625);
}

#[test]
fn test_80() {
    assert_eq!(Solution::my_pow(1.0, -5), 1.0);
}

#[test]
fn test_81() {
    assert_eq!(Solution::my_pow(1.00001, -1000), 0.9900498832512471);
}

#[test]
fn test_82() {
    assert_eq!(Solution::my_pow(-1.5, 5), -7.59375);
}

#[test]
fn test_83() {
    assert_eq!(Solution::my_pow(1.0, 2147483647), 1.0);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::unique_paths(20, 30), 11541847896480);
}

#[test]
fn test_2() {
    assert_eq!(Solution::unique_paths(1, 100), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
}

#[test]
fn test_4() {
    assert_eq!(Solution::unique_paths(5, 5), 70);
}

#[test]
fn test_5() {
    assert_eq!(Solution::unique_paths(10, 10), 48620);
}

#[test]
fn test_6() {
    assert_eq!(Solution::unique_paths(5, 3), 15);
}

#[test]
fn test_7() {
    assert_eq!(Solution::unique_paths(30, 20), 11541847896480);
}

#[test]
fn test_8() {
    assert_eq!(Solution::unique_paths(1, 1), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::unique_paths(3, 2), 3);
}

#[test]
fn test_10() {
    assert_eq!(Solution::unique_paths(100, 1), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::unique_paths(40, 30), 13750991318793417920);
}

#[test]
fn test_12() {
    assert_eq!(Solution::unique_paths(67, 33), 65814642035034133075191231);
}

#[test]
fn test_13() {
    assert_eq!(Solution::unique_paths(30, 30), 30067266499541040);
}

#[test]
fn test_14() {
    assert_eq!(Solution::unique_paths(5, 95), 3612280);
}

#[test]
fn test_15() {
    assert_eq!(Solution::unique_paths(15, 15), 40116600);
}

#[test]
fn test_16() {
    assert_eq!(Solution::unique_paths(1, 50), 1);
}

#[test]
fn test_17() {
    assert_eq!(Solution::unique_paths(30, 40), 13750991318793417920);
}

#[test]
fn test_18() {
    assert_eq!(Solution::unique_paths(99, 99), 5716592448890534420436582360196242777068052430850904489000);
}

#[test]
fn test_19() {
    assert_eq!(Solution::unique_paths(25, 75), 45931679871275969889300);
}

#[test]
fn test_20() {
    assert_eq!(Solution::unique_paths(90, 10), 1573664496040);
}

#[test]
fn test_21() {
    assert_eq!(Solution::unique_paths(75, 75), 23362265873332749085315221863910685052043000);
}

#[test]
fn test_22() {
    assert_eq!(Solution::unique_paths(30, 25), 779255311989700);
}

#[test]
fn test_23() {
    assert_eq!(Solution::unique_paths(75, 25), 45931679871275969889300);
}

#[test]
fn test_24() {
    assert_eq!(Solution::unique_paths(30, 70), 6230496325796261023265040);
}

#[test]
fn test_25() {
    assert_eq!(Solution::unique_paths(100, 5), 4421275);
}

#[test]
fn test_26() {
    assert_eq!(Solution::unique_paths(80, 80), 23156006494021191956342707682359261381151378400);
}

#[test]
fn test_27() {
    assert_eq!(Solution::unique_paths(40, 10), 1677106640);
}

#[test]
fn test_28() {
    assert_eq!(Solution::unique_paths(80, 20), 86623575014757120480);
}

#[test]
fn test_29() {
    assert_eq!(Solution::unique_paths(5, 8), 330);
}

#[test]
fn test_30() {
    assert_eq!(Solution::unique_paths(99, 1), 1);
}

#[test]
fn test_31() {
    assert_eq!(Solution::unique_paths(5, 100), 4421275);
}

#[test]
fn test_32() {
    assert_eq!(Solution::unique_paths(99, 2), 99);
}

#[test]
fn test_33() {
    assert_eq!(Solution::unique_paths(10, 90), 1573664496040);
}

#[test]
fn test_34() {
    assert_eq!(Solution::unique_paths(60, 60), 24356699707654619143838606602026720);
}

#[test]
fn test_35() {
    assert_eq!(Solution::unique_paths(2, 99), 99);
}

#[test]
fn test_36() {
    assert_eq!(Solution::unique_paths(1, 99), 1);
}

#[test]
fn test_37() {
    assert_eq!(Solution::unique_paths(45, 35), 14429347509452441488650);
}

#[test]
fn test_38() {
    assert_eq!(Solution::unique_paths(55, 45), 15362117803534044899180148240);
}

#[test]
fn test_39() {
    assert_eq!(Solution::unique_paths(50, 2), 50);
}

#[test]
fn test_40() {
    assert_eq!(Solution::unique_paths(60, 40), 3332420398982499757882998720);
}

#[test]
fn test_41() {
    assert_eq!(Solution::unique_paths(50, 50), 25477612258980856902730428600);
}

#[test]
fn test_42() {
    assert_eq!(Solution::unique_paths(100, 100), 22750883079422934966181954039568885395604168260154104734000);
}

#[test]
fn test_43() {
    assert_eq!(Solution::unique_paths(15, 5), 3060);
}

#[test]
fn test_44() {
    assert_eq!(Solution::unique_paths(99, 100), 11375441539711467483090977019784442697802084130077052367000);
}

#[test]
fn test_45() {
    assert_eq!(Solution::unique_paths(20, 80), 86623575014757120480);
}

#[test]
fn test_46() {
    assert_eq!(Solution::unique_paths(15, 20), 818809200);
}

#[test]
fn test_47() {
    assert_eq!(Solution::unique_paths(45, 55), 15362117803534044899180148240);
}

#[test]
fn test_48() {
    assert_eq!(Solution::unique_paths(25, 25), 32247603683100);
}

#[test]
fn test_49() {
    assert_eq!(Solution::unique_paths(2, 50), 50);
}

#[test]
fn test_50() {
    assert_eq!(Solution::unique_paths(40, 60), 3332420398982499757882998720);
}

#[test]
fn test_51() {
    assert_eq!(Solution::unique_paths(50, 1), 1);
}

#[test]
fn test_52() {
    assert_eq!(Solution::unique_paths(70, 30), 6230496325796261023265040);
}

#[test]
fn test_53() {
    assert_eq!(Solution::unique_paths(15, 25), 9669554100);
}

#[test]
fn test_54() {
    assert_eq!(Solution::unique_paths(33, 67), 65814642035034133075191231);
}

#[test]
fn test_55() {
    assert_eq!(Solution::unique_paths(35, 45), 14429347509452441488650);
}

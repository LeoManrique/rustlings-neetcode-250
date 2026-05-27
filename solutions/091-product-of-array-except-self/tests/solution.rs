include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::product_except_self(vec![5, 3, 0, 2, 1]), vec![0, 0, 30, 0, 0]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::product_except_self(vec![1, 0, -1, 2, -2]), vec![0, 4, 0, 0, 0]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::product_except_self(vec![1, -1, 2, -2, 3, -3]), vec![-36, 36, -18, 18, -12, 12]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::product_except_self(vec![5, 3, 2, 4, 1]), vec![24, 40, 60, 30, 120]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
}

#[test]
fn test_6() {
    assert_eq!(Solution::product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
}

#[test]
fn test_7() {
    assert_eq!(Solution::product_except_self(vec![2, -3, 4, -5]), vec![60, -40, 30, -24]);
}

#[test]
fn test_8() {
    assert_eq!(Solution::product_except_self(vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
}

#[test]
fn test_9() {
    assert_eq!(Solution::product_except_self(vec![5, 5, 5, 5]), vec![125, 125, 125, 125]);
}

#[test]
fn test_10() {
    assert_eq!(Solution::product_except_self(vec![1, 0, 2, 3]), vec![0, 6, 0, 0]);
}

#[test]
fn test_11() {
    assert_eq!(Solution::product_except_self(vec![1, 0, 3, 4]), vec![0, 12, 0, 0]);
}

#[test]
fn test_12() {
    assert_eq!(Solution::product_except_self(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_14() {
    assert_eq!(Solution::product_except_self(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_16() {
    assert_eq!(Solution::product_except_self(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), vec![19683, 19683, 19683, 19683, 19683, 19683, 19683, 19683, 19683, 19683]);
}

#[test]
fn test_17() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), vec![3628800, 1814400, 1209600, 907200, 725760, 604800, 518400, 453600, 403200, 362880]);
}

#[test]
fn test_18() {
    assert_eq!(Solution::product_except_self(vec![2, 3, 5, 7, 11, 13, 17]), vec![255255, 170170, 102102, 72930, 46410, 39270, 30030]);
}

#[test]
fn test_21() {
    assert_eq!(Solution::product_except_self(vec![10, 20, 30, 40, 50]), vec![1200000, 600000, 400000, 300000, 240000]);
}

#[test]
fn test_24() {
    assert_eq!(Solution::product_except_self(vec![10, 20, 0, 30, 40]), vec![0, 0, 240000, 0, 0]);
}

#[test]
fn test_26() {
    assert_eq!(Solution::product_except_self(vec![30, -20, 10, -5, 4, -2, 1]), vec![-8000, 12000, -24000, 48000, -60000, 120000, -240000]);
}

#[test]
fn test_28() {
    assert_eq!(Solution::product_except_self(vec![2, -3, 5, 7, 0, -1, 4]), vec![0, 0, 0, 0, 840, 0, 0]);
}

#[test]
fn test_29() {
    assert_eq!(Solution::product_except_self(vec![29, -29, 14, -14, 7, -7, 3, -3, 1, -1]), vec![-2506644, 2506644, -5192334, 5192334, -10384668, 10384668, -24230892, 24230892, -72692676, 72692676]);
}

#[test]
fn test_30() {
    assert_eq!(Solution::product_except_self(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_31() {
    assert_eq!(Solution::product_except_self(vec![10, -20, 30, -40, 50]), vec![1200000, -600000, 400000, -300000, 240000]);
}

#[test]
fn test_34() {
    assert_eq!(Solution::product_except_self(vec![0, 0, 0, 1]), vec![0, 0, 0, 0]);
}

#[test]
fn test_36() {
    assert_eq!(Solution::product_except_self(vec![100, 50, 25, 12, 6, 3, 1, -1, -2, -3]), vec![-1620000, -3240000, -6480000, -13500000, -27000000, -54000000, -162000000, 162000000, 81000000, 54000000]);
}

#[test]
fn test_37() {
    assert_eq!(Solution::product_except_self(vec![-1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1]), vec![-1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1]);
}

#[test]
fn test_38() {
    assert_eq!(Solution::product_except_self(vec![2, 5, 1, 9, 3, 6, 8, 7]), vec![45360, 18144, 90720, 10080, 30240, 15120, 11340, 12960]);
}

#[test]
fn test_42() {
    assert_eq!(Solution::product_except_self(vec![0, 0, 0, 0, 1]), vec![0, 0, 0, 0, 0]);
}

#[test]
fn test_43() {
    assert_eq!(Solution::product_except_self(vec![-1, 1, -1, 1, -1, 1, -1, 1, -1, 1]), vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1]);
}

#[test]
fn test_44() {
    assert_eq!(Solution::product_except_self(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1, -1]);
}

#[test]
fn test_48() {
    assert_eq!(Solution::product_except_self(vec![2, 3, 0, 5, 6, 0, 7]), vec![0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_49() {
    assert_eq!(Solution::product_except_self(vec![0, 0, 0, 0, 0]), vec![0, 0, 0, 0, 0]);
}

#[test]
fn test_52() {
    assert_eq!(Solution::product_except_self(vec![0, 0, 0, 0, 0]), vec![0, 0, 0, 0, 0]);
}

#[test]
fn test_53() {
    assert_eq!(Solution::product_except_self(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]), vec![536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 536870912, 1073741824]);
}

#[test]
fn test_58() {
    assert_eq!(Solution::product_except_self(vec![15, 20, 30, 0, 0, 40, 50, 60]), vec![0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_59() {
    assert_eq!(Solution::product_except_self(vec![-10, 9, 0, 5, -2, 3, -1, 8]), vec![0, 0, -21600, 0, 0, 0, 0, 0]);
}

#[test]
fn test_60() {
    assert_eq!(Solution::product_except_self(vec![100, -100, 50, -50, 25, -25]), vec![-156250000, 156250000, -312500000, 312500000, -625000000, 625000000]);
}

#[test]
fn test_64() {
    assert_eq!(Solution::product_except_self(vec![15, 10, 20, 5, 3, 8, 6, 12, 4, 7]), vec![48384000, 72576000, 36288000, 145152000, 241920000, 90720000, 120960000, 60480000, 181440000, 103680000]);
}

#[test]
fn test_65() {
    assert_eq!(Solution::product_except_self(vec![-10, 10, -10, 10, -10]), vec![10000, -10000, 10000, -10000, 10000]);
}

#[test]
fn test_67() {
    assert_eq!(Solution::product_except_self(vec![-1, -2, -3, -4, -5]), vec![120, 60, 40, 30, 24]);
}

#[test]
fn test_68() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4, 5, 0, 7, 8, 9, 10]), vec![0, 0, 0, 0, 0, 604800, 0, 0, 0, 0]);
}

#[test]
fn test_69() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 0, 0, 0, 3, 4, 5]), vec![0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_70() {
    assert_eq!(Solution::product_except_self(vec![2, 3, -5, 7, 11, -13, 17]), vec![255255, 170170, -102102, 72930, 46410, -39270, 30030]);
}

#[test]
fn test_71() {
    assert_eq!(Solution::product_except_self(vec![2, -3, 4, -5, 6, -7]), vec![-2520, 1680, -1260, 1008, -840, 720]);
}

#[test]
fn test_72() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), vec![3628800, 1814400, 1209600, 907200, 725760, 604800, 518400, 453600, 403200, 362880]);
}

#[test]
fn test_74() {
    assert_eq!(Solution::product_except_self(vec![-10, -20, -30, -40, -50]), vec![1200000, 600000, 400000, 300000, 240000]);
}

#[test]
fn test_75() {
    assert_eq!(Solution::product_except_self(vec![-2, -4, -6, -8, -10, -12, -14, -16, -18, -20]), vec![-1857945600, -928972800, -619315200, -464486400, -371589120, -309657600, -265420800, -232243200, -206438400, -185794560]);
}

#[test]
fn test_76() {
    assert_eq!(Solution::product_except_self(vec![-1, -2, -3, -4, -5, -6]), vec![-720, -360, -240, -180, -144, -120]);
}

#[test]
fn test_77() {
    assert_eq!(Solution::product_except_self(vec![0, 0, 0, 1, 2, 3]), vec![0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_78() {
    assert_eq!(Solution::product_except_self(vec![-2, -3, -4, -5, -6]), vec![360, 240, 180, 144, 120]);
}

#[test]
fn test_79() {
    assert_eq!(Solution::product_except_self(vec![1, -1, 1, -1, 1, -1]), vec![-1, 1, -1, 1, -1, 1]);
}

#[test]
fn test_80() {
    assert_eq!(Solution::product_except_self(vec![0, 0, 0, 0, 0, 1]), vec![0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_82() {
    assert_eq!(Solution::product_except_self(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]), vec![19683, 19683, 19683, 19683, 19683, 19683, 19683, 19683, 19683, 19683]);
}

#[test]
fn test_83() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 0, 4, 5, 0, 7, 8, 9, 10]), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_84() {
    assert_eq!(Solution::product_except_self(vec![7, -2, 5, 0, -3, 1]), vec![0, 0, 0, 210, 0, 0]);
}

#[test]
fn test_85() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 0, 4, 5, 0, 7, 8, 9]), vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_86() {
    assert_eq!(Solution::product_except_self(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), vec![-3628800, -1814400, -1209600, -907200, -725760, -604800, -518400, -453600, -403200, -362880]);
}

#[test]
fn test_87() {
    assert_eq!(Solution::product_except_self(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), vec![-1, 1, -1, 1, -1, 1, -1, 1, -1, 1]);
}

#[test]
fn test_88() {
    assert_eq!(Solution::product_except_self(vec![2, 3, -1, 0, 5, 6, -7, 8, -9, 10]), vec![0, 0, 0, -907200, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_89() {
    assert_eq!(Solution::product_except_self(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_90() {
    assert_eq!(Solution::product_except_self(vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5]), vec![-14400, 14400, -7200, 7200, -4800, 4800, -3600, 3600, -2880, 2880]);
}

#[test]
fn test_91() {
    assert_eq!(Solution::product_except_self(vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20]), vec![1857945600, 928972800, 619315200, 464486400, 371589120, 309657600, 265420800, 232243200, 206438400, 185794560]);
}

#[test]
fn test_92() {
    assert_eq!(Solution::product_except_self(vec![7, 1, 2, 3, 4, 5]), vec![120, 840, 420, 280, 210, 168]);
}

#[test]
fn test_95() {
    assert_eq!(Solution::product_except_self(vec![1, -1, 1, -1, 1, -1, 1, -1, 1, -1]), vec![-1, 1, -1, 1, -1, 1, -1, 1, -1, 1]);
}

#[test]
fn test_96() {
    assert_eq!(Solution::product_except_self(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_97() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 0, 3, 4, 0, 5]), vec![0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_98() {
    assert_eq!(Solution::product_except_self(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]), vec![-3628800, -1814400, -1209600, -907200, -725760, -604800, -518400, -453600, -403200, -362880]);
}

#[test]
fn test_100() {
    assert_eq!(Solution::product_except_self(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

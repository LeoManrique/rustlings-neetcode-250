include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]), -1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 2, 1, 1, 2, 1, 1]]), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]), -1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1], vec![2]]), 1);
}

#[test]
fn test_7() {
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 1], vec![1, 0, 1, 2], vec![0, 1, 2, 0]]), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1], vec![1, 1, 1], vec![0, 1, 2]]), 4);
}

#[test]
fn test_10() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0], vec![0, 1, 1], vec![1, 0, 2]]), -1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 2, 1], vec![1, 1, 1, 1]]), 3);
}

#[test]
fn test_13() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 0, 2, 0, 1], vec![1, 0, 1, 2, 1], vec![0, 2, 1, 1, 0], vec![1, 0, 0, 0, 1]]), -1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 2]]), 4);
}

#[test]
fn test_15() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 0, 1, 0], vec![1, 1, 0, 1, 1], vec![0, 1, 2, 0, 1], vec![1, 0, 1, 1, 2], vec![1, 1, 2, 1, 1]]), 4);
}

#[test]
fn test_16() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 1, 1, 0], vec![1, 0, 1, 0, 1, 0], vec![0, 1, 0, 1, 0, 1], vec![1, 0, 1, 0, 1, 0], vec![1, 1, 0, 1, 1, 2]]), -1);
}

#[test]
fn test_17() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 2, 1, 1, 1], vec![1, 1, 0, 1, 1], vec![0, 0, 0, 0, 0], vec![1, 1, 0, 1, 1], vec![1, 2, 1, 1, 1]]), 4);
}

#[test]
fn test_18() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 2, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1]]), 6);
}

#[test]
fn test_19() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 0, 1], vec![1, 0, 1, 1, 1], vec![0, 1, 2, 1, 1], vec![1, 1, 1, 1, 0], vec![1, 0, 1, 1, 2]]), 4);
}

#[test]
fn test_20() {
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0]]), -1);
}

#[test]
fn test_21() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 2]]), -1);
}

#[test]
fn test_22() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1], vec![2, 1, 0, 2], vec![1, 0, 1, 1], vec![0, 2, 1, 1]]), 2);
}

#[test]
fn test_23() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 0, 1, 1, 1], vec![1, 0, 1, 0, 1], vec![1, 0, 1, 0, 1], vec![1, 1, 1, 0, 1]]), 13);
}

#[test]
fn test_24() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1, 1, 1], vec![1, 0, 0, 0, 0, 0, 1], vec![1, 0, 0, 0, 0, 0, 1], vec![1, 0, 0, 2, 0, 0, 1], vec![1, 0, 0, 0, 0, 0, 1], vec![1, 1, 1, 1, 1, 1, 1]]), 11);
}

#[test]
fn test_25() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 2]]), -1);
}

#[test]
fn test_26() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 2]]), 5);
}

#[test]
fn test_27() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 2, 1, 1, 2, 1, 1], vec![1, 1, 0, 0, 0, 1, 1], vec![1, 0, 1, 0, 1, 0, 1], vec![1, 0, 0, 0, 0, 0, 1], vec![1, 1, 1, 1, 1, 1, 1]]), -1);
}

#[test]
fn test_28() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1], vec![1, 0, 2, 0, 1], vec![1, 0, 0, 0, 1], vec![1, 1, 1, 1, 1], vec![1, 0, 1, 0, 1]]), 9);
}

#[test]
fn test_29() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 2, 1], vec![1, 0, 1, 0, 1], vec![1, 1, 1, 1, 2], vec![1, 0, 1, 0, 1], vec![2, 1, 2, 1, 1]]), 3);
}

#[test]
fn test_30() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 0, 1, 0], vec![1, 0, 1, 1, 1, 1], vec![0, 1, 1, 0, 1, 0], vec![1, 0, 1, 1, 1, 1], vec![0, 1, 0, 1, 1, 0]]), -1);
}

#[test]
fn test_31() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1], vec![1, 1, 0, 1, 1], vec![1, 0, 0, 0, 1], vec![1, 1, 0, 1, 1], vec![2, 1, 1, 1, 2]]), 4);
}

#[test]
fn test_32() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 0, 0, 1, 1], vec![1, 0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 0], vec![1, 0, 0, 0, 0, 1], vec![1, 1, 0, 0, 1, 1]]), -1);
}

#[test]
fn test_33() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 0, 0, 0], vec![0, 0, 1, 1, 0, 1], vec![0, 1, 0, 1, 0, 0], vec![0, 0, 1, 0, 1, 0], vec![0, 0, 0, 1, 0, 2]]), -1);
}

#[test]
fn test_34() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 2], vec![2, 2, 0, 2, 2], vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 2]]), 0);
}

#[test]
fn test_35() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1], vec![1, 1, 2, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]]), 5);
}

#[test]
fn test_36() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1], vec![1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1], vec![1, 0, 0, 0, 1], vec![1, 1, 1, 1, 2]]), 4);
}

#[test]
fn test_37() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 2, 1, 1, 2], vec![1, 2, 0, 1, 2], vec![2, 1, 2, 0, 2], vec![1, 2, 2, 2, 1], vec![2, 1, 1, 1, 2]]), 1);
}

#[test]
fn test_38() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1, 1, 1], vec![1, 0, 0, 0, 0, 0, 1], vec![1, 0, 1, 0, 1, 0, 1], vec![1, 0, 0, 0, 0, 0, 1], vec![1, 1, 1, 1, 1, 1, 1]]), -1);
}

#[test]
fn test_39() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 2, 1, 1, 1, 1], vec![1, 1, 1, 1, 2, 1, 1], vec![1, 2, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 2, 1], vec![1, 1, 1, 2, 1, 1, 1]]), 3);
}

#[test]
fn test_40() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 0, 1], vec![1, 1, 0, 1, 1], vec![0, 1, 2, 0, 1], vec![1, 0, 1, 1, 2], vec![1, 1, 2, 1, 1]]), 3);
}

#[test]
fn test_41() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 0, 0, 0, 0], vec![1, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 2]]), 1);
}

#[test]
fn test_42() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 2], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![2, 1, 1, 1, 2]]), 4);
}

#[test]
fn test_43() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1, 2], vec![1, 1, 0, 0, 1, 1], vec![1, 0, 0, 0, 1, 1], vec![1, 0, 0, 0, 1, 1], vec![1, 1, 1, 1, 1, 2]]), 4);
}

#[test]
fn test_44() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 2]]), 10);
}

#[test]
fn test_45() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 0], vec![1, 1, 0, 0], vec![0, 0, 1, 1], vec![1, 2, 1, 0]]), 3);
}

#[test]
fn test_46() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1, 1], vec![1, 0, 0, 0, 0, 1], vec![1, 0, 0, 2, 0, 1], vec![1, 0, 0, 0, 0, 1], vec![1, 1, 1, 1, 1, 1]]), -1);
}

#[test]
fn test_47() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 0, 1, 1], vec![1, 0, 1, 0, 1, 2], vec![1, 0, 1, 1, 0, 1], vec![2, 1, 1, 0, 1, 1], vec![1, 0, 1, 1, 0, 1]]), 4);
}

#[test]
fn test_48() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1]]), 9);
}

#[test]
fn test_49() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]]), 8);
}

#[test]
fn test_50() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 0, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0], vec![1, 0, 1, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 0, 1]]), -1);
}

#[test]
fn test_51() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 0, 1, 0, 1, 0], vec![1, 0, 0, 0, 0, 1], vec![0, 0, 0, 2, 0, 0], vec![1, 0, 0, 0, 0, 1], vec![0, 1, 0, 1, 0, 2]]), -1);
}

#[test]
fn test_52() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 0, 1, 0, 2], vec![0, 1, 1, 1, 0], vec![1, 1, 2, 1, 1], vec![0, 1, 1, 1, 0], vec![2, 0, 1, 0, 2]]), 2);
}

#[test]
fn test_53() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 2]]), 12);
}

#[test]
fn test_54() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 1, 2], vec![1, 0, 1, 0, 2], vec![2, 1, 2, 0, 2], vec![1, 2, 2, 2, 1], vec![2, 1, 1, 1, 2]]), 1);
}

#[test]
fn test_55() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 1, 2]]), -1);
}

#[test]
fn test_56() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1], vec![1, 0, 0, 0, 1], vec![1, 0, 2, 0, 1], vec![1, 0, 0, 0, 1], vec![1, 1, 1, 1, 1]]), -1);
}

#[test]
fn test_57() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]]), -1);
}

#[test]
fn test_58() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 2, 1, 1, 2], vec![1, 1, 1, 2, 1], vec![2, 1, 1, 1, 1], vec![1, 2, 2, 1, 1], vec![1, 1, 1, 2, 1]]), 2);
}

#[test]
fn test_59() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 1], vec![1, 2, 1], vec![2, 1, 1], vec![1, 1, 2]]), 2);
}

#[test]
fn test_60() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 2, 1, 2, 1], vec![2, 1, 0, 1, 2], vec![1, 0, 0, 0, 1], vec![2, 1, 2, 1, 2]]), 1);
}

#[test]
fn test_61() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1], vec![1, 1, 0, 1], vec![1, 0, 1, 1], vec![1, 1, 1, 1]]), -1);
}

#[test]
fn test_62() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1], vec![0, 1, 0, 1, 0, 0], vec![1, 0, 1, 0, 1, 2], vec![1, 1, 0, 1, 0, 1]]), -1);
}

#[test]
fn test_63() {
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 1, 1, 1, 0], vec![1, 1, 1, 1, 1], vec![1, 1, 2, 1, 1], vec![1, 1, 1, 1, 1], vec![0, 1, 1, 1, 0]]), 3);
}

#[test]
fn test_64() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 0, 0, 0, 1, 1], vec![1, 1, 1, 0, 0, 0, 1, 1], vec![0, 0, 0, 2, 1, 1, 0, 0], vec![0, 0, 0, 0, 1, 1, 0, 0], vec![0, 0, 0, 0, 1, 1, 2, 2]]), -1);
}

#[test]
fn test_65() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 0, 1], vec![1, 0, 1, 1, 2], vec![1, 1, 1, 1, 1], vec![1, 0, 1, 0, 1], vec![2, 1, 1, 1, 2]]), 3);
}

#[test]
fn test_66() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 2, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1]]), -1);
}

#[test]
fn test_67() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 2], vec![1, 0, 0, 0, 1], vec![1, 0, 1, 0, 1], vec![1, 0, 0, 0, 1], vec![2, 1, 1, 1, 2]]), -1);
}

#[test]
fn test_68() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 0, 1, 1], vec![1, 0, 0, 1, 1, 2], vec![0, 0, 1, 0, 1, 0], vec![1, 1, 0, 0, 1, 1]]), -1);
}

#[test]
fn test_69() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1, 1], vec![1, 1, 0, 0, 0, 1], vec![1, 0, 0, 0, 0, 1], vec![1, 0, 0, 0, 0, 1], vec![1, 1, 1, 1, 1, 2]]), 4);
}

#[test]
fn test_70() {
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 0, 0, 0, 0], vec![0, 1, 1, 1, 0], vec![0, 1, 2, 1, 0], vec![0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0]]), 2);
}

#[test]
fn test_71() {
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 2, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0]]), 1);
}

#[test]
fn test_72() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 0, 1, 1], vec![1, 2, 1, 1, 2], vec![1, 1, 0, 1, 1], vec![0, 1, 1, 1, 1], vec![2, 1, 1, 1, 0]]), 3);
}

#[test]
fn test_73() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1, 1, 2], vec![1, 1, 0, 1, 1, 1, 1], vec![1, 0, 1, 0, 1, 0, 1], vec![1, 1, 1, 0, 1, 1, 1], vec![1, 0, 1, 1, 1, 0, 1], vec![1, 1, 1, 1, 1, 1, 2]]), 6);
}

#[test]
fn test_74() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 0, 0, 0, 1], vec![0, 0, 1, 0, 0], vec![0, 1, 2, 1, 0], vec![0, 0, 1, 0, 0], vec![1, 0, 0, 0, 1]]), -1);
}

#[test]
fn test_75() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1, 1, 1], vec![1, 0, 0, 0, 0, 0, 1], vec![1, 0, 0, 2, 0, 0, 1], vec![1, 0, 0, 0, 0, 0, 1], vec![1, 1, 1, 1, 1, 1, 1]]), -1);
}

#[test]
fn test_76() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 2, 1], vec![1, 0, 0, 1, 1], vec![1, 1, 0, 1, 2], vec![0, 1, 1, 1, 0], vec![1, 2, 0, 2, 1]]), 2);
}

#[test]
fn test_77() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 0, 1], vec![1, 1, 0, 1, 1], vec![0, 1, 1, 0, 1], vec![1, 0, 1, 1, 2]]), -1);
}

#[test]
fn test_78() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 2, 2, 2, 2], vec![2, 1, 1, 1, 2], vec![2, 1, 0, 1, 2], vec![2, 1, 1, 1, 2], vec![2, 2, 2, 2, 2]]), 1);
}

#[test]
fn test_79() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1], vec![1, 0, 1, 0, 1], vec![1, 0, 1, 0, 1], vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]]), 8);
}

#[test]
fn test_80() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 2], vec![1, 1, 0, 1, 1], vec![1, 1, 1, 1, 1], vec![1, 0, 1, 1, 1], vec![2, 1, 1, 1, 1]]), 4);
}

#[test]
fn test_81() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 1], vec![0, 0, 0, 0], vec![1, 0, 1, 2], vec![0, 1, 2, 0], vec![1, 2, 0, 1]]), -1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 0, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 1, 0], vec![0, 0, 0, 0, 1]]), -1);
}

#[test]
fn test_83() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1]]), 8);
}

#[test]
fn test_84() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1], vec![1, 0, 1, 0, 1], vec![1, 1, 2, 0, 1], vec![1, 0, 1, 0, 1], vec![1, 1, 1, 1, 1]]), 6);
}

#[test]
fn test_85() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 0, 0], vec![1, 0, 1, 1, 1], vec![0, 1, 1, 1, 2], vec![0, 0, 0, 1, 1], vec![0, 1, 2, 1, 2]]), 3);
}

#[test]
fn test_86() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 2, 0, 2, 1], vec![1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1], vec![1, 1, 0, 1, 1]]), -1);
}

#[test]
fn test_87() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1, 1, 1, 1], vec![1, 0, 0, 0, 0, 0, 0, 1], vec![1, 0, 0, 0, 0, 0, 0, 1], vec![1, 0, 0, 2, 0, 0, 0, 1], vec![1, 0, 0, 0, 0, 0, 0, 1], vec![1, 1, 1, 1, 1, 1, 1, 1]]), -1);
}

#[test]
fn test_88() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 0, 1, 2, 1, 1], vec![1, 1, 0, 1, 1, 1, 0], vec![0, 1, 1, 1, 0, 1, 1], vec![1, 0, 1, 1, 0, 1, 0], vec![1, 1, 0, 1, 1, 0, 1], vec![1, 1, 1, 0, 1, 1, 2]]), -1);
}

#[test]
fn test_89() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 0, 2, 0, 1], vec![0, 1, 0, 1, 0], vec![2, 0, 1, 0, 2], vec![0, 1, 0, 1, 0], vec![1, 0, 2, 0, 1]]), -1);
}

#[test]
fn test_90() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 2, 1, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 1]]), -1);
}

#[test]
fn test_91() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 0, 0, 0, 0, 1], vec![0, 1, 0, 0, 1, 0], vec![0, 0, 1, 1, 0, 0], vec![0, 0, 1, 1, 0, 0], vec![0, 1, 0, 0, 1, 0], vec![1, 0, 0, 0, 0, 1]]), -1);
}

#[test]
fn test_92() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 2, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1]]), 6);
}

#[test]
fn test_93() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 2, 1, 1], vec![1, 0, 0, 0, 0, 1], vec![1, 0, 0, 0, 0, 1], vec![1, 0, 0, 0, 0, 1], vec![1, 1, 1, 1, 1, 2]]), 4);
}

#[test]
fn test_94() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 2, 1, 0, 1, 2], vec![1, 0, 1, 1, 1, 1], vec![0, 1, 1, 1, 1, 1], vec![1, 0, 1, 0, 1, 1], vec![2, 1, 1, 1, 1, 0]]), 4);
}

#[test]
fn test_95() {
    assert_eq!(Solution::oranges_rotting(vec![vec![1, 1, 1, 1, 1], vec![1, 1, 0, 1, 1], vec![1, 1, 0, 1, 1], vec![1, 1, 0, 1, 1], vec![1, 1, 1, 1, 1]]), -1);
}

#[test]
fn test_96() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 0, 1], vec![1, 0, 1, 1, 1], vec![1, 1, 0, 1, 1], vec![0, 1, 1, 1, 2]]), 3);
}

include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1, 0], vec![0, 0]]), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0]]), 17);
}

#[test]
fn test_3() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 0], vec![0, 1, 0, 0]]), 7);
}

#[test]
fn test_4() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 0, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0]]), 7);
}

#[test]
fn test_5() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1]]), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]), 6);
}

#[test]
fn test_7() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 0, 0]]), 4);
}

#[test]
fn test_8() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 1, 0], vec![0, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 0, 0]]), 7);
}

#[test]
fn test_9() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0]]), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]), 2);
}

#[test]
fn test_11() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 0]]), 2);
}

#[test]
fn test_12() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 1]]), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]), 11);
}

#[test]
fn test_14() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 35);
}

#[test]
fn test_15() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]), 1);
}

#[test]
fn test_16() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![1, 0]]), 1);
}

#[test]
fn test_17() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![1, 1], vec![0, 0]]), 0);
}

#[test]
fn test_19() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]), 0);
}

#[test]
fn test_20() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]), 8);
}

#[test]
fn test_21() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0]]), 0);
}

#[test]
fn test_22() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0]]), 86);
}

#[test]
fn test_23() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 1, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0]]), 11);
}

#[test]
fn test_24() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0], vec![1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 1);
}

#[test]
fn test_25() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0]]), 87);
}

#[test]
fn test_26() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 35);
}

#[test]
fn test_27() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 0);
}

#[test]
fn test_28() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0], vec![1, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 24);
}

#[test]
fn test_29() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 1, 0], vec![0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0]]), 303);
}

#[test]
fn test_30() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0], vec![0, 0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0]]), 13);
}

#[test]
fn test_31() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0]]), 53);
}

#[test]
fn test_32() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 1, 1, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0]]), 26);
}

#[test]
fn test_33() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0]]), 33);
}

#[test]
fn test_34() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0]]), 540);
}

#[test]
fn test_35() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 0, 0, 0, 0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0]]), 1078);
}

#[test]
fn test_36() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 1], vec![0, 1, 0, 0], vec![0, 0, 1, 0], vec![1, 0, 0, 0]]), 2);
}

#[test]
fn test_37() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 1, 0], vec![0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0]]), 231);
}

#[test]
fn test_38() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0]]), 252);
}

#[test]
fn test_39() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 282);
}

#[test]
fn test_40() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0]]), 77);
}

#[test]
fn test_41() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0]]), 25);
}

#[test]
fn test_42() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0]]), 47);
}

#[test]
fn test_43() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 1, 0]]), 4);
}

#[test]
fn test_44() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![1, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 0, 0]]), 101);
}

#[test]
fn test_45() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 1, 0], vec![0, 0, 0, 1, 0], vec![1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0]]), 0);
}

#[test]
fn test_46() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0]]), 14);
}

#[test]
fn test_47() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0]]), 0);
}

#[test]
fn test_48() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 1, 0, 0], vec![0, 1, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 1, 0], vec![0, 1, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0]]), 38);
}

#[test]
fn test_49() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0]]), 1058);
}

#[test]
fn test_50() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0]]), 5);
}

#[test]
fn test_51() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0]]), 3);
}

#[test]
fn test_52() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0], vec![0, 1, 0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0]]), 19);
}

#[test]
fn test_53() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0, 1, 0], vec![0, 0, 0, 0, 1, 0, 0, 0], vec![0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0]]), 133);
}

#[test]
fn test_54() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 400);
}

#[test]
fn test_55() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0]]), 68);
}

#[test]
fn test_56() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0]]), 2);
}

#[test]
fn test_57() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 1, 0], vec![0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0, 0]]), 25);
}

#[test]
fn test_58() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 1871);
}

#[test]
fn test_59() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0]]), 5);
}

#[test]
fn test_60() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0]]), 264);
}

#[test]
fn test_61() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0]]), 15);
}

#[test]
fn test_62() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 1], vec![0, 0, 0, 1, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0]]), 10);
}

#[test]
fn test_63() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0], vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0]]), 72);
}

#[test]
fn test_64() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 239);
}

#[test]
fn test_65() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0], vec![0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0]]), 161);
}

#[test]
fn test_66() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 1, 0, 0], vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 9446);
}

#[test]
fn test_67() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0]]), 4);
}

#[test]
fn test_68() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 1, 0], vec![1, 1, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 3);
}

#[test]
fn test_69() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0], vec![0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0]]), 7);
}

#[test]
fn test_70() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 0]]), 3);
}

#[test]
fn test_71() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0]]), 850);
}

#[test]
fn test_72() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0]]), 14);
}

#[test]
fn test_73() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 6);
}

#[test]
fn test_74() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 24);
}

#[test]
fn test_75() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 1], vec![0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0]]), 17);
}

#[test]
fn test_76() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![1, 0, 1, 0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 1, 0, 1]]), 0);
}

#[test]
fn test_77() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 1, 0, 0, 0], vec![0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0]]), 37);
}

#[test]
fn test_78() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0]]), 3225);
}

#[test]
fn test_79() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]]), 0);
}

#[test]
fn test_80() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0]]), 16);
}

#[test]
fn test_81() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1, 0, 0], vec![0, 0, 0, 1, 0, 0, 0], vec![0, 1, 0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0]]), 24);
}

#[test]
fn test_82() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 1, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0]]), 5);
}

#[test]
fn test_83() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 1, 0, 0, 0], vec![0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0, 0]]), 11);
}

#[test]
fn test_84() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0]]), 27);
}

#[test]
fn test_85() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]]), 190);
}

#[test]
fn test_86() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 1, 0], vec![1, 0, 0, 0, 1], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0]]), 0);
}

#[test]
fn test_87() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 2513);
}

#[test]
fn test_88() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0]]), 39);
}

#[test]
fn test_89() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0], vec![1, 0, 0, 0, 1], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0]]), 7);
}

#[test]
fn test_90() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 0, 1, 0, 1, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 1, 0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0]]), 20);
}

#[test]
fn test_91() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 0, 0], vec![0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0]]), 2);
}

#[test]
fn test_92() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0]]), 18);
}

#[test]
fn test_93() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 4588);
}

#[test]
fn test_94() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 53812);
}

#[test]
fn test_95() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0]]), 72);
}

#[test]
fn test_96() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]), 184756);
}

#[test]
fn test_97() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 34);
}

#[test]
fn test_98() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 1, 0, 0], vec![0, 0, 1, 0, 0, 1, 0, 0, 0], vec![0, 1, 0, 0, 0, 0, 0, 1, 0], vec![0, 0, 0, 1, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 1, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0, 0]]), 167);
}

#[test]
fn test_99() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 1, 1, 1, 1, 0], vec![0, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0, 0, 0]]), 37);
}

#[test]
fn test_100() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 1, 0], vec![0, 0, 0, 0, 0], vec![1, 0, 1, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]]), 20);
}

#[test]
fn test_101() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0]]), 20);
}

#[test]
fn test_102() {
    assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1]]), 0);
}

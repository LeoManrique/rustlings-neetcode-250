include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::alien_order(vec!["wrt".to_string(), "wrf".to_string(), "er".to_string(), "ett".to_string(), "rftt".to_string()]), "wertf".to_string());
}

#[test]
fn test_2() {
    assert_eq!(Solution::alien_order(vec!["abc".to_string(), "ab".to_string()]), "".to_string());
}

#[test]
fn test_3() {
    assert_eq!(Solution::alien_order(vec!["a".to_string(), "b".to_string(), "ca".to_string(), "cc".to_string()]), "abc".to_string());
}

#[test]
fn test_4() {
    assert_eq!(Solution::alien_order(vec!["a".to_string(), "b".to_string(), "ca".to_string()]), "abc".to_string());
}

#[test]
fn test_5() {
    assert_eq!(Solution::alien_order(vec!["z".to_string(), "x".to_string(), "z".to_string()]), "".to_string());
}

#[test]
fn test_6() {
    assert_eq!(Solution::alien_order(vec!["z".to_string(), "x".to_string()]), "zx".to_string());
}

#[test]
fn test_7() {
    assert_eq!(Solution::alien_order(vec!["ab".to_string(), "abc".to_string(), "abcd".to_string(), "abcde".to_string(), "abcdef".to_string()]), "abcdef".to_string());
}

#[test]
fn test_8() {
    assert_eq!(Solution::alien_order(vec!["abcd".to_string(), "abdc".to_string(), "acbd".to_string(), "dacb".to_string(), "adcb".to_string(), "acdb".to_string(), "adbc".to_string(), "cabd".to_string(), "dcab".to_string(), "dbac".to_string(), "dcba".to_string(), "bdac".to_string(), "bcad".to_string(), "bcda".to_string(), "bacd".to_string(), "badc".to_string(), "bdca".to_string(), "cdab".to_string(), "cdba".to_string(), "cbad".to_string(), "cbda".to_string(), "cdba".to_string(), "dcba".to_string(), "dacb".to_string(), "dbca".to_string(), "dcab".to_string(), "dabc".to_string(), "dbac".to_string(), "bdac".to_string(), "bcad".to_string(), "bcda".to_string(), "bacd".to_string(), "badc".to_string(), "bdca".to_string()]), "".to_string());
}

#[test]
fn test_9() {
    assert_eq!(Solution::alien_order(vec!["a".to_string(), "b".to_string(), "c".to_string(), "a".to_string(), "b".to_string(), "c".to_string(), "a".to_string(), "b".to_string(), "c".to_string()]), "".to_string());
}

#[test]
fn test_10() {
    assert_eq!(Solution::alien_order(vec!["apple".to_string(), "apply".to_string(), "app".to_string()]), "".to_string());
}

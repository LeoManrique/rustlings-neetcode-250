include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::sort_array(vec![0]), vec![0]);
}

#[test]
fn test_2() {
    assert_eq!(Solution::sort_array(vec![-1]), vec![-1]);
}

#[test]
fn test_3() {
    assert_eq!(Solution::sort_array(vec![50000]), vec![50000]);
}

#[test]
fn test_4() {
    assert_eq!(Solution::sort_array(vec![-50000]), vec![-50000]);
}

#[test]
fn test_5() {
    assert_eq!(Solution::sort_array(vec![1]), vec![1]);
}

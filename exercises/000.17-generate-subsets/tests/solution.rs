include!("../src/lib.rs");

#[test]
fn test_1() {
    let mut res = Solution::subsets(vec![1, 2]);
    res.sort();
    assert_eq!(res, vec![vec![], vec![1], vec![1, 2], vec![2]]);
}
#[test]
fn test_2() {
    let mut res = Solution::subsets(vec![]);
    assert_eq!(res, vec![vec![]]);
}
#[test]
fn test_3() {
    let mut res = Solution::subsets(vec![1]);
    res.sort();
    assert_eq!(res, vec![vec![], vec![1]]);
}
#[test]
fn test_4() {
    let res = Solution::subsets(vec![1, 2, 3]);
    assert_eq!(res.len(), 8);
}

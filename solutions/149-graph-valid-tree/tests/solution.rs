include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::valid_tree(2, vec![vec![0,1]]), true);
}

#[test]
fn test_2() {
    assert_eq!(Solution::valid_tree(5, vec![vec![0,1], vec![0,2], vec![0,3], vec![1,4]]), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::valid_tree(5, vec![vec![0,1], vec![1,2], vec![2,3], vec![1,3], vec![1,4]]), false);
}

#[test]
fn test_4() {
    assert_eq!(Solution::valid_tree(6, vec![vec![0,1], vec![0,2], vec![2,3], vec![2,4], vec![4,5]]), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::valid_tree(6, vec![vec![0,1], vec![0,2], vec![0,3], vec![1,2]]), false);
}

#[test]
fn test_6() {
    assert_eq!(Solution::valid_tree(4, vec![vec![0,1], vec![0,2], vec![1,2], vec![2,3]]), false);
}

#[test]
fn test_7() {
    assert_eq!(Solution::valid_tree(3, vec![vec![0,1]]), false);
}

#[test]
fn test_8() {
    assert_eq!(Solution::valid_tree(6, vec![vec![0,1], vec![0,2], vec![2,3], vec![4,5]]), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::valid_tree(6, vec![vec![0,1], vec![1,2], vec![2,3], vec![3,4], vec![4,5], vec![5,0]]), false);
}

#[test]
fn test_10() {
    assert_eq!(Solution::valid_tree(6, vec![vec![0,1], vec![0,2], vec![0,3], vec![1,4], vec![4,5]]), true);
}

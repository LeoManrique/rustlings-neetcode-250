include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::count_components(6, vec![vec![0,1], vec![1,2], vec![3,4], vec![4,5]]), 2);
}

#[test]
fn test_2() {
    assert_eq!(Solution::count_components(7, vec![vec![0,1], vec![1,2], vec![2,3], vec![3,4], vec![4,5], vec![5,6]]), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::count_components(3, vec![vec![0,1]]), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::count_components(6, vec![vec![0,1], vec![2,3], vec![4,5]]), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::count_components(4, vec![vec![0,1], vec![2,3]]), 2);
}

#[test]
fn test_6() {
    assert_eq!(Solution::count_components(5, vec![vec![0,1], vec![1,2], vec![3,4]]), 2);
}

#[test]
fn test_7() {
    assert_eq!(Solution::count_components(7, vec![vec![0,1], vec![1,2], vec![3,4], vec![4,5], vec![5,6]]), 2);
}

#[test]
fn test_8() {
    assert_eq!(Solution::count_components(5, vec![vec![0,1], vec![1,2], vec![2,3], vec![3,4]]), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::count_components(4, vec![vec![0,1], vec![1,2], vec![2,3], vec![3,0]]), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::count_components(12, vec![vec![0,1], vec![0,2], vec![1,3], vec![1,4], vec![2,5], vec![2,6], vec![3,7], vec![4,8], vec![5,9], vec![6,10], vec![7,8], vec![9,10], vec![10,11]]), 1);
}

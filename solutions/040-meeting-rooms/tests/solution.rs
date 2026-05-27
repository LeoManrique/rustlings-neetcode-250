include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,5], vec![2,3]]), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,2]]), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,2], vec![2,3]]), true);
}

#[test]
fn test_4() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,2], vec![3,4], vec![5,6]]), true);
}

#[test]
fn test_5() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,10]]), true);
}

#[test]
fn test_6() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,1000000], vec![1000001,2000000]]), true);
}

#[test]
fn test_7() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,2], vec![2,3], vec![3,4]]), true);
}

#[test]
fn test_8() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,5], vec![2,6], vec![3,7]]), false);
}

#[test]
fn test_9() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,5], vec![6,10], vec![11,15]]), true);
}

#[test]
fn test_10() {
    assert_eq!(Solution::can_attend_meetings(vec![vec![1,3], vec![4,5], vec![6,7], vec![8,9]]), true);
}

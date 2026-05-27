include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![1,2], vec![2,3], vec![3,4], vec![4,5]]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![1,3], vec![2,6], vec![8,10], vec![15,18]]), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![1,13], vec![15,24], vec![8,18], vec![3,19]]), 3);
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![1,2], vec![2,3], vec![3,4]]), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![1,13], vec![15,24], vec![8,18], vec![3,19], vec![15,16], vec![10,15], vec![24,29], vec![5,12], vec![3,7], vec![7,13]]), 6);
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![2,11], vec![6,16], vec![11,16]]), 2);
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![11,12], vec![1,14], vec![11,15], vec![5,13], vec![12,16], vec![4,14]]), 5);
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![2,15], vec![36,45], vec![9,29], vec![16,23], vec![4,9]]), 2);
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![1,5], vec![8,9], vec![8,9]]), 2);
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![1,5], vec![6,10], vec![11,15]]), 1);
}

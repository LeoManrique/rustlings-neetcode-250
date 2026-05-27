include!("../src/lib.rs");

#[test]
fn canonical_example() {
    // From LC: k=3, nums=[4,5,8,2]; add 3 -> 4; add 5 -> 5; add 10 -> 5; add 9 -> 8; add 4 -> 8.
    let mut kl = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(kl.add(3), 4);
    assert_eq!(kl.add(5), 5);
    assert_eq!(kl.add(10), 5);
    assert_eq!(kl.add(9), 8);
    assert_eq!(kl.add(4), 8);
}

#[test]
fn empty_initial_then_add() {
    // k=1 with no initial elements: after add(5) the kth (1st) largest is 5.
    let mut kl = KthLargest::new(1, vec![]);
    assert_eq!(kl.add(5), 5);
    assert_eq!(kl.add(2), 5);
    assert_eq!(kl.add(10), 10);
    assert_eq!(kl.add(9), 10);
    assert_eq!(kl.add(4), 10);
}

#[test]
fn k_equals_one_tracks_max() {
    let mut kl = KthLargest::new(1, vec![3]);
    assert_eq!(kl.add(1), 3);
    assert_eq!(kl.add(5), 5);
    assert_eq!(kl.add(4), 5);
}

#[test]
fn initial_size_less_than_k_then_grows() {
    // k=3, only two initial values; first add reaches k.
    let mut kl = KthLargest::new(3, vec![4, 5]);
    assert_eq!(kl.add(2), 2); // sorted desc: [5,4,2] -> 3rd = 2
    assert_eq!(kl.add(3), 3); // [5,4,3,2] -> 3rd = 3
    assert_eq!(kl.add(6), 4); // [6,5,4,3,2] -> 3rd = 4
}

#[test]
fn handles_negative_numbers() {
    let mut kl = KthLargest::new(2, vec![-5, -10]);
    assert_eq!(kl.add(-3), -5); // [-3,-5,-10] -> 2nd = -5
    assert_eq!(kl.add(0), -3); // [0,-3,-5,-10] -> 2nd = -3
}

#[test]
fn handles_duplicates() {
    let mut kl = KthLargest::new(2, vec![5, 5, 5]);
    assert_eq!(kl.add(5), 5);
    assert_eq!(kl.add(6), 5); // top two = [6,5] -> 2nd = 5
    assert_eq!(kl.add(7), 6); // [7,6,5] -> 2nd = 6
}

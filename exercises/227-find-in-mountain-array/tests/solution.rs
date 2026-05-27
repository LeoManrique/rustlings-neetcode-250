include!("../src/lib.rs");

#[test]
fn lc_example_1() {
    let arr = MountainArray::new(vec![1, 2, 3, 4, 5, 3, 1]);
    assert_eq!(Solution::find_in_mountain_array(3, &arr), 2);
}

#[test]
fn lc_example_2() {
    let arr = MountainArray::new(vec![0, 1, 2, 4, 2, 1]);
    assert_eq!(Solution::find_in_mountain_array(3, &arr), -1);
}

#[test]
fn target_is_peak() {
    let arr = MountainArray::new(vec![1, 5, 2]);
    assert_eq!(Solution::find_in_mountain_array(5, &arr), 1);
}

#[test]
fn target_at_start() {
    let arr = MountainArray::new(vec![1, 5, 2]);
    assert_eq!(Solution::find_in_mountain_array(1, &arr), 0);
}

#[test]
fn target_only_on_descending_side() {
    let arr = MountainArray::new(vec![1, 2, 5, 4, 3]);
    assert_eq!(Solution::find_in_mountain_array(3, &arr), 4);
}

#[test]
fn target_above_peak() {
    let arr = MountainArray::new(vec![1, 2, 5, 4, 3]);
    assert_eq!(Solution::find_in_mountain_array(99, &arr), -1);
}

#[test]
fn target_below_floor() {
    let arr = MountainArray::new(vec![3, 5, 4]);
    assert_eq!(Solution::find_in_mountain_array(1, &arr), -1);
}

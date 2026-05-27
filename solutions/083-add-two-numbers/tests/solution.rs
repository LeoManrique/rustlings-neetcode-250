include!("../src/lib.rs");

#[allow(dead_code)]
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vals.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

#[allow(dead_code)]
fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut current = head;
    while let Some(node) = current {
        result.push(node.val);
        current = &node.next;
    }
    result
}

#[test]
fn test_1() {
    let result = Solution::add_two_numbers(build_list(&[9, 8, 7]), build_list(&[1, 2, 3]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 1, 1]);
}

#[test]
fn test_2() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), build_list(&[5, 6, 4]));
    assert_eq!(list_to_vec(&result), vec![6, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_3() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), build_list(&[5, 6, 4]));
    assert_eq!(list_to_vec(&result), vec![6, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_4() {
    let result = Solution::add_two_numbers(build_list(&[2, 4, 3]), build_list(&[5, 6, 4, 1]));
    assert_eq!(list_to_vec(&result), vec![7, 0, 8, 1]);
}

#[test]
fn test_5() {
    let result = Solution::add_two_numbers(build_list(&[1, 2, 3, 4, 5]), build_list(&[9, 8, 7, 6, 5]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 1, 1, 1, 1]);
}

#[test]
fn test_6() {
    let result = Solution::add_two_numbers(build_list(&[1]), build_list(&[9, 9, 9, 9, 9, 9, 9, 9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_7() {
    let result = Solution::add_two_numbers(build_list(&[5, 5, 5]), build_list(&[5, 5, 5, 5, 5]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 1, 6, 5]);
}

#[test]
fn test_8() {
    let result = Solution::add_two_numbers(build_list(&[9, 9, 9, 9, 9, 9, 9]), build_list(&[9, 9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
}

#[test]
fn test_9() {
    let result = Solution::add_two_numbers(build_list(&[9, 9, 9, 9, 9, 9, 9, 9, 9, 9]), build_list(&[1]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_10() {
    let result = Solution::add_two_numbers(build_list(&[1]), build_list(&[9, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 1]);
}

#[test]
fn test_11() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0]), build_list(&[9, 9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 1]);
}

#[test]
fn test_12() {
    let result = Solution::add_two_numbers(build_list(&[9, 8, 7, 6, 5, 4, 3, 2, 1]), build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_13() {
    let result = Solution::add_two_numbers(build_list(&[1, 8, 9]), build_list(&[0, 0, 9]));
    assert_eq!(list_to_vec(&result), vec![1, 8, 8, 1]);
}

#[test]
fn test_14() {
    let result = Solution::add_two_numbers(build_list(&[2, 4, 3]), build_list(&[5, 6, 4]));
    assert_eq!(list_to_vec(&result), vec![7, 0, 8]);
}

#[test]
fn test_15() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), build_list(&[5, 6, 4]));
    assert_eq!(list_to_vec(&result), vec![6, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_16() {
    let result = Solution::add_two_numbers(build_list(&[2, 4, 6]), build_list(&[1, 3, 5]));
    assert_eq!(list_to_vec(&result), vec![3, 7, 1, 1]);
}

#[test]
fn test_17() {
    let result = Solution::add_two_numbers(build_list(&[1, 1, 1]), build_list(&[9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 1, 1]);
}

#[test]
fn test_18() {
    let result = Solution::add_two_numbers(build_list(&[1, 8]), build_list(&[0]));
    assert_eq!(list_to_vec(&result), vec![1, 8]);
}

#[test]
fn test_19() {
    let result = Solution::add_two_numbers(build_list(&[1, 8]), build_list(&[0]));
    assert_eq!(list_to_vec(&result), vec![1, 8]);
}

#[test]
fn test_20() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), build_list(&[4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5]));
    assert_eq!(list_to_vec(&result), vec![5, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_21() {
    let result = Solution::add_two_numbers(build_list(&[1, 8, 9]), build_list(&[9, 1]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 1]);
}

#[test]
fn test_22() {
    let result = Solution::add_two_numbers(build_list(&[7, 2, 4, 3]), build_list(&[5, 6, 4]));
    assert_eq!(list_to_vec(&result), vec![2, 9, 8, 3]);
}

#[test]
fn test_23() {
    let result = Solution::add_two_numbers(build_list(&[1]), build_list(&[1, 2, 3, 4, 5]));
    assert_eq!(list_to_vec(&result), vec![2, 2, 3, 4, 5]);
}

#[test]
fn test_24() {
    let result = Solution::add_two_numbers(build_list(&[1]), build_list(&[9, 9, 9, 9, 9, 9, 9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_25() {
    let result = Solution::add_two_numbers(build_list(&[6, 4, 5]), build_list(&[0, 4, 5]));
    assert_eq!(list_to_vec(&result), vec![6, 8, 0, 1]);
}

#[test]
fn test_26() {
    let result = Solution::add_two_numbers(build_list(&[5]), build_list(&[5]));
    assert_eq!(list_to_vec(&result), vec![0, 1]);
}

#[test]
fn test_27() {
    let result = Solution::add_two_numbers(build_list(&[7, 2, 4, 3]), build_list(&[5, 6, 4, 2]));
    assert_eq!(list_to_vec(&result), vec![2, 9, 8, 5]);
}

#[test]
fn test_28() {
    let result = Solution::add_two_numbers(build_list(&[1, 2]), build_list(&[3, 4, 5, 6]));
    assert_eq!(list_to_vec(&result), vec![4, 6, 5, 6]);
}

#[test]
fn test_29() {
    let result = Solution::add_two_numbers(build_list(&[9, 9, 9]), build_list(&[1]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 1]);
}

#[test]
fn test_30() {
    let result = Solution::add_two_numbers(build_list(&[9, 9]), build_list(&[1]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 1]);
}

#[test]
fn test_31() {
    let result = Solution::add_two_numbers(build_list(&[1]), build_list(&[9, 9, 9, 9, 9, 9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_32() {
    let result = Solution::add_two_numbers(build_list(&[1]), build_list(&[9]));
    assert_eq!(list_to_vec(&result), vec![0, 1]);
}

#[test]
fn test_33() {
    let result = Solution::add_two_numbers(build_list(&[1, 2, 3]), build_list(&[9, 8, 7]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 1, 1]);
}

#[test]
fn test_34() {
    let result = Solution::add_two_numbers(build_list(&[1, 2, 3]), build_list(&[7, 8, 9]));
    assert_eq!(list_to_vec(&result), vec![8, 0, 3, 1]);
}

#[test]
fn test_35() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), build_list(&[5, 6, 4]));
    assert_eq!(list_to_vec(&result), vec![6, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_36() {
    let result = Solution::add_two_numbers(build_list(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), build_list(&[9, 9, 9, 9, 9, 9, 9, 9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_37() {
    let result = Solution::add_two_numbers(build_list(&[1, 2, 3]), build_list(&[4, 5, 6]));
    assert_eq!(list_to_vec(&result), vec![5, 7, 9]);
}

#[test]
fn test_38() {
    let result = Solution::add_two_numbers(build_list(&[1, 2, 3, 4, 5]), build_list(&[5, 4, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![6, 6, 6, 6, 6]);
}

#[test]
fn test_39() {
    let result = Solution::add_two_numbers(build_list(&[0, 0, 1]), build_list(&[0, 0, 1]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 2]);
}

#[test]
fn test_40() {
    let result = Solution::add_two_numbers(build_list(&[9, 9]), build_list(&[1, 1, 1]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 2]);
}

#[test]
fn test_41() {
    let result = Solution::add_two_numbers(build_list(&[6, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), build_list(&[3, 4, 2]));
    assert_eq!(list_to_vec(&result), vec![9, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_42() {
    let result = Solution::add_two_numbers(build_list(&[9]), build_list(&[1]));
    assert_eq!(list_to_vec(&result), vec![0, 1]);
}

#[test]
fn test_43() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0]), build_list(&[1]));
    assert_eq!(list_to_vec(&result), vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_44() {
    let result = Solution::add_two_numbers(build_list(&[1, 2, 3]), build_list(&[4, 5, 6]));
    assert_eq!(list_to_vec(&result), vec![5, 7, 9]);
}

#[test]
fn test_45() {
    let result = Solution::add_two_numbers(build_list(&[1, 8, 9]), build_list(&[2, 5, 8]));
    assert_eq!(list_to_vec(&result), vec![3, 3, 8, 1]);
}

#[test]
fn test_46() {
    let result = Solution::add_two_numbers(build_list(&[9, 9, 9, 9, 9, 9, 9]), build_list(&[9, 9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
}

#[test]
fn test_47() {
    let result = Solution::add_two_numbers(build_list(&[3, 2, 1]), build_list(&[9, 8, 7]));
    assert_eq!(list_to_vec(&result), vec![2, 1, 9]);
}

#[test]
fn test_48() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), build_list(&[5, 6, 4]));
    assert_eq!(list_to_vec(&result), vec![6, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_49() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), build_list(&[5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6]));
    assert_eq!(list_to_vec(&result), vec![6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7]);
}

#[test]
fn test_50() {
    let result = Solution::add_two_numbers(build_list(&[1, 2, 3, 4, 5]), build_list(&[1, 2, 3, 4, 5]));
    assert_eq!(list_to_vec(&result), vec![2, 4, 6, 8, 0, 1]);
}

#[test]
fn test_51() {
    let result = Solution::add_two_numbers(build_list(&[0]), build_list(&[0]));
    assert_eq!(list_to_vec(&result), vec![0]);
}

#[test]
fn test_52() {
    let result = Solution::add_two_numbers(build_list(&[2, 4, 9]), build_list(&[5, 6, 4, 9]));
    assert_eq!(list_to_vec(&result), vec![7, 0, 4, 0, 1]);
}

#[test]
fn test_53() {
    let result = Solution::add_two_numbers(build_list(&[2, 4, 3]), build_list(&[5, 6, 4]));
    assert_eq!(list_to_vec(&result), vec![7, 0, 8]);
}

#[test]
fn test_54() {
    let result = Solution::add_two_numbers(build_list(&[2, 4, 3, 2, 5, 5, 5]), build_list(&[5, 6, 4]));
    assert_eq!(list_to_vec(&result), vec![7, 0, 8, 2, 5, 5, 5]);
}

#[test]
fn test_55() {
    let result = Solution::add_two_numbers(build_list(&[1, 0, 0, 0, 0]), build_list(&[9, 9, 9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_56() {
    let result = Solution::add_two_numbers(build_list(&[2, 4, 6, 8]), build_list(&[1, 3, 5, 7]));
    assert_eq!(list_to_vec(&result), vec![3, 7, 1, 6, 1]);
}

#[test]
fn test_57() {
    let result = Solution::add_two_numbers(build_list(&[9]), build_list(&[1, 9, 9, 9, 9, 9, 9, 9, 9, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}

#[test]
fn test_58() {
    let result = Solution::add_two_numbers(build_list(&[0, 1]), build_list(&[0, 1]));
    assert_eq!(list_to_vec(&result), vec![0, 2]);
}

#[test]
fn test_59() {
    let result = Solution::add_two_numbers(build_list(&[7, 1, 6]), build_list(&[5, 9, 2]));
    assert_eq!(list_to_vec(&result), vec![2, 1, 9]);
}

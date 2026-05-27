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
    let result = Solution::merge_two_lists(build_list(&[1, 1, 1]), build_list(&[1, 1, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_2() {
    let result = Solution::merge_two_lists(build_list(&[10, 20, 30]), build_list(&[5, 15, 25, 35]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35]);
}

#[test]
fn test_3() {
    let result = Solution::merge_two_lists(build_list(&[-1, 0, 2]), build_list(&[-2, -1, 0]));
    assert_eq!(list_to_vec(&result), vec![-2, -1, -1, 0, 0, 2]);
}

#[test]
fn test_4() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9]), build_list(&[2, 4, 6, 8, 10]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_5() {
    let result = Solution::merge_two_lists(build_list(&[10]), build_list(&[10]));
    assert_eq!(list_to_vec(&result), vec![10, 10]);
}

#[test]
fn test_6() {
    let result = Solution::merge_two_lists(build_list(&[100]), build_list(&[50, 75, 100]));
    assert_eq!(list_to_vec(&result), vec![50, 75, 100, 100]);
}

#[test]
fn test_7() {
    let result = Solution::merge_two_lists(build_list(&[-100, 0, 100]), build_list(&[-50, 50]));
    assert_eq!(list_to_vec(&result), vec![-100, -50, 0, 50, 100]);
}

#[test]
fn test_8() {
    let result = Solution::merge_two_lists(build_list(&[5]), build_list(&[4]));
    assert_eq!(list_to_vec(&result), vec![4, 5]);
}

#[test]
fn test_9() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7]), build_list(&[2, 4, 6, 8]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_10() {
    let result = Solution::merge_two_lists(build_list(&[]), build_list(&[]));
    assert_eq!(list_to_vec(&result), vec![]);
}

#[test]
fn test_11() {
    let result = Solution::merge_two_lists(build_list(&[5]), build_list(&[1, 2, 3]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 5]);
}

#[test]
fn test_12() {
    let result = Solution::merge_two_lists(build_list(&[-100, 0, 100]), build_list(&[-50, 50]));
    assert_eq!(list_to_vec(&result), vec![-100, -50, 0, 50, 100]);
}

#[test]
fn test_13() {
    let result = Solution::merge_two_lists(build_list(&[]), build_list(&[0]));
    assert_eq!(list_to_vec(&result), vec![0]);
}

#[test]
fn test_14() {
    let result = Solution::merge_two_lists(build_list(&[-10, -5, 0]), build_list(&[-20, -15, -10]));
    assert_eq!(list_to_vec(&result), vec![-20, -15, -10, -10, -5, 0]);
}

#[test]
fn test_15() {
    let result = Solution::merge_two_lists(build_list(&[1, 1, 1, 1]), build_list(&[1, 1, 1, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_16() {
    let result = Solution::merge_two_lists(build_list(&[-1, -2, -3]), build_list(&[-4, -5, -6]));
    assert_eq!(list_to_vec(&result), vec![-4, -5, -6, -1, -2, -3]);
}

#[test]
fn test_17() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 4]), build_list(&[1, 3, 4]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 3, 4, 4]);
}

#[test]
fn test_18() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3]), build_list(&[4, 5, 6]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_19() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5]), build_list(&[6, 7, 8, 9, 10]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_20() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), build_list(&[11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
}

#[test]
fn test_21() {
    let result = Solution::merge_two_lists(build_list(&[-100, -50, 0, 50, 100]), build_list(&[-200, -150, -100, -50, 0]));
    assert_eq!(list_to_vec(&result), vec![-200, -150, -100, -100, -50, -50, 0, 0, 50, 100]);
}

#[test]
fn test_22() {
    let result = Solution::merge_two_lists(build_list(&[-99, -98, -97]), build_list(&[-96, -95, -94]));
    assert_eq!(list_to_vec(&result), vec![-99, -98, -97, -96, -95, -94]);
}

#[test]
fn test_23() {
    let result = Solution::merge_two_lists(build_list(&[-100, -99, -98]), build_list(&[-100, -99, -98]));
    assert_eq!(list_to_vec(&result), vec![-100, -100, -99, -99, -98, -98]);
}

#[test]
fn test_24() {
    let result = Solution::merge_two_lists(build_list(&[-10, -5, 0, 5, 10]), build_list(&[-20, -15, -10, -5, 0]));
    assert_eq!(list_to_vec(&result), vec![-20, -15, -10, -10, -5, -5, 0, 0, 5, 10]);
}

#[test]
fn test_25() {
    let result = Solution::merge_two_lists(build_list(&[30, 40, 50]), build_list(&[10, 20, 30, 40, 50, 60]));
    assert_eq!(list_to_vec(&result), vec![10, 20, 30, 30, 40, 40, 50, 50, 60]);
}

#[test]
fn test_26() {
    let result = Solution::merge_two_lists(build_list(&[2, 4, 6, 8]), build_list(&[1, 3, 5, 7, 9, 11, 13, 15]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 15]);
}

#[test]
fn test_27() {
    let result = Solution::merge_two_lists(build_list(&[-99, -50, -20, 0, 20, 50]), build_list(&[-98, -49, -19, -1, 19, 49]));
    assert_eq!(list_to_vec(&result), vec![-99, -98, -50, -49, -20, -19, -1, 0, 19, 20, 49, 50]);
}

#[test]
fn test_28() {
    let result = Solution::merge_two_lists(build_list(&[-10, -5, 0, 5, 10]), build_list(&[-11, -6, -1, 6, 11]));
    assert_eq!(list_to_vec(&result), vec![-11, -10, -6, -5, -1, 0, 5, 6, 10, 11]);
}

#[test]
fn test_29() {
    let result = Solution::merge_two_lists(build_list(&[1]), build_list(&[2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_30() {
    let result = Solution::merge_two_lists(build_list(&[0, 1, 2, 3, 4]), build_list(&[-4, -3, -2, -1, 0]));
    assert_eq!(list_to_vec(&result), vec![-4, -3, -2, -1, 0, 0, 1, 2, 3, 4]);
}

#[test]
fn test_31() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9, 11, 13]), build_list(&[2, 4, 6, 8, 10, 12, 14]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
}

#[test]
fn test_32() {
    let result = Solution::merge_two_lists(build_list(&[50, 100, 150, 200, 250, 300]), build_list(&[25, 75, 125, 175, 225, 275, 325]));
    assert_eq!(list_to_vec(&result), vec![25, 50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300, 325]);
}

#[test]
fn test_33() {
    let result = Solution::merge_two_lists(build_list(&[50, 45, 40, 35, 30]), build_list(&[25, 20, 15, 10, 5, 0]));
    assert_eq!(list_to_vec(&result), vec![25, 20, 15, 10, 5, 0, 50, 45, 40, 35, 30]);
}

#[test]
fn test_34() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), build_list(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_35() {
    let result = Solution::merge_two_lists(build_list(&[-1, 2, 4, 6, 8]), build_list(&[0, 3, 5, 7, 9]));
    assert_eq!(list_to_vec(&result), vec![-1, 0, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_36() {
    let result = Solution::merge_two_lists(build_list(&[10, 20, 30, 40, 50]), build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20]));
    assert_eq!(list_to_vec(&result), vec![2, 4, 6, 8, 10, 10, 12, 14, 16, 18, 20, 20, 30, 40, 50]);
}

#[test]
fn test_37() {
    let result = Solution::merge_two_lists(build_list(&[0, 2, 4, 6, 8, 10]), build_list(&[1, 3, 5, 7, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_38() {
    let result = Solution::merge_two_lists(build_list(&[-1, -2, -3, -4, -5]), build_list(&[-5, -4, -3, -2, -1]));
    assert_eq!(list_to_vec(&result), vec![-5, -4, -3, -2, -1, -2, -3, -4, -5, -1]);
}

#[test]
fn test_39() {
    let result = Solution::merge_two_lists(build_list(&[-99, -98, -97, -96]), build_list(&[-95, -94, -93, -92]));
    assert_eq!(list_to_vec(&result), vec![-99, -98, -97, -96, -95, -94, -93, -92]);
}

#[test]
fn test_40() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5]), build_list(&[0, -1, -2, -3, -4, -5]));
    assert_eq!(list_to_vec(&result), vec![0, -1, -2, -3, -4, -5, 1, 2, 3, 4, 5]);
}

#[test]
fn test_41() {
    let result = Solution::merge_two_lists(build_list(&[-5, -4, -3, -2, -1]), build_list(&[0, 1, 2, 3, 4, 5]));
    assert_eq!(list_to_vec(&result), vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_42() {
    let result = Solution::merge_two_lists(build_list(&[-1, 2, -3, 4, -5]), build_list(&[1, -2, 3, -4, 5]));
    assert_eq!(list_to_vec(&result), vec![-1, 1, -2, 2, -3, 3, -4, 4, -5, 5]);
}

#[test]
fn test_43() {
    let result = Solution::merge_two_lists(build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), build_list(&[5, 15, 25, 35, 45, 55, 65, 75, 85, 95]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100]);
}

#[test]
fn test_44() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9]), build_list(&[10, 20, 30, 40, 50]));
    assert_eq!(list_to_vec(&result), vec![1, 3, 5, 7, 9, 10, 20, 30, 40, 50]);
}

#[test]
fn test_45() {
    let result = Solution::merge_two_lists(build_list(&[0, 2, 4, 6, 8]), build_list(&[1, 3, 5, 7, 9]));
    assert_eq!(list_to_vec(&result), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_46() {
    let result = Solution::merge_two_lists(build_list(&[-100, -50, 0, 50, 100]), build_list(&[]));
    assert_eq!(list_to_vec(&result), vec![-100, -50, 0, 50, 100]);
}

#[test]
fn test_47() {
    let result = Solution::merge_two_lists(build_list(&[-10, -8, -6, -4, -2]), build_list(&[-9, -7, -5, -3, -1]));
    assert_eq!(list_to_vec(&result), vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1]);
}

#[test]
fn test_48() {
    let result = Solution::merge_two_lists(build_list(&[-100, -50, 0, 50, 100]), build_list(&[-75, -25, 25, 75]));
    assert_eq!(list_to_vec(&result), vec![-100, -75, -50, -25, 0, 25, 50, 75, 100]);
}

#[test]
fn test_49() {
    let result = Solution::merge_two_lists(build_list(&[1]), build_list(&[2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_50() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9]), build_list(&[1, 3, 5, 7, 9]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 3, 3, 5, 5, 7, 7, 9, 9]);
}

#[test]
fn test_51() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), build_list(&[11, 12, 13, 14, 15, 16, 17, 18, 19, 20]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_52() {
    let result = Solution::merge_two_lists(build_list(&[-50, -40, -30]), build_list(&[-20, -10, 0, 10, 20, 30]));
    assert_eq!(list_to_vec(&result), vec![-50, -40, -30, -20, -10, 0, 10, 20, 30]);
}

#[test]
fn test_53() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9]), build_list(&[2, 4, 6, 8, 10, 12, 14]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14]);
}

#[test]
fn test_54() {
    let result = Solution::merge_two_lists(build_list(&[100, 101, 102, 103]), build_list(&[104, 105, 106, 107]));
    assert_eq!(list_to_vec(&result), vec![100, 101, 102, 103, 104, 105, 106, 107]);
}

#[test]
fn test_55() {
    let result = Solution::merge_two_lists(build_list(&[-100, -50, -10]), build_list(&[-200, -150, -100, -50, -10]));
    assert_eq!(list_to_vec(&result), vec![-200, -150, -100, -100, -50, -50, -10, -10]);
}

#[test]
fn test_56() {
    let result = Solution::merge_two_lists(build_list(&[1]), build_list(&[1, 2, 3, 4, 5]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 3, 4, 5]);
}

#[test]
fn test_57() {
    let result = Solution::merge_two_lists(build_list(&[10, 20, 30]), build_list(&[5, 15, 25, 35, 45, 55, 65]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35, 45, 55, 65]);
}

#[test]
fn test_58() {
    let result = Solution::merge_two_lists(build_list(&[100, 90, 80, 70, 60]), build_list(&[50, 40, 30, 20, 10]));
    assert_eq!(list_to_vec(&result), vec![50, 40, 30, 20, 10, 100, 90, 80, 70, 60]);
}

#[test]
fn test_59() {
    let result = Solution::merge_two_lists(build_list(&[10]), build_list(&[5, 15, 25, 35, 45]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 25, 35, 45]);
}

#[test]
fn test_60() {
    let result = Solution::merge_two_lists(build_list(&[100, 200, 300]), build_list(&[50, 150, 250, 350]));
    assert_eq!(list_to_vec(&result), vec![50, 100, 150, 200, 250, 300, 350]);
}

#[test]
fn test_61() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), build_list(&[5]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_62() {
    let result = Solution::merge_two_lists(build_list(&[-99, -50, 0, 25, 75, 100]), build_list(&[-100, -75, -25, -1, 50, 100]));
    assert_eq!(list_to_vec(&result), vec![-100, -99, -75, -50, -25, -1, 0, 25, 50, 75, 100, 100]);
}

#[test]
fn test_63() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 24, 26, 28, 30]);
}

#[test]
fn test_64() {
    let result = Solution::merge_two_lists(build_list(&[2, 4, 6, 8, 10]), build_list(&[1, 3, 5, 7, 9]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_65() {
    let result = Solution::merge_two_lists(build_list(&[100]), build_list(&[-100, -50, 0, 50, 75]));
    assert_eq!(list_to_vec(&result), vec![-100, -50, 0, 50, 75, 100]);
}

#[test]
fn test_66() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3]), build_list(&[4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_67() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5]), build_list(&[]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_68() {
    let result = Solution::merge_two_lists(build_list(&[-1, 0, 1, 2, 3]), build_list(&[-3, -2, -1, 0, 1]));
    assert_eq!(list_to_vec(&result), vec![-3, -2, -1, -1, 0, 0, 1, 1, 2, 3]);
}

#[test]
fn test_69() {
    let result = Solution::merge_two_lists(build_list(&[10, 20, 30, 40, 50]), build_list(&[5, 15, 25, 35, 45]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]);
}

#[test]
fn test_70() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9]), build_list(&[2, 4, 6, 8, 10]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_71() {
    let result = Solution::merge_two_lists(build_list(&[-1, -3, -5, -7, -9]), build_list(&[-2, -4, -6, -8, -10]));
    assert_eq!(list_to_vec(&result), vec![-2, -4, -6, -8, -10, -1, -3, -5, -7, -9]);
}

#[test]
fn test_72() {
    let result = Solution::merge_two_lists(build_list(&[10, 20, 30]), build_list(&[5, 15, 25, 35, 45]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35, 45]);
}

#[test]
fn test_73() {
    let result = Solution::merge_two_lists(build_list(&[-1, 0, 1]), build_list(&[-2, -1, 0, 1, 2]));
    assert_eq!(list_to_vec(&result), vec![-2, -1, -1, 0, 0, 1, 1, 2]);
}

#[test]
fn test_74() {
    let result = Solution::merge_two_lists(build_list(&[50]), build_list(&[50, 50, 50, 50, 50]));
    assert_eq!(list_to_vec(&result), vec![50, 50, 50, 50, 50, 50]);
}

#[test]
fn test_75() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3]), build_list(&[1, 2, 3, 4, 5]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 2, 3, 3, 4, 5]);
}

#[test]
fn test_76() {
    let result = Solution::merge_two_lists(build_list(&[50]), build_list(&[-50, 0, 50, 100]));
    assert_eq!(list_to_vec(&result), vec![-50, 0, 50, 50, 100]);
}

#[test]
fn test_77() {
    let result = Solution::merge_two_lists(build_list(&[-5, -3, -1]), build_list(&[-4, -2, 0, 2, 4]));
    assert_eq!(list_to_vec(&result), vec![-5, -4, -3, -2, -1, 0, 2, 4]);
}

#[test]
fn test_78() {
    let result = Solution::merge_two_lists(build_list(&[5, 10, 15, 20]), build_list(&[25, 30, 35, 40, 45, 50]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]);
}

#[test]
fn test_79() {
    let result = Solution::merge_two_lists(build_list(&[-99, -50, 0, 25, 75]), build_list(&[-100, -25, -1, 50, 100]));
    assert_eq!(list_to_vec(&result), vec![-100, -99, -50, -25, -1, 0, 25, 50, 75, 100]);
}

#[test]
fn test_80() {
    let result = Solution::merge_two_lists(build_list(&[-10, -5, 0, 5, 10]), build_list(&[-20, -15, -10, 0, 15, 20]));
    assert_eq!(list_to_vec(&result), vec![-20, -15, -10, -10, -5, 0, 0, 5, 10, 15, 20]);
}

#[test]
fn test_81() {
    let result = Solution::merge_two_lists(build_list(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), build_list(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20]);
}

#[test]
fn test_82() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_83() {
    let result = Solution::merge_two_lists(build_list(&[0, 0, 0, 0, 0]), build_list(&[0, 0, 0, 0, 0]));
    assert_eq!(list_to_vec(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_84() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9]), build_list(&[2, 4, 6, 8, 10, 11, 12]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
}

#[test]
fn test_85() {
    let result = Solution::merge_two_lists(build_list(&[-50, -40, -30, -20, -10]), build_list(&[-60, -55, -45, -35, -25]));
    assert_eq!(list_to_vec(&result), vec![-60, -55, -50, -45, -40, -35, -30, -25, -20, -10]);
}

#[test]
fn test_86() {
    let result = Solution::merge_two_lists(build_list(&[-3, -2, -1, 0, 1, 2, 3]), build_list(&[-4, -3, -2, -1, 0, 1, 2, 3, 4]));
    assert_eq!(list_to_vec(&result), vec![-4, -3, -3, -2, -2, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3, 4]);
}

#[test]
fn test_87() {
    let result = Solution::merge_two_lists(build_list(&[0, 0, 0, 0]), build_list(&[-1, -2, -3, -4]));
    assert_eq!(list_to_vec(&result), vec![-1, -2, -3, -4, 0, 0, 0, 0]);
}

#[test]
fn test_88() {
    let result = Solution::merge_two_lists(build_list(&[-10, -5, 0, 5, 10]), build_list(&[-20, -15, -10, -5, 0, 5, 10]));
    assert_eq!(list_to_vec(&result), vec![-20, -15, -10, -10, -5, -5, 0, 0, 5, 5, 10, 10]);
}

#[test]
fn test_89() {
    let result = Solution::merge_two_lists(build_list(&[-10, -5, 0, 5, 10]), build_list(&[-15, -10, -5, 0, 5]));
    assert_eq!(list_to_vec(&result), vec![-15, -10, -10, -5, -5, 0, 0, 5, 5, 10]);
}

#[test]
fn test_90() {
    let result = Solution::merge_two_lists(build_list(&[]), build_list(&[-50, -25, 0, 25, 50, 75]));
    assert_eq!(list_to_vec(&result), vec![-50, -25, 0, 25, 50, 75]);
}

#[test]
fn test_91() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5]), build_list(&[5, 6, 7, 8, 9]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 9]);
}

#[test]
fn test_92() {
    let result = Solution::merge_two_lists(build_list(&[100, 200, 300]), build_list(&[50, 150, 250, 350]));
    assert_eq!(list_to_vec(&result), vec![50, 100, 150, 200, 250, 300, 350]);
}

#[test]
fn test_93() {
    let result = Solution::merge_two_lists(build_list(&[10, 20, 30, 40, 50]), build_list(&[5, 15, 25, 35, 45]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]);
}

#[test]
fn test_94() {
    let result = Solution::merge_two_lists(build_list(&[-1, 0, 3, 10, 20, 30, 40, 50]), build_list(&[1, 2, 4, 5, 8, 13, 21, 25, 35, 45, 55]));
    assert_eq!(list_to_vec(&result), vec![-1, 0, 1, 2, 3, 4, 5, 8, 10, 13, 20, 21, 25, 30, 35, 40, 45, 50, 55]);
}

#[test]
fn test_95() {
    let result = Solution::merge_two_lists(build_list(&[-20, -10, 0, 10, 20]), build_list(&[-15, -5, 5, 15, 25]));
    assert_eq!(list_to_vec(&result), vec![-20, -15, -10, -5, 0, 5, 10, 15, 20, 25]);
}

#[test]
fn test_96() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9, 11]), build_list(&[2, 4, 6, 8, 10, 12]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
}

#[test]
fn test_97() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49]), build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50]);
}

#[test]
fn test_98() {
    let result = Solution::merge_two_lists(build_list(&[10, 20, 30, 40, 50]), build_list(&[5, 15, 25, 35, 45, 55]));
    assert_eq!(list_to_vec(&result), vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]);
}

#[test]
fn test_99() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(list_to_vec(&result), vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10]);
}

#[test]
fn test_100() {
    let result = Solution::merge_two_lists(build_list(&[1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), build_list(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_101() {
    let result = Solution::merge_two_lists(build_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), build_list(&[11, 12, 13, 14, 15, 16, 17, 18, 19, 20]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}

#[test]
fn test_102() {
    let result = Solution::merge_two_lists(build_list(&[-99, -98, -97, -96, -95]), build_list(&[-94, -93, -92, -91, -90]));
    assert_eq!(list_to_vec(&result), vec![-99, -98, -97, -96, -95, -94, -93, -92, -91, -90]);
}

#[test]
fn test_103() {
    let result = Solution::merge_two_lists(build_list(&[10, 20, 30, 40, 50]), build_list(&[25, 35, 45, 55, 65]));
    assert_eq!(list_to_vec(&result), vec![10, 20, 25, 30, 35, 40, 45, 50, 55, 65]);
}

#[test]
fn test_104() {
    let result = Solution::merge_two_lists(build_list(&[100]), build_list(&[-100]));
    assert_eq!(list_to_vec(&result), vec![-100, 100]);
}

#[test]
fn test_105() {
    let result = Solution::merge_two_lists(build_list(&[1, 4, 7, 10]), build_list(&[2, 5, 8, 11]));
    assert_eq!(list_to_vec(&result), vec![1, 2, 4, 5, 7, 8, 10, 11]);
}

#[test]
fn test_106() {
    let result = Solution::merge_two_lists(build_list(&[-1, 0, 1]), build_list(&[-2, -3, -4, -5, -6, -7, -8, -9, -10]));
    assert_eq!(list_to_vec(&result), vec![-2, -3, -4, -5, -6, -7, -8, -9, -10, -1, 0, 1]);
}

#[test]
fn test_107() {
    let result = Solution::merge_two_lists(build_list(&[50, 60, 70, 80, 90]), build_list(&[40, 55, 65, 75, 85]));
    assert_eq!(list_to_vec(&result), vec![40, 50, 55, 60, 65, 70, 75, 80, 85, 90]);
}

#[test]
fn test_108() {
    let result = Solution::merge_two_lists(build_list(&[5, 10, 15, 20, 25]), build_list(&[3, 8, 13, 18, 23]));
    assert_eq!(list_to_vec(&result), vec![3, 5, 8, 10, 13, 15, 18, 20, 23, 25]);
}

#[test]
fn test_109() {
    let result = Solution::merge_two_lists(build_list(&[-1, -2, -3, -4, -5]), build_list(&[-1, -2, -3, -4, -5]));
    assert_eq!(list_to_vec(&result), vec![-1, -2, -3, -4, -5, -1, -2, -3, -4, -5]);
}

#[test]
fn test_110() {
    let result = Solution::merge_two_lists(build_list(&[-50, -40, -30, -20, -10]), build_list(&[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]));
    assert_eq!(list_to_vec(&result), vec![-50, -40, -30, -20, -10, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);
}

#[test]
fn test_111() {
    let result = Solution::merge_two_lists(build_list(&[-10, -20, -30, -40, -50]), build_list(&[-5, -15, -25, -35, -45]));
    assert_eq!(list_to_vec(&result), vec![-10, -20, -30, -40, -50, -5, -15, -25, -35, -45]);
}
